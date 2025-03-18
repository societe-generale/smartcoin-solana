# Project Overview
Smartcoin Solana is a custom smart contract that enables to extend a standard Token2022 token with a few additional functionalities for the token's issuer(here called Registrar):
- Pause/Unpause the contract, which forbids all transfers while paused
- Approve/Disapprove transfers to special addresses (used when clients require either redemption or buy-back of their tokens)
- Forbid transfers to special addresses(Registrar/Operations) unless previously approved by the registrar operator

## Functional requirements

### Roles
Smartcoin Solana has two roles : 
- Registrar : the admninistrator of the token. The Registrar's public key is defined in the mint's metadata.
- Operations : a special address that token holders cannot transfer to unless previously approved. The Operations' public key is defined in the mint's metadata.

### Features
- Pause the contract (Registrar)
- Unpause the contract (Registrar)
- Approve transfer to Registrar or Operations (Registrar)
- Disapprove transfer to Registrar or Operations (Registrar)

### Use cases


## Technical requirements
This project has been developed with **Rust** language, using [Anchor](https://www.anchor-lang.com/) as a development environment. **Typescript** is the selected language for testing and scripting.
In addition, the project makes use of the [Solana Program Library aka SPL](https://spl.solana.com/).

### Architecture overview

### Contract information
