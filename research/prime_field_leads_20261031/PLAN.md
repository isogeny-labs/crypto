# Prime-field leads, 2026-10-31: pre-registered plan

Companion to
[`research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md`](../notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md),
which states each lead, its falsifier and its stop rule. This file freezes the
thresholds before any run and names the output location of each lead.

| Lead | Kind | Model | Output directory | Decision threshold (frozen) | Null prediction |
|---|---|---|---|---|---|
| L1 | theorem draft | Sonnet | `L1_lower_bound/` and `research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md` | every lemma proved or its gap named | n/a |
| L2a | counted ladder | Sonnet | `L2a_slope_e_m3/` | bootstrap CI of e below 0.5 reopens; CI containing 2 closes | e ≥ 1 |
| L2b | counted ladder | Sonnet | `L2b_dreg_trajectory/` | flat d_reg while the random control grows reopens | both grow alike |
| L3 | write-up | Haiku | `L3_precomputation/` and a section in `RESEARCH_ECDLP_STATE_OF_THE_ART.md` | n/a | n/a |
| C1 | derivation | Haiku | `C1_lift_torsion/` | splitting argument holds or its error is named | holds |
| C2 | counted comparison | Sonnet | `C2_volcano/` | gap above 2x in counted work or one full solving degree, crater vs floor | identical within spread |
| C3 | scoring table | Haiku | `C3_heavy_tail_ceiling/` | n/a | all mechanisms below the ceiling |
| C4 | counted comparison | Sonnet | `C4_pseudo_generic/` | energy ratio departs from 1 beyond the binomial spread and grows with p | ratio 1 within spread |
| C5 | closure note | Haiku | `C5_special_primes/` | n/a | n/a |

Rules in force for every lead:

- Counted metrics only (field multiplications, solver decisions, degrees,
  ranks, matrix sizes). No wall-time claim: host load average was about 90
  on 14 cores when this plan was written.
- No Python for result generation (AGENTS.md "Implementation language: no
  Python"); Rust harnesses, msolve and `sage-run` as tools; Python only to
  plot recorded values.
- Each directory holds `REPORT.md`, at least one SVG graph with its data
  beside it, and a PDF, per the `research-visuals` skill. A write-up with
  fewer than two comparable values leaves a diagram of what was derived.
- Every attempted cell is reported, including the ones that timed out, with
  the recorded reason.
- No sentence claims a speedup, win or exponent change unless a counted
  measurement in the same directory admits it.
- Conductor was unreachable (404) at planning time; no task card exists for
  this work.
