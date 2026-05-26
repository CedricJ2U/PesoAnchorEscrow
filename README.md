# PesoAnchor Escrow

An instant, low-cost milestone escrow dApp built on Soroban to prevent cross-border wage theft for LATAM freelancers.

## Problem & Solution
Digital migrant freelancers working across the Colombia-Venezuela border lose significant percentages of their earnings to traditional banking friction, localized asset freezes, and extortionate cash-out agents. This dApp uses Soroban smart contracts on the Stellar network to lock contract funding in a programmatic escrow account, unlocking funds instantly to the worker's wallet without financial intermediaries.

## Timeline
Developed within a standard 48-hour hackathon/bootcamp timeline.

## Stellar Features Used
* **Soroban Smart Contracts:** Logic container ensuring automatic execution.
* **Stellar USDC Assets:** Provides stability for cross-border transactions.
* **Trustlines & On-chain Security:** Restricts illegal asset injection or asset drain attacks.

## Vision and Purpose
To empower underbanked cross-border gig economy workers in emerging markets, boosting local financial independence through zero-trust decentralized tooling.

## Prerequisites
* Rust: `rustc 1.70.0` or higher
* Soroban CLI: Version `20.0.0` or higher
* Target target: `wasm32-unknown-unknown`

## How to Build
```bash
soroban contract build