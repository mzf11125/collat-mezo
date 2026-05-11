# Collat — Product Requirements Document

**Status:** Draft v1  
**Author:** Muhammad Zidan Fatonie  
**Project:** Collat (collat-mezo)  
**Last Updated:** 2026-05-11

---

## 1. Executive Summary

Collat is a Bitcoin-collateralized spending protocol built on Mezo. It lets BTC holders deposit their Bitcoin once, then spend MUSD at any merchant without manual borrowing steps. Collat auto-borrows the exact MUSD needed at checkout, settles the merchant instantly, and lets users repay on their own schedule. No selling Bitcoin. No manual loan steps. No KYC delays.

The product targets three Mezo ecosystem tracks: Bitcoin Track (Bank on Bitcoin), MEZO Track (Mezo Utilization), and MUSD Track (Supernormal dApps).

---

## 2. Problem Statement

### 2.1 Current State

Bitcoin holders today face a painful choice: hold their BTC and have zero spending power, or sell it and lose future upside. Existing solutions are broken:

**Merchants don't accept BTC.** Very few merchants accept Bitcoin directly. Converting BTC to fiat requires centralized exchanges, takes days, and incurs fees.

**BTC lending is overcomplicated.** Existing platforms require overcollateralization (often 150%+), lengthy KYC processes, manual borrow steps, and rigid repayment schedules.

**No instant spending.** There is no way to use BTC collateral at a physical checkout or online merchant in real time.

### 2.2 User Pain Points

- "I want to spend my Bitcoin without selling it."
- "Loan platforms take too long and ask too many questions."
- "I should be able to swipe a card backed by my BTC."
- "Why do I need to manually borrow every time?"

---

## 3. Target Users

### Primary: Bitcoin Holders (Retail)

- Individuals holding $5,000+ in self-custodied BTC
- Want to spend their wealth without triggering taxable events from selling
- Frustrated by complexity of existing DeFi lending
- Geographically diverse, including regions with limited banking

### Secondary: Mezo Ecosystem Users

- Users already holding MUSD or active on Mezo
- Looking for yield-bearing use cases for their stablecoins
- Want to participate in Mezo's Supernormal dApps program

### Tertiary: Underbanked Markets

- Users in regions with limited access to traditional credit cards
- Holding BTC as their primary store of value
- Need a way to spend without converting to local fiat

---

## 4. Product Overview

### 4.1 Core Value Proposition

> Deposit BTC once. Shop normally. Collat does the rest.

Collat eliminates the manual borrow step. Users deposit BTC collateral once, then any purchase at checkout triggers an automatic MUSD borrow, merchant settlement, and position update — all in one transaction.

### 4.2 Key Differentiators

| Factor | Collat | Traditional BTC Lending |
|--------|--------|------------------------|
| Borrow step | Auto-triggered at checkout | Manual each time |
| Repayment | Flexible, no deadlines | Fixed term or interest |
| Card | Virtual + physical debit | None |
| BTC custody | Always on-chain | Centralized or wrapped |
| KYC | None (DeFi native) | Often required |

---

## 5. Features

### 5.1 MVP (Phase 1 — Smart Contracts)

| Feature | Priority | Description |
|---------|----------|-------------|
| BTC Vault | P0 | Deposit and withdraw BTC as collateral. Vault is a PDA with SPL token custody. |
| MUSD Borrow | P0 | Borrow MUSD up to 60% LTV against BTC collateral. |
| Repay | P0 | Repay MUSD to reduce debt and unlock BTC. |
| Liquidation | P0 | Liquidate positions exceeding 75% LTV with 5% penalty to liquidators. |
| Price Oracle | P0 | Admin-set price feed for BTC/USD. |

### 5.2 Phase 2 — Auto-Pay Integration

| Feature | Priority | Description |
|---------|----------|-------------|
| Checkout SDK | P1 | Merchant-facing SDK that triggers auto-borrow on purchase. |
| Auto-borrow Engine | P1 | Bundles borrow + transfer into one atomic transaction at checkout. |
| Position Dashboard | P1 | UI showing BTC deposited, MUSD borrowed, LTV, and repayment history. |

### 5.3 Phase 3 — Mainnet

| Feature | Priority | Description |
|---------|----------|-------------|
| Security Audit | P0 | Third-party audit of all smart contracts. |
| Mainnet Deployment | P0 | Deploy to Mezo mainnet with real BTC deposits. |
| Public Beta | P1 | Limited user onboarding with deposit caps. |

### 5.4 Phase 4 — Collat Card

| Feature | Priority | Description |
|---------|----------|-------------|
| Virtual Card | P1 | Instant virtual debit card, auto-borrows MUSD on swipe. |
| Physical Card | P2 | Physical Visa/Mastercard shipped to users post-mainnet. |
| Card Dashboard | P1 | Manage card, view transactions, repay from UI. |

---

## 6. User Flows

### 6.1 First-Time Deposit Flow

```
1. User connects Mezo wallet
2. User deposits BTC into Collat vault (approve + deposit)
3. User position created with BTC balance
4. User sets desired spending limit
5. Done. One-time setup.
```

### 6.2 Auto-Pay Checkout Flow

```
1. User shops at merchant integrated with Collat SDK
2. User selects "Pay with Collat" at checkout
3. Collat checks user's available borrowing power (LTV check)
4. Collat auto-borrows exact MUSD needed
5. MUSD transferred to merchant
6. User position updated with new debt
7. All in one atomic transaction
```

### 6.3 Repayment Flow

```
1. User opens Collat dashboard
2. User sends MUSD to vault
3. Position debt reduced proportionally
4. BTC becomes available for withdrawal
5. User can withdraw full BTC when debt = 0
```

### 6.4 Liquidation Flow

```
1. BTC price drops, pushing position above 75% LTV
2. Any external liquidator can repay debt
3. Liquidator receives seized BTC (debt + 5% penalty)
4. User position reduced
```

---

## 7. Business Model

### 7.1 Revenue Streams

| Stream | Mechanics | Maturity |
|--------|-----------|----------|
| Interest Spread | Difference between MUSD borrow rate and liquidity provider rate | Phase 1 |
| Transaction Fees | Small fee per auto-borrow at checkout | Phase 2 |
| Collat Card Revenue | Interchange fees + monthly card fee (physical only) | Phase 4 |

### 7.2 The Flywheel

```
More BTC deposited → Higher collateral base
Higher collateral base → More purchasing power
More purchasing power → More transaction fees
More transaction fees → Attract more LPs → Better rates → More BTC deposited
```

---

## 8. Technical Architecture

### 8.1 Smart Contracts (Solana / Anchor)

```
Program: collat_mezo (Anchor 0.31)

Accounts:
- VaultConfig (PDA): Global config, authority, mints, totals
- UserPosition (PDA): Per-user BTC deposited, MUSD borrowed
- PriceFeed (PDA): Oracle price, updated by authorized oracle

Instructions: initializeVault, setPrice, updatePrice,
              depositCollateral, withdrawCollateral,
              borrow, repay, liquidate, closePosition
```

### 8.2 Parameters (MVP)

| Parameter | Value |
|-----------|-------|
| Max LTV | 60% (6000 bps) |
| Liquidation LTV | 75% (7500 bps) |
| Liquidation Penalty | 5% (500 bps) |
| Collateral Token | SPL BTC (8 decimals) |
| Borrow Token | MUSD (6 decimals) |
| Price Feed | Admin-set, upgradeable to oracle |

### 8.3 Limitations

- Price feed is admin-set (MVP). Must be upgraded to a decentralized oracle (Pyth, Switchboard) before mainnet.
- No interest accrual model yet (MVP uses fixed parameters).
- Single collateral type (BTC only). Multi-collateral support is post-MVP.

---

## 9. Mezo Tracks

### Bitcoin Track: Bank on Bitcoin

Collat is the spending layer for Bitcoin. Every BTC deposited becomes spendable without selling. This is the core thesis: turn the world's hardest money into the world's most useful money.

### MEZO Track: Mezo Utilization

Every Collat position drives Mezo TVL through BTC deposits and MUSD borrowing volume. The protocol is Mezo-native: BTC bridging, MUSD settlement, and low-fee transactions all run on Mezo.

### MUSD Track: Supernormal dApps

MUSD becomes a spendable currency. Collat turns a stablecoin into a payment rail with auto-borrow, merchant settlement, and card integration. This is DeFi composability at the checkout counter.

---

## 10. Roadmap

| Phase | Milestone | Deliverables | Timeline |
|-------|-----------|-------------|----------|
| 1 | Smart Contracts | Vault + borrow/repay/liquidate on testnet | Now |
| 2 | Auto-Pay Integration | Checkout SDK, auto-borrow engine, dashboard | Next |
| 3 | Mainnet Launch | Audit, mainnet deploy, public beta | Soon |
| 4 | Collat Card | Virtual + physical Visa card | Future |

---

## 11. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| BTC deposited | $1M+ TVL at launch | Smart contract balance |
| Active users | 500+ in first 3 months on mainnet | Unique position owners |
| Transaction volume | $100K+ monthly checkout volume | Auto-borrow event count |
| Collat Card users | 100+ card holders (Phase 4) | Card issuance records |
| Liquidations avoided | 95%+ healthy positions | LTV tracking |

---

## 12. Risks and Mitigations

| Risk | Severity | Mitigation |
|------|----------|------------|
| BTC price crash | High | Conservative 60% max LTV, 75% liq threshold gives 25% buffer |
| Smart contract bug | Critical | Third-party audit before mainnet, bug bounty program |
| Oracle manipulation | High | Upgrade to decentralized oracle (Pyth) for mainnet |
| Low adoption | Medium | Target Mezo ecosystem first, build card for mainstream |
| Regulatory uncertainty | Medium | DeFi-native, non-custodial, no KYC threshold for MVP |

---

## 13. Appendices

### A. Glossary

| Term | Definition |
|------|------------|
| BTC | Bitcoin, the collateral asset |
| MUSD | Mezo-native stablecoin, the borrow asset |
| LTV | Loan-to-Value ratio |
| PDA | Program Derived Address (Solana) |
| Vault | Smart contract holding deposited collateral |
| Position | Per-user record of deposits and debt |

### B. References

- Pitch Deck: `Collat_Pitch_Deck.pptx`
- Source Code: `github.com/mzf11125/collat-mezo`
- Mezo: `mezo.org`
