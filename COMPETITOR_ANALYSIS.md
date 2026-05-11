# Collat — Competitor Analysis

**Author:** Muhammad Zidan Fatonie
**Last Updated:** 2026-05-11
**Context:** Mezo hackathon grant application

---

## 1. Competitive Landscape Overview

Collat sits at the intersection of three markets: Bitcoin lending, crypto debit cards, and BTC yield protocols. No existing product does all three in a single checkout flow.

| Category | Collat Position |
|----------|----------------|
| Bitcoin Lending | Native BTC on Mezo, auto-borrow at checkout |
| Crypto Debit Cards | Visa/MC card backed by on-chain BTC collateral |
| BTC Yield / Restaking | Collateral is always on-chain, never custodied |

---

## 2. Crypto Debit Cards

These let users spend crypto at traditional merchants but all require selling or converting to fiat first.

### 2.1 Coinbase Card

| Dimension | Detail |
|-----------|--------|
| Model | User chooses which crypto to sell at checkout. Coinbase converts to fiat and settles merchant. |
| Crypto | Supports all Coinbase-listed assets |
| Geography | US and 20+ EU countries |
| Fees | No monthly fee, 2.5% conversion spread |
| Key Difference from Collat | Forces a taxable sale every transaction. No borrowing. User loses upside. |

### 2.2 Crypto.com Visa

| Dimension | Detail |
|-----------|--------|
| Model | Pre-paid card. User tops up fiat wallet or spends from crypto wallet (auto-sell). |
| Crypto | 20+ assets |
| Geography | 90+ countries |
| Fees | No monthly fee (CRO staking tiers), 2% conversion spread |
| Key Difference from Collat | Requires pre-loading or selling crypto. CRO stake requirement for higher tiers. |

### 2.3 Fold

| Dimension | Detail |
|-----------|--------|
| Model | BTC rewards debit card. Spend fiat, earn BTC back. |
| Crypto | BTC rewards only (not a spending vehicle) |
| Geography | US only |
| Key Difference from Collat | Fold is a fiat-spend card with BTC rewards. Collat lets you spend BTC itself without selling. |

### 2.4 Uniswap Card

| Dimension | Detail |
|-----------|--------|
| Model | Self-custodied crypto debit card. User chooses which tokens to swap and pay with. |
| Crypto | Any token on supported chains |
| Geography | US (waitlist) |
| Key Difference from Collat | Still a sell-to-spend model. Every transaction is a swap. No leverage of collateral. |

### 2.5 RedotPay

| Dimension | Detail |
|-----------|--------|
| Model | Crypto-to-fiat prepaid card. User top-up via crypto deposit. |
| Crypto | USDT, USDC, BTC, ETH |
| Geography | Global (Asia-focused) |
| Key Difference from Collat | Centralized custody. User deposits crypto to RedotPay wallet. No self-custody. No borrowing. |

### 2.6 CryptMi

| Dimension | Detail |
|-----------|--------|
| Model | Crypto payment gateway + card. Spend directly from wallet. |
| Crypto | Multi-chain |
| Geography | Global |
| Key Difference from Collat | Merchant-first platform. Collat is consumer-first with auto-borrow at checkout. |

### 2.7 Bybit Card

| Dimension | Detail |
|-----------|--------|
| Model | Pre-paid card linked to Bybit exchange account. Spend from spot or funding wallet. |
| Crypto | USDT, USDC, BTC, ETH |
| Geography | 100+ countries |
| Key Difference from Collat | Centralized exchange custody. Not DeFi native. Requires Bybit account and KYC. |

### 2.8 Binance Card

| Dimension | Detail |
|-----------|--------|
| Model | Pre-paid card linked to Binance account. Auto-converts crypto to fiat at checkout. |
| Crypto | BTC, ETH, BNB, USDT, others |
| Geography | EEA |
| Key Difference from Collat | Centralized exchange custody. Full KYC. Taxable sale every transaction. Not available outside EEA. |

---

## 3. Bitcoin Lending Protocols

These let BTC holders borrow stablecoins against their Bitcoin. Collat competes here on checkout-flow automation.

### 3.1 Aave (WBTC)

| Dimension | Detail |
|-----------|--------|
| Collateral | WBTC (wrapped, not native BTC) |
| Borrow Asset | GHO, USDC, USDT, DAI |
| Max LTV | 70-75% |
| Platform | Ethereum L1, L2s |
| Key Difference | Wrapped BTC only (not native Bitcoin). Manual borrow + repay steps. No card. |

### 3.2 Compound (cWBTC)

| Dimension | Detail |
|-----------|--------|
| Collateral | WBTC |
| Borrow Asset | USDC, USDT |
| Max LTV | 65-70% |
| Platform | Ethereum, Base, Polygon |
| Key Difference | Same wrapping requirement. Manual borrow. No checkout automation. |

### 3.3 Maple Finance (BTC Cash Management)

| Dimension | Detail |
|-----------|--------|
| Collateral | WBTC, BTC (institutional) |
| Borrow Asset | USDC |
| Max LTV | Custom per pool |
| Platform | Ethereum, Solana |
| Key Difference | Institutional lending. KYC required. Not a consumer checkout product. |

### 3.4 BTC.Link

| Dimension | Detail |
|-----------|--------|
| Collateral | Native BTC (via Stacks bridge) |
| Borrow Asset | sUSDT, sUSDC |
| Max LTV | 50-65% |
| Platform | Stacks (Bitcoin L2) |
| Key Difference | Stacks ecosystem. No card. Manual borrow. Limited liquidity. |

### 3.5 SolvBTC

| Dimension | Detail |
|-----------|--------|
| Model | Liquid staking token for Bitcoin. Deposit BTC, get SolvBTC. Use SolvBTC in DeFi. |
| Platform | Multiple chains |
| Key Difference | Not a lending protocol. Liquid staking, not collateralized borrowing. No spending mechanism. |

### 3.6 Lombard / Bedrock (LBTC / uniBTC)

| Dimension | Detail |
|-----------|--------|
| Model | Bitcoin restaking. Deposit BTC, get LBTC/uniBTC, restake for yield. |
| Platform | Babylon ecosystem, multiple chains |
| Key Difference | Yield generation, not spending. No auto-borrow. No card. No checkout flow. |

### 3.7 Ether.fi (BTC)

| Dimension | Detail |
|-----------|--------|
| Model | BTC liquid restaking via Babylon integration |
| Platform | Multiple chains (EigenLayer ecosystem) |
| Key Difference | Restaking yield, not spending. No borrowing mechanism. |

---

## 4. Bitcoin L2 / Ecosystem Lending

These are the closest competitors — native BTC DeFi on Bitcoin L2s.

### 4.1 Stacks (sBTC)

| Dimension | Detail |
|-----------|--------|
| BTC Model | sBTC — trust-minimized BTC peg on Stacks |
| Lending | Zest Protocol (BTC lending), BitFlow |
| Auto-Borrow | No |
| Card | No |
| Key Difference | Stacks has BTC DeFi but no checkout automation and no card. Zest still requires manual borrow. |

### 4.2 CoreDAO

| Dimension | Detail |
|-----------|--------|
| BTC Model | Non-custodial BTC staking |
| Lending | Core lending markets |
| Auto-Borrow | No |
| Card | No |
| Key Difference | Staking-focused. No spending infrastructure. |

### 4.3 Botanix (BTC Spiderchain)

| Dimension | Detail |
|-----------|--------|
| BTC Model | EVM-equivalent Bitcoin L2 |
| Lending | EVM lending protocols (Aave forks) |
| Auto-Borrow | No |
| Card | No |
| Key Difference | General DeFi on Bitcoin. No specialized checkout or card product. |

### 4.4 BOB (Build on Bitcoin)

| Dimension | Detail |
|-----------|--------|
| BTC Model | Hybrid L2 (Bitcoin security + EVM composability) |
| Lending | Standard EVM lending |
| Auto-Borrow | No |
| Card | No |
| Key Difference | Infrastructure layer, not a consumer spending product. |

---

## 5. Mezo Ecosystem Projects

From the Mezo hackathon submissions, these are the closest projects Collat competes or complements with:

| Project | What They Do | Collat Relationship |
|---------|-------------|-------------------|
| Various BTC lending protocols | Borrow MUSD against BTC on Mezo | Direct competitors on lending, but none have auto-borrow or card |
| Mezo DeFi wallets | Wallet interfaces for Mezo | Complement — Collat integrates with any Mezo wallet |
| MUSD stablecoin projects | MUSD issuance and utility | Complement — Collat drives MUSD demand through spending |
| Card/payment projects | Payment infrastructure on Mezo | Potential competitors if they build lending + card |

---

## 6. Key Competitive Advantages

| Advantage | Why It Matters |
|-----------|----------------|
| Auto-borrow at checkout | Users never manually borrow. One deposit, infinite spends. No other protocol does this. |
| Native BTC on Mezo | No wrapping, no bridging to Ethereum. Native Bitcoin L2. |
| Collat Card | Virtual + physical Visa/MC. Accepted anywhere cards work. |
| No taxable events | Borrowing is not selling. Users keep BTC upside. |
| No KYC | DeFi native. No identity checks for MVP. |
| Mezo-native | Leverages Mezo's BTC bridge, MUSD stablecoin, and low-fee infrastructure. |

---

## 7. Competitive Weaknesses (Risks to Address)

| Weakness | Mitigation |
|----------|------------|
| Mezo ecosystem still small | Focus on Mezo adoption; Collat drives TVL and MUSD demand which benefits Mezo |
| No card issuer partnership yet | Card is Phase 4; partnership discussions start after smart contract MVP |
| Price feed is admin-set (MVP) | Upgrade to Pyth/Switchboard before mainnet |
| Single collateral type (BTC only) | Multi-collateral post-MVP |
| No established user base | Mezo ecosystem is early; Collat can grow with it |

---

## 8. Positioning Statement

> Every crypto card today makes you sell your crypto to spend it. Every BTC lending protocol makes you borrow manually. Collat is the first product that lets you deposit BTC once, then spend at any merchant without any manual steps in between.
