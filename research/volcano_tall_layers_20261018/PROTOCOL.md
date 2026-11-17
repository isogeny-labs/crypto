# Protocol: generic ECDLP cost across the levels of tall prime-field volcanoes

**Frozen 2026-10-18, after the specs were written and before any session was run.**
Native tools only:
- `crypto volcano sample` builds and verifies each volcano and writes the specs (`make_specs.sh`; logs in `specs/<vid>.sample.log`).
- `ecbench` measures.
- `examples/volcano_tall_analyze.rs` computes every number in the result.

## Why this study

The two earlier studies could not tell "no effect" from "too little data":
- `research/volcano_prime_layers_20261018` used two volcanoes with 6 targets per curve, and its intervals were about ±30%.
- `research/volcano_koblitz_layers_20261018` covered crater versus level 1 only.

This study is sized to *bound* any level effect. It turns "the interval contains 1" into either "equivalent within ±5%" or a detected difference.

## Question

In an ℓ-isogeny volcano of ordinary curves over `F_p`, every curve has the same `#E` and the same prime-order subgroup size `r`. Does the level (crater, middle, floor) change the counted cost of the ECDLP in that subgroup for the generic reference solver `rho.negation`?

## Volcanoes

The crater is one vertex in every case, since class number one is required. Craters cover all three splitting types and the extra-automorphism craters `j = 0` and `j = 1728`. `m = 6` makes the group structure change with the level.

| id | D | ℓ | (D/ℓ) | depth k | m | p | #E | r | levels by BFS |
|---|--:|--:|---|--:|--:|--:|---|--:|--:|
| a1 | −7 | 2 | split | 20 | 0 | 79739179114697 | 2² · r | 19934790368033 (2^44.2) | 0–10 |
| a2 | −3 | 2 | inert, crater j = 0 | 20 | 0 | 104602353229057 | 2² · r | 26150583213697 (2^44.6) | 0–10 |
| a3 | −4 | 2 | ramified, crater j = 1728 | 20 | 0 | 132738671602697 | 2² · r | 33184662163969 (2^44.9) | 0–10 |
| a4 | −8 | 2 | ramified | 20 | 6 | 88368960461169281 | 2¹² · 3 · r | 7191484364147 (2^42.7) | 0–10 |
| a5 | −11 | 3 | split | 12 | 0 | 75665619317083 | 5 · r | 15133120401881 (2^43.8) | 0–7 |
| a6 | −3 | 3 | ramified, crater j = 0 | 12 | 0 | 74457264432181 | 13 · r | 5727480553771 (2^42.4) | 0–7 |
| a7 | −19 | 5 | split | 8 | 0 | 113047896909809 | r | 113047875713291 (2^46.7) | 0–5 |
| a8 | −43 | 7 | inert | 6 | 0 | 133869096336161 | 13 · r | 10297621016051 (2^43.2) | 0–4 |
| b1 | −7 | 2 | split | 24 | 0 | 5263679187971873 | 2² · r | 1315919762456401 (2^50.2) | 0–10 |
| b2 | −8 | 3 | split | 15 | 0 | 4561544860849099 | 2 · r | 2280772366005899 (2^51.0) | 0–7 |

**Verification (`crypto volcano sample`, all passed before freezing):**
- Every level inside the BFS range has exactly `h(O_(ℓ^i))` vertices.
- The degree rule holds at every expanded vertex and at every step of every random descent: ℓ + 1 rational kernels above the floor, 1 on it.
- A blind descent from every sampled vertex, using only `p`, `t`, `ℓ` and `j`, gives the sampled level.
- The group structure measured from point orders equals the prediction from `O_d/(π−1)` at every sampled curve.
- All sampled curves share `#E` and `r`.

Floor sizes (class numbers): 524288 (a1–a3), 1048576 (a4), 354294 (a5), 177147 (a6), 312500 (a7), 134456 (a8), 8388608 (b1), 9565938 (b2).

## Sampling and size

- Up to 4 curves per level: all of them on levels with fewer, chosen at random within the BFS range, and from random descents from the crater below it.
- Solves (independent walks; ecbench gives every round its own seed) are fixed per level, whatever the number of curves:
  - tier a: 300 at the crater and the floor, 80 at every other level;
  - tier b: 120 and 20.
- 2 measured rounds and no warm-up. Counted operations only are used; a warm-up round prepares nothing for them.
- Planned totals: about 14,660 records in tier a (2,400 crater and 2,400 floor solves) and 1,516 in tier b.

**Size rationale.** The per-solve coefficient of variation of rho is about 0.52. With 2,400 solves on each side, a 95% interval on the crater/floor ratio has a half-width of about 1.96 · 0.52 · √(2/2400) ≈ 3%, inside the margin below.

## Arms

- **Level sessions:** `rho.negation` (the prime-curve reference, AGENTS.md §1).
- **Positive control** (`control.json`, floor curves, same workloads and seeds): `rho.negation` against `rho.plain`. A known factor of about √2 in theory (`A = 2` against `A = 1`). If the design cannot resolve it, the study cannot claim equivalence.

## Hypotheses and decision rules (fixed now)

Unit: per solve, `S = total_gae / √r`, divided by its volcano's mean `S`. Intervals: 95% stratified hierarchical bootstrap (curves within each (volcano, level) stratum, then solves within curve), 4000 resamples.

**H1 (equivalence), margin δ = 5%.** For tier a pooled, and for all ten volcanoes pooled:
- the floor/crater, middle/crater and floor/middle ratios of mean normalised `S` have 95% intervals inside [0.95, 1.0526];
- the least-squares crater→floor slope of normalised `S` on `level/k` has its 95% interval inside [−5%, +5%].

Readings:
- An interval excluding the null value is a **detected difference**, reported as such, whatever its size.
- An interval containing the null value but reaching outside the margin is **inconclusive**, not equivalence.

**H2 (distribution).** The crater and floor distributions of normalised `S` are the same. Test: the two-sample Kolmogorov–Smirnov statistic, with a p-value from 4000 permutations of the crater/floor labels within each volcano. p < 0.05 rejects H2.

**Control gate.** The tier's pooled `Σ S(plain)/Σ S(neg)` interval must lie above 1.0526. Otherwise that tier's H1 reading is void.

**Reported, not decided:**
- per-volcano ratios and slopes (ten volcanoes × 4 rows; about two 95% intervals are expected to exclude the null by chance);
- tier b alone (sized as a scale check, with half-widths of about 7%);
- mean raw `S` against the floor `√(π/4)`.

## Inadmissible

The following may not be changed after the first session starts:
- the workloads, seeds, specs, unit, normalisation, groups, margin, tests and bootstrap.

The following may not be done:
- dropping a volcano, a session or a record;
- re-running a session to replace one.

A session that fails to run or to audit is reported, and its records are listed by the analysis (`unverified`). Every measured record that did not verify is listed in the tables.

## Not claimed

- Nothing about non-generic attacks (index calculus, transfer along explicit isogenies, pairings).
- No automorphism-folded rho on the `j = 0` / `j = 1728` craters: ecbench's prime-curve solvers fold only negation. The crater's extra automorphisms are therefore unused, which is the generic setting this study tests.
- Nothing about deployed curves.
- Curves are not in the ICV1 registry (`registered=false` in every plan) and are named in prose by volcano id and level, not by slug.
- Counts only, isolation L0: no wall time.

## Freeze record

SHA-256 over `sha256sum PROTOCOL.md specs/*/*.json` taken at freezing (before this
section was appended): `4e15ad7f52756cb5ea83d3285efa9bc9f2ecf45c28b25e7f987f484d7a8606c4`.
Sessions are run by `run_sessions.sh` (four lanes, each audited with 8 exact replays; the
script meant to start the longest specs first, but `ls` sorts its arguments, so they start in
alphabetical order of path — scheduling only, nothing measured depends on it).
