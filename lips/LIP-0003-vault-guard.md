---
lip: 0003
title: Vault Guard – Protected Minimum
author: VOVAN1980
status: Draft
type: Core
---


## Summary


Defines the **Vault Guard** mechanism that enforces a protected minimum balance of VU for each node.


## Motivation


Users should not be able to lose everything in a single mistake or exploit. Vault Guard reserves a minimum balance that cannot be spent, providing a built-in recovery layer.


## Specification


- Each node maintains a `protected_min` amount of VU.
- Transfers that would reduce `selfVU` below `protected_min` are rejected.
- The drift logic may adjust `protected_min` slowly over time based on TC or global parameters.


## Security considerations


- Implementation must avoid rounding errors that allow bypassing the minimum.
- Protocol upgrades that change `protected_min` require careful migration.
