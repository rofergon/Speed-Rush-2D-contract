use super::car_part::{ExecuteMsg as CarPartExecuteMsg, PartType};
use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    SubMsg, Uint128, WasmMsg, BankMsg, Coin, Reply,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

// Estructuras principales
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarComposition {
    pub part_ids: Vec<u64>,
    pub car_image_uri: String,
    pub slot_occupied: Vec<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PartData {
    pub part_type: PartType,
    pub stat1: u8,
    pub stat2: u8,
    pub stat3: u8,
    pub image_uri: String,
}

// Estado del contrato
pub struct CarNftContract<'a> {
    pub cars: Map<'a, u64, CarComposition>,
    pub car_conditions: Map<'a, u64, u8>,
    pub workshop_contract: Item<'a, Addr>,
    pub leaderboard_contract: Item<'a, Addr>,
    pub car_part_contract: Item<'a, Addr>,
    pub mint_price: Item<'a, Uint128>,
    pub current_car_id: Item<'a, u64>,
}

impl<'a> Default for CarNftContract<'a> {
    fn default() -> Self {
        Self {
            cars: Map::new("cars"),
            car_conditions: Map::new("car_conditions"),
            workshop_contract: Item::new("workshop_contract"),
            leaderboard_contract: Item::new("leaderboard_contract"),
            car_part_contract: Item::new("car_part_contract"),
            mint_price: Item::new("mint_price"),
            current_car_id: Item::new("current_car_id"),
        }
    }
}

// Mensajes de inicialización
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub car_part_contract: String,
    pub mint_price: Uint128,
}

// Mensajes de ejecución
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintCar {
        car_image_uri: String,
        parts_data: Vec<PartData>,
    },
    UnequipPart {
        car_id: u64,
        part_id: u64,
    },
    EquipPart {
        car_id: u64,
        part_id: u64,
        slot_index: u64,
    },
    ReplacePart {
        car_id: u64,
        old_part_id: u64,
        new_part_id: u64,
    },
    SetWorkshopContract {
        address: String,
    },
    SetLeaderboardContract {
        address: String,
    },
    SetMintPrice {
        price: Uint128,
    },
    WithdrawFunds {},
}

// Mensajes de consulta
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCarComposition {
        car_id: u64,
    },
    GetCompactCarStats {
        car_id: u64,
    },
    GetFullCarMetadata {
        car_id: u64,
    },
    GetLastTokenId {},
    GetMintPrice {},
}

// Respuestas de consulta
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CompactCarStats {
    pub car_id: u64,
    pub condition: u8,
    pub part_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FullCarMetadata {
    pub car_id: u64,
    pub condition: u8,
    pub car_image_uri: String,
    pub part_ids: Vec<u64>,
    pub slot_occupied: Vec<bool>,
}

// Entry points
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let contract = CarNftContract::default();
    
    // Inicializar el precio de minteo
    contract.mint_price.save(deps.storage, &msg.mint_price)?;
    
    // Inicializar el ID actual del coche
    contract.current_car_id.save(deps.storage, &1u64)?;
    
    // Guardar la dirección del contrato de partes
    let car_part_addr = deps.api.addr_validate(&msg.car_part_contract)?;
    contract.car_part_contract.save(deps.storage, &car_part_addr)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("mint_price", msg.mint_price.to_string())
        .add_attribute("car_part_contract", msg.car_part_contract))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let contract = CarNftContract::default();
    
    match msg {
        ExecuteMsg::MintCar { car_image_uri, parts_data } => {
            execute_mint_car(deps, env, info, contract, car_image_uri, parts_data)
        },
        ExecuteMsg::UnequipPart { car_id, part_id } => {
            execute_unequip_part(deps, env, info, contract, car_id, part_id)
        },
        ExecuteMsg::EquipPart { car_id, part_id, slot_index } => {
            execute_equip_part(deps, env, info, contract, car_id, part_id, slot_index)
        },
        ExecuteMsg::ReplacePart { car_id, old_part_id, new_part_id } => {
            execute_replace_part(deps, env, info, contract, car_id, old_part_id, new_part_id)
        },
        ExecuteMsg::SetWorkshopContract { address } => {
            execute_set_workshop_contract(deps, env, info, contract, address)
        },
        ExecuteMsg::SetLeaderboardContract { address } => {
            execute_set_leaderboard_contract(deps, env, info, contract, address)
        },
        ExecuteMsg::SetMintPrice { price } => {
            execute_set_mint_price(deps, env, info, contract, price)
        },
        ExecuteMsg::WithdrawFunds {} => {
            execute_withdraw_funds(deps, env, info, contract)
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    let contract = CarNftContract::default();
    
    match msg {
        QueryMsg::GetCarComposition { car_id } => {
            to_json_binary(&query_car_composition(deps, contract, car_id)?)
        },
        QueryMsg::GetCompactCarStats { car_id } => {
            to_json_binary(&query_compact_car_stats(deps, contract, car_id)?)
        },
        QueryMsg::GetFullCarMetadata { car_id } => {
            to_json_binary(&query_full_car_metadata(deps, contract, car_id)?)
        },
        QueryMsg::GetLastTokenId {} => {
            to_json_binary(&query_last_token_id(deps, contract)?)
        },
        QueryMsg::GetMintPrice {} => {
            let price = contract.mint_price.load(deps.storage)?;
            to_json_binary(&price)
        },
    }
}

// Funciones de ejecución
fn execute_mint_car(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: CarNftContract,
    car_image_uri: String,
    parts_data: Vec<PartData>,
) -> StdResult<Response> {
    // Verificar el pago
    let mint_price = contract.mint_price.load(deps.storage)?;
    let payment = info.funds.iter()
        .find(|coin| coin.denom == "uxion")
        .ok_or_else(|| StdError::generic_err("No se encontró el pago en uxion"))?;
    
    if payment.amount < mint_price {
        return Err(StdError::generic_err("Pago insuficiente"));
    }

    // Verificar la cantidad de partes
    if parts_data.len() > 3 {
        return Err(StdError::generic_err("Demasiadas partes"));
    }

    // Obtener el ID actual del carro
    let car_id = contract.current_car_id.load(deps.storage)?;
    
    // Verificar que tenga todas las partes necesarias
    let mut has_engine = false;
    let mut has_transmission = false;
    let mut has_wheels = false;

    // Preparar los arrays para la composición del carro
    let mut part_ids = vec![0u64; 3];
    let mut slot_occupied = vec![false; 3];

    // Obtener la dirección del contrato de partes
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;

    // Crear un vector para almacenar los mensajes de submensajes
    let mut messages = vec![];

    // Procesar cada parte y crear los NFTs de partes
    for (i, part) in parts_data.iter().enumerate() {
        match part.part_type {
            PartType::Engine => {
                has_engine = true;
                slot_occupied[0] = true;
            },
            PartType::Transmission => {
                has_transmission = true;
                slot_occupied[1] = true;
            },
            PartType::Wheels => {
                has_wheels = true;
                slot_occupied[2] = true;
            },
        }

        // Validar stats
        if part.stat1 > 10 || part.stat2 > 10 || part.stat3 > 10 {
            return Err(StdError::generic_err("Los stats deben ser <= 10"));
        }

        // Crear el mensaje para mintear la parte
        let mint_msg = CarPartExecuteMsg::Mint {
            to: info.sender.to_string(),
            part_type: part.part_type.clone(),
            stat1: part.stat1,
            stat2: part.stat2,
            stat3: part.stat3,
            image_uri: part.image_uri.clone(),
            car_id,
        };

        // Crear el submensaje para mintear la parte y obtener el ID
        messages.push(SubMsg::reply_on_success(WasmMsg::Execute {
            contract_addr: car_part_contract.to_string(),
            msg: to_json_binary(&mint_msg)?,
            funds: vec![],
        }, (i as u64) + 1)); // Usar el índice + 1 como ID del submensaje
    }

    // Verificar que tenga todas las partes necesarias
    if !has_engine || !has_transmission || !has_wheels {
        return Err(StdError::generic_err("Faltan partes necesarias (motor, transmisión o ruedas)"));
    }

    // Guardar la composición del carro
    let car_composition = CarComposition {
        part_ids,
        car_image_uri: car_image_uri.clone(),
        slot_occupied,
    };
    contract.cars.save(deps.storage, car_id, &car_composition)?;

    // Inicializar la condición del carro
    contract.car_conditions.save(deps.storage, car_id, &100u8)?;

    // Incrementar el ID del carro para el siguiente
    contract.current_car_id.save(deps.storage, &(car_id + 1))?;

    Ok(Response::new()
        .add_submessages(messages)
        .add_attribute("method", "mint_car")
        .add_attribute("car_id", car_id.to_string())
        .add_attribute("owner", info.sender))
}

fn execute_unequip_part(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: CarNftContract,
    car_id: u64,
    part_id: u64,
) -> StdResult<Response> {
    // Verificar que el carro existe
    let car = contract.cars.load(deps.storage, car_id)?;
    
    // Verificar que el remitente es el dueño del carro
    // TODO: Implementar verificación de propiedad cuando se implemente CW721
    
    // Verificar que la parte está equipada en el carro
    let mut found = false;
    let mut slot_index = 0;
    for (i, &pid) in car.part_ids.iter().enumerate() {
        if pid == part_id {
            found = true;
            slot_index = i;
            break;
        }
    }
    
    if !found {
        return Err(StdError::generic_err("La parte no está equipada en este carro"));
    }
    
    // Obtener la dirección del contrato de partes
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    
    // Crear el mensaje para actualizar el estado de equipamiento en el contrato de partes
    let set_equipped_msg = CarPartExecuteMsg::SetEquippedState {
        part_id,
        car_id: 0, // 0 indica que la parte no está equipada
    };
    
    // Actualizar el estado del carro
    let mut updated_car = car;
    updated_car.part_ids[slot_index] = 0;
    updated_car.slot_occupied[slot_index] = false;
    contract.cars.save(deps.storage, car_id, &updated_car)?;
    
    Ok(Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: car_part_contract.to_string(),
            msg: to_json_binary(&set_equipped_msg)?,
            funds: vec![],
        })
        .add_attribute("method", "unequip_part")
        .add_attribute("car_id", car_id.to_string())
        .add_attribute("part_id", part_id.to_string()))
}

fn execute_set_mint_price(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: CarNftContract,
    price: Uint128,
) -> StdResult<Response> {
    // Verificar que el remitente es el dueño del contrato
    // TODO: Implementar verificación de propiedad cuando se implemente CW721
    
    // Actualizar el precio de minteo
    contract.mint_price.save(deps.storage, &price)?;
    
    Ok(Response::new()
        .add_attribute("method", "set_mint_price")
        .add_attribute("new_price", price.to_string()))
}

fn execute_equip_part(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: CarNftContract,
    car_id: u64,
    part_id: u64,
    slot_index: u64,
) -> StdResult<Response> {
    // Verificar que el carro existe
    let mut car = contract.cars.load(deps.storage, car_id)?;
    
    // Verificar que el slot es válido
    if slot_index >= car.part_ids.len() as u64 {
        return Err(StdError::generic_err("Índice de slot inválido"));
    }
    
    // Verificar que el slot no está ocupado
    if car.slot_occupied[slot_index as usize] {
        return Err(StdError::generic_err("El slot ya está ocupado"));
    }
    
    // Obtener la dirección del contrato de partes
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    
    // Crear el mensaje para actualizar el estado de equipamiento
    let set_equipped_msg = CarPartExecuteMsg::SetEquippedState {
        part_id,
        car_id,
    };
    
    // Actualizar el estado del carro
    car.part_ids[slot_index as usize] = part_id;
    car.slot_occupied[slot_index as usize] = true;
    contract.cars.save(deps.storage, car_id, &car)?;
    
    Ok(Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: car_part_contract.to_string(),
            msg: to_json_binary(&set_equipped_msg)?,
            funds: vec![],
        })
        .add_attribute("method", "equip_part")
        .add_attribute("car_id", car_id.to_string())
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("slot_index", slot_index.to_string()))
}

fn execute_replace_part(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: CarNftContract,
    car_id: u64,
    old_part_id: u64,
    new_part_id: u64,
) -> StdResult<Response> {
    // Primero desequipamos la parte antigua
    let mut response = Response::new();
    
    // Verificar que el carro existe
    let mut car = contract.cars.load(deps.storage, car_id)?;
    
    // Verificar que la parte antigua está equipada
    let mut slot_index = None;
    for (i, &pid) in car.part_ids.iter().enumerate() {
        if pid == old_part_id {
            slot_index = Some(i);
            break;
        }
    }
    
    let slot_index = slot_index.ok_or_else(|| StdError::generic_err("La parte antigua no está equipada en este carro"))?;
    
    // Obtener la dirección del contrato de partes
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    
    // Desequipar la parte antigua
    let unequip_msg = CarPartExecuteMsg::SetEquippedState {
        part_id: old_part_id,
        car_id: 0,
    };
    
    response = response.add_message(WasmMsg::Execute {
        contract_addr: car_part_contract.to_string(),
        msg: to_json_binary(&unequip_msg)?,
        funds: vec![],
    });
    
    // Equipar la nueva parte
    let equip_msg = CarPartExecuteMsg::SetEquippedState {
        part_id: new_part_id,
        car_id,
    };
    
    response = response.add_message(WasmMsg::Execute {
        contract_addr: car_part_contract.to_string(),
        msg: to_json_binary(&equip_msg)?,
        funds: vec![],
    });
    
    // Actualizar el estado del carro
    car.part_ids[slot_index] = new_part_id;
    contract.cars.save(deps.storage, car_id, &car)?;
    
    Ok(response
        .add_attribute("method", "replace_part")
        .add_attribute("car_id", car_id.to_string())
        .add_attribute("old_part_id", old_part_id.to_string())
        .add_attribute("new_part_id", new_part_id.to_string()))
}

fn execute_set_workshop_contract(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract: CarNftContract,
    address: String,
) -> StdResult<Response> {
    let workshop_addr = deps.api.addr_validate(&address)?;
    contract.workshop_contract.save(deps.storage, &workshop_addr)?;
    
    Ok(Response::new()
        .add_attribute("method", "set_workshop_contract")
        .add_attribute("address", address))
}

fn execute_set_leaderboard_contract(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract: CarNftContract,
    address: String,
) -> StdResult<Response> {
    let leaderboard_addr = deps.api.addr_validate(&address)?;
    contract.leaderboard_contract.save(deps.storage, &leaderboard_addr)?;
    
    Ok(Response::new()
        .add_attribute("method", "set_leaderboard_contract")
        .add_attribute("address", address))
}

fn execute_withdraw_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _contract: CarNftContract,
) -> StdResult<Response> {
    // Verificar que el remitente es el dueño del contrato
    // TODO: Implementar verificación de propiedad cuando se implemente CW721
    
    // Obtener el balance del contrato
    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    
    if balance.is_empty() {
        return Err(StdError::generic_err("El contrato no tiene fondos para retirar"));
    }
    
    // Crear el mensaje para enviar los fondos al remitente
    let bank_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: balance.clone(),
    };
    
    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("method", "withdraw_funds")
        .add_attribute("recipient", info.sender)
        .add_attribute("amount", format!("{:?}", balance)))
}

// Funciones auxiliares
fn query_car_composition(deps: Deps, contract: CarNftContract, car_id: u64) -> StdResult<CarComposition> {
    contract.cars.load(deps.storage, car_id)
}

fn query_compact_car_stats(deps: Deps, contract: CarNftContract, car_id: u64) -> StdResult<CompactCarStats> {
    let car = contract.cars.load(deps.storage, car_id)?;
    let condition = contract.car_conditions.load(deps.storage, car_id)?;
    
    Ok(CompactCarStats {
        car_id,
        condition,
        part_ids: car.part_ids,
    })
}

fn query_full_car_metadata(deps: Deps, contract: CarNftContract, car_id: u64) -> StdResult<FullCarMetadata> {
    let car = contract.cars.load(deps.storage, car_id)?;
    let condition = contract.car_conditions.load(deps.storage, car_id)?;
    
    Ok(FullCarMetadata {
        car_id,
        condition,
        car_image_uri: car.car_image_uri,
        part_ids: car.part_ids,
        slot_occupied: car.slot_occupied,
    })
}

fn query_last_token_id(deps: Deps, contract: CarNftContract) -> StdResult<u64> {
    contract.current_car_id.load(deps.storage)
}

// Función para manejar las respuestas de los submensajes
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let contract = CarNftContract::default();
    
    // Obtener el ID del submensaje (1-based index)
    let submsg_id = msg.id;
    let slot_index = (submsg_id - 1) as usize;
    
    // Extraer el ID de la parte del evento
    let part_id = msg.result.unwrap().events
        .iter()
        .find(|e| e.ty == "wasm")
        .and_then(|e| e.attributes.iter().find(|a| a.key == "part_id"))
        .map(|a| a.value.parse::<u64>())
        .ok_or_else(|| StdError::generic_err("No se pudo obtener el ID de la parte"))?
        .map_err(|_| StdError::generic_err("Error al convertir el ID de la parte a número"))?;
    
    // Obtener el último car_id
    let car_id = contract.current_car_id.load(deps.storage)? - 1;
    
    // Actualizar el car_composition con el ID de la parte
    let mut car = contract.cars.load(deps.storage, car_id)?;
    car.part_ids[slot_index] = part_id;
    contract.cars.save(deps.storage, car_id, &car)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_part_id")
        .add_attribute("car_id", car_id.to_string())
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("slot_index", slot_index.to_string()))
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
            car_part_contract: "car_part_contract".to_string(),
            mint_price: Uint128::new(10000000), // 0.01 en la unidad más pequeña
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(2, res.attributes.len());
    }
} 