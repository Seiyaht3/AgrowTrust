# AgrowTrust
Collateralized agricultural input micro-financing built on the Stellar Network using Soroban.

## Problem & Solution
Smallholder coffee farmers in Huila, Colombia experience severe financial friction purchasing pre-harvest fertilizer, risking poor crop yield due to local predatory loan rates (40% APR). AgrowTrust resolves this by allowing cooperative-validated harvest commitments to be locked on-chain via smart contracts, generating instant custom tokens acceptable by local input suppliers as low-risk payment guarantees.

## Timeline
* **Phase 1:** Smart Contract Core MVP Architecture (Current Bootcamp Target)
* **Phase 2:** Local Anchor Integration & Mobile Wallet Interface

## Stellar Features Used
* Soroban Smart Contracts
* Stellar Asset Issuance
* Trustlines

## Vision and Purpose
To economically empower underbanked rural agricultural workers in Latin America by converting future harvest stability into immediate local purchasing power without reliance on predatory traditional lending institutions.

## Prerequisites
* Rust (v1.75+)
* Soroban CLI (v20.0.0+)
* Target `wasm32-unknown-unknown`

## How to Build
```bash
soroban contract build
## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the three main functions:

- `create_note()` - Create a new note with a title and content
- `get_notes()` - Retrieve all stored notes from the contract
- `delete_note()` - Remove a specific note by its ID

---

**Stellar Notes DApp** - Securing Your Thoughts on the Blockchain
