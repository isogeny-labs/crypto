# PKM Kummer chain at m = 8, 9, 10: degree-7 closure certificates and the m = 8, N = 12 value

Standing: **every number here is [pcc:unverified]**. No pcc claim was filed.
- Local fallback on a shared 4 vCPU / 15 GB container (L0, shared with another agent's msolve jobs), not isolab-eks.
- Not committed or pushed. This is a scratch mirror.

Parent: `research/pkm_m8_runs_20261115/`. Its section 6 kill rule is applied unchanged.

Protocol:
- `PRESET.txt` (round 1): frozen 2026-11-15T23:40:01Z. Round 1 produced no value; the container died.
- `PRESET_R2.txt` (round 2 addendum): frozen 2026-11-18T01:29:15Z, sha256 `99abeb24…0ab6`. Frozen before any
  round-2 data.

## Route (repository route tried first)

| step | result |
|---|---|
| `aws sts get-caller-identity` | `InvalidClientTokenId` (ambient keys rejected) |
| `aws eks list-clusters` | `UnrecognizedClientException` |
| kubectl | v1.31.2 downloads and runs, so the client is not the blocker |
| `gh auth status` | GH_TOKEN invalid, so no `isolab-experiment.yml` dispatch |
| `scripts/checkpoint.sh push` | S3 `InvalidAccessKeyId` (`data/checkpoint_push.txt`) |
| `tools/benchrec.py` | record `20261115T2340Z-pkm-kummer-m8-settle` open in the scratch mirror; the I1 runs ran under `benchrec.py run`; metrics recorded; no S3 upload |
| director → executor card | not possible: no Agent tool in this subagent |

There was also a disk problem. At start the root volume had 0 B free because of an orphaned 2.7 GB git fetch temp
pack from 2026-11-15. It was removed. A pull of crypto main was not retried, because it would refill the volume.

## Instruments

- **I1**: msolve 0.6.5 (apt), `-t 3 -v 2 -g 2`, RLIMIT_AS 12e9, 7200 s wall. This is reading A, the parent's
  observable.
- **I1-lowmem**: the same command with `-m 5000`, frozen in PRESET.txt s3 as the variant to use when I1 dies on
  memory. It counts for reading A only where no default run of the cell finished.
- **I4**: msolve 0.6.5 built from upstream tag v0.6.5 (5e72b9d9) with a 9-line patch,
  `scripts/msolve-0.6.5-trunc.patch`.
  - With env `MSOLVE_TRUNC_DEG=c`, the patch sets msolve's internal `max_gb_degree`, so F4 stops once every
    remaining pair has degree > c and prints the basis G.
  - Selftest with the variable unset: on m=8 N=8..11 r0 the output was byte-identical to apt msolve and the
    round-degree trace was identical (4/4 SAME, `data/selftest.txt`).
- **Certificate that 1 ∉ V_c**. The checks are run in Singular 4.3.2, which is independent code
  (`scripts/i4tool.py`):
  - (a) every input generator reduces to 0 mod G;
  - (b) every S-pair with deg lcm ≤ c reduces to 0 mod G, after Buchberger's product criterion and the strict chain
    criterion;
  - (c) no leading term is constant.
  - Then V_c ⊆ W = span{t·g : deg(t·g) ≤ c} and LM(W) = ⟨LT(G)⟩≤c.
  - Rank deficit: dim F_p[x]≤c − dim V_c ≥ δ_c = #monomials of degree ≤ c outside ⟨LT(G)⟩. δ_c > 0 means 1 ∉ V_c,
    so D_PC ≥ c+1. Equality holds because G was built inside the truncation.
- **Controls**:
  - δ counter vs brute force: 60/60 agree.
  - Checker with one basis element dropped: flagged, with 2 generators and 26 pairs nonzero.
  - Checker with one coefficient perturbed: flagged, with 1 pair nonzero.
  - Truncated run at c = 7 on a D = 7 cell: returns `[1]`.
  - The I4 c=7 stop on m=8 N=12 sits exactly at the round where both I1 traces first enter degree 8 (round 41,
    same degree-run prefix).

## Results

### Degree-c closure certificates (I4; both draws r0/r1 gave identical |G| and δ in every cell; all checks FULL)

| m | N | c | \|G\| | dim F_p[x]≤c | δ_c (rank deficit) | S-pairs checked (+ generators) | ⇒ D_PC |
|---|---|---|---|---|---|---|---|
| 8 | 8 | 6 | 116 | 38760 | 7003 | 636 (+15) | ≥ 7 |
| 8 | 9 | 6 | 98 | 54264 | 14414 | 444 (+16) | ≥ 7 |
| 8 | 10 | 6 | 120 | 74613 | 21496 | 788 (+17) | ≥ 7 |
| 8 | 11 | 6 | 160 | 100947 | 31384 | 1269 (+18) | ≥ 7 |
| 8 | 12 | 7 | 528 | 480700 | **88129** | 2947 (+19) | **≥ 8** |
| 8 | 13 | 7 | 525 | 657800 | 132126 | 4119 (+20) | ≥ 8 |
| 9 | 9 | 6 | 120 | 74613 | 19390 | 639 (+17) | ≥ 7 |
| 9 | 10 | 6 | 102 | 100947 | 33862 | 447 (+18) | ≥ 7 |
| 9 | 11 | 6 | 124 | 134596 | 47582 | 791 (+19) | ≥ 7 |
| 9 | 12 | 6 | 164 | 177100 | 65801 | 1272 (+20) | ≥ 7 |
| 9 | 13 | 7 | 823 | 888030 | **209387** | 9440 (+21) | **≥ 8** |
| 9 | 14 | 7 | 542 | 1184040 | **302729** | 2964 (+22) | **≥ 8** |
| 10 | 10 | 6 | 124 | 134596 | 44005 | 642 (+19) | ≥ 7 |
| 10 | 11 | 6 | 106 | 177100 | 69714 | 450 (+20) | ≥ 7 |
| 10 | 12 | 7 | — | — | — | — | no value (see below) |

Each truncated msolve run took ≤ 8 s and ≤ 128 MB. The Singular checks took 1–495 s.

### msolve runs on m = 8, N = 12 (the deciding cell)

| run | result |
|---|---|
| I1 r0 (default) | Killed by the container's memory cgroup (OOM) at 5063 s, peak RSS 7.2 GB, inside a single degree-7 round (153291 × 202495, 15641 pairs). This is the third censor of this run. The co-tenant used ~5–7 GB. |
| I1-lowmem r0 (`-m 5000`) | **Finished: GB = {1}, max round degree 8**, 2499 s, peak RSS 7.39 GB, 60 rounds |
| I1-lowmem r1 (`-m 5000`) | **Finished: GB = {1}, max round degree 8**, 1982 s, peak RSS 7.41 GB, same degree sequence |

So D_PC(m=8, N=12) = 8 for both draws, a value:
- lower bound L = 8 from the δ_7 certificate;
- upper bound U = 8 from a finished F4 whose rounds all have degree ≤ 8.

The msolve D_cell is also 8.

### m = 10, N = 12: no value

- The I4 c=7 run never reached a degree-8 pair:
  - first attempt: deferred after 17 min, for memory;
  - second attempt: killed by the cgroup OOM at 3458 s, 7.7 GB, still in degree-7 rounds with a pair list above
    235k;
  - third attempt, with `-m 5000`: stopped by me at 10 min, at 3.8 GB, because the co-tenant was at 7.0 GB and
    the shared cgroup would have OOM-killed one of the two jobs.
- The m=10 rounds select ≤ 4795 pairs, so `-m 5000` gives no memory relief there. The memory sits in the basis and
  pair list, not in the matrix.

## Readings under the frozen rule

| m | reading A (msolve D_cell, exact rungs) | reading B (D_PC bounds, PRESET_R2 s2) |
|---|---|---|
| 8 | N=8..12: D = 7,7,7,7,8. σ̂ = **0.200** ≥ α*(8,1) = 0.1561 → **KILL** | N=8..12 are all values: σ = 0.200. With N=13 (L=8): σ_lo = 0.229. → **KILL** |
| 9 | exact N=9..12 only; ΔN = 3 < 5 → INCONCLUSIVE | N=9..14 with L=8 at N=13 and 14: σ_lo = 0.229 ≥ 0.1740 → **KILL** |
| 10 | exact N=10..11 only → INCONCLUSIVE | N=12 has no U → INCONCLUSIVE |

- **Item verdict (m = 8, 9, 10): INCONCLUSIVE** in both readings, because m = 10 is unresolved.
- Per-arity verdict at m = 8: **KILL in both readings.** The parent's INCONCLUSIVE at m = 8 is now a value.

Derived and not deciding: the parent's cost model gives an implied exponent e = 2/(m+1 − w·m·H(σ)).
- m=8 at σ = 0.2: e = 0.620 for w=1; e = ∞ for w=2.
- m=8 at σ_lo = 0.229: e ≥ 0.715 for w=1.
- m=9 at σ_lo = 0.229: e ≥ 0.662 for w=1.
- The exponent rises with σ, so these are lower bounds.

Predictions made before the data (PRESET_R2 s5):
- Q1 (δ_7 > 0 at N=12, δ_6 > 0 at N=8..11; B-KILL at m=8): held.
- Q2 (I1 at m=8 N=12 dies before 2 h): held for the default run. The `-m 5000` variant finished.
- Q3 (m=9 B-KILL if δ_7 > 0 at N=13, 14): held.
- Q4 (m=10 and the item verdict stay INCONCLUSIVE): held.

## Deviations from PRESET_R2 (disclosed)

- s4(4) planned I1 on m=9 N=13 r0. It was not run. Reading A at m=9 needs exact rungs at N=13 and N=14 for both
  draws (span ≥ 5), so one N=13 run could not change reading A. The budget went to m=10 N=12 instead, which also
  failed for memory.
- I1-lowmem r1 was run because r0 finished, as PRESET.txt s2 requires. This is not a deviation.
- The m=10 N=12 I4 runs used AS 11e9/12e9, T=3 and `-m 5000`. Those caps were set during the run. They do not
  affect a certificate, which is checked independently.
- Total heavy wall time was about 4.5 h, within the 8 h budget.

## Not shown / limits

- Toy N ≤ 14. One presentation. D_PC is a property of this presentation over F_p, not an algorithm-independent
  bound for ECDLP. An OLS slope over integer steps is not an asymptotic statement.
- The certificates rest on Singular's `reduce` and on the counting code, which passed brute-force and corruption
  controls. A Lean or second-CAS replay was not done.

Files: `analysis.json`; `D_PC_vs_N.{svg,png,pdf}`; `data/results.jsonl`; `data/I4_bases.sha256` (the truncated bases
themselves are in the scratch tree `r2/i4/`, 143 MB, not mirrored); msolve logs and results in `data/`; scripts in
`scripts/`.
