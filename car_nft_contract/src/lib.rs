use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    SubMsg, Uint128, WasmMsg, BankMsg, Reply,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};
use car_types::{PartType, PartStats, PartData};

// Estructuras principales
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarComposition {
    pub part_ids: Vec<u64>,
    pub car_image_uri: String,
    pub slot_occupied: Vec<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PartMetadata {
    pub part_id: u64,
    pub part_type: PartType,
    pub stats: PartStats,
    pub slot_index: u8,
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
    pub owner_cars: Map<'a, (Addr, u64), bool>,
}

impl<'a> Clone for CarNftContract<'a> {
    fn clone(&self) -> Self {
        Self {
            cars: Map::new("cars"),
            car_conditions: Map::new("car_conditions"),
            workshop_contract: Item::new("workshop_contract"),
            leaderboard_contract: Item::new("leaderboard_contract"),
            car_part_contract: Item::new("car_part_contract"),
            mint_price: Item::new("mint_price"),
            current_car_id: Item::new("current_car_id"),
            owner_cars: Map::new("owner_cars"),
        }
    }
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
            owner_cars: Map::new("owner_cars"),
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
    GetOwnerCars {
        owner: String,
    },
    GetPartStats {
        part_id: u64,
    },
    GetPartType {
        part_id: u64,
    },
    GetAllCarMetadata {
        owner: String,
    },
}

// Respuestas de consulta
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerCarsResponse {
    pub car_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CompactCarStats {
    pub image_uri: String,
    pub speed: u8,
    pub acceleration: u8,
    pub handling: u8,
    pub drift_factor: u8,
    pub turn_factor: u8,
    pub max_speed: u8,
    pub condition: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FullCarMetadata {
    pub car_id: u64,
    pub car_image_uri: String,
    pub parts: Vec<PartMetadata>,
    pub total_stats: CompactCarStats,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllCarMetadataResponse {
    pub cars: Vec<FullCarMetadata>,
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
        ExecuteMsg::Mint { to, part_type, stat1, stat2, stat3, image_uri, car_id } => {
            execute_mint(deps, env, info, contract, to, part_type, stat1, stat2, stat3, image_uri, car_id)
        },
        ExecuteMsg::SetEquippedState { part_id, car_id } => {
            execute_set_equipped_state(deps, env, info, contract, part_id, car_id)
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
            to_json_binary(&query_mint_price(deps, contract)?)
        },
        QueryMsg::GetOwnerCars { owner } => {
            to_json_binary(&query_owner_cars(deps, contract, owner)?)
        },
        QueryMsg::GetPartStats { part_id } => {
            to_json_binary(&query_part_stats(deps, contract, part_id)?)
        },
        QueryMsg::GetPartType { part_id } => {
            to_json_binary(&query_part_type(deps, contract, part_id)?)
        },
        QueryMsg::GetAllCarMetadata { owner } => {
            to_json_binary(&query_all_car_metadata(deps, contract, owner)?)
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let contract = CarNftContract::default();
    
    // Obtener el ID de la parte del atributo del evento
    let part_id = msg.result
        .into_result()
        .map_err(|_| StdError::generic_err("Error al procesar la respuesta del minteo"))?
        .events
        .iter()
        .find(|e| e.ty == "wasm")
        .ok_or_else(|| StdError::generic_err("No se encontró el evento wasm"))?
        .attributes
        .iter()
        .find(|a| a.key == "part_id")
        .ok_or_else(|| StdError::generic_err("No se encontró el ID de la parte"))?
        .value
        .parse::<u64>()
        .map_err(|_| StdError::generic_err("Error al parsear el ID de la parte"))?;
    
    // El ID de respuesta corresponde al índice + 1 de la parte en el array original
    let part_index = (msg.id - 1) as usize;
    
    // Obtener el último carro minteado
    let car_id = contract.current_car_id.load(deps.storage)? - 1;
    let mut car = contract.cars.load(deps.storage, car_id)?;
    
    // Actualizar el ID de la parte en el slot correspondiente
    let slot_index = if part_index == 0 { 0 } // Engine
        else if part_index == 1 { 1 } // Transmission
        else { 2 }; // Wheels
    
    car.part_ids[slot_index] = part_id;
    contract.cars.save(deps.storage, car_id, &car)?;
    
    Ok(Response::new()
        .add_attribute("method", "reply")
        .add_attribute("part_id", part_id.to_string())
        .add_attribute("car_id", car_id.to_string())
        .add_attribute("slot_index", slot_index.to_string()))
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
    let part_ids = vec![0u64; 3];
    let mut slot_occupied = vec![false; 3];
    let mut messages = Vec::new();

    // Obtener la dirección del contrato de partes
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;

    // Procesar cada parte
    for (index, part) in parts_data.iter().enumerate() {
        let slot_index = match part.part_type {
            PartType::Engine => {
                has_engine = true;
                0
            },
            PartType::Transmission => {
                has_transmission = true;
                1
            },
            PartType::Wheels => {
                has_wheels = true;
                2
            },
        };

        // Validar stats
        if part.stat1 > 10 || part.stat2 > 10 || part.stat3 > 10 {
            return Err(StdError::generic_err("Los stats deben ser <= 10"));
        }

        // Crear el mensaje para mintear la parte
        let mint_msg = ExecuteMsg::Mint {
            to: info.sender.to_string(),
            part_type: part.part_type.clone(),
            stat1: part.stat1,
            stat2: part.stat2,
            stat3: part.stat3,
            image_uri: part.image_uri.clone(),
            car_id,
        };

        // Crear el submensaje para mintear la parte
        let mint_submsg = SubMsg::reply_on_success(WasmMsg::Execute {
            contract_addr: car_part_contract.to_string(),
            msg: to_json_binary(&mint_msg)?,
            funds: vec![],
        }, (index + 1) as u64);

        // Marcar el slot como ocupado
        slot_occupied[slot_index] = true;

        // Agregar el mensaje
        messages.push(mint_submsg);
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

    // Inicializar la condición del carro al 100%
    contract.car_conditions.save(deps.storage, car_id, &100u8)?;

    // Registrar el propietario del carro
    contract.owner_cars.save(deps.storage, (info.sender.clone(), car_id), &true)?;

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
    _info: MessageInfo,
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
    let set_equipped_msg = ExecuteMsg::SetEquippedState {
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
    _info: MessageInfo,
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
    _info: MessageInfo,
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
    let set_equipped_msg = ExecuteMsg::SetEquippedState {
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
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
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
    let unequip_msg = ExecuteMsg::SetEquippedState {
        part_id: old_part_id,
        car_id: 0,
    };
    
    response = response.add_message(WasmMsg::Execute {
        contract_addr: car_part_contract.to_string(),
        msg: to_json_binary(&unequip_msg)?,
        funds: vec![],
    });
    
    // Equipar la nueva parte
    let equip_msg = ExecuteMsg::SetEquippedState {
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

fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: CarNftContract,
    to: String,
    part_type: PartType,
    stat1: u8,
    stat2: u8,
    stat3: u8,
    image_uri: String,
    car_id: u64,
) -> StdResult<Response> {
    // Verificar que el remitente es el dueño del contrato
    // TODO: Implementar verificación de propiedad cuando se implemente CW721
    
    // Obtener el ID actual del carro
    let current_car_id = contract.current_car_id.load(deps.storage)?;
    
    // Verificar que el carro existe
    let car = contract.cars.load(deps.storage, current_car_id)?;
    
    // Verificar que el slot es válido
    if car.part_ids.len() < 3 {
        return Err(StdError::generic_err("El carro no tiene suficientes partes"));
    }
    
    // Obtener la dirección del contrato de partes
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    
    // Crear el mensaje para actualizar el estado de equipamiento
    let set_equipped_msg = ExecuteMsg::SetEquippedState {
        part_id: car.part_ids[2],
        car_id,
    };
    
    // Guardar el part_id antes de actualizar el carro
    let part_id = car.part_ids[2];
    
    // Actualizar el estado del carro
    let mut updated_car = car;
    updated_car.part_ids[2] = 0;
    contract.cars.save(deps.storage, current_car_id, &updated_car)?;
    
    Ok(Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: car_part_contract.to_string(),
            msg: to_json_binary(&set_equipped_msg)?,
            funds: vec![],
        })
        .add_attribute("method", "mint")
        .add_attribute("car_id", current_car_id.to_string())
        .add_attribute("owner", info.sender)
        .add_attribute("to", to)
        .add_attribute("part_type", part_type.to_string())
        .add_attribute("stat1", stat1.to_string())
        .add_attribute("stat2", stat2.to_string())
        .add_attribute("stat3", stat3.to_string())
        .add_attribute("image_uri", image_uri)
        .add_attribute("part_id", part_id.to_string()))
}

fn execute_set_equipped_state(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract: CarNftContract,
    part_id: u64,
    car_id: u64,
) -> StdResult<Response> {
    // Verificar que el remitente es el dueño del contrato
    // TODO: Implementar verificación de propiedad cuando se implemente CW721
    
    // Obtener el ID actual del carro
    let current_car_id = contract.current_car_id.load(deps.storage)?;
    
    // Verificar que el carro existe
    let car = contract.cars.load(deps.storage, current_car_id)?;
    
    // Verificar que la parte está equipada en el carro
    let mut found = false;
    for &pid in &car.part_ids {
        if pid == part_id {
            found = true;
            break;
        }
    }
    
    if !found {
        return Err(StdError::generic_err("La parte no está equipada en este carro"));
    }
    
    // Obtener la dirección del contrato de partes
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    
    // Crear el mensaje para actualizar el estado de equipamiento
    let set_equipped_msg = ExecuteMsg::SetEquippedState {
        part_id,
        car_id,
    };
    
    // Actualizar el estado del carro
    let mut updated_car = car;
    updated_car.part_ids[2] = 0;
    contract.cars.save(deps.storage, current_car_id, &updated_car)?;
    
    Ok(Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: car_part_contract.to_string(),
            msg: to_json_binary(&set_equipped_msg)?,
            funds: vec![],
        })
        .add_attribute("method", "set_equipped_state")
        .add_attribute("car_id", current_car_id.to_string())
        .add_attribute("part_id", part_id.to_string()))
}

// Funciones auxiliares
fn query_car_composition(deps: Deps, contract: CarNftContract, car_id: u64) -> StdResult<CarComposition> {
    contract.cars.load(deps.storage, car_id)
}

// Funciones auxiliares de consulta
fn query_owner_cars(deps: Deps, contract: CarNftContract, owner: String) -> StdResult<OwnerCarsResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let last_token_id = contract.current_car_id.load(deps.storage)?;
    let mut car_ids = Vec::new();

    // Iterar sobre todos los posibles IDs de carros y verificar si pertenecen al propietario
    for car_id in 1..last_token_id {
        if contract.owner_cars.may_load(deps.storage, (owner_addr.clone(), car_id))?.unwrap_or(false) {
            car_ids.push(car_id);
        }
    }

    Ok(OwnerCarsResponse { car_ids })
}

fn query_compact_car_stats(deps: Deps, contract: CarNftContract, car_id: u64) -> StdResult<CompactCarStats> {
    let car = contract.cars.load(deps.storage, car_id)?;
    let condition = contract.car_conditions.load(deps.storage, car_id)?;
    
    // Aquí deberías calcular los stats basados en las partes del carro
    // Por ahora retornamos valores por defecto
    Ok(CompactCarStats {
        image_uri: car.car_image_uri,
        speed: 0,
        acceleration: 0,
        handling: 0,
        drift_factor: 0,
        turn_factor: 0,
        max_speed: 0,
        condition,
    })
}

fn query_full_car_metadata(deps: Deps, contract: CarNftContract, car_id: u64) -> StdResult<FullCarMetadata> {
    let car = contract.cars.load(deps.storage, car_id)?;
    // Obtener la condición del carro, si no existe usar 100 como valor por defecto
    let condition = contract.car_conditions.may_load(deps.storage, car_id)?.unwrap_or(100u8);
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    let mut parts = Vec::new();
    let mut total_speed = 0u8;
    let mut total_acceleration = 0u8;
    let mut total_handling = 0u8;

    // Obtener los metadatos de cada parte
    for (slot_index, &part_id) in car.part_ids.iter().enumerate() {
        if part_id > 0 && car.slot_occupied[slot_index] {
            // Consultar stats de la parte
            let query_msg = QueryMsg::GetPartStats { part_id };
            let part_stats: PartStats = deps.querier.query_wasm_smart(
                car_part_contract.clone(),
                &query_msg,
            )?;

            // Consultar tipo de parte
            let query_msg = QueryMsg::GetPartType { part_id };
            let part_type: PartType = deps.querier.query_wasm_smart(
                car_part_contract.clone(),
                &query_msg,
            )?;

            // Agregar los stats al total
            match part_type {
                PartType::Engine => {
                    total_speed += part_stats.stat1;
                    total_acceleration += part_stats.stat2;
                    total_handling += part_stats.stat3;
                },
                PartType::Transmission => {
                    total_speed += part_stats.stat1;
                    total_acceleration += part_stats.stat2;
                    total_handling += part_stats.stat3;
                },
                PartType::Wheels => {
                    total_speed += part_stats.stat1;
                    total_acceleration += part_stats.stat2;
                    total_handling += part_stats.stat3;
                },
            }

            parts.push(PartMetadata {
                part_id,
                part_type,
                stats: part_stats,
                slot_index: slot_index as u8,
            });
        }
    }

    // Calcular stats totales
    let total_stats = CompactCarStats {
        image_uri: car.car_image_uri.clone(),
        speed: total_speed / 3,
        acceleration: total_acceleration / 3,
        handling: total_handling / 3,
        drift_factor: (total_handling + total_acceleration) / 4,
        turn_factor: (total_handling + total_speed) / 4,
        max_speed: total_speed,
        condition,
    };

    Ok(FullCarMetadata {
        car_id,
        car_image_uri: car.car_image_uri,
        parts,
        total_stats,
    })
}

fn query_last_token_id(deps: Deps, contract: CarNftContract) -> StdResult<u64> {
    contract.current_car_id.load(deps.storage)
}

// Funciones de consulta
fn query_part_stats(deps: Deps, contract: CarNftContract, part_id: u64) -> StdResult<PartStats> {
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    deps.querier.query_wasm_smart(
        car_part_contract,
        &QueryMsg::GetPartStats { part_id },
    )
}

fn query_part_type(deps: Deps, contract: CarNftContract, part_id: u64) -> StdResult<PartType> {
    let car_part_contract = contract.car_part_contract.load(deps.storage)?;
    deps.querier.query_wasm_smart(
        car_part_contract,
        &QueryMsg::GetPartType { part_id },
    )
}

// Función para obtener todos los metadatos de los carros de un usuario
pub fn query_all_car_metadata(
    deps: Deps,
    contract: CarNftContract,
    owner: String,
) -> StdResult<AllCarMetadataResponse> {
    let _owner_addr = deps.api.addr_validate(&owner)?;
    let owner_cars = query_owner_cars(deps, contract.clone(), owner)?;
    
    let mut cars = Vec::new();
    for car_id in owner_cars.car_ids {
        if let Ok(car_metadata) = query_full_car_metadata(deps, contract.clone(), car_id) {
            cars.push(car_metadata);
        }
    }
    
    Ok(AllCarMetadataResponse { cars })
}

// Función para consultar el precio de minteo
fn query_mint_price(deps: Deps, contract: CarNftContract) -> StdResult<Uint128> {
    contract.mint_price.load(deps.storage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            car_part_contract: "car_part_contract".to_string(),
            mint_price: Uint128::new(1000000),
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(4, res.attributes.len());
    }

    #[test]
    fn test_mint_car() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = "creator";
        let info = mock_info(creator, &coins(1000000, "uxion"));

        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_part_contract: "car_part_contract".to_string(),
            mint_price: Uint128::new(1000000),
        };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Crear datos de prueba para las partes
        let parts_data = vec![
            PartData {
                part_type: PartType::Engine,
                stat1: 8,
                stat2: 7,
                stat3: 6,
                image_uri: "engine_uri".to_string(),
            },
            PartData {
                part_type: PartType::Transmission,
                stat1: 5,
                stat2: 6,
                stat3: 7,
                image_uri: "transmission_uri".to_string(),
            },
            PartData {
                part_type: PartType::Wheels,
                stat1: 4,
                stat2: 5,
                stat3: 6,
                image_uri: "wheels_uri".to_string(),
            },
        ];

        // Intentar mintear un carro
        let msg = ExecuteMsg::MintCar {
            car_image_uri: "car_uri".to_string(),
            parts_data: parts_data.clone(),
        };

        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        
        // Verificar que se crearon los mensajes correctos
        assert_eq!(3, res.messages.len()); // Un mensaje por cada parte
        
        // Verificar que se guardó la composición del carro
        let query_msg = QueryMsg::GetCarComposition { car_id: 1 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let car_comp: CarComposition = from_json(&res).unwrap();
        
        assert_eq!(3, car_comp.part_ids.len());
        assert_eq!(3, car_comp.slot_occupied.len());
        assert_eq!("car_uri", car_comp.car_image_uri);

        // Verificar que el carro se registró para el propietario
        let query_msg = QueryMsg::GetOwnerCars { owner: creator.to_string() };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let owner_cars: OwnerCarsResponse = from_json(&res).unwrap();
        assert_eq!(vec![1], owner_cars.car_ids);
    }

    #[test]
    fn test_mint_car_insufficient_payment() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = "creator";
        let info = mock_info(creator, &coins(500000, "uxion")); // Pago insuficiente

        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_part_contract: "car_part_contract".to_string(),
            mint_price: Uint128::new(1000000),
        };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Crear datos de prueba
        let parts_data = vec![
            PartData {
                part_type: PartType::Engine,
                stat1: 8,
                stat2: 7,
                stat3: 6,
                image_uri: "engine_uri".to_string(),
            },
        ];

        // Intentar mintear un carro
        let msg = ExecuteMsg::MintCar {
            car_image_uri: "car_uri".to_string(),
            parts_data,
        };

        let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
        assert!(err.to_string().contains("Pago insuficiente"));
    }

    #[test]
    fn test_mint_car_missing_parts() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = "creator";
        let info = mock_info(creator, &coins(1000000, "uxion"));

        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_part_contract: "car_part_contract".to_string(),
            mint_price: Uint128::new(1000000),
        };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Crear datos de prueba sin todas las partes necesarias
        let parts_data = vec![
            PartData {
                part_type: PartType::Engine,
                stat1: 8,
                stat2: 7,
                stat3: 6,
                image_uri: "engine_uri".to_string(),
            },
            // Falta transmisión y ruedas
        ];

        // Intentar mintear un carro
        let msg = ExecuteMsg::MintCar {
            car_image_uri: "car_uri".to_string(),
            parts_data,
        };

        let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
        assert!(err.to_string().contains("Faltan partes necesarias"));
    }

    #[test]
    fn test_equip_unequip_part() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = "creator";
        let info = mock_info(creator, &coins(1000000, "uxion"));

        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_part_contract: "car_part_contract".to_string(),
            mint_price: Uint128::new(1000000),
        };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Crear un carro con todas sus partes
        let parts_data = vec![
            PartData {
                part_type: PartType::Engine,
                stat1: 8,
                stat2: 7,
                stat3: 6,
                image_uri: "engine_uri".to_string(),
            },
            PartData {
                part_type: PartType::Transmission,
                stat1: 5,
                stat2: 6,
                stat3: 7,
                image_uri: "transmission_uri".to_string(),
            },
            PartData {
                part_type: PartType::Wheels,
                stat1: 4,
                stat2: 5,
                stat3: 6,
                image_uri: "wheels_uri".to_string(),
            },
        ];

        let msg = ExecuteMsg::MintCar {
            car_image_uri: "car_uri".to_string(),
            parts_data,
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Desequipar una parte
        let msg = ExecuteMsg::UnequipPart {
            car_id: 1,
            part_id: 0, // El motor
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Verificar que la parte se desequipó
        let query_msg = QueryMsg::GetCarComposition { car_id: 1 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let car_comp: CarComposition = from_json(&res).unwrap();
        assert_eq!(0, car_comp.part_ids[0]); // El slot del motor debe estar vacío
        assert!(!car_comp.slot_occupied[0]); // El slot no debe estar ocupado

        // Equipar una parte en un slot vacío
        let msg = ExecuteMsg::EquipPart {
            car_id: 1,
            part_id: 0,
            slot_index: 0,
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Verificar que la parte se equipó correctamente
        let query_msg = QueryMsg::GetCarComposition { car_id: 1 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let car_comp: CarComposition = from_json(&res).unwrap();
        assert_eq!(0, car_comp.part_ids[0]); // El slot del motor debe tener la parte
        assert!(car_comp.slot_occupied[0]); // El slot debe estar ocupado
    }

    #[test]
    fn test_replace_part() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator = "creator";
        let info = mock_info(creator, &coins(1000000, "uxion"));

        // Inicializar el contrato
        let msg = InstantiateMsg {
            car_part_contract: "car_part_contract".to_string(),
            mint_price: Uint128::new(1000000),
        };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Crear un carro con todas sus partes
        let parts_data = vec![
            PartData {
                part_type: PartType::Engine,
                stat1: 8,
                stat2: 7,
                stat3: 6,
                image_uri: "engine_uri".to_string(),
            },
            PartData {
                part_type: PartType::Transmission,
                stat1: 5,
                stat2: 6,
                stat3: 7,
                image_uri: "transmission_uri".to_string(),
            },
            PartData {
                part_type: PartType::Wheels,
                stat1: 4,
                stat2: 5,
                stat3: 6,
                image_uri: "wheels_uri".to_string(),
            },
        ];

        let msg = ExecuteMsg::MintCar {
            car_image_uri: "car_uri".to_string(),
            parts_data,
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Reemplazar una parte
        let msg = ExecuteMsg::ReplacePart {
            car_id: 1,
            old_part_id: 0, // El motor original
            new_part_id: 3, // Un nuevo motor
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Verificar que la parte se reemplazó correctamente
        let query_msg = QueryMsg::GetCarComposition { car_id: 1 };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let car_comp: CarComposition = from_json(&res).unwrap();
        assert_eq!(3, car_comp.part_ids[0]); // El slot del motor debe tener la nueva parte
        assert!(car_comp.slot_occupied[0]); // El slot debe seguir ocupado
    }
} 