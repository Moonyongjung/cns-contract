#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{AccountResponse, DomainResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{DomainMappingState, Config, CONFIGSTATE, DOMAINSTATE, ACCOUNTSTATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cns";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {  
    let mapping_address = deps.api.addr_validate(&msg.owner).unwrap();
    let config_state = Config {
        owner: info.sender.clone(),
    };

    let domain_mapping_state = DomainMappingState {
        domain: "contractowner".to_string(),
        account_address: mapping_address,
    };    

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    DOMAINSTATE.save(deps.storage, "contractowner".to_string(), &domain_mapping_state)?;
    CONFIGSTATE.save(deps.storage, &config_state)?;

    Ok(Response::new())        
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIGSTATE.load(deps.storage)?;
    if config.owner != info.sender{
        return Err(ContractError::CustomError { val: ("No contract owner".to_string()) });
    }
    match msg {
        ExecuteMsg::SaveDomainAddressMapping {
            domain,
            account_address
        } => save_domain_address_mapping(deps, domain, account_address),   
    }
}

pub fn save_domain_address_mapping(
    deps: DepsMut,
    domain: String,
    account_address: String
) -> Result<Response, ContractError> {    
    let mapping_address = deps.api.addr_validate(&account_address).unwrap();    
    
    let domain_mapping_state = DomainMappingState {
        domain: domain.clone(),
        account_address: mapping_address.clone(),
    };    

    DOMAINSTATE.save(deps.storage, domain.clone(), &domain_mapping_state)?;
    ACCOUNTSTATE.save(deps.storage, account_address.clone(), &domain_mapping_state)?;

    Ok(Response::new().add_attribute("method", "save_domain_address_mapping"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps,
            _env: Env,
            msg: QueryMsg
        ) -> StdResult<Binary> {
        match msg {
            QueryMsg::DomainMapping {
                domain,
            } => to_binary(&query_account_by_domain(domain, _env, deps)?),

            QueryMsg::AccountMapping {
                account_address,
            } => to_binary(&query_domain_by_account(account_address, _env, deps)?)
        }
}

fn query_account_by_domain(
    domain: String,
    _env: Env,
    deps: Deps,
) -> StdResult<AccountResponse> {
    let domain_mapping_state = DOMAINSTATE.load(deps.storage, domain.clone())?;
    Ok(AccountResponse { account: domain_mapping_state.account_address})
}

fn query_domain_by_account(
    account_address: String,
    _env: Env,
    deps: Deps,
) -> StdResult<DomainResponse> {
    let domain_mapping_state = ACCOUNTSTATE.load(deps.storage, account_address.clone())?;
    Ok(DomainResponse { domain: domain_mapping_state.domain})
}