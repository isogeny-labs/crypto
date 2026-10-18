# Round 315 preregistration: multi-target amortization of the transported-log candidate

Date: 2026-10-17

## Open request

Continue the end-to-end index-calculus line toward parity with a
square-root Pollard-rho reference.  This is the separate, explicit
multi-target research question that AGENTS.md reserves for amortized
measurement; it does not replace the one-target rule and the Round 314
one-target parity gates stay closed as they were measured.

## Motivation from Round 314

The hash-scheduled transported-log candidate's cold one-target cost is
2.121–3.267 times the matched strong signed-Frobenius rho reference, and
the folded pair table alone consumes 60.8–69.0% of the cold counted GAE
(`../ecbench_known_log_transport_round314_20261014/RESULT.md`).  The
table is target-independent: it depends only on the base and the curve.
If one shared table serves `T` public targets, the amortized per-target
unit is

```
S_amortized(T) = (table + verification + base + T * online) / (T * sqrt(r)),
```

and the mechanism crosses the rho reference when
`table / (T * sqrt(r)) > online_deficit`, i.e. at `T >= 8` by the Round
314 n37 phase split (0.608/0.596 vs 0.281/0.596 per sqrt(r) unit).
This is a mechanism screen on the amortized unit, not a one-target
speedup claim.

## Hypothesis

With one cold shared base and pair table charged once and divided over
`T` public targets, the amortized per-target GAE of the hash-scheduled
transported-log candidate falls below the matched strong rho one-target
cost at the degree-37 cell for `T >= 8`, with every target's recovered
scalar verified by full-point replay.

## Frozen method boundary

Candidate method ID: `ic.transported_logs_hash_multi`.

- Factor base: `known-log-orbit:columns=<K>,scalar_cap=1000000,schedule=hash,seed=<S>`
  with the Round 314 frozen `K` rule for each `(r, n)`.
- `amortized_targets=T` public targets per execution: the workload's own
  public target first, then `T-1` derived co-targets from
  `public_binary_target` under the same `hash_to_subgroup_v1` law and a
  frozen `co_target_seed`.  No co-target may equal the workload target.
- Every phase is charged exactly once per execution: shared base, shared
  folded table, shared transport verification, and per-target query, PDP,
  relation check, descent, and scalar replay.  The derived co-targets'
  hash derivation is uncharged, exactly as the workload target's own
  derivation is outside every solver's measured interval.
- The record's recovered logarithm answers the workload target; the
  report carries all targets' attempts and the whole shared cost.  The
  amortized per-target unit is computed only in the frozen analyzer as
  `total_gae / T`.

Reference: `rho.signed_frobenius_strong` with the Round 314 parameters
(`lanes=32, dp_bits=4, step_cap_factor=2000`), one target per execution,
unchanged.  rho performs no reusable precomputation, so its per-target
cost is the same cold one-target unit as Round 314; the comparison is
therefore amortized candidate versus cold rho per target.  The
distinguished-point fleet control from
`research/n83_precomputed_rho_control_20261016` (PR #1987) is the
standing caveat: a rho fleet sharing distinguished points across targets
can also amortize, so a pass here is a mechanism screen for
multi-target workloads, not an asymptotic IC advantage over every rho
deployment.

## Development and freeze rule

1. Correctness tests use the registered `a=0, n=17` Koblitz instance
   and deterministic point-only targets (committed Cargo test).
2. Development may run the degree-37 cell with target seed
   `20261017315001` and co-target seed `20261017315002` for
   `T in {1, 4, 8, 16}` to fix nothing but the schedule; the frozen
   `K` and attempt caps come from Round 314 and may not be retuned.
3. Holdout targets use seed `20261017315003`, are public
   hash-to-subgroup points, and may not influence any frozen rule.

## Holdout panel and gates

Primary cell: `icv1-f2m37-tm534059-32aad96b` (Round 314's degree-37
instance).  Eight independent public targets, five measured rounds, one
warmup.  Arms: the multi-target candidate at the frozen `T`, the
matched strong rho reference, and an identical A/A control.

Promotion gates, all conjunctive:

- every record verifies and replays; every target's scalar replays
  `[d]G = Q`;
- exact counted A/A for both candidate and reference;
- amortized per-target `S(T) / S_rho <= 1.00` at the degree-37 cell
  for the frozen `T`, with a 95% paired interval excluding 1.00;
- the shared table, verification, and base costs are reported separately
  from the per-target online cost, and `T` is named in every figure.

Failure of any gate closes the round as a negative amortization result.

## What this round does not claim

- No one-target speedup: Round 314's one-target gates stay failed.
- No P-256 transfer: the P-256 Dickson membership gate is untouched.
- No asymptotic claim: `T` is finite and named; the distinguished-point
  rho fleet control is the standing caveat on any "cheaper than rho"
  reading.
