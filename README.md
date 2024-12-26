# Speed Rush 2D - Sistema de NFTs

Un sistema de NFTs para el juego Speed Rush 2D desplegado en la Lens Network Testnet que permite a los usuarios crear y personalizar carros usando partes NFT.

## Características

- Sistema de partes de carro como NFTs (Motor, Transmisión, Ruedas)
- Sistema de estadísticas dinámico donde cada parte contribuye a múltiples atributos
- Taller de reparación para mantener los carros en buen estado
- Sistema de degradación de carros durante las carreras
- Tabla de clasificación para competencias

## Contratos Desplegados

Los contratos están desplegados en Lens Network Testnet en las siguientes direcciones:
- CarPart: `0x4bF1Cf69D3Cdc11dD7cBe0b1942Ce183f27FE402`
- CarNFT: `0xEd0fA4fFDB1B33B6D6c6611B77F6806DB50b21aE`
- CarWorkshop: `0x92cb777a96BE6f617959c8220388e4A046DA8669`
- RaceLeaderboard: `0x9caEBCA084c2072904083008a0b3AE99068571b6`

## Sistema de Estadísticas

### Motor (ENGINE)
- stat1: Velocidad
- stat2: Velocidad Máxima
- stat3: Aceleración

### Transmisión (TRANSMISSION)
- stat1: Aceleración
- stat2: Velocidad
- stat3: Manejo

### Ruedas (WHEELS)
- stat1: Manejo
- stat2: Derrape
- stat3: Giro

Cada estadística final del carro es afectada por al menos dos atributos de diferentes partes:
- Velocidad: Motor (stat1) y Transmisión (stat2)
- Velocidad Máxima: Motor (stat2) y Transmisión (stat1)
- Aceleración: Motor (stat3) y Transmisión (stat1)
- Manejo: Transmisión (stat3) y Ruedas (stat1)
- Derrape: Ruedas (stat2) y Transmisión (stat3)
- Giro: Ruedas (stat3) y Ruedas (stat2)

## Prerequisitos

- Node.js >= v16
- npm o yarn
- Una wallet con fondos en Lens Network Testnet

## Instalación

1. Clonar el repositorio:
```bash
git clone <url-del-repo>
cd Speed-Rush-2D
```

2. Instalar dependencias:
```bash
npm install
```

3. Crear un archivo `.env` en el directorio raíz y agregar tu clave privada:
```env
PRIVATE_KEY=tu_clave_privada_aqui
```

## Scripts de Gestión

### Mintear un Carro
```bash
npx hardhat run scripts/testMintCar.js --network lensTestnet
```
Crea un nuevo carro con partes personalizadas.

### Verificar Partes de un Carro
```bash
npx hardhat run scripts/checkCarParts.js --network lensTestnet
```
Muestra los detalles de las partes de un carro específico.

### Reparar un Carro
```bash
npx hardhat run scripts/repairCar.js --network lensTestnet
```
Repara un carro dañado en el taller.

## Funciones Principales

### CarNFT
- `mintCar(string memory carImageURI, PartData[] calldata partsData)`: Mintea un nuevo carro con sus partes
- `replacePart(uint256 carId, uint256 oldPartId, uint256 newPartId)`: Reemplaza una parte de un carro
- `getCompactCarStats(uint256 carId)`: Obtiene las estadísticas de un carro

### CarPart
- `mint(address to, PartType partType, uint8 stat1, uint8 stat2, uint8 stat3, string memory imageURI)`: Mintea una nueva parte
- `getPartStats(uint256 partId)`: Obtiene las estadísticas de una parte

### CarWorkshop
- `repairCar(uint256 carId)`: Repara un carro dañado
- `setRepairPrice(uint256 _newPrice)`: Establece el precio de reparación

## Desarrollo

1. Compilar los contratos:
```bash
npm run compile
```

2. Desplegar en Lens Network Testnet:
```bash
npm run deploy
```

## Configuración de Red

El proyecto está configurado para trabajar con Lens Network Testnet:
- Nombre de la Red: Lens Network Sepolia Testnet
- URL RPC: https://rpc.testnet.lens.dev
- ID de Cadena: 37111
- Explorador de Bloques: https://block-explorer.testnet.lens.dev

## Seguridad

- Los contratos utilizan implementaciones probadas de OpenZeppelin
- Todas las funciones sensibles están protegidas con modificadores apropiados
- Sistema de permisos entre contratos para operaciones seguras

## Licencia

MIT

## Soporte

Para soporte, por favor abre un issue en el repositorio o contacta al equipo de desarrollo.
