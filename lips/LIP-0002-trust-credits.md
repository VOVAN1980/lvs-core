---
lip: 0002
title: Trust Credits (TC)
author: VOVAN1980
status: Draft
type: Core
---


## Summary


Defines the role of **Trust Credits (TC)** in LVS and how they evolve over time.


## Motivation


TC is a core primitive used to:


- weight node influence;
- limit value flows;
- reward long-term honest behaviour.


A clear model is required to keep implementations compatible.


## Specification


(High-level sketch – detailed math is kept in the research papers.)


- Each node `i` has trust `TC_i` within a bounded range.
- On each tick, TC is updated using a drift function depending on:
- previous trust;
- success/failure of actions;
- network-level parameters.


Nodes with very low TC may:


- lose the ability to process large VU transfers;
- be deprioritised or rejected by consensus.


## Security considerations


- TC must not be easily farmed or reset.
- Sudden TC drops should not cause cascading failures.
