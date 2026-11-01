# Rank 3: `m=4` generalized-root-finding disposition

Date: 2026-11-01

Scope: reconciliation of retained measured rows and the registered complete-cost
model; no new solver row

## Result

Both implemented `m=4` routes are rejected at their stated practical scopes.
The broader PKM generalized-root-finding family is not disproved: the decisive
`N=20` last-fall closure and an identical-system hybrid backend remain open.

| Route | Retained evidence | Disposition |
| --- | --- | --- |
| Direct 2+2 MITM | planted query exponent about `0.4`; uniform relation rarity contributes `p/B^2 = p^0.6`; observed 40-to-44-bit local relation-cost slope `1.41` | rejected as an end-to-end relation oracle for this base-size model |
| Registered Kummer F4, `m=4` | measured solving degrees `6,7,7` at `N=8,12,16`; `N=16` used 76,837 columns, 55.5 minutes, 8.06 GB and about `2.7e13` multiply-adds | rejected for the recorded `m=4` rho-competitive complete-cost criterion |
| `N=20` closure | projected 240--380 GB and `0.85e16`--`2e16` multiply-adds for uncapped F4; registered closure may require 80--510 GB | blocked on an appropriate 256--512 GB durable route; still a degree-growth diagnostic |
| CDCL(T)/`F_p` or QF_FF on the identical PKM systems | production theory backend absent | open and dependent on rank 2 |

## Why the existing F4 rows already decide the practical `m=4` scope

The registered optimistic complete-cost model gives the largest
rho-competitive degree at `m=4` as `d=4`.  Its 256-bit estimate places fixed
`d=7` about 25 bits above the reference, even if the degree stays flat with
size.  The measured solver stage is already at degree 6 on the smallest
recorded rung and degree 7 on the next two.  Therefore the `N=20` closure can
still distinguish rise/flat/fall of the last-fall mechanism, but none of its
registered readings makes the current `m=4`, degree-7 F4 route competitive in
that model.

This is an implementation-and-model rejection, not a lower bound on all
generalized root-finding algorithms.  In particular, it does not exclude a
different representation, a hybrid solver, or a favorable degree pattern at
larger arity.  Those are separate obligations and currently have no completing
backend.

## Evidence and limits

- Measured F4 rows and projections:
  `research/prime_ic_lead_review_20261031/figures/pkm-m4-degree.csv` and
  `research/prime_ic_program_20261029/fall_degree_relaunch/AMENDMENT_8_DRAFT_m4.md`.
- Registered reading rules:
  `research/prime_fall_degree_20261027/PREREGISTRATION.md`, Amendment 8.
- Direct MITM rows:
  `research/ic_m4_mitm_scaling_20261031/README.md` and its retained outputs.

The resource projection is not a measured `N=20` result.  Missing cloud or
checkpoint identity is not counted as negative scientific evidence.
