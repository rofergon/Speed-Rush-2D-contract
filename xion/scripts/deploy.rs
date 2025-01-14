use std::process::Command;
use std::env;
use std::fs;
use serde_json::Value;

const WALLET_NAME: &str = "wallet";
const WALLET_PASSWORD: &str = "saritu12";
const CHAIN_ID: &str = "xion-testnet-1";
const NODE_URL: &str = "https://rpc.xion-testnet-1.burnt.com:443";
const GAS_PRICES: &str = "0.1uxion";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando el proceso de despliegue...");

    // 1. Optimizar el contrato
    println!("📦 Optimizando el contrato...");
    println!("   Directorio actual: {:?}", env::current_dir()?);
    
    let docker_cmd = Command::new("sudo")
        .args(&[
            "docker", "run", "--rm",
            "-v", format!("{}:/code", env::current_dir()?.display()).as_str(),
            "--mount", "type=volume,source=target_cache,target=/code/target",
            "--mount", "type=volume,source=registry_cache,target=/usr/local/cargo/registry",
            "cosmwasm/rust-optimizer:0.16.0"
        ])
        .output()?;

    println!("   Salida de Docker:");
    println!("   stdout: {}", String::from_utf8_lossy(&docker_cmd.stdout));
    println!("   stderr: {}", String::from_utf8_lossy(&docker_cmd.stderr));

    if !docker_cmd.status.success() {
        return Err(format!("Error al optimizar el contrato: {}", 
            String::from_utf8_lossy(&docker_cmd.stderr)).into());
    }

    // 2. Verificar que el archivo wasm existe
    let wasm_path = "artifacts/xion_contracts.wasm";
    if !fs::metadata(wasm_path).is_ok() {
        return Err("No se encontró el archivo WASM optimizado".into());
    }
    println!("✅ Archivo WASM optimizado encontrado");

    // 3. Subir el contrato a la cadena
    println!("📤 Subiendo el contrato a la cadena...");
    let store_cmd = Command::new("xiond")
        .args(&[
            "tx", "wasm", "store", wasm_path,
            "--from", WALLET_NAME,
            "--chain-id", CHAIN_ID,
            "--node", NODE_URL,
            "--gas-prices", GAS_PRICES,
            "--gas", "auto",
            "--gas-adjustment", "1.3",
            "-y",
            "--output", "json",
            "--broadcast-mode", "sync"
        ])
        .output()?;

    println!("   Salida del comando store:");
    println!("   stdout: {}", String::from_utf8_lossy(&store_cmd.stdout));
    println!("   stderr: {}", String::from_utf8_lossy(&store_cmd.stderr));

    let store_response = String::from_utf8_lossy(&store_cmd.stdout);
    let store_json: Value = serde_json::from_str(&store_response)
        .map_err(|e| format!("Error al parsear JSON de store: {} - Response: {}", e, store_response))?;
    
    let txhash = store_json["txhash"]
        .as_str()
        .ok_or("No se encontró el txhash en la respuesta")?;
    println!("📝 TX Hash: {}", txhash);

    // 4. Esperar unos segundos para que la transacción se procese
    println!("⏳ Esperando que la transacción se procese...");
    std::thread::sleep(std::time::Duration::from_secs(6));

    // 5. Obtener el CODE_ID del contrato
    println!("🔍 Obteniendo el CODE_ID del contrato...");
    let query_output = Command::new("xiond")
        .args(&["query", "tx", txhash, "--output", "json"])
        .output()?;

    let query_response = String::from_utf8_lossy(&query_output.stdout);
    let query_json: Value = serde_json::from_str(&query_response)?;
    
    let code_id = query_json["logs"][0]["events"]
        .as_array()
        .and_then(|events| events.iter().find(|e| e["type"] == "store_code"))
        .and_then(|event| event["attributes"].as_array())
        .and_then(|attrs| attrs.iter().find(|a| a["key"] == "code_id"))
        .and_then(|attr| attr["value"].as_str())
        .ok_or("No se pudo obtener el CODE_ID")?;

    println!("📋 CODE_ID: {}", code_id);

    // 6. Instanciar el contrato
    println!("🔧 Instanciando el contrato...");
    let init_msg = r#"{"car_part_contract": "xion1234567890", "mint_price": "1000000"}"#;
    let instantiate_output = Command::new("xiond")
        .args(&[
            "tx", "wasm", "instantiate", code_id, init_msg,
            "--from", WALLET_NAME,
            "--chain-id", CHAIN_ID,
            "--node", NODE_URL,
            "--gas-prices", GAS_PRICES,
            "--gas", "auto",
            "--gas-adjustment", "1.3",
            "--label", "car nft contract",
            "--no-admin",
            "-y",
            "--output", "json",
            "--broadcast-mode", "sync"
        ])
        .output()?;

    let inst_response = String::from_utf8_lossy(&instantiate_output.stdout);
    let inst_json: Value = serde_json::from_str(&inst_response)?;
    let inst_txhash = inst_json["txhash"].as_str().ok_or("No se encontró el txhash de instanciación")?;
    println!("📝 TX Hash de instanciación: {}", inst_txhash);

    // Esperar que se procese la instanciación
    println!("⏳ Esperando que se procese la instanciación...");
    std::thread::sleep(std::time::Duration::from_secs(6));

    // Obtener la dirección del contrato
    println!("🔍 Obteniendo la dirección del contrato...");
    let contract_output = Command::new("xiond")
        .args(&["query", "wasm", "list-contract-by-code", code_id, "--output", "json"])
        .output()?;

    let contract_response = String::from_utf8_lossy(&contract_output.stdout);
    let contract_json: Value = serde_json::from_str(&contract_response)?;
    let contract_address = contract_json["contracts"][0].as_str().ok_or("No se encontró la dirección del contrato")?;

    println!("\n✅ Despliegue completado exitosamente!");
    println!("📋 Información del contrato:");
    println!("   CODE_ID: {}", code_id);
    println!("   Dirección: {}", contract_address);
    println!("   TX Hash Store: {}", txhash);
    println!("   TX Hash Instantiate: {}", inst_txhash);

    // Guardar la información en un archivo
    let deploy_info = format!(
        "CODE_ID: {}\nDirección: {}\nTX Hash Store: {}\nTX Hash Instantiate: {}\n",
        code_id, contract_address, txhash, inst_txhash
    );
    fs::write("deploy_info.txt", deploy_info)?;
    println!("\n📄 Información guardada en deploy_info.txt");

    Ok(())
} 