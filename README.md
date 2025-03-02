# RustyChain

**RustyChain** is a minimal Proof-of-Work (PoW) blockchain project written in Rust.  
It demonstrates the basics of decentralized consensus, cryptographic hashing,  
and simple peer-to-peer networking using libp2p.

---

## Features

- **Proof-of-Work**: Uses SHA-2 hashing for mining with a configurable difficulty target.
- **P2P Networking**: Gossips new blocks to peers, ensuring the network converges on a single chain.
- **Longest Chain Rule**: If a longer (valid) chain is found, the node will switch to it.
---

## Basic Commands

- **Build the project**  
  ```bash
  make build
  ```
  
- **Run the project**  
  ```bash
  make run
  ```

- **Lint the code**
  ```bash
  make lint
  ```

- **Run unit tests**  
  ```bash
  make unit_test
  ```

- **Pre-commit check** (lint + unit tests)  
  ```bash
  make pre_commit
  ```

---

## Roadmap Checklist

- [ ] Implement Block & Blockchain structures
- [ ] Implement static difficulty mining (PoW)
- [ ] Add basic networking with libp2p
- [ ] Automate difficulty adjustment
- [ ] Expand test coverage (integration tests, property-based tests)
- [ ] Set up a continuous integration (CI) pipeline

---

## License

RustyChain is distributed under the [MIT License](LICENSE).