# Lane B probes on the char-2 D4 (k = 4) Frobenius form: exponent coset and constant quadratic part

Date 2026-11-12. Leads (preregistered in `research/leads_20261111/lane_B_binary_ic.md`):
`LEAD-ICB-d4bin-exponent-coset` (section 3) and `LEAD-ICB-d4bin-constant-quadratic-elimination`
(section 4, **step 1 only**). Base commit `6b7fc4520ea165c8dad946e233ba9d2599660fb6` plus the
uncommitted changes listed below. Every result statement here is **[pcc:unverified]**: no pcc
claim has been filed.

## Verdicts (short)

| Lead | Preregistered falsifier | Outcome | Verdict |
|---|---|---|---|
| exponent-coset | "Both forms fail on at least one carrying monomial of at least one instance" | Both forms fail on **every** instance (300 of 300, m = 8, 10, 12). The F_2-rank of exponent differences is 4 on every instance, and no nonzero linear form mod 2 is constant on the support | **falsified, closed** [pcc:unverified] |
| constant-quadratic-elimination, step 1 | "rank < 3 on any target (then the premise is false for those trials ...)", or "the staircase already shows the {1, S3} x 32 structure" | The 4 x 3 quadratic-part matrix has rank **2 on all 300 of 300** targets at m = 12 (and 50 of 50 at m = 8), because the monomial S2*S3 never occurs. The staircase is **not** {1, S3} x 32 | **step-1 falsifier fired: premise false as written** [pcc:unverified]; step 2 not run |

## What was changed (opt-in, default unchanged)

- `src/cryptanalysis/gaudry_quartic_bin.rs`: a new public function `macaulay_staircase` (+52 lines,
  no lines removed). It builds the weighted Macaulay matrix exactly as `solve_once` does without a
  replay: same `triangularise`, same row skip rule, same columns, same `echelon` and `is_std` rule.
  It returns the triangularised leads and the standard monomials. No solver path calls it.
- `examples/d4bin_coset_quadratic_probe.rs` (new): modes `coset` and `quad`. Cargo discovers
  examples automatically, so `Cargo.toml` is unchanged.
- Both files were formatted with `rustfmt --edition 2021` on those files only.

The run below reproduces the committed lever record bit for bit (`control_profile_m12.txt`), which
shows the default is unchanged:
`examples/d4_char2_profile --m 12 --seed 1 --trials 300` gives TOTAL 224,196 (baseline) and 191,435
(frob_roots). The component values are echelon 54,809, nf 21,913, charpoly 87,728 and roots
55,505 -> 22,977. Each value equals `research/ic_d4_char2_lever_20261023/REPORT.md` line 52.
There were 0 disagreements.

## Commands (single-threaded, release build, binary sha256 in `binary.sha256`)

```sh
CARGO_INCREMENTAL=0 cargo build --release --example d4bin_coset_quadratic_probe
RAYON_NUM_THREADS=1 target/release/examples/d4bin_coset_quadratic_probe coset 8 100    # coset_m8.txt   (5 s)
RAYON_NUM_THREADS=1 target/release/examples/d4bin_coset_quadratic_probe coset 10 100   # coset_m10.txt  (40 s)
RAYON_NUM_THREADS=1 target/release/examples/d4bin_coset_quadratic_probe coset 12 100   # coset_m12.txt  (11 min, precompute)
RAYON_NUM_THREADS=1 target/release/examples/d4bin_coset_quadratic_probe quad 12 300    # quad_m12.txt   (2 min)
RAYON_NUM_THREADS=1 target/release/examples/d4bin_coset_quadratic_probe quad 8 50      # sanity run, terminal only; same structure as m = 12
RAYON_NUM_THREADS=1 target/release/examples/d4_char2_profile --m 12 --seed 1 --trials 300  # control_profile_m12.txt
```

The `coset` instances are `generate_instance(m, seed)` for seed = 1..100, with
`SymmetrisedBin::precompute(..., 8)`. Carrying monomials come from the symbolic coefficients, as in
`parity_counts`. The run also takes the per-target support of the four `system_frobenius` components
for 3 random targets per instance. The `quad` targets are those of `d4_char2_profile`: curve
`find_bin_curve(12, 1, 16)` and target RNG seed 77, so the 300 targets are the same ones the
profile uses.

## Probe 1: LEAD-ICB-d4bin-exponent-coset

Controls:
- The existing `parity_counts` gives odd = 0 and `frobenius_reducible` holds on 300 of 300
  instances (m = 8, 10, 12; 0 precompute failures).
- On a synthetic set, the "a even" set passes (constant forms {a, b+c+d, a+b+c+d}). A planted
  monomial [1,0,0,0] is rejected (form "a" no longer constant).

| m | instances | #carrying | F_2-rank of differences | constant mod-2 forms | all a-even (S2) | all b-even (S3) | per-target support rank |
|---|---|---|---|---|---|---|---|
| 8 | 100 | 21 (all) | 4 (all) | none | 0 | 0 | 4 (300/300) |
| 10 | 100 | 21 (all) | 4 (all) | none | 0 | 0 | 4 (300/300) |
| 12 | 100 | 21 (all) | 4 (all) | none | 0 | 0 | 4 (300/300) |

[pcc:unverified] The support in (S2, S3, s4, P) is **the same 21 monomials on all 300 instances**:

```
1, P^4, s4, s4P, s4P^2, s4P^3, s4^2, s4^2P, s4^2P^2, s4^3, s4^3P, s4^4,
S3, S3P^2, S3s4, S3s4P, S3^2,
S2s4, S2s4P, S2s4^2, S2^2
```

"a even" fails on 3 of the 21 monomials (S2s4, S2s4P, S2s4^2: 18/21 a-even). "b even" fails on 4
(S3, S3P^2, S3s4, S3s4P: 17/21). The prediction expected that both forms fail (P = 0.95 on failure),
and they did. One detail of the prediction was wrong: it assumed S2 and S3 are both carrying
monomials. S3 is, but S2 alone is not. S2 appears only as S2*s4, S2*s4*P, S2*s4^2 and S2^2. The
exponent vectors are recorded in `coset_m*.txt`. The lead is closed.

## Probe 2: LEAD-ICB-d4bin-constant-quadratic-elimination, step 1 (m = 12, 300 targets)

| Item | Result [pcc:unverified] |
|---|---|
| (a) monomials present | The same 21 monomials as above. 1197 of 1200 components have all 21; 3 have 20 (an accidental zero coefficient on 2 monomials). S2*S3 is **never present** |
| (b) rank of the 4 x 3 matrix of (S2^2, S2S3, S3^2) coefficients | **2 on 300 of 300** (the S2S3 column is identically zero) |
| (c) triangularised leads | [S2^2, S3^2, S2*s4^2, s4^4] on 300 of 300 |
| (c) leading terms in (S2, S3) | S2^2, S3^2, S2, 1 |
| (c) standard monomials per (S2, S3) pattern | {1: 32, S3: 16, S2: 12, S2S3: 4} on 300 of 300 (total 64) |
| control: staircase size vs solver `dim` | 64 = 64 on 300 of 300 (solver dim 64 on all) |

Reading against the registered falsifiers:
- "rank < 3 on any target": **fired on every target**. The premise is that row reduction gives one
  combination g without a quadratic term and three with leads S2^2, S2S3, S3^2. That premise is false
  on all trials: there are **two** combinations free of the quadratic part (the two components whose
  triangularised leads are S2*s4^2 and s4^4), and S2*S3 never appears. The lane file says the lever
  is then "conditional" for those trials. Here the condition fails on every trial, so the claim as
  written cannot be tested in step 2.
- "staircase already shows the {1, S3} x 32 module structure": **did not fire**. The staircase has
  four (S2, S3) patterns with 32/16/12/4 standard monomials. It is not a free {1, S3} module.
- The lane registered step 2 (a day of Rust, outside the 20-minute budget) with P(> 1.3) = 0.55 on
  its own count. It was not attempted.

## Measured side fact (relevant to LEAD-ICB-d4bin-chain-variable-sweep)

[pcc:unverified] The profile control prints `chains 12.0` at m = 12, seed 1, 300 trials. The lane
file *inferred* k = 16 for the chain count ("k = 16 inferred, not recorded"). The measured value
with the default mvar = 3 (P) is 12. 65 * 12^3/3 = 37,440, while the counted charpoly is 87,728
muls/trial, so the (dim + 1) * k^3/3 model in the lane file does not reproduce the counted charpoly
either. This was not investigated further here; it belongs to the chain-variable-sweep lane.

## What is blocked

Nothing in these two probes needs AWS, S3 or Iso Lab: both are exact structural computations, with
no timing claims. These steps were not done:
- No bench.record or VALIDATION.json was produced. This is not a benchmark, and no row moves.
- The research-visuals report, graph and PDF were not made (the workflow scope says no docs or
  benchmarks edits; the coordinator decides).
- No pcc claim was filed.

## Next step (suggestions, not run)

1. A variant of the elimination lead built on what step 1 found [pcc:unverified]. There are two
   equations linear in (S2, S3): leads S2*s4^2 and s4^4. Each has the form
   S2*A_i(s4) + S3*B_i(s4, P) + C_i(s4, P), where A_i is built from s4, s4P and s4^2, and B_i from
   1, P^2, s4 and s4P. Cramer's rule gives S2 and S3 as rational functions with denominator
   A1B2 + A2B1. Substituting them into the S2^2 and S3^2 equations gives a bivariate (s4, P) system.
   This is a new preregistration, not a rescue of this one. Its first item would be a free degree
   count (the degrees of the two resulting bivariate equations and their Bezout bound against 64).
2. The fact that S2 enters only through S2*s4 and S2^2 suggests s4 = 0 is special (the S2 terms
   vanish on it). This has no cost consequence by itself.
3. The chain-count discrepancy (12 measured against 16 inferred) should go to the chain-variable-sweep lane.

## Files

- `coset_m8.txt`, `coset_m10.txt`, `coset_m12.txt`: raw output of probe 1
- `quad_m12.txt`: raw output of probe 2, step 1
- `control_profile_m12.txt`: default-unchanged control
- `binary.sha256`: sha256 of the probe binary
- source: `examples/d4bin_coset_quadratic_probe.rs`, `src/cryptanalysis/gaudry_quartic_bin.rs` (`macaulay_staircase`)
