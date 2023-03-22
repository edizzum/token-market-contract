#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, to_binary};
use cw2::set_contract_version;
use cw20::{BalanceResponse};
use cw20_base::state::BALANCES;
use cw20_base::contract::{
    execute_send, execute_mint,
};

use crate::error::{ContractError};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, Config, CONFIG};


const CONTRACT_NAME: &str = "crates.io:edo-sale-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const ADMIN: &str = "juno158rzc9qglayywqhmccnaqa2yuagxd8mllpzu6f";
const JUNO: &str = "juno";


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config { admin: Addr::unchecked(ADMIN) };
    CONFIG.save(deps.storage, &config)?;
    let owner = msg.owner.unwrap_or_else(|| info.sender.to_string());
    let validated_owner = deps.api.addr_validate(&owner)?;
    let state = State {
        owner: validated_owner.clone(),
        denom: String::from(JUNO),
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("admin", ADMIN))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Send {
            contract, 
            amount, 
            msg,
        } => Ok(execute_send(deps, env, info, contract, amount, msg)?),
        ExecuteMsg::Mint {
            recipient,
            amount,
        } => Ok(execute_mint(deps, env, info, recipient, amount)?)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BalanceOfContract { address } => query_get_balance(deps, env, address),
    }?;
    Ok(Binary::default())
}

pub fn query_get_balance(deps: Deps, _env: Env, address: String) -> StdResult<Binary> {
    let newaddress = Addr::unchecked(address);
    let balance = BALANCES.may_load(deps.storage, &newaddress)?.unwrap_or_default();
    to_binary(&BalanceResponse { balance })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn init_test() {
        pub const ADDR1: &str = "addr1";
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { owner: Some(ADDR1.to_string()) };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn send_test() {
        pub const ADDR1: &str = "addr1";
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { owner: Some(ADDR1.to_string()) };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
    }

}
