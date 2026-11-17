# Prime-field k-sum table: k = 5, and the law at 44 and 48 bits (pre-registration, 2026-10-28)

**Status.** Registered before any timed run. Public synthetic curves; no
security claim; no claim against rho. Stacked on
`research/prime_ksum_table_20261028/` (isogeny-labs/crypto#2808).

## Baseline

The prime-field IC row of record is now `ic-ksum4` (`ksum:m=4`) from
`research/prime_ksum_table_20261028/`: mean `S` at `w = 0` of 6.74, 6.71,
7.18, 7.93, 9.83, 11.56 on the frontier's 20–40-bit `a = −3` ladder, fitted
`α = 0.521 [0.506, 0.538]` (20–32 bits). Measured relations per base
abscissa `c₄ = 0.98–1.06`; the table build is 37–46 % of its operations.

## Change

1. **`k = 5`.** `KSumTable` takes `k = 3, 4, 5`. Its entries now pack the
   summand indices into one `u64` (12 bits each) instead of `[u32; 4]`, which
   cuts the table's memory by about a third. The insert order and the
   keep-first rule are unchanged, so group-operation counts stay identical.
   *Checked before this registration:* `ecbench verify --replay-all` with the
   new binary on the frozen 32- and 40-bit test sessions of #2808 reproduced
   every replay.
   Generic law `2^{k−1}C(A,k) + c_k·A·p/(2^k C(A,k))`, exponent `5/9` for
   `k = 5`.
2. **Two larger sizes.** 44 and 48 bits, the `prime_fast::find_a3_curve`
   rungs (largest `p ≡ 3 mod 4` below `2^bits`, smallest `b` giving prime
   order), produced by `examples/ic_frontier_prime_curves.rs --bits 44,48`:
   - 44: `p = 17592186044399`, `b = 95`, `r = 17592192476281`, `G = (1, 8379642404149)`
   - 48: `p = 281474976710591`, `b = 9`, `r = 281474991897067`, `G = (1, 111984877719828)`

**Class if it lands: engineering** (the prime IC baseline's `S` falls; no
generic bound moves). The family's limit is still BSGS.

## Predictions (model: optimum of `2^{k−1}C(A,k) + A·p/(2^k C(A,k))`, `c_k = 1`)

The same model gives `A = 64`, `S = 11.4` for `k = 4` at 40 bits, against the
frozen `64` and the measured 11.56.

| bits | `k = 4`: `A`, `S` | `k = 5`: `A`, `S` | `k = 5` entries |
|--:|--:|--:|--:|
| 20 | frozen 10 | 9, 4.3 | 2 k |
| 24 | frozen 14 | 11, 4.9 | 7 k |
| 28 | frozen 21 | 15, 5.5 | 48 k |
| 32 | frozen 30 | 19, 6.2 | 186 k |
| 36 | frozen 44 | 25, 7.1 | 0.85 M |
| 40 | frozen 64 | 33, 8.2 | 3.8 M |
| 44 | 95, 13.9 | 44, 9.5 | 17 M |
| 48 | 140, 16.9 | 59, 11.0 | 80 M |

So `k = 5` is predicted at about 0.7× `k = 4` at 40 bits: a constant-factor
gain, not a large one, because the exponent only moves from `4/7` to `5/9`.
`c₅` is unmeasured and is read off the runs.

**Memory.** At 48 bits `k = 4` keeps about 123 M entries and `k = 5` about
80 M. Packed, that is about 4.6 GB and 2.3 GB of hash table on a 48 GB
host. Runs execute one at a time. `ic-pair` (about 9·10⁸ entries at its
optimum) and `ic-ksum3` (about 2.6·10⁸) are not run at 44–48 bits.

## Selection (train panel, as #2808)

Target seed `20261015100`, 4 targets per size, grid
`A ∈ round(A* · {0.5, 0.7, 1, 1.4, 2})` around the model optimum `A*`,
extended until the optimum is interior: `k = 5` at every size, `k = 4` at 44
and 48 bits only (20–40 bits stay at #2808's frozen values). At 44 and 48
bits the grid is `A* · {0.6, 0.75, 0.9, 1, 1.15, 1.3}`, and any point whose
table would exceed 3·10⁸ entries is dropped (the 48-bit `k = 4` point at
1.3 and the 48-bit `k = 5` point at 1.3) ([`make_specs.sh`](make_specs.sh)). Pick the
smallest mean `S` at `w = 0.1` among candidates verified on all four
targets. Freeze in `params.json` before the test panel runs.

## Test panel

Eight curves (the frontier's six plus 44 and 48), target seed `20261015200`,
16 targets × 2 rounds, warm-up 1, alternate order. Arms: `rho-neg`
(reference), `bsgs-neg`, `rho-neg-aa` (A/A control), `ic-ksum4`, `ic-ksum5`.
Every answer verified; `ecbench verify --replay-all` on every session;
`ecbench band` at `w = 0, 0.1, 1`; `ecbench bound fit` per arm and tier.
At 20–40 bits `ic-ksum4` must reproduce #2808's per-run counts on the same
targets (same spec, same seeds); a mismatch stops the study.

## Success and falsification

- **Success (engineering):** at 36 and 40 bits `ic-ksum5`'s mean `S` is below
  `ic-ksum4`'s at every `w` in the band.
- **Law check (reported, not gating):** at 44 and 48 bits each arm's mean
  `S` against the prediction above; whether `ic-ksum5 < ic-ksum4` holds there
  across the band.
- **Falsified:** `ic-ksum5` is not below `ic-ksum4` at 40 bits at some `w`.
  Abandoned if any run fails verification.
- **Reported regardless:** each row's gap to `rho-neg` and to `bsgs-neg`. The
  headline must say the arm is still above BSGS.
