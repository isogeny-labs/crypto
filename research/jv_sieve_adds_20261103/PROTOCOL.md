# Protocol: cutting the additions of line enumeration and the sieve (frozen 2026-11-03)

Frozen before any measured run. `results/FROZEN.sha` records the SHA-256 of this file, `run.sh` and the two
binaries (P: the code before the change, N: the code with it) and a UTC timestamp from the system clock (no
git). **Prototyping disclosure:** the changes were developed and smoke-tested on seeds 1 and 2 (p = 1009) and
seed 1 (p = 1511, 1823) with the unfrozen binary `proto_N`; no instance with a seed in 17-32 was run before
the freeze. The prototype's S+ and S ratios are quoted below as the *model prediction*; if a threshold proves
unsuitable the outcome is reported as failed, not changed.

**Scope.** Weak class only (full rational 2-torsion, `#E = 4l`, genus-3 cover over `F_{p^2}`). **Class of
claim:** a measured, paired comparison of two implementations of one algorithm (the JV12 section 3.2 sieve
route, `jv_sieve::run_cover_sieve_dlp_on`) that must produce the same lines, hits, relations and answer, on the
same instances, in the same counted unit. **Labels:** *measured* = read from a record; *derived* = computed
from records in Rust; *model* = a stated cost model or extrapolation.

## Where the additions go (V2, p = 1511, sparse; read from `raw/ic_1511_sparse.json` and the code)

`m = 9`, `B = x + b0 + b1 t`, 328228 B's, 167444 lines (0.51 per B). `enum_adds` (3.98e9) is all one thing:
`roots_by_table`, the root search of `G = Im(B^2 h)` (degree 8) by forward differences over all `x in F_p`,
`8p + 36 = 12.1k` additions per B. `sieve_adds` (4.82e9) is the difference-table advance of `P2 = w A1'^2`
(8 additions), `P3 = Re(B^2 h)` (9) and `P4 = N(B)` (2) at every `x in F_p`, `19p = 28.7k` per line plus 84
for the tables; `sieve_lookups` (2.57e8) is one counter update per root.

## Changes (opt-out; default on in the new binary)

* **INC (`JV_ADDS_INC=0` turns off).** For `m = 9`, `b1 != 0`: `G(x) = (U^2 + w V^2) h1(x) + 2UV h0(x)` with
  `U = x + b0`, `V = b1`. With `R = h0/h1` this vanishes iff `U = nu V`, `nu^2 + 2R nu + w = 0`; so `x` is a
  root of `G` iff `(b0, b1)` lies on the line `b0 = nu b1 - x` of the plane. A per-round scan walks each such
  line over `b1` (one addition, one slot lookup per step) against the round's B's (all threads' ranges) and
  returns each B's roots ascending; `factor_lazy` uses them in place of `roots_by_table` when `G` is squarefree.
  Otherwise (`b1 = 0`, non-squarefree `G`, a root of `h` in `F_p`) the old search runs and is counted as before.
  The `nu` table (`h0, h1` at every `x`, a batch inversion, a square root by the existing table) is built once
  and its multiplications are added to `table_muls`.
* **SHARE (`JV_ADDS_SHARE=0` turns off).** The lines of one B have the same `P3` and `P4`. For a B with two or
  more lines their values at every `x` are computed once (same difference tables and advances, counted in
  `sieve_adds`/`sieve_muls`), and each line reads them at its base steps (two reads per base step, counted in
  `sieve_shared_reads`). A B with one line is sieved as before.
* Every addition, lookup and multiplication executed is emitted; nothing is reclassified. New record fields:
  `enum_lookups` (the scan's lookups, including marking and unmarking), `sieve_shared_reads`, `inc_hinted_bs`,
  `shared_bs`; `total_plus` adds `enum_lookups + sieve_shared_reads`. Index arithmetic of the new loops (slot
  index, the repeated index decomposition of each B, the stores of the shared arrays) is emitted to the
  `bitop/v1` word counter. Not emitted: loop control, the `b_from_index` call that the scan repeats beyond its
  word charge, the integer `p^2` slot array's allocation.
* Unit tests (`cargo test --release --lib jv_sieve::tests`): the scan returns exactly `roots_by_table`'s roots
  for every B with `b1 != 0` at p = 53, 101, 103 (all `p^2` B's); lines from the hinted enumeration equal the
  plain ones, in order, from the same rng, on 1500 B's at p = 1009; a shared-array sieve returns the same hits,
  roots and lookups as the plain one on every multi-line B.
* Considered and rejected under the constraint that multiplications must not rise (model, not run): tabulating
  `A1'` (4 additions per `x`) and forming `a = w A1'^2` by a multiplication at base steps saves 4 of the 19
  additions per step but adds one multiplication per base step (+3.7% of `total_muls`); evaluating `Re, N, G` at
  `x` from per-run `h0, h1` tables needs 3-4 multiplications per `x`; sieving by the roots of the line
  polynomial or early termination cannot prune (a counter needs `m - c` more of the remaining base steps, true
  only in the last few); gap-skipping by Newton shifts costs as many additions as single steps.

## Instances and options (V2's sieve arm, exactly)

`p in {1009, 1511, 1823}`, seeds 17-32 (16 per arm and prime, 48 instances), `jv_cover --exp sieve --stop 64
--sizes p --first-seed 17 --seeds 16 --rho-runs 0`, `JV_LIN=sparse`, default thread count (4). No rho is run.

## Arms

| arm | binary | environment | role |
|:--|:--|:--|:--|
| P | P (old) | none | control; must reproduce V2's sparse records |
| N0 | N (new) | `JV_ADDS_INC=0 JV_ADDS_SHARE=0` | code-path control; must equal P |
| N | N | none | **primary** |
| U | P | `JV_FB_LINE=32768 JV_FB_LINE_MODE=table` | line-search reference |
| NU | N | same as U | **primary 2** |
| NI | N | `JV_ADDS_SHARE=0` | INC alone (descriptive) |
| NS | N | `JV_ADDS_INC=0` | SHARE alone (descriptive) |

## Metrics, per run (from the record)

`S = total_muls / (c_add_e sqrt(l))`; `S+ = total_plus / (c_add_e sqrt(l))` (V2's definition: every emitted add
and lookup at one multiplication). Band (section 5a): cost(w) = `total_muls + w c_add_e (total_plus -
total_muls)` at `w = 0, 0.1, 1` GAE per native operation, identical on both arms; F4 and descent additions
are unemitted on both arms, so every cell is a lower bound on both. **Secondary, model:** a `bitop/v1` proxy
in units of an 11-bit field multiplication (docs/units/BITOPS.md at b = 11: add 0.096, word op 0.043), with the
linear algebra's `la_muls` at 1 and at 27.3 (a 58-bit mod-l multiplication, 40542/1485), words as listed in
`fit/src/main.rs::proxy`; it reports the ratio and the share of the summed proxy removed. The proxy is a model
(it prices from counters, not from the bitop recorder) and is not a decision statistic.

## Statistics

Paired by seed; geometric mean of new/old ratios per prime (16 pairs) and pooled (48, resampling seeds within
each prime), 95% percentile bootstrap, 20000 resamples, fixed seed, Rust only (`fit/`).

* **H1 (N vs P):** the upper end of the pooled S ratio is at most 1 (S does not rise) **and** the upper end of
  the pooled S+ ratio is below 1. **H2:** the same for NU vs U. **H3 (band):** the change improves across the
  band iff the upper end of the pooled ratio is below 1 at all of `w = 0, 0.1, 1`; otherwise nothing is
  claimed at that `w`. NI, NS and the proxy are descriptive.

## Controls (all must pass or every H-decision is withheld and the failure reported)

| id | requirement |
|:--|:--|
| C1 | P equals V2's sparse records on 48/48 in every field but `wall_ms` |
| C2 | N0 equals P on 48/48 in every field but `wall_ms` (new fields zero) |
| C3 | N vs P and NU vs U: every field outside {table, enum, sieve muls, enum adds/lookups, sieve adds, shared reads, hinted/shared B counts and derived totals/ratios} is equal, including `bs, lines, hits, roots, relations, descent_digest, descent_residuals, la_ops, solved, correct`; `d(total_muls) = d(table+enum+sieve muls)` exactly; the search's native count identical |
| C4 | every run of every arm solved, correct, 0 failed verifications, not exhausted |
| C5 | the unit tests above pass |

## Model prediction (from the prototype, before data)

S+ N/P: 0.63 (p = 1009 seed 1), 0.72 (p = 1009 seed 2), 0.64 (p = 1511 seed 1), 0.59 (p = 1823 seed 1), i.e. a
pooled value in about 0.59-0.72; S N/P: 0.989-0.994. No prediction for NU/U beyond "similar".

## Cut rule and budget

Order: p = 1009, 1511, 1823 for P, N0, N, U, NU, then NI and NS for the three primes. At most 2 h wall on the
shared 4-core box; counted metrics do not depend on the host. If the budget runs out NI and NS are dropped,
then NU at p = 1823; a cut removes a cell, never a seed.

## Reported regardless

Every record, log, version; controls; per-prime and pooled S, S+, band and proxy ratios; component tables;
deviations. No leaderboard or `benchmarks/` row (no `VALIDATION.json`). Wall time is an observation only.

## Limits declared in advance

One host, one code revision; `m = 9` only (the incidence scan is specific to `B` of degree 1; larger `m` falls
back); p in 1009-1823; the proxy prices an add at 0.096 of a multiplication regardless of the code that does it;
the changes leave the 1.3e9-mult enumeration (factoring) and 0.8e9-mult sieve multiplications untouched
except for the table-build multiplications they no longer execute.
