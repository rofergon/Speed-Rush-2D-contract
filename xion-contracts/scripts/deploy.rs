use std::process::Command;
use std::env;
use std::fs;
use serde_json::Value;

// Leer las variables de entorno del archivo .env
fn load_env() {
    let env_path = "../xion/.env";
    if let Ok(contents) = fs::read_to_string(env_path) {
        for line in contents.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                env::set_var(key, value.trim_matches('"'));
            }
        }
    }
}

// Verificar la configuración de la billetera
fn verify_wallet(wallet_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Verificando la configuración de la billetera...");
    
    // Verificar si la billetera existe
    let wallet_check = Command::new("xiond")
        .args(&["keys", "show", wallet_name, "--keyring-backend", "test"])
        .output()?;

    if !wallet_check.status.success() {
        // Si la billetera no existe, intentar importarla usando el mnemónico del .env
        println!("⚠️  Billetera no encontrada, intentando importar desde .env...");
        
        if let Ok(contents) = fs::read_to_string("../xion/.env") {
            if let Some(mnemonic_line) = contents.lines().find(|line| line.contains("XION_MNEMONIC")) {
                if let Some((_, mnemonic)) = mnemonic_line.split_once('=') {
                    let mnemonic = mnemonic.trim().trim_matches('"');
                    
                    let import_cmd = Command::new("sh")
                        .arg("-c")
                        .arg(format!(
                            "echo '{}' | xiond keys add {} --recover --keyring-backend test",
                            mnemonic, wallet_name
                        ))
                        .output()?;

                    if !import_cmd.status.success() {
                        return Err(format!(
                            "Error al importar la billetera: {}", 
                            String::from_utf8_lossy(&import_cmd.stderr)
                        ).into());
                    }
                    println!("✅ Billetera importada exitosamente");
                } else {
                    return Err("No se encontró el mnemónico en el archivo .env".into());
                }
            } else {
                return Err("No se encontró la variable XION_MNEMONIC en el archivo .env".into());
            }
        } else {
            return Err("No se pudo leer el archivo .env".into());
        }
    } else {
        println!("✅ Billetera verificada correctamente");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando el proceso de despliegue...");
    
    // Cargar variables de entorno
    load_env();

    let wallet_name = "wallet";
    let chain_id = "xion-testnet-1";
    let node_url = "https://rpc.xion-testnet-1.burnt.com:443";
    let gas_prices = "0.1uxion";

    // Verificar la billetera antes de continuar
    verify_wallet(wallet_name)?;

    println!("📦 Optimizando el contrato...");
    
    // Cambiar al directorio del contrato
    let contract_path = env::current_dir()?.display().to_string();
    println!("   Directorio actual: {}", contract_path);
    println!("⚠️  Se solicitará la contraseña de sudo para ejecutar Docker...");
    
    let docker_cmd = Command::new("sudo")
        .args(&[
            "docker", "run", "--rm",
            "-v", format!("{}:/code", contract_path).as_str(),
            "--mount", "type=volume,source=target_cache,target=/code/target",
            "--mount", "type=volume,source=registry_cache,target=/usr/local/cargo/registry",
            "cosmwasm/rust-optimizer:0.16.0"
        ])
        .output()?;

    if !docker_cmd.status.success() {
        println!("❌ Error al ejecutar Docker:");
        println!("   stdout: {}", String::from_utf8_lossy(&docker_cmd.stdout));
        println!("   stderr: {}", String::from_utf8_lossy(&docker_cmd.stderr));
        return Err("Error al optimizar el contrato".into());
    }

    println!("✅ Contrato optimizado correctamente");

    // Verificar que el archivo wasm existe
    let wasm_path = "artifacts/xion_contracts.wasm";
    if !fs::metadata(wasm_path).is_ok() {
        return Err("No se encontró el archivo WASM optimizado".into());
    }
    println!("✅ Archivo WASM optimizado encontrado");

    // Subir el contrato a la cadena
    println!("📤 Subiendo el contrato a la cadena...");
    let store_cmd = Command::new("xiond")
        .args(&[
            "tx", "wasm", "store", wasm_path,
            "--from", wallet_name,
            "--chain-id", chain_id,
            "--node", node_url,
            "--gas-prices", gas_prices,
            "--gas", "auto",
            "--gas-adjustment", "1.3",
            "--keyring-backend", "test",
            "-y",
            "--output", "json",
            "--broadcast-mode", "sync"
        ])
        .output()?;

    println!("   Respuesta del comando store:");
    let store_response = String::from_utf8_lossy(&store_cmd.stdout);
    let store_stderr = String::from_utf8_lossy(&store_cmd.stderr);
    
    if !store_stderr.is_empty() {
        println!("   Información adicional: {}", store_stderr);
    }

    let store_json: Value = serde_json::from_str(&store_response)
        .map_err(|e| format!("Error al parsear JSON de store: {} - Response: {}", e, store_response))?;
    
    let txhash = store_json["txhash"]
        .as_str()
        .ok_or("No se encontró el txhash en la respuesta")?;
    println!("📝 TX Hash: {}", txhash);

    // Esperar que se procese la transacción
    println!("⏳ Esperando que la transacción se procese...");
    for i in 1..=10 {
        println!("   Intento {} de 10...", i);
        std::thread::sleep(std::time::Duration::from_secs(2));

        let query_output = Command::new("xiond")
            .args(&[
                "query", "tx", txhash,
                "--node", node_url,
                "--output", "json"
            ])
            .output()?;

        let query_response = String::from_utf8_lossy(&query_output.stdout);
        
        if !query_response.contains("not found") {
            println!("\n🔍 Analizando respuesta de la transacción:");
            println!("{}", query_response);
            
            // Buscar el CODE_ID en la salida completa
            if let Some(code_id_pos) = query_response.find("code_id") {
                let substr = &query_response[code_id_pos..];
                if let Some(value_start) = substr.find(char::is_numeric) {
                    let value_end = substr[value_start..].find(|c: char| !c.is_numeric())
                        .map_or(substr.len() - value_start, |i| value_start + i);
                    let code_id = &substr[value_start..value_end];
                    println!("📋 CODE_ID encontrado: {}", code_id);
                    return Ok(());
                }
            }
        }
    }

    Err("No se pudo obtener el CODE_ID después de varios intentos".into())
} 