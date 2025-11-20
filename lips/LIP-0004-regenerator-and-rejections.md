---
lip: 0004
title: Regenerator and Rejection Statistics
author: VOVAN1980
status: Draft
type: Core
---


## Summary


This LIP specifies how LVS tracks rejected actions and uses regenerative logic to restore network health.


## Specification


- Each tick produces:
- number of successful transfers;
- number of rejected transfers;
- aggregate statistics for VU/TC.
- Regenerative logic may:
- slowly restore TC for long-lived honest nodes;
- shrink influence of consistently bad actors.


The simulator in `src/sim.ts` MUST log rejection statistics so researchers can tune parameters.
