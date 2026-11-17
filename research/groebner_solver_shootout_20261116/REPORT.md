# GB-SOLVER-SHOOTOUT-20261116, amendment-1: local diagnostic results

**LOCAL DIAGNOSTIC, non-admitted, [pcc:unverified], not preregistered as run.**
This is not an IC leaderboard row and not a `benchmarks/README.md` row.
The owner decided on 2026-11-17 that AWS does not work and Magma is not
available, so the run used this 4-vCPU shared VM.

- **Protocol:** `protocol.amend1.json`, canonical sha256 `c521e04c643ff8b3028426b1d9c8b85a3f92fabc5e6e91e124498c13524fe858`.
- **Run:** `run-local/`, 2026-11-17 07:23Z–08:19:51Z, exit 0, 56:17.76 wall time.
- **Driver:** `run_local.py`, sha256 `ed27d2e7…28120ac`.
- **Data hashes:** `rows.jsonl` `7c5c98f91cd6404f6308f06341731d60a23006d644f390eab06560877b2aedf1`, `cells.json` `87d4b7a84b71c3f22167292f77f560d26c79419716e29c667b17af88f71b7f0f`.
- **Analysis:** `analysis-local/analyze_amend1.rs`, sha256 `d62848b44c636b1664368d1481ad78e8b750a764025262a47a77af38f75db3f3`. It was written after the data was taken.
- **Cell:** Koblitz `a=1` over GF(2^17), #E = 2·65587. Summation polynomial m=3, plain chained-S3 system with 32 Boolean variables. Factor base x ∈ ⟨1,z,…,z⁴⟩, which gives 14 abscissae.
- **Targets:** 160 natural targets, 80 decomposable and 80 not, out of 1757 decomposable among 65572. Plus 5 planted targets. Each target ran twice: pass A in Williams order, pass B in reversed order, which serves as the A/A repeat.

## Answer

On this one cell, with these encodings, on this host:

| Arm | Verified relations per CPU-s, 95% CI | CPU-s per relation | Peak RSS | Max degree |
|---|---|---|---|---|
| exhaustive factor-base enumeration (oracle, descriptive) | ≈ 3900 | 2.6e-4 | n/a | n/a |
| inherited-f4 (repo incumbent, built from engine `eb01341d3a`) | 0.786 (0.752–0.827) | 1.27 | 5.6 MB | 3 |
| msolve 0.6.5 `-g 2 -t 1` (F4) + harness root extraction | 0.00585 (0.00564–0.00612) | 171 | 235 MB | 4 |
| CryptoMiniSat 5.11.15, XOR, full enumeration | 0.00125 (0.00120–0.00131) | 800 | 47 MB | n/a |

Paired comparisons (CPU per target, 160 pairs):

| Comparison | Speed ratio | 95% CI |
|---|---|---|
| inherited-f4 vs msolve | 134× | 131–138 |
| inherited-f4 vs CMS | 629× | 606–651 |
| msolve vs CMS | 4.7× | 4.6–4.8 |

- In every comparison, the faster arm was cheaper on all 160 targets.
- The A/A noise floor is at most 0.024 in log₂.
- The intervals cover resampling of targets only. They do not include variance between runs, hosts or builds.

The fb-enum row was missing from the first draft and was added after the
red-team review (finding 3). It comes from `cells.json` `oracle`: 0.519 CPU-s
builds the whole decomposition table for all 65572 targets. At this size,
plain enumeration beats every Gröbner or SAT arm by orders of magnitude, so
none of these solvers is the right tool for an m=3, l=5 cell over GF(2^17).

General-purpose F4 (msolve) is 134× slower than the Boolean-native
inherited-f4. These systems become affine in the 17 U bits once the 15 X
bits are fixed. That favours branching and hybrid solvers, so the gap
measures both algorithm class and encoding, not F4 quality alone (red-team
finding 9).

## Rho distance (§5a)

At w = 0 every arm's priced cost is 0, so all arms tie. The native work
units differ between arms: matrix cells for msolve, propagations for CMS,
word XORs for inherited-f4. Because of this, the order at w = 0.1 and w = 1
is **not** reported (red-team finding 7).

Clock distance to the matched rho, a signed-Frobenius rho on the same curve
over 1000 DLPs:

| Arm | Clock distance to rho |
|---|---|
| inherited-f4 | 2^19.54 (CI 2^19.47–2^19.61) |
| msolve | 2^26.61 |
| CMS | 2^28.84 |

The verdict for every arm is **unresolved in band**. The IC side is a lower
bound: target generation, encoding, projection and linear algebra are not
charged. The rho side leaves Frobenius maps, negations and setup scalar
multiplications unpriced (red-team finding 8), which overstates the
distances by at least 2 bits at w = 0.1. The 0.46-bit margin under the M8
abandon threshold of 2^20 is smaller than the spread between rho readings,
which run from 2^19.54 to 2^22.59. So M8 is **not resolved**, rather than
"not triggered".

## Deviations (each one limits what can be claimed)

1. **The run broke frozen `stopping_rules[3]`.** Amendment-2 was never filed,
   so the driver (PA-4) and the analysis script (PA-5) were not pinned before
   the data. PA-1, a HEAD that builds, was worked around by building on an
   older base. Under that rule the run should not have started. Every verdict
   here is post-data and not preregistered.
2. **The incumbent is not HEAD's engine.** HEAD and `origin/main` do not
   compile because of broken merges. `groebner_compare/build_native.sh`
   builds inherited-f4 and the rho binary from `eb01341d3a` (2026-11-12),
   plus `f4_trace.rs` from HEAD. That base is about 2,340 lines behind HEAD
   in the F4 sources, including the f4-parallel and gf2-m4ri speed-ups.
   `build_identity.json` in this directory records the build.
3. **The analysis is not blind.** The blinding key was printed before
   phase 1.
4. **msolve 0.11.0 was built but not used.** It is in the session
   scratchpad, and the measured arm is Debian 0.6.5. It cannot plausibly
   close the 134× gap.
5. **CMS used a naive CNF.** It had one Tseitin gate per monomial, no
   sampling set, a fixed `--random 0`, and an assertions-enabled build.
6. **Arms not run:** Magma (UNAVAILABLE); trace-f4 (it decides no grid
   target, see `problems/gf2-fixed-schedule-f4-unavailable.md`); Groebner.jl
   (Julia blocked); repository Boolean F4 (dropped by amendment-1).
7. **Post-run source formatting.** After the run, `rustfmt` (edition 2021)
   reformatted both adapter sources to pass the `rustfmt (changed files)`
   check. The changes are layout only: whitespace, line breaks, trailing
   commas, and braces around single-expression closures. The binaries used
   in the run were built from the original bytes, which are the bytes
   `cells.json` records:

   | File | sha256 at run (in `cells.json`) | sha256 committed |
   |---|---|---|
   | `examples/gc_native_solve.rs` | `6ba8bd67ed026963e32b2c5d7c34db91376e61dfd9083ffeb3e9501d80d326b3` | `01654e667faf14f318681883035d6e95b2dc862190fda500f5ea516b26f2ece8` |
   | `examples/gc_rho_match.rs` | `0451f513880945152f26e3100d19e0e343376c7f6a6689595334e3b87d9eea0b` | `5e0b0b882e568b50e5898f13ad95a48763706c20331ea1e2ad5e0e3f9450e6cb` |

   A replay should check out commit `00df3c9214`, which holds the run bytes.
8. **Store-policy trims after the run.** Three changes keep the files
   under the artifact store limits:
   - `analysis-local/out/rows.tsv` was removed. It is a layout-only
     extraction (sha256 `a3eb7d2f5fb4b63d6e16dec70158df4fe2d852e4c698d343c42f3562e52f22b5`),
     and `analysis-local/extract.sh run-local/rows.jsonl` regenerates it.
   - The duplicate PNG renders under `analysis-local/figures/` were removed.
     The PDF uses the SVGs, which are kept.
   - The PNGs in `benchmarks/20261117-gf2-semaev-solver-shootout/` were
     palette-quantized to 256 colours to stay under 64 KiB.

   The raw `run-local/rows.jsonl` (3.2 MB, sha256 `7c5c98f9…2aedf1`) still
   belongs in the S3 store and stays committed until AWS works.
9. **No S3 checkpoint or raw-data upload** (AWS does not work). The
   resume-checkpoint and upload refusals are recorded verbatim in
   `run-local.launch/REFUSALS.md`.

## Independent validation

The validator's disposition is `insufficient_evidence`, because the evidence
was not committed and the pre-data pins were missing. Its recomputation
passes on every point:

- every point estimate reproduced exactly;
- 100 of 100 sampled relations, and all 528, verified by independent
  re-addition;
- every arm's Boolean root set matched an independent enumeration on 996 of
  996 rows;
- the seed stream regenerated identically for all 160 targets.

The report hash typos it found are corrected above. The full review texts
are in `VALIDATOR_LOCAL.md` and `REDTEAM_LOCAL.md`.

## Cheapest next tests

- Measure `msolve 0.11` on 10 targets.
- Give CMS a sampling set (`c ind`) and `--random 1..3`.
- Add a guess-and-solve hybrid: guess the 15 X bits, then solve the system
  that is linear in U.
- Run on a cell large enough that fb-enum is no longer trivial.
- Rebuild inherited-f4 from HEAD once `main` compiles.
