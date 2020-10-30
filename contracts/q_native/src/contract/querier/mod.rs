use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, Querier, StdResult, Storage, Uint128};

use crate::msg::{ConfigResponse, QueryMsg, BalanceResponse, AllowanceResponse};
use crate::state::{get_allowance, get_balance, get_config};

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => {
            let config = get_config(&deps.storage)?;
            let out = to_binary(&ConfigResponse {
                name: config.name,
                total_supply: Uint128::from(config.total_supply),
                decimals: config.decimals,
                symbol: config.symbol,
                denom: config.denom,
                intital_exchange_rate: Uint128::from(config.initial_exchange_rate),
                reserve_factor: Uint128::from(config.reserve_factor),
                borrow_index: Uint128::from(config.borrow_index)
            })?;
            Ok(out)
        }
        QueryMsg::Balance { address } => {
            let address_key = deps.api.canonical_address(&address)?;
            let balance = get_balance(&deps.storage, &address_key)?;
            let out = to_binary(&BalanceResponse {
                balance: Uint128::from(balance),
            })?;
            Ok(out)
        }
        QueryMsg::Allowance { owner, spender } => {
            let owner_key = deps.api.canonical_address(&owner)?;
            let spender_key = deps.api.canonical_address(&spender)?;
            let allowance = get_allowance(&deps.storage, &owner_key, &spender_key)?;
            let out = to_binary(&AllowanceResponse {
                allowance: Uint128::from(allowance),
            })?;
            Ok(out)
        }
    }
}
