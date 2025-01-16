use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Addr, to_json_binary, StdError, Order,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

// Enums y estructuras principales
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PartType {
    Engine,
    Transmission,
    Wheels,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PartStats {
    pub part_type: PartType,
    pub stat1: u8,
    pub stat2: u8,
    pub stat3: u8,
    pub image_uri: String,
}

// Estado del contrato
pub struct CarPartContract<'a> {
    pub part_stats: Map<'a, u64, PartStats>,
    pub equipped_in_car: Map<'a, u64, u64>, // part_id => car_id (0 si no está equipado)
    pub current_part_id: Item<'a, u64>,
    pub car_contract: Item<'a, Addr>,
    pub owner_parts: Map<'a, String, Vec<u64>>, // owner => part_ids
    pub owner_parts_by_type: Map<'a, (String, String), Vec<u64>>, // (owner, part_type) => part_ids
    pub owner_equipped_parts: Map<'a, String, Vec<u64>>, // owner => equipped_part_ids
    pub owner_unequipped_parts: Map<'a, String, Vec<u64>>, // owner => unequipped_part_ids
}

impl<'a> Default for CarPartContract<'a> {
    fn default() -> Self {
        Self {
            part_stats: Map::new("part_stats"),
            equipped_in_car: Map::new("equipped_in_car"),
            current_part_id: Item::new("current_part_id"),
            car_contract: Item::new("car_contract"),
            owner_parts: Map::new("owner_parts"),
            owner_parts_by_type: Map::new("owner_parts_by_type"),
            owner_equipped_parts: Map::new("owner_equipped_parts"),
            owner_unequipped_parts: Map::new("owner_unequipped_parts"),
        }
    }
}

// Mensajes de inicialización
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
}

// Mensajes de ejecución
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetCarContract {
        address: String,
    },
    Mint {
        to: String,
        part_type: PartType,
        stat1: u8,
        stat2: u8,
        stat3: u8,
        image_uri: String,
        car_id: u64,
    },
    SetEquippedState {
        part_id: u64,
        car_id: u64,
    },
    TransferPart {
        from: String,
        to: String,
        part_id: u64,
    },
}

// Mensajes de consulta
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPartStats {
        part_id: u64,
    },
    GetPartType {
        part_id: u64,
    },
    IsEquipped {
        part_id: u64,
    },
    GetEquippedCar {
        part_id: u64,
    },
    GetOwnerParts {
        owner: String,
    },
    GetOwnerPartsByType {
        owner: String,
        part_type: PartType,
    },
    GetOwnerEquippedParts {
        owner: String,
    },
    GetOwnerUnequippedParts {
        owner: String,
    },
}

// Entry points
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let contract = CarPartContract::default();
    
    // Inicializar el ID actual de la parte
    contract.current_part_id.save(deps.storage, &0u64)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let contract = CarPartContract::default();
    
    match msg {
        ExecuteMsg::SetCarContract { address } => {
            execute_set_car_contract(deps, info, contract, address)
        },
        ExecuteMsg::Mint { to, part_type, stat1, stat2, stat3, image_uri, car_id } => {
            execute_mint(deps, env, info, contract, to, part_type, stat1, stat2, stat3, image_uri, car_id)
        },
        ExecuteMsg::SetEquippedState { part_id, car_id } => {
            execute_set_equipped_state(deps, info, contract, part_id, car_id)
        },
        ExecuteMsg::TransferPart { from, to, part_id } => {
            execute_transfer_part(deps, info, contract, from, to, part_id)
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    let contract = CarPartContract::default();
    
    match msg {
        QueryMsg::GetPartStats { part_id } => {
            to_json_binary(&query_part_stats(deps, contract, part_id)?)
        },
        QueryMsg::GetPartType { part_id } => {
            to_json_binary(&query_part_type(deps, contract, part_id)?)
        },
        QueryMsg::IsEquipped { part_id } => {
            to_json_binary(&query_is_equipped(deps, contract, part_id)?)
        },
        QueryMsg::GetEquippedCar { part_id } => {
            to_json_binary(&query_equipped_car(deps, contract, part_id)?)
        },
        QueryMsg::GetOwnerParts { owner } => {
            to_json_binary(&query_owner_parts(deps, contract, owner)?)
        },
        QueryMsg::GetOwnerPartsByType { owner, part_type } => {
            to_json_binary(&query_owner_parts_by_type(deps, contract, owner, part_type)?)
        },
        QueryMsg::GetOwnerEquippedParts { owner } => {
            to_json_binary(&query_owner_equipped_parts(deps, contract, owner)?)
        },
        QueryMsg::GetOwnerUnequippedParts { owner } => {
            to_json_binary(&query_owner_unequipped_parts(deps, contract, owner)?)
        },
    }
} 

// Funciones de ejecución
fn execute_set_car_contract(
    deps: DepsMut,
    info: MessageInfo,
    contract: CarPartContract,
    address: String,
) -> StdResult<Response> {
    // Validar y guardar la dirección del contrato de carros
    let car_contract_addr = deps.api.addr_validate(&address)?;
    contract.car_contract.save(deps.storage, &car_contract_addr)?;
    
    Ok(Response::new()
        .add_attribute("method", "set_car_contract")
        .add_attribute("car_contract", address))
}

fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: CarPartContract,
    to: String,
    part_type: PartType,
    stat1: u8,
    stat2: u8,
    stat3: u8,
    image_uri: String,
    car_id: u64,
) -> StdResult<Response> {
    // Validar que solo el contrato de carros pueda mintear
    let car_contract = contract.car_contract.load(deps.storage)?;
    if info.sender != car_contract {
        return Err(StdError::generic_err("Solo el contrato de carros puede mintear partes"));
    }

    // Validar stats
    if stat1 > 10 || stat2 > 10 || stat3 > 10 {
        return Err(StdError::generic_err("Los stats deben ser <= 10"));
    }

    // Obtener y actualizar el ID de la parte
    let part_id = contract.current_part_id.load(deps.storage)?;
    contract.current_part_id.save(deps.storage, &(part_id + 1))?;

    // Guardar los stats de la parte
    let part_stats = PartStats {
        part_type: part_type.clone(),
        stat1,
        stat2,
        stat3,
        image_uri: image_uri.clone(),
    };
    contract.part_stats.save(deps.storage, part_id, &part_stats)?;

    // Marcar la parte como equipada en el carro
    contract.equipped_in_car.save(deps.storage, part_id, &car_id)?;

    // Actualizar los mapeos del propietario
    let mut owner_parts = contract.owner_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    owner_parts.push(part_id);
    contract.owner_parts.save(deps.storage, to.clone(), &owner_parts)?;

    let part_type_str = format!("{:?}", part_type);
    let mut owner_parts_by_type = contract.owner_parts_by_type
        .may_load(deps.storage, (to.clone(), part_type_str.clone()))?
        .unwrap_or_default();
    owner_parts_by_type.push(part_id);
    contract.owner_parts_by_type.save(deps.storage, (to.clone(), part_type_str), &owner_parts_by_type)?;

    let mut owner_equipped_parts = contract.owner_equipped_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    owner_equipped_parts.push(part_id);
    contract.owner_equipped_parts.save(deps.storage, to.clone(), &owner_equipped_parts)?;

    Ok(Response::new()
        .add_attribute("method", "mint")
        .add_attribute("to", to)
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("car_id", car_id.to_string()))
}

fn execute_set_equipped_state(
    deps: DepsMut,
    info: MessageInfo,
    contract: CarPartContract,
    part_id: u64,
    car_id: u64,
) -> StdResult<Response> {
    // Validar que solo el contrato de carros pueda cambiar el estado
    let car_contract = contract.car_contract.load(deps.storage)?;
    if info.sender != car_contract {
        return Err(StdError::generic_err("Solo el contrato de carros puede cambiar el estado de equipamiento"));
    }

    // Verificar que la parte existe
    if !contract.part_stats.has(deps.storage, part_id) {
        return Err(StdError::generic_err("La parte no existe"));
    }

    // Obtener el estado actual
    let current_car_id = contract.equipped_in_car.load(deps.storage, part_id)?;
    let owner = get_part_owner(deps.as_ref(), &contract, part_id)?;

    // Actualizar el estado de equipamiento
    contract.equipped_in_car.save(deps.storage, part_id, &car_id)?;

    // Actualizar los mapeos del propietario
    if current_car_id == 0 && car_id != 0 {
        // La parte está siendo equipada
        let mut unequipped_parts = contract.owner_unequipped_parts
            .may_load(deps.storage, owner.clone())?
            .unwrap_or_default();
        if let Some(pos) = unequipped_parts.iter().position(|&x| x == part_id) {
            unequipped_parts.remove(pos);
        }
        contract.owner_unequipped_parts.save(deps.storage, owner.clone(), &unequipped_parts)?;

        let mut equipped_parts = contract.owner_equipped_parts
            .may_load(deps.storage, owner.clone())?
            .unwrap_or_default();
        equipped_parts.push(part_id);
        contract.owner_equipped_parts.save(deps.storage, owner, &equipped_parts)?;
    } else if current_car_id != 0 && car_id == 0 {
        // La parte está siendo desequipada
        let mut equipped_parts = contract.owner_equipped_parts
            .may_load(deps.storage, owner.clone())?
            .unwrap_or_default();
        if let Some(pos) = equipped_parts.iter().position(|&x| x == part_id) {
            equipped_parts.remove(pos);
        }
        contract.owner_equipped_parts.save(deps.storage, owner.clone(), &equipped_parts)?;

        let mut unequipped_parts = contract.owner_unequipped_parts
            .may_load(deps.storage, owner.clone())?
            .unwrap_or_default();
        unequipped_parts.push(part_id);
        contract.owner_unequipped_parts.save(deps.storage, owner, &unequipped_parts)?;
    }

    Ok(Response::new()
        .add_attribute("method", "set_equipped_state")
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("car_id", car_id.to_string()))
}

fn execute_transfer_part(
    deps: DepsMut,
    _info: MessageInfo,
    contract: CarPartContract,
    from: String,
    to: String,
    part_id: u64,
) -> StdResult<Response> {
    // Verificar que la parte existe
    if !contract.part_stats.has(deps.storage, part_id) {
        return Err(StdError::generic_err("La parte no existe"));
    }

    // Verificar que el remitente es el propietario
    let current_owner = get_part_owner(deps.as_ref(), &contract, part_id)?;
    if current_owner != from {
        return Err(StdError::generic_err("No eres el dueño de esta parte"));
    }

    // Verificar que la parte no esté equipada
    let equipped_car = contract.equipped_in_car.load(deps.storage, part_id)?;
    if equipped_car != 0 {
        return Err(StdError::generic_err("No puedes transferir una parte equipada"));
    }

    // Obtener los datos de la parte
    let part_stats = contract.part_stats.load(deps.storage, part_id)?;
    let part_type_str = format!("{:?}", part_stats.part_type);

    // Actualizar los mapeos del propietario anterior
    let mut from_parts = contract.owner_parts
        .may_load(deps.storage, from.clone())?
        .unwrap_or_default();
    if let Some(pos) = from_parts.iter().position(|&x| x == part_id) {
        from_parts.remove(pos);
    }
    contract.owner_parts.save(deps.storage, from.clone(), &from_parts)?;

    let mut from_parts_by_type = contract.owner_parts_by_type
        .may_load(deps.storage, (from.clone(), part_type_str.clone()))?
        .unwrap_or_default();
    if let Some(pos) = from_parts_by_type.iter().position(|&x| x == part_id) {
        from_parts_by_type.remove(pos);
    }
    contract.owner_parts_by_type.save(deps.storage, (from.clone(), part_type_str.clone()), &from_parts_by_type)?;

    let mut from_unequipped_parts = contract.owner_unequipped_parts
        .may_load(deps.storage, from.clone())?
        .unwrap_or_default();
    if let Some(pos) = from_unequipped_parts.iter().position(|&x| x == part_id) {
        from_unequipped_parts.remove(pos);
    }
    contract.owner_unequipped_parts.save(deps.storage, from.clone(), &from_unequipped_parts)?;

    // Actualizar los mapeos del nuevo propietario
    let mut to_parts = contract.owner_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    to_parts.push(part_id);
    contract.owner_parts.save(deps.storage, to.clone(), &to_parts)?;

    let mut to_parts_by_type = contract.owner_parts_by_type
        .may_load(deps.storage, (to.clone(), part_type_str.clone()))?
        .unwrap_or_default();
    to_parts_by_type.push(part_id);
    contract.owner_parts_by_type.save(deps.storage, (to.clone(), part_type_str), &to_parts_by_type)?;

    let mut to_unequipped_parts = contract.owner_unequipped_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    to_unequipped_parts.push(part_id);
    contract.owner_unequipped_parts.save(deps.storage, to.clone(), &to_unequipped_parts)?;

    Ok(Response::new()
        .add_attribute("method", "transfer_part")
        .add_attribute("from", from)
        .add_attribute("to", to)
        .add_attribute("part_id", part_id.to_string()))
}

// Funciones auxiliares
fn get_part_owner(deps: Deps, contract: &CarPartContract, part_id: u64) -> StdResult<String> {
    // Buscar el propietario iterando sobre los mapeos
    let range_result = contract.owner_parts.range(deps.storage, None, None, Order::Ascending);
    for result in range_result {
        let (owner, parts) = result?;
        if parts.contains(&part_id) {
            return Ok(owner);
        }
    }
    Err(StdError::generic_err("No se encontró el propietario de la parte"))
}

// Funciones de consulta
fn query_part_stats(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<PartStats> {
    contract.part_stats.load(deps.storage, part_id)
}

fn query_part_type(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<PartType> {
    let stats = contract.part_stats.load(deps.storage, part_id)?;
    Ok(stats.part_type)
}

fn query_is_equipped(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<bool> {
    let car_id = contract.equipped_in_car.load(deps.storage, part_id)?;
    Ok(car_id != 0)
}

fn query_equipped_car(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<u64> {
    contract.equipped_in_car.load(deps.storage, part_id)
}

fn query_owner_parts(deps: Deps, contract: CarPartContract, owner: String) -> StdResult<Vec<u64>> {
    Ok(contract.owner_parts.may_load(deps.storage, owner)?.unwrap_or_default())
}

fn query_owner_parts_by_type(
    deps: Deps,
    contract: CarPartContract,
    owner: String,
    part_type: PartType,
) -> StdResult<Vec<u64>> {
    let part_type_str = format!("{:?}", part_type);
    Ok(contract.owner_parts_by_type.may_load(deps.storage, (owner, part_type_str))?.unwrap_or_default())
}

fn query_owner_equipped_parts(deps: Deps, contract: CarPartContract, owner: String) -> StdResult<Vec<u64>> {
    Ok(contract.owner_equipped_parts.may_load(deps.storage, owner)?.unwrap_or_default())
}

fn query_owner_unequipped_parts(deps: Deps, contract: CarPartContract, owner: String) -> StdResult<Vec<u64>> {
    Ok(contract.owner_unequipped_parts.may_load(deps.storage, owner)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            owner: "owner".to_string(),
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(2, res.attributes.len());
    }
} 