use cosmwasm_std::{log, to_binary, Api, Binary, Extern, Querier, StdResult, Storage};

use crate::msg::{ConfigResponse, QueryMsg};
use crate::state::get_config;

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => {
            let config = get_config(&deps.storage)?;
            let out: Binary = to_binary(&ConfigResponse {
                name: config.name,
                total_supply: config.total_supply,
                decimals: config.decimals,
                symbol: config.symbol,
                intital_exchange_rate: config.intital_exchange_rate,
            })?;
            Ok(out)
        }
    }
}
