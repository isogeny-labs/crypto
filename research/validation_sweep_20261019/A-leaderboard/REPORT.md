---
title: "Validation sweep, slice A: the crypto IC leaderboard"
subtitle: "Audit of docs/ic/LEADERBOARD.md and docs/ic/leaderboard.json at tree cb2ec6873, 2026-10-19"
---

# Summary

**Scope.** Every row of Tables A, B and C, and every section outside them (n37, n9 F6-IC, the F6-IC ladder, F4/F5/F6, n41/n53, conductor-gap, N83 holdout), plus two groups that exist only in `leaderboard.json` (the oracle-pricing cells and the linear-algebra engine rows) and the five frontier-audit statements. 55 verdict records in [`verdicts.jsonl`](verdicts.jsonl).

**This audit repairs nothing and changes no research state.** It does not regenerate the leaderboard. Rows that should be demoted are listed under "Demotion flags".

@@VERDICT_TABLE@@

**What held.**

- Every published number in Tables A, B and C recomputes exactly from the raw records (all digits). So do the n37, F6-IC ladder, F4/F5/F6, n41/n53, conductor-gap, N83 and linear-algebra figures I could reach from raw records (`recomputed_ok`).
- Every sampled recovered scalar verifies. An independent verifier, written for this audit and sharing no code with the repository, checked [d]G = Q and [r]G = O on 150+ seeded records over GF(2^n) and GF(p), including 5 of 5 N83 points on GF(2^83). A negative control (scalar + 1) fails as it should.
- The leaderboard's own central claim, that no row is below its reference, survives everything below. Where I priced the reference differently, IC stays above rho.

**What did not hold, in order of weight.**

1. **Table A's reference rho is not priced like its IC arm.** The IC unit is one 1024-lane batched addition; rho runs 32 lanes and costs 9.7 to 11.8 units a step against a canonical step of 4.6 to 5.4. At the canonical step the published x reference of 5.2 to 21 becomes 9.1 to 41, so the gap to rho is understated by 1.75 to 1.96 times. Table C prices its rho at the canonical step, so A and C use different rho.
2. **The inherited-F4 and F6-IC counters do not replay on current main.** See "Known failure": on this host the F6-IC ladder sessions reproduce 24 of 72 replays at m = 13 and at m = 19, and the root-finder session reproduces 72 of 168, not 120 of 168. The failure is wider than the 48 inherited-F4 rows the board names.
3. **Section 5a was not applied.** No row carries the 0 / 0.1 / 1 band. I computed it: Table B's order holds at every weight (IC stays 4.7 to 55 times rho), and the repository's own `ecbench band` makes every n37 IC pair "unrankable (uncounted unpriced work)".
4. **Table C's reference is from a different experiment** (S20, 2026-09-26) from its IC figure (S22 isolated, 2026-09-30), re-checked at one size only.
5. **Freeze order is not demonstrable for 8 section groups** (squashed PRs put protocol and data in one commit).
6. **One admitted row has no locatable evidence:** the n61 compact-orbit online bar.

# Method and model split

Judgement (rho fairness, pricing, classification, verdicts) was done by the Sonnet 5.5 session. Three mechanical cards ran on Haiku subagents with frozen commands: the provenance cross-check, the independent re-run of the three recompute scripts, and the seeded Table A replay. I diffed the first two against my own runs: byte-identical. The Table A replay output was reviewed row by row. Replays too large for a subagent (native ladder replays, `ecbench band`) were run by the session itself.

The replay seed was committed before any replay (`REPLAY_SEED.md`, commit 0cb3bc9e3). Runs are drawn by `sha256(seed|row|run)`; no run was dropped or swapped. One draw was wrong: the `n53:ic-k16` row of the session replay returned records of the n41 curve, because ic-k16 never verified at n = 53 (0 of 80). I removed it from the n53 row's replay count and say so in that row.

Conductor: `conductor check` reported the scope clear; `coord_start_work` returned HTTP 500 three times, so the session was not registered as a claim.

# Known failure: which rows do the root-finder counters feed?

**Published:** 120 of 168 replays identical, 48 inherited-F4 rows differing in solver phase counts (session `ECBS1hfd5bf1885227`).

**Fresh replay-all, this host (x86_64 Linux), binary built from main `cb2ec6873`** (`raw/replay_all/`):

@@REPLAY_TABLE@@

Every failing row is an IC row. All rho rows replay. In the root-finder session the four failing arms are ic-f4-m2, ic-f4-m3, ic-rootfind-m2 and ic-rootfind-m3, 24 rows each, so the root-finder arms fail too, which the published receipt says they did not. The committed 2026-10-05 ladder audits passed on their own binaries, so the drift came with later code or a different host class; I did not bisect it. The same current binary adds a `row_ops` counter to `ic price`, which is the one counter difference I saw in the Table A and C replays and which changes nothing archived.

**Counters at fault:** solver word XORs, reductions, geometric additions and the phase GAE derived from them (the published seq 58 shows 301,961 word XORs now against 300,531 archived).

**Rows they feed:**

| board row | consumes | status after the fresh replay |
|:--|:--|:--|
| F6-IC E_0 ladder, m = 13, 19, 23 (F4 and F6-IC S/rho, median log2 W4/W6) | inherited-F4 and F6-IC solver counters directly | m = 13, 19: replay fails on every IC row. m = 23: @@N23@@ |
| F4/F5/F6 cross-family (inherited F4 S, F6-IC S, W F5/F4, W F4 matrix/inherited, median log2 W inh/W F6) | the same counters | @@XFAM@@ |
| Latest frontier audit, replay block | reports the failure | says 48 F4 rows; fresh replay shows the wider failure |
| n9 F6-IC control | its own `phase_closure` replay certificate, not the ecbench counters | not affected by this failure; not replayed here |
| Tables A, B, C | `ic price` and `ic boundary` oracles (pair table, MITM, semaev), not F4 | not affected: 18 of 18, 13 of 13 instances and 27 of 27 replays match |
| n37, n41/n53 | counted MITM-Frobenius PDP, not F4 | not affected; committed Linux audits 320/320 (cited, not re-run) |

The effect on S: the root-finder study's F4 stage-count comparison is not current-binary evidence; for the ladder and cross-family rows, the S lower bounds include charged geometric additions, which are among the drifting counters. The recovered scalars verify in every case.

# Rho reference, row group by row group

| table | reference named by the protocol | strongest available? | priced in the same unit? | kernel advantage on the IC side |
|:--|:--|:--|:--|:--|
| A (rule v3) | one-target rho as built, same process | strongest implemented: 100 ns/step against 228 ns/step for the standing fixture (the protocol's own admissibility check). Not the strongest possible: rho later gained 20% in instructions (2026-10-14). | yes, wall-clock units, same process | yes: unit = 1024-lane batched add; rho 32 lanes, 2.2 to 2.3 times the canonical step. IC-side kernel work cut the unit 2.3 to 3.2 times against section 23 |
| B | matched negation-map rho, A = 2 | yes for prime and random binary (negation is the only automorphism) | no: IC natives priced by pinned calibration, rho canonicalisations uncharged (45 to 4,196 per run) | none found |
| C | batch rho on the same 32 targets, canonical step, from S20 | yes, at the canonical step | not the same process or session as the IC figure | none; rho gets the canonical (batched) step |

The task statement mentions a rule v7 with rho at 9.4 to 10.6 units a step against 4.2 to 4.9. No rule v7 exists in this tree: `research/ic_tool_program/rule/` holds v2 (declared, never run) and v3. The v3 figures here are 9.7 to 11.8 against 4.6 to 5.4, the same pattern.

![Left: Table A, x reference as published (rho as built) and with rho at the canonical step. Middle: Table B x reference under the section 5a band (0, 0.1, 1 GAE per native operation) against the published value. Right: replays reproduced by `ecbench verify --replay-all` on this host. Values from raw/.](figures/audit_summary.png)

# Pricing (section 5a)

- **A, C:** the unit is wall-clock for both arms, so there are no per-operation unpriced counters; the band was not run and is not directly applicable. `pricing_ok` is false because the board does not say that, and a counted check is missing.
- **B:** recomputed band (`raw/recompute_B_haiku.json`). IC / rho at w = 0, 0.1, 1: fp10 6.26 / 6.24 / 6.11 (published 6.34); fp24 13.0 / 13.6 / 16.3 (13.46); f2m27 37.7 / 40.7 / 54.8 (38.2). The order never changes; the magnitude can be 43 percent higher than published at w = 1.
- **n37:** `ecbench band` (`raw/band/`): every IC pair unrankable because field arithmetic, hashing and modular combination are named unpriced counters with no count. The K16-over-K42 ranking (0.349) is a ratio of lower bounds that section 5a does not allow to rank.
- **Ladder and cross-family:** F6-IC against F4 is "unresolved inside the band" at every size; rho is below every IC arm except F4 at m = 13, which is unresolved.
- **conductor-gap:** rho below IC at every weight.
- **n41/n53:** "incomplete" (IC arms did not verify).

# Distance remaining (success criterion)

No row is below its matched rho. The distance is the x reference, recomputed:

| table | x reference range | smallest | largest |
|:--|:--|:--|:--|
| A (rho as built) | 5.23 to 20.97 | f2m47 | f2m61 |
| A (rho at canonical step, recomputed) | 9.13 to 41.13 | f2m47 | f2m61 |
| B | 5.47 to 38.21 | fp12 | f2m27 |
| C | 3.62 to 14.90 | f2m41 | f2m19 |

n37 K16 against strong rho: 5.03 counted (lower bounds), 4.70 complete-solve instructions; target-only online instructions 0.153 of rho. This audit claims no improvement.

# Demotion flags (not applied)

1. **F6-IC ladder, m = 13, 19 (23 pending):** counters do not replay; freeze not demonstrable. Mark as historical, not current-binary evidence.
2. **F4/F5/F6 cross-family:** see table above (@@XFAM_SHORT@@); freeze not demonstrable.
3. **Frontier replay-block statement:** replace "48 inherited-F4 counter mismatches" with the fresh scope.
4. **n61 compact-orbit online bar:** keep as unvalidated until the 2026-10-12 sessions are linked from the board's sources.
5. **Table A header:** disclose the 32-lane versus 1024-lane kernel and show the canonical-step ratio beside it.
6. **Table C header:** it says rho is measured "in the same process"; for the isolated re-run it is S20's. Say so.
7. **n41 K16 256x quotient:** label as a survivor mean (23 of 80 verified).
8. **n37 K16 "lead":** label as unrankable under section 5a.

# Per-row notes

See `verdicts.jsonl`; each record's `notes` field carries the row-level reasoning. Table A, B and C records are generated by `scripts/build_verdicts_ABC.sh` from `raw/`, the section records by `scripts/build_verdicts_sections.sh`.

# Provenance and freeze order

First-commit order of protocol against raw data (`raw/provenance_haiku.jsonl`, identical to my own run):

| evidence | protocol before raw data in git? | notes |
|:--|:--|:--|
| Table A, rule v3 | yes, 2026-10-06 vs 10-07, no later edits | |
| Table B, Round 5 | yes, but BOUNDARY_TARGETS.md is edited 41 times afterwards | living document; recipe chosen as min of 9 on the measured targets |
| matched rho (B reference) | same commit | README says boundaries were committed first; not checkable |
| Table C (S22) | yes; v2 amendment before isolated runs | |
| n37 four studies | yes; session commit 5508af3 precedes data | k8-k16 analyzer frozen after data |
| conductor-gap, frontier rho, la_engines | yes (la_engines amended five times, adaptively) | |
| N83 holdout | yes: answers frozen 10:17 'before grid result', selection 11:55, results 14:21 | |
| n9, F6 ladder, cross-family, n41/n53, frontier 20261013 | same squashed commit | not demonstrable |

# Not covered, and the command that would cover it

- Ir rows (n37 K8/K16, online): sums of recorded per-workload values, not the raw callgrind zip. `ecbench callgrind-ir --prefix` on `RAW-CALLGRIND.zip` contents.
- Seeded replay of the 144-run cold bound and the n61 sessions (not located).
- R13 and R19 independent scalar checks: random binary curves carry no a, b in the record.
- Table C independent certificates: no target points archived.
- Native replay-all of the n37, n41/n53 sessions (Mac arm64 records): `ecbench verify --dir <session> --replay-all`.
- Bisecting the counter drift: replay the ladder session on the commit of each audit in `f6_ic_ecbench_ladder_20261005/audits/`.
- A repository-supported pricing band for Tables A and C: needs per-operation counts for the descent and PDP phases.

# What this audit does not establish

It does not show any row is wrong in its arithmetic, and it does not show IC beats or approaches rho anywhere. It does not establish that the replay drift is a defect in the counters rather than an intended change that the old sessions predate. Verdicts say how far the evidence supports each row's published statement; they are not a research decision.
