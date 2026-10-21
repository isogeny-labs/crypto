# Inversions and residue tests run as `bitop/v1` prices them; one self-check policy

**Report:** [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)).
**Analysis:** `examples/ic_acct4_analyse.rs` → [`analysis/`](analysis/).

These are items 4 and 5 of the index-calculus cost-accounting follow-ups.

## Item 4: what executed was not what was priced

`bitop/v1` prices:
- a field inversion as a binary extended Euclid, `2b · 13b` gates;
- a quadratic-residue test as the same gcd-type loop;
- a square root as an exponentiation (`docs/units/BITOPS.md`).

At `b` = 6–16 bits, the inversion and the residue test each cost about 2.1
field multiplications.

The hyperelliptic arithmetic did not execute those algorithms.
- **Inversion:** `fp_poly::fp_inv` ran a Fermat exponentiation.
- **Residue tests and square roots:** these ran Euler's criterion and
  `(p+1)/4` powers.
- **The cost:** all of these went through `num-bigint`'s `modpow`. Its
  Montgomery path uses a fixed 4-bit window over whole 64-bit limbs, about 93
  products whatever the size of `p`. That reading comes from the library
  source, `num-bigint 0.4.8`, `biguint/monty.rs`.

So each charged inversion or residue test executed some 40× the priced work.
This is the gap the coverage diagnostic picked up as extra instructions per
gate.

**The fix: execute the priced algorithm, change no price.**
- **Inversion:** on a word-sized odd `p`, `fp_inv` now runs a binary extended
  Euclid (`utils::inv_binary_u64`).
- **Residue tests:** these now run a Jacobi symbol (`utils::jacobi_u64`).
- **Square roots:** `sqrt_mod_p` takes a word path with the same roots.
- **Small public powers:** these use square-and-multiply on words.

The charge per call is unchanged, and so are the frozen prices and vectors.

## Results

**1. The committed counts are reproduced exactly.** I re-ran five rows of
`research/hec_bitop_20261020` with the study's seeds, 5 IC and 40 rho runs
each. For both arms, the per-run totals, per-phase sums, validation subsets,
recovered keys and verification flags are byte-identical to the committed rows.

| genus | p | identical |
|--:|--:|:--|
| 2 | 41 | yes |
| 2 | 61 | yes |
| 2 | 101 | yes |
| 3 | 23 | yes |
| 3 | 31 | yes |

Three of the primes are ≡ 1 mod 4, so the Tonelli–Shanks path ran too. The
`0.114` genus-4 result is not affected.

**2. The coverage gap closed.** Index calculus now runs fewer instructions per
counted gate than rho. The figures are Callgrind's marginal instructions over
two extra runs of each arm.

| row | IC | rho | IC/rho now | IC/rho recorded before |
|:--|--:|--:|--:|--:|
| genus 2, p = 41 | 1.212 | 1.304 | 0.930 | 1.01 |
| genus 3, p = 23 | 1.611 | 1.681 | 0.958 | 1.16 |

The "before" column comes from the hec study's whole-run Callgrind
measurement, a different method from this marginal one. Read the comparison as
direction, not as a precise change.

## Item 5: one policy for self-check work

`docs/units/BITOPS.md` now has a section, "Self-check work", with three rules:
- The domain (curve, generator, target) is checked once, outside counted work,
  as ecbench already does.
- A method's check of its own answer is algorithm work, charged per target.
- Operand re-validation inside the arithmetic is counted but left out of the
  headline. In the hyperelliptic `checked_*` arithmetic this was 39–59% of
  counted gates.

The harness keeps `bitops_per_run` unchanged, so committed files reproduce. It
now also reports `algorithm_bitops_per_run`, the total that the policy
compares.

Under the policy, IC/rho rises on every re-run row, because re-validation is a
larger share of rho's work:

| genus | p | IC/rho, total | IC/rho, algorithm |
|--:|--:|--:|--:|
| 2 | 41 | 0.207 | 0.243 |
| 2 | 61 | 0.221 | 0.268 |
| 2 | 101 | 0.299 | 0.406 |
| 3 | 23 | 0.200 | 0.226 |
| 3 | 31 | 0.257 | 0.299 |

For the genus-4, `p = 61` row, the policy-conforming figure is the hec study's
existing "excluding validation" ratio, **0.132**. The headline that the
scoreboard should carry under this policy is 0.132, not 0.114; it is still
below rho.

**Class: accounting.** No algorithm of either method changed what it computes.

## Files

| path | what |
|:--|:--|
| `raw/g{2,3}_rerun.jsonl` | the re-run rows (`examples/hyperelliptic_ic_vs_rho_bitops`, commit `1961d1878`) |
| `raw/callgrind.json` | Callgrind instruction totals and counted bitops of the six diagnostic runs |
| `analysis/` | `analysis.json` and `tables.md` from `examples/ic_acct4_analyse.rs` |
| `figures/` | SVG/PNG and `plots.py` (plots recorded values only) |
