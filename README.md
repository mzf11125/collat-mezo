# Collat

Bank on Bitcoin. Spend your Bitcoin without selling it.

Deposit BTC once. Shop normally. Collat does the rest.

Collat lets you deposit BTC as collateral and auto-borrows the exact MUSD needed at checkout. No manual steps. No selling your Bitcoin. Your BTC stays on-chain in your vault.

## How It Works

```
01. Deposit BTC
    Lock BTC as collateral in your Collat vault. One-time setup.

02. Shop Normally
    Pay online or in-store. Collat auto-borrows the exact MUSD
    needed and settles the merchant instantly.

03. Repay and Unlock
    Repay MUSD at any time. BTC is released proportionally back to you.
```

### Why it feels different

**No manual borrow step** — Collat triggers the loan automatically at checkout.

**BTC stays on-chain** — Your collateral never leaves the Mezo smart contract.

**Repay on your schedule** — No fixed deadlines. No liquidation on everyday purchases.

## Business Model

Collat earns across three revenue streams:

| Stream | Description |
|--------|-------------|
| Interest Spread | Difference between MUSD borrow rate and what liquidity providers receive. Core recurring revenue. |
| Transaction Fees | Small fee on each auto-borrow event at checkout. Scales directly with payment volume. |
| Collat Card Revenue | Interchange fees from card swipes plus a modest monthly card fee for physical card holders. |

**The flywheel:** More BTC deposited raises the collateral base. More purchases drive transaction fee revenue. More card holders add interchange income. All three reinforce each other.

## Collat Card

A physical and virtual debit card backed by your BTC collateral, accepted anywhere Visa is accepted.

- Auto-borrows MUSD the moment you swipe. No pre-loading required.
- Works at any physical or online merchant worldwide.
- BTC stays on-chain in your vault. Never held by us.
- Virtual card available instantly; physical card shipped on mainnet.

## Architecture

```
User Wallet
    |
    v
+------------------+     +------------------+
|  BTC Vault       | --> |  Auto-Borrow     |
|  (deposit/       |     |  Engine          |
|   withdraw)      |     |  (checkout CPI)  |
+------------------+     +------------------+
    |                         |
    v                         v
+------------------+     +------------------+
|  User Position   |     |  Price Feed      |
|  (per-user)      |     |  (oracle price)  |
+------------------+     +------------------+
```

## AI Features

If someone forks the same contract and ships the same card, the winner is whoever has better risk management and UX. AI is Collat's defensible moat for three reasons: it gets better with more data, most DeFi protocols still use dumb fixed thresholds, and BTC holders value predictability above all else.

### Liquidation Prediction

Collat monitors on-chain flows, funding rates, and macro signals to predict BTC volatility before it hits your position. Instead of waiting for price to cross a fixed 75% LTV threshold, the AI sends proactive alerts: "BTC looks shaky this week. Consider adding collateral." Rule-based systems cannot catch regime shifts early.

### Smart Spending Limits

Static LTV caps (60% max, 75% liquidation) are worst-case math designed for the dumbest possible user. Collat's AI sets dynamic spending limits per transaction based on current volatility regime, repayment history, and MUSD liquidity depth. Most days you can spend more than 60% LTV safely. Some days the AI tightens the limit before the market drops your liquidation buffer.

### Tax-Intelligent Routing

Borrowing is generally not a taxable event, but specific spending patterns can create reporting complexity. Collat's AI classifies each auto-borrow by category and generates a year-end tax report. BTC holders care deeply about tax efficiency.

### Natural Language Position Management

"Hey Collat, how much can I spend on a laptop?" instead of opening a dashboard. Conversational interface to check LTV, repay, adjust position, or review spending history.

### What AI Does NOT Touch

Payment rails must be instant. Collat never runs AI inference between the user and their swipe. The model pre-computes limits and alerts offline. At checkout, the only check is a fast deterministic LTV gate.

## Hackathon Tooling

Collat integrates every Mezo hackathon technical partner across the stack.

| Tool | Role in Collat |
|------|----------------|
| **Spectrum Nodes** | Primary RPC provider on Mezo testnet. Powers position queries and tx broadcasting. |
| **Validation Cloud** | Enterprise RPC provider on Mezo mainnet for production. |
| **Goldsky** | Indexes vault data (positions, liquidations, events) via subgraphs on Mezo + Bitcoin Turbo pipelines. Powers the dashboard. AI Skills auto-generate analytics. |
| **Tenderly** | Simulates auto-borrow and liquidation scenarios pre-deploy. Monitors production for failures. Traces cross-chain issues. |
| **Boar Network** | Secondary RPC + Blockchain MCP for the AI agent. MCP lets the natural language interface query on-chain positions directly. |

**Architecture:**
```
Spectrum / Validation Cloud → Auto-Borrow Engine ← → Goldsky Subgraphs
                                                          |
                                                          v
Mezo Blockchain                                     Collat Dashboard
    |
    v
Tenderly (simulate, monitor, trace)
Boar MCP (AI blockchain queries)
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

## Roadmap

| Phase | Milestone | Status |
|-------|-----------|--------|
| 1 | Smart contracts on Mezo testnet | Now |
| 2 | Auto-pay checkout SDK. Collat auto-borrows MUSD on every transaction. | Next |
| 3 | Audit, mainnet deployment, public beta with real BTC deposits | Soon |
| 4 | Collat Card: physical and virtual debit card backed by BTC | Future |

## Tracks

Collat qualifies for all three hackathon tracks simultaneously.

### Bitcoin Track: Bank on Bitcoin
BTC holders finally put their Bitcoin to work without selling. Collat unlocks dormant liquidity for everyday spending.

### MEZO Track: Mezo Utilization
Drives TVL through BTC deposits and MUSD borrowing volume on Mezo. Every position adds to the ecosystem.

### MUSD Track: Supernormal dApps
MUSD becomes a spendable currency. Checkout flows, merchant integration, and DeFi composability on a stable foundation.

## Judging Fit

| Criteria | Weight | How Collat Scores |
|----------|--------|-------------------|
| Mezo Integration | 30% | MUSD is the spend asset in every transaction. Not an add-on — core to the product. |
| Technical Implementation | 20% | Anchor smart contracts, AI moat, Goldsky subgraphs, Tenderly simulations, Boar MCP. |
| Business Viability | 30% | Interest spread + tx fees + card interchange. Real revenue, not token emissions. |
| User Experience | 10% | Deposit once, swipe infinitely. No manual borrow step. Natural language management. |
| Presentation | 10% | Demo: deposit BTC > swipe > auto-borrow > settled. Clear narrative.

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
