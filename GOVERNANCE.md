# LVS Core Governance


This document describes how decisions are made for the **LVS Core** repository.


## Roles


- **Maintainer** – has write access to the repository, reviews and merges pull requests.
- **Contributor** – anyone submitting PRs or issues.
- **Researcher** – authors of LIPs and protocol-level designs.


Initially, governance is simple: the project is maintained by **@VOVAN1980**.


---


## Decision process


### Code and documentation


- Small fixes and improvements can be merged after at least **one maintainer review**.
- Behaviour‑changing changes must:
- be covered by tests, and
- reference an accepted or draft **LIP**.


### LVS Improvement Proposals (LIPs)


1. A new LIP is opened as a PR in the `lips/` directory.
2. The LIP is discussed publicly in the PR.
3. When consensus is reached, the LIP is marked as **Accepted** or **Rejected**.
4. Implementations referencing the LIP must follow the final text.


---


## Repository vs protocol


This repository governs the **reference implementation and docs only**.


The long‑term production protocol and network governance (validators, on‑chain voting, etc.) will be defined in separate documents and may involve a foundation or DAO once LVS matures.


---


## Changes to this document


Changes to `GOVERNANCE.md` require a PR reviewed and approved by a maintainer.
