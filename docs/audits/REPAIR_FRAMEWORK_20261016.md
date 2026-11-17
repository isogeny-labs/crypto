# Repair framework: claims built on unverified foundations (2026-10-16)

Rebuilt 2026-10-19. The original draft was never committed and was lost when
the AWS workers stopped. The register below is reconstructed from the audit
summary. Each item's status was checked against `origin/main` at `3eb078b08`
(2026-10-19 09:24 -0700) and updated on 2026-10-25 against `57d03de21`. The
repository is now `isogeny-labs/crypto`.

## 0. Status labels

Every headline number carries exactly one label. A repair is done when the
label is accurate, not when the number improves.

| label | meaning |
|---|---|
| `MEASURED` | measured end to end against a matched opponent (same precomputation, memory, best implementation) |
| `MEASURED-UNMATCHED` | measured, but the opponent was handicapped or the clock excludes a phase the other side pays |
| `DERIVED` | computed from a cost model; names the model and its inputs |
| `ASSUMED` | rests on a construction or bound that has not been exhibited |
| `SUPERSEDED` | overturned; links the result that overturned it |

## 1. Repair register

| id | problem | fix / done when | status on main |
|---|---|---|---|
| R1 | N83 "IC beats rho" (12.29× hosted K600, 113× Tier 1 K3300): only IC is allowed precomputation (400–770 s, 1–14 GiB); rho starts with an empty DP table (`autolab/protocol.json`). The IC online phase is a table scan (`examples/koblitz_orbit_dlp_fast_online.rs:811`). The rho baseline runs at 599 ns/step against 240 ns/step for fold rho. Ratio of means is 2.8×, and IC loses on 4/16 targets. The DLP is in a ~2^53 subgroup. | Bernstein–Lange rho, time- and memory-matched, at K600 and K3300 on the same 16 targets, with the decision rule frozen before timing. | **DONE**: #1987 (`bb023d5d7`), `research/20261016-n83-precomputed-rho-control/`. Precomputed rho wins every matched arm: K3300 3.74× (time) / 40.61× (memory); K600 81.52× / 214.73× (time) / 395.53× (memory). Both readings are `SUPERSEDED`. The equal-bytes co-measured form at n = 73/83 is #2163 (merged as `c2e15599c`). |
| R2 | Jacobian-transfer model uses 10g² per Cantor op; #1846 measured 4–10× more (`src/cryptanalysis/jacobian_transfer/cost.rs:49`). | Measured Cantor counts in `cost.rs`; registry regenerated. | **DONE**: #1983 (`008ef79f4`). `formula_advantage` routes went from 815 to 598. |
| R3 | k = 6 rows priced at a genus-9 cover; Joux–Vitse give genus 9 only as a minimum. | Rows carry `conditional_on` an exhibited cover. | **DONE**: #1983. 574 rows conditional. Label `ASSUMED` until a cover is exhibited. |
| R4 | `log2_extension_ic` ~2^21.6 too cheap vs the repo's measured k = 4 cost. | k! trials per relation plus measured C₃, C₄ (otherwise a D³ floor). | **DONE**: #1983. All 214 extension-IC advantages are now `boundary`. **Follow-up R4b (open, #2302):** the generic plain-S₅ C₄ was re-measured with #1792's solver options, falling from 1.227e12 to 8.69e10 (13.9×). The PR updates `cost.rs` and reports no verdict changes, but its `jacobian-transfers` CI job was cancelled, so that claim is not CI-confirmed yet. Also open: #1792's D₄ cost (~2^26) is below the D³ = 2^36 "floor" the `cost.rs` comment and test assume. That floor holds only for generic FGLM. |
| R5 | PKM lead 1 still "open" although the 10-14 msolve run measured D ≥ 8 at m = 4, N = 20 (`research/ic_next10_20261014/03-pkm-degree`). | Encyclopedia and card cite the negative result; card rescoped. | **DONE**: #1983. Card rescoped to the m = 3 cell. |
| R6 | P-192 aux-curve card (98,352 core-h): t = 48 is a solver-probe size, not the §3.7 optimum (~2^39 at m = 4); it prices `z2xz8` but runs `z2xz2` (362k core-h); v₂(p+1) = 64 makes the search unnecessary. | Card abandoned or rescoped. | **DONE**: #1983. `XP-20261015-p192-isogeny-aux-search-t48` abandoned as superseded. |
| R7 | Overturned or overstated headlines left standing (K3350 "7.18% win", round-1 free-relations 0.957×, N83 12.29× row, k = 4 2^150→2^135 crossover). | Each annotated in place, linking what overturned it. | **DONE**: #1983. Re-check that `benchmarks/README.md` rows now cite #1987, not only the 2.80× ratio of means. |
| R8 | Shipped defaults never paired against "off". The linear-harvest default regresses in its own fit data (`ic_improve3…/fit_test.jsonl:78`, 0.87× CPU, labelled REGRESSES) while its README says "MATCHES". | Paired default-vs-off run on each shipped default; README matches the data or the default is reverted. | **OPEN**: no commit found on main. |
| R9 | Switches that change results are missing from run records, so runs can't be compared. | Every result-affecting flag is written into the run record. | **OPEN**: no commit found on main. |
| R10 | No pinned IC baseline. `research/ic_tool_program/baselines.json` stops at v3 (last touched 2026-10-06), so each round picks its own incumbent (the genus-3 "−0.525" gain is ~−0.06 against the tuned arm). | Ledger pinned to the current best per curve/size; rounds compare only against the ledger. | **IN REVIEW**: #2286 adds `pinned_r10` (additions only). #2289 annotates the re-rated round claims (G5), stacked on #2286. Both have been on validation hold since 2026-10-23: `validation-gate` plus the dashboard/leaderboard jobs. Re-rating findings so far: round 1's free-relations arm ran at the untuned K = 255, about 2.1× the tuned K = 125 incumbent at n = 41. Every k = 4 D₄ round priced C₄ at 2.54e10, while #1792 measured ~310–370× cheaper. |
| R11 | Linear-algebra filter default not validated. | Paired filter-on/off run at the sizes the default ships for. | **OPEN, unowned.** |
| R12 | ecbench rho trend not reconciled with the fold-backend rho speed. | Trend rebased on the fastest in-repo rho, or the gap explained. | **OPEN, unowned.** |

## 2. Order and gates

1. R1 first. **Landed.** The N83 chain (K3300/3350, root4/11/21/22, Bloom,
   sparse selector, Tier 1, S3 archives) moves along a time-memory tradeoff
   curve. It is not progress toward beating rho.
2. No new campaign starts until R1, R5, R6 and R10 have landed. **R10 is the
   last blocker; it lands when #2286 merges.**
3. Check against the gate: N83 work merged after the R1 verdict, such as
   #2262 (`65f975d94`, K600 base replay) and #1803 (greedy Koblitz factor
   base, `62c2e7d5b`). Any such work must report against precomputed rho, or
   say it is engineering only.

## 3. Proposed AGENTS.md rules (G1–G9)

These rules target the root cause. They are not adopted yet; whether to adopt
them is the owner's call. AGENTS.md:66 already asks for the distance to
precomputation rho. G1 makes that a requirement for any "beats" claim.

- **G1** Both sides of a comparison get the same precomputation, the same memory and the best available implementation. A one-sided precomputation result is `MEASURED-UNMATCHED`.
- **G2** A pinned baseline ledger; a round compares against the ledger, never against an incumbent it picked.
- **G3** Report the ratio of means and the count of targets where the candidate loses next to any median.
- **G4** Small deltas (inside the run-order effect, ~3 ms at N83) need a balanced (ABBA) run order.
- **G5** When a result is overturned, the same PR finds and marks everything that cited it.
- **G6** Before re-asking a question, cite any existing negative result on it.
- **G7** Nothing is priced on a construction until that construction has been exhibited.
- **G8** A default flips only after a paired test against "off".
- **G9** Every result-affecting switch is recorded in the run record.

## 4. Audited and found sound (do not redo)

- PKM construction: Vélu's formulas, the factor base V, the curves-per-success statistics.
- P-192 2-adic valuations: v₂(p−1) = 1, v₂(p+1) = 64.
- IC improvement logs: all verified; no correctness bugs.
- d4-s5 (minor solve bug, handled consistently), cyclotomic falsifier, shape search, Dickson torus.
