---
title: "Protocol: same-session k-sum table vs the prime-field mitm incumbent"
date: 2026-10-29
status: registered before any timed run (hash in REGISTRATION.sha256)
---

# Protocol: same-session k-sum table vs the prime-field mitm incumbent

**Track: generic decomposition (bounded by √r; cannot beat rho).** Both arms are generic decomposition oracles (AGENTS.md "Tracks of index calculus", 2026-10-29). Nothing here can ratchet the algebraic track.

## 1. Question

The k-sum table (isogeny-labs/crypto#2808, #2852) and the mitm pair-table incumbent (`research/prime_base_oracle_tuning_20261029`, leaderboard 20-bit row) were measured on different curves. Their rho multiples differ widely: k = 5 is 5.2–8.9× rho-neg at 24–32 bits on its curves, while the incumbent is 13.8–33.3× rho-neg on its holdout curves. The tracks rule allows a ranking only on the same instances. **On the incumbent's own holdout curves and targets, in one session, is `ic.pipeline` with `ksum:m=5` cheaper than the incumbent across the AGENTS §5a band?**

## 2. Instances (fixed, not chosen here)

These are the holdout panel of `prime_base_oracle_tuning_20261029`, exactly as in its `specs/holdout-P<b>.json`:

- curve `prime_search` at bits 20, 24, 28, 32 with seed 20261018202, resolving to `icv1-fp20-tm565-6d7d1d32`, `icv1-fp24-tm4739-bb3c4b72`, `icv1-fp28-tm1499-56171ddc` and `icv1-fp32-tm36751-2eaac89e`;
- 48 public targets with target seed 20261018211;
- measurement: 2 rounds, 1 warm-up, alternate order, measurement seed 20261018221, isolation L0, timeout 1800 s.

## 3. Arms (`specs/P<b>.json`)

| arm | role | method |
|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` |
| `rho-neg-aa` | control (A/A) | `rho.negation` |
| `bsgs-neg` | reference | `bsgs.negation` (the generic track's limit) |
| `inc` | baseline | `ic.pipeline`, `mitm:negation_folded=1`, the incumbent base N: 45 (20 bits; the promoted leaderboard selection), 162, 407, 1024 |
| `ksum4` | candidate | `ic.pipeline`, `ksum:m=4`, A = 10, 14, 21, 30 |
| `ksum5` | candidate | `ic.pipeline`, `ksum:m=5`, A = 9, 11, 15, 19 |

- **The `inc` method hashes equal the incumbent session's.** Checked with `ecbench plan` before registration: ECM1h0fc574520f33, ECM1h7291b44e96ad, ECM1h1729b9b88605 and ECM1h61a6431fe629.
- **The k-sum base sizes are frozen.** They come from `prime_ksum_table_20261028/params.json` (k = 4) and `prime_ksum_k5_20261028/params.json` (k = 5), and both were selected on those studies' own train curves. **No parameter is tuned on these curves.**

**Binary.** `ecbench-v2`, sha256 fc86ee11178da3bc373abecfaa1de532939637ab92fb86ecadf92e9ee1f9c195: isogeny-labs/crypto 26249b4 plus the #2852 code diff, which is the binary of the #2852 test panel. It contains both oracles.

**Accounting parity.** Both candidates and the baseline are `ic.pipeline` arms in one binary, priced by the same counters. Each run is one target, cold: the pair table or k-sum table is built inside the run and charged. So no build is amortised in either arm.

## 4. Stop condition: replication

Same curves, targets, method hashes and measurement seed mean that `inc` and `rho-neg` must reproduce the incumbent's recorded means in its `analysis/band-holdout.md` at w = 0, 0.1 and 1, to the 3 decimals printed there:

| bits | inc reference | rho-neg reference |
|--:|:--|:--|
| 20 | 10.515 / 11.829 / 23.655 | 1.357 / 1.474 / 2.529 |
| 24 | 13.976 / 15.168 / 25.892 | 1.013 / 1.104 / 1.919 |
| 28 | 20.478 / 22.204 / 37.737 | 0.939 / 1.031 / 1.851 |
| 32 | 32.240 / 35.309 / 62.931 | 0.968 / 1.064 / 1.930 |

If any value differs, analysis stops at that size. The cause is diagnosed (seed derivation, binary) and recorded before anything is compared. A difference that comes from per-arm seed derivation, with the means statistically consistent, is reported as such in an amendment hashed before analysis continues.

## 5. Registered test

**Primary**, in each of the 24, 28 and 32-bit cells: `ksum5` vs `inc` with `ecbench band --weights 0,0.1,1 --ranks`. Success in a cell means `ksum5 < inc` at all three weights.

**Secondary:**

- the 20-bit cell, with the same rule;
- `ksum4` vs `inc` in every cell;
- `ksum5` vs `ksum4` in every cell;
- the distance remaining, `ksum5 / rho-neg` and `ksum5 / bsgs-neg`.

**Reading.**

- If the primary test holds in a cell, k-sum is the better generic-track recipe on that same instance. That is a measured improvement of the generic-track baseline at that size, labelled generic.
- If it fails or is unresolved, the incumbent stays.
- There is one curve per size, so no exponent claim and no pooled claim. Same host, so the result is UNQUALIFIED until a second-host replay and a `claims/*.json` record. Leaderboard promotion needs both.

## 6. Predictions (registered)

| bits | predicted `ksum5 / inc` at w = 0 | basis |
|--:|:--|:--|
| 20 | ≈ 0.7 (could tie) | k5 S 7.27 on its curve vs inc 10.515 |
| 24 | ≈ 0.45 | 6.16 vs 13.976 |
| 28 | ≈ 0.33 | 6.69 vs 20.478 |
| 32 | ≈ 0.22 | 6.96 vs 32.240 |

Basis: k5 means from `prime_ksum_k5_20261028` §3.1, on different curves. Earlier this session I expected k-sum to lose or tie below about 20 bits. The k5 table contradicts that, so the 20-bit cell stays secondary. `ksum5` should still be above `bsgs-neg` at every size, because the method is generic.

## 7. Analysis

Analysis uses native `ecbench` only:

- `verify --replay-all` on every session;
- `band --weights 0,0.1,1 --ranks --json`;
- `compare` for the paired ratios with cluster-bootstrap intervals.

Plots and tables are made by node scripts (tooling). The report, graph and PDF follow the research-visuals skill. A `benchmarks/` entry and a §7a dashboard point are added only if the primary test holds.
