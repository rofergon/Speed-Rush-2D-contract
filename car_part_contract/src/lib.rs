use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Addr, to_json_binary, StdError,
};
use car_types::{PartType, PartStats};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

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
    pub car_contract: String,
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
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let contract = CarPartContract::default();
    
    // Guardar la dirección del contrato de carros
    let car_addr = deps.api.addr_validate(&msg.car_contract)?;
    contract.car_contract.save(deps.storage, &car_addr)?;
    
    // Inicializar el ID actual de parte
    contract.current_part_id.save(deps.storage, &0u64)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("car_contract", msg.car_contract))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let contract = CarPartContract::default();
    
    match msg {
        ExecuteMsg::SetCarContract { address } => {
            execute_set_car_contract(deps, info, contract, address)
        },
        ExecuteMsg::Mint { to, part_type, stat1, stat2, stat3, image_uri, car_id } => {
            execute_mint(deps, _env, info, contract, to, part_type, stat1, stat2, stat3, image_uri, car_id)
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
    _info: MessageInfo,
    contract: CarPartContract,
    address: String,
) -> StdResult<Response> {
    // TODO: Implementar verificación de propietario
    let car_addr = deps.api.addr_validate(&address)?;
    contract.car_contract.save(deps.storage, &car_addr)?;
    
    Ok(Response::new()
        .add_attribute("method", "set_car_contract")
        .add_attribute("address", address))
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MintResponse {
    pub token_id: String,
    pub part_id: u64,
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
    // Verificar que el remitente es el contrato de carros
    let car_contract = contract.car_contract.load(deps.storage)?;
    if info.sender != car_contract {
        return Err(StdError::generic_err("Solo el contrato de carros puede mintear partes"));
    }

    // Validar stats
    if stat1 > 10 || stat2 > 10 || stat3 > 10 {
        return Err(StdError::generic_err("Los stats deben ser <= 10"));
    }

    // Obtener el ID actual de parte
    let part_id = contract.current_part_id.load(deps.storage)?;
    
    // Guardar los stats de la parte
    let part_stats = PartStats {
        part_type: part_type.clone(),
        stat1,
        stat2,
        stat3,
        image_uri: image_uri.clone(),
    };
    contract.part_stats.save(deps.storage, part_id, &part_stats)?;

    // Actualizar los mapeos de seguimiento
    deps.api.addr_validate(&to)?;
    
    // Actualizar owner_parts
    let mut owner_parts = contract.owner_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    owner_parts.push(part_id);
    contract.owner_parts.save(deps.storage, to.clone(), &owner_parts)?;
    
    // Actualizar owner_parts_by_type
    let part_type_str = format!("{:?}", part_type);
    let mut owner_parts_by_type = contract.owner_parts_by_type
        .may_load(deps.storage, (to.clone(), part_type_str.clone()))?
        .unwrap_or_default();
    owner_parts_by_type.push(part_id);
    contract.owner_parts_by_type.save(deps.storage, (to.clone(), part_type_str), &owner_parts_by_type)?;
    
    // Actualizar owner_unequipped_parts ya que la parte comienza sin equipar
    let mut owner_unequipped_parts = contract.owner_unequipped_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    owner_unequipped_parts.push(part_id);
    contract.owner_unequipped_parts.save(deps.storage, to.clone(), &owner_unequipped_parts)?;
    
    // Si car_id es diferente de 0, marcar la parte como equipada
    if car_id != 0 {
        contract.equipped_in_car.save(deps.storage, part_id, &car_id)?;
        
        // Mover la parte de unequipped a equipped
        if let Some(pos) = owner_unequipped_parts.iter().position(|&x| x == part_id) {
            owner_unequipped_parts.remove(pos);
            contract.owner_unequipped_parts.save(deps.storage, to.clone(), &owner_unequipped_parts)?;
            
            let mut owner_equipped_parts = contract.owner_equipped_parts
                .may_load(deps.storage, to.clone())?
                .unwrap_or_default();
            owner_equipped_parts.push(part_id);
            contract.owner_equipped_parts.save(deps.storage, to.clone(), &owner_equipped_parts)?;
        }
    }
    
    // Incrementar el ID para la siguiente parte
    contract.current_part_id.save(deps.storage, &(part_id + 1))?;
    
    Ok(Response::new()
        .add_attribute("method", "mint")
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("owner", to.clone())
        .add_attribute("part_type", format!("{:?}", part_type)))
}

fn execute_set_equipped_state(
    deps: DepsMut,
    info: MessageInfo,
    contract: CarPartContract,
    part_id: u64,
    car_id: u64,
) -> StdResult<Response> {
    // Verificar que el remitente es el contrato de carros
    let car_contract = contract.car_contract.load(deps.storage)?;
    if info.sender != car_contract {
        return Err(StdError::generic_err("Solo el contrato de carros puede cambiar el estado de equipamiento"));
    }

    // Actualizar el estado de equipamiento
    if car_id > 0 {
        contract.equipped_in_car.save(deps.storage, part_id, &car_id)?;
    } else {
        contract.equipped_in_car.remove(deps.storage, part_id);
    }

    Ok(Response::new()
        .add_attribute("method", "set_equipped_state")
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("car_id", car_id.to_string()))
}

fn execute_transfer_part(
    deps: DepsMut,
    info: MessageInfo,
    contract: CarPartContract,
    from: String,
    to: String,
    part_id: u64,
) -> StdResult<Response> {
    // Verificar que la parte existe
    let part_stats = contract.part_stats.load(deps.storage, part_id)?;
    
    // Verificar que el remitente es el propietario o el contrato de carros
    let car_contract = contract.car_contract.load(deps.storage)?;
    if info.sender != car_contract {
        return Err(StdError::generic_err("Solo el contrato de carros puede transferir partes"));
    }

    // Verificar que la parte no está equipada
    if contract.equipped_in_car.may_load(deps.storage, part_id)?.unwrap_or(0) > 0 {
        return Err(StdError::generic_err("No se puede transferir una parte equipada"));
    }

    // Actualizar los mapeos de seguimiento
    // Remover de las listas del propietario anterior
    let mut from_parts = contract.owner_parts.load(deps.storage, from.clone())?;
    from_parts.retain(|&x| x != part_id);
    contract.owner_parts.save(deps.storage, from.clone(), &from_parts)?;

    let mut from_parts_by_type = contract.owner_parts_by_type
        .load(deps.storage, (from.clone(), part_stats.part_type.to_string()))?;
    from_parts_by_type.retain(|&x| x != part_id);
    contract.owner_parts_by_type.save(deps.storage, (from.clone(), part_stats.part_type.to_string()), &from_parts_by_type)?;

    let mut from_unequipped_parts = contract.owner_unequipped_parts.load(deps.storage, from.clone())?;
    from_unequipped_parts.retain(|&x| x != part_id);
    contract.owner_unequipped_parts.save(deps.storage, from.clone(), &from_unequipped_parts)?;

    // Agregar a las listas del nuevo propietario
    let mut to_parts = contract.owner_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    to_parts.push(part_id);
    contract.owner_parts.save(deps.storage, to.clone(), &to_parts)?;

    let mut to_parts_by_type = contract.owner_parts_by_type
        .may_load(deps.storage, (to.clone(), part_stats.part_type.to_string()))?
        .unwrap_or_default();
    to_parts_by_type.push(part_id);
    contract.owner_parts_by_type.save(deps.storage, (to.clone(), part_stats.part_type.to_string()), &to_parts_by_type)?;

    let mut to_unequipped_parts = contract.owner_unequipped_parts
        .may_load(deps.storage, to.clone())?
        .unwrap_or_default();
    to_unequipped_parts.push(part_id);
    contract.owner_unequipped_parts.save(deps.storage, to.clone(), &to_unequipped_parts)?;

    Ok(Response::new()
        .add_attribute("method", "transfer_part")
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("from", from)
        .add_attribute("to", to))
}

// Funciones de consulta
fn query_part_stats(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<PartStats> {
    contract.part_stats.load(deps.storage, part_id)
}

fn query_part_type(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<PartType> {
    let part_stats = contract.part_stats.load(deps.storage, part_id)?;
    Ok(part_stats.part_type)
}

fn query_is_equipped(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<bool> {
    Ok(contract.equipped_in_car.may_load(deps.storage, part_id)?.unwrap_or(0) > 0)
}

fn query_equipped_car(deps: Deps, contract: CarPartContract, part_id: u64) -> StdResult<u64> {
    Ok(contract.equipped_in_car.may_load(deps.storage, part_id)?.unwrap_or(0))
}

fn query_owner_parts(deps: Deps, contract: CarPartContract, owner: String) -> StdResult<Vec<u64>> {
    Ok(contract.owner_parts.may_load(deps.storage, owner)?.unwrap_or_default())
}

fn query_owner_parts_by_type(deps: Deps, contract: CarPartContract, owner: String, part_type: PartType) -> StdResult<Vec<u64>> {
    Ok(contract.owner_parts_by_type.may_load(deps.storage, (owner, part_type.to_string()))?.unwrap_or_default())
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
    use cosmwasm_std::from_json;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            car_contract: "car_contract".to_string(),
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(2, res.attributes.len());
    }

    #[test]
    fn test_mint_part() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_contract: "car_contract".to_string(),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Intentar mintear una parte desde una dirección no autorizada
        let unauthorized_info = mock_info("unauthorized", &[]);
        let mint_msg = ExecuteMsg::Mint {
            to: "owner".to_string(),
            part_type: PartType::Engine,
            stat1: 8,
            stat2: 7,
            stat3: 6,
            image_uri: "engine_uri".to_string(),
            car_id: 1,
        };

        let err = execute(deps.as_mut(), env.clone(), unauthorized_info, mint_msg.clone()).unwrap_err();
        assert!(err.to_string().contains("Solo el contrato de carros puede mintear partes"));

        // Mintear una parte desde el contrato de carros
        let car_contract_info = mock_info("car_contract", &[]);
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), mint_msg).unwrap();
        
        // Verificar que se guardaron los stats correctamente
        let query_msg = QueryMsg::GetPartStats { part_id: 0 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let part_stats: PartStats = from_json(&res).unwrap();
        
        assert_eq!(PartType::Engine, part_stats.part_type);
        assert_eq!(8, part_stats.stat1);
        assert_eq!(7, part_stats.stat2);
        assert_eq!(6, part_stats.stat3);
        assert_eq!("engine_uri", part_stats.image_uri);

        // Verificar que la parte está equipada
        let query_msg = QueryMsg::IsEquipped { part_id: 0 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let is_equipped: bool = from_json(&res).unwrap();
        assert!(is_equipped);

        // Verificar que está equipada en el carro correcto
        let query_msg = QueryMsg::GetEquippedCar { part_id: 0 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let equipped_car: u64 = from_json(&res).unwrap();
        assert_eq!(1, equipped_car);

        // Verificar que se agregó a la lista de partes del propietario
        let query_msg = QueryMsg::GetOwnerParts { owner: "owner".to_string() };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let owner_parts: Vec<u64> = from_json(&res).unwrap();
        assert_eq!(vec![0], owner_parts);

        // Verificar que se agregó a la lista de partes por tipo
        let query_msg = QueryMsg::GetOwnerPartsByType { 
            owner: "owner".to_string(),
            part_type: PartType::Engine,
        };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let owner_parts_by_type: Vec<u64> = from_json(&res).unwrap();
        assert_eq!(vec![0], owner_parts_by_type);
    }

    #[test]
    fn test_mint_multiple_parts() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_contract: "car_contract".to_string(),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        let car_contract_info = mock_info("car_contract", &[]);

        // Mintear un motor
        let mint_msg = ExecuteMsg::Mint {
            to: "owner".to_string(),
            part_type: PartType::Engine,
            stat1: 8,
            stat2: 7,
            stat3: 6,
            image_uri: "engine_uri".to_string(),
            car_id: 1,
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), mint_msg).unwrap();

        // Mintear una transmisión
        let mint_msg = ExecuteMsg::Mint {
            to: "owner".to_string(),
            part_type: PartType::Transmission,
            stat1: 5,
            stat2: 6,
            stat3: 7,
            image_uri: "transmission_uri".to_string(),
            car_id: 1,
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), mint_msg).unwrap();

        // Mintear ruedas
        let mint_msg = ExecuteMsg::Mint {
            to: "owner".to_string(),
            part_type: PartType::Wheels,
            stat1: 4,
            stat2: 5,
            stat3: 6,
            image_uri: "wheels_uri".to_string(),
            car_id: 1,
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), mint_msg).unwrap();

        // Verificar que todas las partes se mintearon correctamente
        let query_msg = QueryMsg::GetOwnerParts { owner: "owner".to_string() };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let owner_parts: Vec<u64> = from_json(&res).unwrap();
        assert_eq!(vec![0, 1, 2], owner_parts);

        // Verificar que cada parte tiene los stats correctos
        let query_msg = QueryMsg::GetPartStats { part_id: 0 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let part_stats: PartStats = from_json(&res).unwrap();
        assert_eq!(PartType::Engine, part_stats.part_type);

        let query_msg = QueryMsg::GetPartStats { part_id: 1 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let part_stats: PartStats = from_json(&res).unwrap();
        assert_eq!(PartType::Transmission, part_stats.part_type);

        let query_msg = QueryMsg::GetPartStats { part_id: 2 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let part_stats: PartStats = from_json(&res).unwrap();
        assert_eq!(PartType::Wheels, part_stats.part_type);
    }

    #[test]
    fn test_set_equipped_state() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_contract: "car_contract".to_string(),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Mintear una parte
        let car_contract_info = mock_info("car_contract", &[]);
        let mint_msg = ExecuteMsg::Mint {
            to: "owner".to_string(),
            part_type: PartType::Engine,
            stat1: 8,
            stat2: 7,
            stat3: 6,
            image_uri: "engine_uri".to_string(),
            car_id: 1,
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), mint_msg).unwrap();

        // Verificar que la parte está equipada inicialmente
        let query_msg = QueryMsg::IsEquipped { part_id: 0 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let is_equipped: bool = from_json(&res).unwrap();
        assert!(is_equipped);

        // Desequipar la parte
        let set_equipped_msg = ExecuteMsg::SetEquippedState {
            part_id: 0,
            car_id: 0,
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), set_equipped_msg).unwrap();

        // Verificar que la parte está desequipada
        let query_msg = QueryMsg::IsEquipped { part_id: 0 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let is_equipped: bool = from_json(&res).unwrap();
        assert!(!is_equipped);

        // Equipar la parte en otro carro
        let set_equipped_msg = ExecuteMsg::SetEquippedState {
            part_id: 0,
            car_id: 2,
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), set_equipped_msg).unwrap();

        // Verificar que la parte está equipada en el nuevo carro
        let query_msg = QueryMsg::GetEquippedCar { part_id: 0 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let equipped_car: u64 = from_json(&res).unwrap();
        assert_eq!(2, equipped_car);
    }

    #[test]
    fn test_transfer_part() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_contract: "car_contract".to_string(),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Mintear una parte
        let car_contract_info = mock_info("car_contract", &[]);
        let mint_msg = ExecuteMsg::Mint {
            to: "owner1".to_string(),
            part_type: PartType::Engine,
            stat1: 8,
            stat2: 7,
            stat3: 6,
            image_uri: "engine_uri".to_string(),
            car_id: 0, // No equipada
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), mint_msg).unwrap();

        // Transferir la parte
        let transfer_msg = ExecuteMsg::TransferPart {
            from: "owner1".to_string(),
            to: "owner2".to_string(),
            part_id: 0,
        };
        let _res = execute(deps.as_mut(), env.clone(), car_contract_info.clone(), transfer_msg).unwrap();

        // Verificar que la parte ya no pertenece al propietario original
        let query_msg = QueryMsg::GetOwnerParts { owner: "owner1".to_string() };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let owner_parts: Vec<u64> = from_json(&res).unwrap();
        assert!(owner_parts.is_empty());

        // Verificar que la parte pertenece al nuevo propietario
        let query_msg = QueryMsg::GetOwnerParts { owner: "owner2".to_string() };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let owner_parts: Vec<u64> = from_json(&res).unwrap();
        assert_eq!(vec![0], owner_parts);
    }
} 