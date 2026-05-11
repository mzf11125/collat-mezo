# Collat

Bank on Bitcoin. Spend your Bitcoin without selling it.

Collat lets you deposit BTC as collateral, borrow MUSD, and spend at any merchant. Your Bitcoin stays in your custody. You only borrow what you need at checkout.

Built for the Mezo ecosystem across three tracks: Bitcoin Track, MEZO Track, MUSD Track.

## Architecture

```
User Wallet
    |
    v
+------------------+     +------------------+
|  BTC Vault       | --> |  MUSD Borrow     |
|  (deposit/       |     |  (borrow/repay/   |
|   withdraw)      |     |   liquidate)      |
+------------------+     +------------------+
    |                         |
    v                         v
+------------------+     +------------------+
|  User Position   |     |  Price Feed      |
|  (per-user)      |     |  (oracle price)  |
+------------------+     +------------------+
```

## Smart Contract

The program is at `programs/collat-mezo/src/lib.rs`.

### Instructions

| Instruction | Description |
|-------------|-------------|
| `initializeVault` | Set up vault config, BTC/MUSD mints, token accounts |
| `setPrice` | Initialize oracle price feed |
| `updatePrice` | Update BTC/USD price |
| `depositCollateral` | Deposit BTC as collateral |
| `withdrawCollateral` | Withdraw BTC (no outstanding loan) |
| `borrow` | Borrow MUSD against BTC (max 60% LTV) |
| `repay` | Repay MUSD to reduce debt |
| `liquidate` | Liquidate undercollateralized positions (>75% LTV) |
| `closePosition` | Close a zero-balance position account |

### Parameters

- Max LTV: 60%
- Liquidation threshold: 75%
- Liquidation penalty: 5%
- Collateral: SPL BTC token (8 decimals)
- Borrow: MUSD token (6 decimals)

## Getting Started

```bash
# Install dependencies
yarn install

# Build
anchor build

# Test (requires local validator)
anchor test
```

## Deployment

```bash
anchor deploy --provider.cluster devnet
```

## Tracks

### Bitcoin Track: Bank on Bitcoin
BTC holders finally put their Bitcoin to work without selling. Collat unlocks dormant liquidity for everyday spending.

### MEZO Track: Mezo Utilization
Drives TVL through BTC deposits and MUSD borrowing volume on Mezo. Every position adds to the ecosystem.

### MUSD Track: Supernormal dApps
MUSD becomes a spendable currency. Checkout flows, merchant integration, and DeFi composability on a stable foundation.
