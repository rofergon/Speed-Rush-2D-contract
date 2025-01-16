# Guía Detallada de Despliegue - Speed Rush 2D

## Tabla de Contenidos
1. [Preparación del Entorno](#1-preparación-del-entorno)
2. [Estructura del Proyecto](#2-estructura-del-proyecto)
3. [Contrato car_part](#3-contrato-car_part)
4. [Contrato car_nft](#4-contrato-car_nft)
5. [Verificación y Pruebas](#5-verificación-y-pruebas)
6. [Solución de Problemas](#6-solución-de-problemas)

## 1. Preparación del Entorno

### 1.1 Requisitos del Sistema
- Sistema Operativo: Windows 10/11 con WSL2
- Docker Desktop
- Rust y Cargo
- Git

### 1.2 Instalación de Herramientas
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Agregar target wasm32
rustup target add wasm32-unknown-unknown

# Verificar instalación
rustc --version
cargo --version
```

### 1.3 Configuración de WSL
```bash
# Abrir PowerShell como administrador y ejecutar:
wsl --install
wsl --set-default-version 2

# Reiniciar el sistema después de la instalación
```

### 1.4 Configuración de Docker
- Instalar Docker Desktop
- Habilitar integración con WSL2
- Verificar instalación:
```bash
docker --version
docker run hello-world
```

## 2. Estructura del Proyecto

### 2.1 Organización de Directorios
```
Speed-Rush-2D contract/
├── xion-contracts/
│   ├── car_part/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── car_nft/
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
├── car_part_contract_info.txt
├── car_nft_contract_info.txt
└── DEPLOYMENT_GUIDE.md
```

### 2.2 Archivos de Configuración
Verificar contenido de `Cargo.toml` en ambos contratos:
```toml
[package]
name = "car_part_contract"  # o "car_nft_contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
cosmwasm-std = "1.5.0"
schemars = "0.8.15"
serde = { version = "1.0.188", features = ["derive"] }
cw-storage-plus = "1.1.0"
```

## 3. Contrato car_part

### 3.1 Compilación
```bash
# Entrar a WSL
wsl

# Navegar al directorio
cd xion-contracts/car_part

# Limpiar build anterior
cargo clean

# Compilar
cargo build --release --target wasm32-unknown-unknown
```

### 3.2 Optimización
```bash
# Optimizar usando rust-optimizer
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.16.0

# Verificar archivo generado
ls -l artifacts/
```

### 3.3 Subida a XION Testnet
```bash
# Subir contrato
xiond tx wasm store artifacts/car_part_contract.wasm \
  --from saritu12 \
  --node https://rpc.xion-testnet-1.burnt.com:443 \
  --chain-id xion-testnet-1 \
  --gas-prices 0.0001uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y

# Guardar TX_HASH para consulta
```

### 3.4 Obtención de CODE_ID
```bash
# Consultar transacción
xiond query tx <TX_HASH> --node https://rpc.xion-testnet-1.burnt.com:443

# Buscar y guardar CODE_ID en la respuesta
```

### 3.5 Instanciación
```bash
# Instanciar contrato
xiond tx wasm instantiate <CODE_ID> '{}' \
  --from saritu12 \
  --label "Speed Rush Car Part" \
  --no-admin \
  --node https://rpc.xion-testnet-1.burnt.com:443 \
  --chain-id xion-testnet-1 \
  --gas-prices 0.0001uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y

# Guardar TX_HASH para consulta
```

### 3.6 Registro de Información
Crear/actualizar `car_part_contract_info.txt`:
```
CODE_ID=<CODE_ID>
CONTRACT_ADDRESS=<CONTRACT_ADDRESS>
TX_HASH=<TX_HASH>
CHECKSUM=<CHECKSUM>
OWNER=<WALLET_ADDRESS>
```

## 4. Contrato car_nft

### 4.1 Compilación
```bash
# Navegar al directorio
cd ../car_nft

# Limpiar y compilar
cargo clean
cargo build --release --target wasm32-unknown-unknown
```

### 4.2 Optimización
```bash
# Optimizar contrato
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.16.0
```

### 4.3 Subida a XION Testnet
```bash
# Subir contrato
xiond tx wasm store artifacts/car_nft_contract.wasm \
  --from saritu12 \
  --node https://rpc.xion-testnet-1.burnt.com:443 \
  --chain-id xion-testnet-1 \
  --gas-prices 0.0001uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y
```

### 4.4 Instanciación
```bash
# Instanciar con parámetros
xiond tx wasm instantiate <CODE_ID> \
  '{"car_part_contract":"<CAR_PART_CONTRACT_ADDRESS>","mint_price":"1000000"}' \
  --from saritu12 \
  --label "Speed Rush Car NFT" \
  --no-admin \
  --node https://rpc.xion-testnet-1.burnt.com:443 \
  --chain-id xion-testnet-1 \
  --gas-prices 0.0001uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y
```

### 4.5 Registro de Información
Crear/actualizar `car_nft_contract_info.txt`:
```
CODE_ID=<CODE_ID>
CONTRACT_ADDRESS=<CONTRACT_ADDRESS>
TX_HASH=<TX_HASH>
CHECKSUM=<CHECKSUM>
CAR_PART_CONTRACT=<CAR_PART_CONTRACT_ADDRESS>
MINT_PRICE=1000000
```

## 5. Verificación y Pruebas

### 5.1 Verificar car_part
```bash
# Consultar estado del contrato
xiond query wasm contract-state smart <CAR_PART_CONTRACT_ADDRESS> \
  '{"get_contract_info":{}}' \
  --node https://rpc.xion-testnet-1.burnt.com:443
```

### 5.2 Verificar car_nft
```bash
# Consultar precio de minteo
xiond query wasm contract-state smart <CAR_NFT_CONTRACT_ADDRESS> \
  '{"get_mint_price":{}}' \
  --node https://rpc.xion-testnet-1.burnt.com:443

# Verificar dirección del contrato car_part
xiond query wasm contract-state smart <CAR_NFT_CONTRACT_ADDRESS> \
  '{"get_car_part_contract":{}}' \
  --node https://rpc.xion-testnet-1.burnt.com:443
```

## 6. Solución de Problemas

### 6.1 Errores Comunes
1. **Error de Compilación**
   - Verificar dependencias en Cargo.toml
   - Actualizar Rust y target wasm32
   - Limpiar cache: `cargo clean`

2. **Error de Optimización**
   - Verificar Docker en ejecución
   - Verificar permisos de directorio
   - Reiniciar Docker

3. **Error de Subida**
   - Verificar saldo de wallet
   - Verificar conexión a testnet
   - Ajustar gas-adjustment

4. **Error de Instanciación**
   - Verificar formato JSON
   - Verificar direcciones de contratos
   - Verificar parámetros requeridos

### 6.2 Comandos Útiles
```bash
# Verificar saldo
xiond query bank balances <WALLET_ADDRESS> \
  --node https://rpc.xion-testnet-1.burnt.com:443

# Verificar estado de nodo
xiond status --node https://rpc.xion-testnet-1.burnt.com:443

# Listar contratos
xiond query wasm list-code --node https://rpc.xion-testnet-1.burnt.com:443
```

