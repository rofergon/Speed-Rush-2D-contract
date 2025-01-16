# Información de Contratos Desplegados en XION

## Direcciones de Contratos en XION Testnet

- `car_part_contract`: `xion1jleg22pemep9gtw8s64xn7ham030xffusy484jvf27y76fll6xvqe5nfx6`
- `car_nft_contract`: `xion13wm99zenpc8vaff98nwg5n5jd9hnlcuva07n4ykh69ye9f54xjcsl94xlj`

## Configuración

- Precio de minteo: 100 uxion
- Owner: xion13w6wtafq4tjsqpck4tqlxpkky8da6zp2cyzqq4

## Car Part Contract
- **Dirección**: `xion1jleg22pemep9gtw8s64xn7ham030xffusy484jvf27y76fll6xvqe5nfx6`
- **CODE_ID**: 1902
- **Owner**: `xion13w6wtafq4tjsqpck4tqlxpkky8da6zp2cyzqq4`

## Car NFT Contract
- **Dirección**: `xion14836hqy3km4sardm2e89s2tw6l23d9lvcfwgjwlvp44lztlxjp4q3h5uck`
- **CODE_ID**: 1904
- **Owner**: `xion13w6wtafq4tjsqpck4tqlxpkky8da6zp2cyzqq4`
- **Precio de Minteo**: 100 uxion
- **Car Part Contract**: `xion1jleg22pemep9gtw8s64xn7ham030xffusy484jvf27y76fll6xvqe5nfx6`

## Formato de Minteo

### 1. Minteo de Partes (Car Part Contract)
```json
{
  "mint": {
    "to": "xion13w6wtafq4tjsqpck4tqlxpkky8da6zp2cyzqq4",
    "part_type": 0,  // 0 = ENGINE, 1 = TRANSMISSION, 2 = WHEELS
    "stat1": 6,      // ENGINE: speed | TRANSMISSION: acceleration | WHEELS: handling
    "stat2": 10,     // ENGINE: max_speed | TRANSMISSION: speed | WHEELS: drift_factor
    "stat3": 5,      // ENGINE: acceleration | TRANSMISSION: handling | WHEELS: turn_factor
    "image_uri": "https://gateway.lighthouse.storage/ipfs/bafybeibznaf7xrabqsniwpumo42fnlidkroafbuy34lqj45wsnz4xvrfuu",
    "car_id": 0      // Se asigna automáticamente al mintear el carro
  }
}
```

### 2. Minteo de Carro (Car NFT Contract)
```json
{
  "mint_car": {
    "car_image_uri": "https://gateway.lighthouse.storage/ipfs/bafkreidbxidemaqbefskombrtcckrfxkqompsud6s74iuv57xklp4agwiy",
    "parts_data": [
      {
        "part_type": "Engine",  // ENGINE
        "stat1": 6,      // speed
        "stat2": 10,     // max_speed
        "stat3": 5,      // acceleration
        "image_uri": "https://gateway.lighthouse.storage/ipfs/bafybeibznaf7xrabqsniwpumo42fnlidkroafbuy34lqj45wsnz4xvrfuu"
      },
      {
        "part_type": "Transmission",  // TRANSMISSION
        "stat1": 6,      // acceleration
        "stat2": 2,      // speed
        "stat3": 10,     // handling
        "image_uri": "https://gateway.lighthouse.storage/ipfs/bafybeifkrxgj6rk2yw65wqyjlhoj4bzd3f4fkj73vnh4icblao2uzqazta"
      },
      {
        "part_type": "Wheels",  // WHEELS
        "stat1": 6,      // handling
        "stat2": 4,      // drift_factor
        "stat3": 2,      // turn_factor
        "image_uri": "https://gateway.lighthouse.storage/ipfs/bafkreigzrqkuuivv3jvadlqlshgdnsu4f53k24ankxgtf4glguqqu2jg3e"
      }
    ]
  }
}
```

### Notas Importantes:
1. El minteo de partes se hace automáticamente al mintear el carro
2. Cada parte tiene 3 estadísticas que varían según el tipo de parte:
   - **ENGINE**: speed, max_speed, acceleration
   - **TRANSMISSION**: acceleration, speed, handling
   - **WHEELS**: handling, drift_factor, turn_factor
3. Los valores de las estadísticas deben ser entre 0 y 10
4. Se requieren las tres partes (ENGINE, TRANSMISSION, WHEELS) para mintear un carro
5. El precio de minteo (100 uxion) debe ser enviado con la transacción

## Resultados del Primer Minteo

### Carro
- **ID**: 1
- **Imagen**: `https://gateway.lighthouse.storage/ipfs/bafkreidbxidemaqbefskombrtcckrfxkqompsud6s74iuv57xklp4agwiy`
- **Propietario**: `xion13w6wtafq4tjsqpck4tqlxpkky8da6zp2cyzqq4`
- **Hash de Transacción**: `EFC756551039BFEADEEBF4A5C39E657E75CD18AE5CF11104357255C89D0FBBFC`

### Partes
1. **Motor (ID: 0)**
   - Tipo: Engine
   - Speed: 6
   - Max Speed: 10
   - Acceleration: 5
   - Imagen: `https://gateway.lighthouse.storage/ipfs/bafybeibznaf7xrabqsniwpumo42fnlidkroafbuy34lqj45wsnz4xvrfuu`

2. **Transmisión (ID: 1)**
   - Tipo: Transmission
   - Acceleration: 6
   - Speed: 2
   - Handling: 10
   - Imagen: `https://gateway.lighthouse.storage/ipfs/bafybeifkrxgj6rk2yw65wqyjlhoj4bzd3f4fkj73vnh4icblao2uzqazta`

3. **Ruedas (ID: 2)**
   - Tipo: Wheels
   - Handling: 6
   - Drift Factor: 4
   - Turn Factor: 2
   - Imagen: `https://gateway.lighthouse.storage/ipfs/bafkreigzrqkuuivv3jvadlqlshgdnsu4f53k24ankxgtf4glguqqu2jg3e`

# Resultados del Segundo Minteo

### Carro
- **ID**: 1
- **Imagen**: `https://gateway.lighthouse.storage/ipfs/bafkreidbxidemaqbefskombrtcckrfxkqompsud6s74iuv57xklp4agwiy`
- **Propietario**: `xion13w6wtafq4tjsqpck4tqlxpkky8da6zp2cyzqq4`
- **Hash de Transacción**: `BEB148E1B71ED0E84B07CCCE9D6D535F406C6AAA324CC9CEEFE741C191ACB15D`

### Partes
1. **Motor (ID: 3)**
   - Tipo: Engine
   - Speed: 6
   - Max Speed: 10
   - Acceleration: 5
   - Imagen: `https://gateway.lighthouse.storage/ipfs/bafybeibznaf7xrabqsniwpumo42fnlidkroafbuy34lqj45wsnz4xvrfuu`

2. **Transmisión (ID: 4)**
   - Tipo: Transmission
   - Acceleration: 6
   - Speed: 2
   - Handling: 10
   - Imagen: `https://gateway.lighthouse.storage/ipfs/bafybeifkrxgj6rk2yw65wqyjlhoj4bzd3f4fkj73vnh4icblao2uzqazta`

3. **Ruedas (ID: 5)**
   - Tipo: Wheels
   - Handling: 6
   - Drift Factor: 4
   - Turn Factor: 2
   - Imagen: `https://gateway.lighthouse.storage/ipfs/bafkreigzrqkuuivv3jvadlqlshgdnsu4f53k24ankxgtf4glguqqu2jg3e` 