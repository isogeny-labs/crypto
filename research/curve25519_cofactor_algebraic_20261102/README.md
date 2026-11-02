# Cofactor-aware algebraic oracle (`pkm-semaev:lifts=1`) on the k = 20 Curve25519 miniature

Protocol: [PROTOCOL.md](PROTOCOL.md) (frozen before data; cells, seeds and rules unchanged).
Everything below is scoped to k = 20 and the five cells measured; nothing is generalised.
F4 multiply-adds are native, unpriced work (AGENTS.md section 5a). No rho arm, no cross-method claim, no leaderboard row.

## What was built

- `src/cryptanalysis/ic_framework/pkm_semaev.rs`: parameter `lifts=1`. `prepare` computes the rational h-torsion `E(F_q)[h]` with `cofactor_fiber` (charged to set-up); `decompose` tries `R + K` for each K whose lift abscissa has not yet been tried (`K = O` first), one F4 system per lift, first liftable relation wins; every lift addition and every F4 call is charged. `setup_native()` reports fiber size, cofactor, completeness and samples. With h = 1 it is the blind oracle.
- `src/bin/curve25519_family.rs`: `f4cost ... <cap_s> 2 full <0|1>` reports `lifts`, `hits_exact`, `setup_group_ops`, `setup_native`.
- `run_cells.sh`: shell runner (`ulimit -v 3500000`, `timeout 900`, per-cell stderr and `.status`), appends to `analysis/f4cost_lifts.jsonl` (7.4 KB, committed).

The lift code compiled as written (lib and bin) with no source fix. The one defect found was in the new unit test, not in the oracle (see Deviations).

## Unit test

`cargo test --release --lib pkm_semaev`: `test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 6054 filtered out; finished in 542.29s` (the 9 existing tests plus `cofactor_lifts_match_the_brute_force_fiber`).
Counts printed by the new test (h = 4 curve over the 10007 field, 80 random subgroup targets): blind-decomposable 3, lift-decomposable 21 (brute force over the torsion found by enumeration); 36 relations from a non-identity lift summed over the two lifted oracles (guess = 0 and guess = 1). All verdicts equalled the brute-force sets; every relation had `Sum - R` in E[h], `[h]Sum = [h]R` and satisfied the pipeline row `sum coef*x = h*a (mod r)`; h = 1 gave identical relations with and without `lifts=1`; extra F4 calls were charged.
The test prints that line with `eprintln!` (added to the WIP test).

## Measurement

Binary `/home/user/frozen-bin/curve25519_family_lead2`, sha256 `9aa09afccbc9f59bc302252db81dc1e6a99becfc5e98a810b438f80a266a97bc`, built from commit ad50d0892f plus the uncommitted test fix. One process at a time on one host, pairs interleaved (lifts=0 then lifts=1, same seed). All 10 runs finished (`.status` all ok), none capped (longest 281 s of the 600 s cap), `undecided` = 0 everywhere.
Fiber: h = 8, size 8, complete, in every lifted cell.

| form | t | guess | arm | targets | hits | hits_exact | hits verified | F4 calls/target | mul-adds/target | mul-adds/hit | set-up group ops | wall s |
|:--|:-:|:-:|:-:|:-:|:-:|:-:|:-:|--:|--:|--:|--:|--:|
| isogeny | 4 | 0 | lifts=0 | 200 | 0 | 0 | 0 | 1.0 | 5.23e5 | undefined | 0 | 1 |
| isogeny | 4 | 0 | lifts=1 | 200 | 1 | 0 | 1 | 7.995 | 4.18e6 | 8.36e8 | 181 | 10 |
| isogeny | 5 | 0 | lifts=0 | 200 | 0 | 0 | 0 | 1.0 | 3.96e6 | undefined | 0 | 3 |
| isogeny | 5 | 0 | lifts=1 | 200 | 0 | 0 | 0 | 8.0 | 3.17e7 | undefined | 34 | 24 |
| isogeny | 6 | 0 | lifts=0 | 200 | 0 | 0 | 0 | 1.0 | 5.55e7 | undefined | 0 | 11 |
| isogeny | 6 | 0 | lifts=1 | 200 | 0 | 0 | 0 | 8.0 | 4.44e8 | undefined | 141 | 91 |
| bits | 5 | 3 | lifts=0 | 200 | 0 | 0 | 0 | 8.0 | 3.04e6 | undefined | 0 | 7 |
| bits | 5 | 3 | lifts=1 | 200 | 0 | 0 | 0 | 64.0 | 2.43e7 | undefined | 34 | 56 |
| bits | 6 | 3 | lifts=0 | 200 | 0 | 0 | 0 | 8.0 | 1.72e8 | undefined | 0 | 35 |
| bits | 6 | 3 | lifts=1 | 200 | 4 | 0 | 4 | 63.18 | 1.36e9 | 6.81e10 | 34 | 281 |

Ratios lifts / no-lifts (exact counts): F4 calls per target x7.995, x8.000, x8.000, x8.000, x7.898 (cells in table order); mul-adds per target x7.995, x8.000, x8.000, x8.000, x7.898. Mul-adds per relation ratio: undefined in every cell (the no-lifts arm has 0 hits in all five).

Hit rates, Poisson (exact) 95% intervals on the count, per 200 targets: 0 hits [0, 3.69] (rate 0 to 1.8%); 1 hit [0.03, 5.57] (0.01% to 2.8%); 4 hits [1.09, 10.24] (0.5% to 5.1%). Pooled over the five cells: no-lifts 0 / 1000 targets, lifts 5 / 1000. Descriptive and post hoc (not preregistered): if the 5 pooled hits were split evenly between arms, all 5 falling in the lifts arm has two-sided probability 2 x (1/2)^5 = 0.0625.
Exact projected coverage lambda of the factor bases (support from `covered_points / r`, in the jsonl): 1.4e-4, 5.5e-4, 1.3e-3 (isogeny t = 4, 5, 6), 3.2e-4, 1.2e-3 (bits t = 5, 6), i.e. 0.03 to 0.25 expected exact hits per 200 targets and at most 8x that with lifts; the cells are underpowered, as the protocol stated.
All 5 lifted hits came from a non-identity K (`pkm_lift_hits_nonidentity` 1 and 4), so `hits_exact` = 0; each passes the pipeline rule `[8]Sum = [8]R`.

Figure: [analysis/lifts.svg](analysis/lifts.svg) (mul-adds per target and per hit, both arms; n/a where an arm has no hit).

## Predictions

- **L1** (hit rate up 3-5x, never above 8x): not testable. The no-lifts arm has 0 hits in every cell, so no factor can be formed; the lifts arm gave 5 hits in 1000 targets versus 0 in 1000 (descriptive p = 0.06, above). Neither confirmed nor refuted.
- **L2** (F4 calls per target rise at most 4x): refuted in these cells. The rise is x7.9 to x8.0 in all five (nearly every target is refuted, so all 8 lifts are tried).
- **L3** (per-relation cost ratio lifts / no-lifts): undefined in every cell; no statement possible. The per-target cost ratio is x7.9 to x8.0, so a per-relation gain would require the hit-rate gain to exceed about 8x, which is the cap L1 itself states.

## Classification (preregistered rule)

Not an improvement: the criterion needs at least 3 hits in both arms in a cell, and no cell has even 1 hit in the no-lifts arm, so the per-relation ratio is undefined everywhere and cannot be below 1. Of the two remaining labels, the data show the cost side exactly (about 8x F4 work per target) and a hit-rate rise only at the level of 5 versus 0 hits, so I label it "yield gain at a cost, unproven": hit count 0 to 5, per-target cost x8. No per-relation gain is measured. Gap to matched rho: not measured here, unchanged from the ladder study.

## Deviations

1. Compile: the WIP was reported as never compiled, but `cargo check --release --lib` and `--bin curve25519_family` passed with no source change.
2. Unit test fix: the first run of `cofactor_lifts_match_the_brute_force_fiber` failed at the row assertion (`left 677, right 1626`). Cause: the test took one discrete log per factor-base point, but the base lists P (coef 1) and -P (coef r-1) on one column, so the negative point's coefficient was applied to an already negated log. Fixed in the test only: one log per column, taken from the coef-1 member, indexed by `col_of`. The lift code itself needed no change. Also added the `eprintln!` count line.
3. The measured binary was built from the working tree (HEAD ad50d0892f + the test edits, which do not affect the binary's oracle or CLI code).
4. `scripts/checkpoint.sh init` worked; `.resume/RESUME.md` was filled and `checkpoint.sh push` uploaded to S3 before launch. `.resume/` is not part of the research commit.
5. Runs were local (not isolab-eks), as instructed by the task; counts are operation counts, independent of load.
6. No supplementary larger-target run was done.
7. Clippy: `cargo clippy --release --lib --bin curve25519_family -- -D warnings` exit 0 (only the environmental unknown-lint `clippy::chunks_exact_to_as_chunks` warning).
