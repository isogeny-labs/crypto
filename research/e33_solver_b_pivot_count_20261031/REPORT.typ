#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E33: solver B's pivot selection, counted (and two cheaper selections)

*Run report and results, 2026-11-01.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.35 (pre-registration, commit `27663cc4d7`, before any code) and §9.37 (results). Code `c443138920` (unit-test compile fix and checks example `c689d19821`; library build unchanged). Binaries pinned by sha256 in `experiments/37_solver_b_pivot_count/binary_sha256.txt`. Records: `experiments/37_solver_b_pivot_count/` (`stage_a.{json,log}` with the JSON in the artifact store, sha256 `7bc6e00b…`; `e2e_*_karatsuba_p*s*.json` and logs; `identity_existing_solvers.json`; `checks.log`; `tables.md`; `summary.json`). Host: the 4-vCPU Claude Code container, at most two cores. isolab-eks is not reachable from it (no AWS CLI; the GitHub route was refused through the proxy). No isolation tier is earned: operation counts only, no wall-time claim.

== What ran
- *Stage A:* E31's calls exactly. On each of E25's ten GLS curves ($p in {101,149,197,269,397}$, seeds 1, 2), the first 1,000 targets of E25 arm A's stream, one thread. Seven solvers ran per call from the same per-target seed: the existing solver (E25), A, A counted, B, B counted, B′ and B″. That is 70,000 solves, 0 failures.
- *End to end:* E27's command (E31's pipeline with Karatsuba $FF_(p^4)$ products) with B counted, B′ and B″, on the ten curves, both arms, `RAYON_NUM_THREADS=4`. 30 runs, 0 failures. B″ was carried by the registered rule: its Stage A $S(0.1)$ was below A's on every curve.
- *Identity:* the pre-E33 binary and E33's at $p = 101$ seed 2 for `macaulay`, `chain`, `chain-sparsest` and `chain --karatsuba`. The checks example `e33_checks` (32 calls, two negative controls).

== Counting rule (registered)
One native operation per Macaulay entry examined for its zero status and one per key comparison, priced at $g dot 97$ $FF_p$ multiplications, $g in {0, 0.1, 1}$. B: an initial count of every entry, $|"bucket"| - 1$ comparisons per contested column, and a recount of $"cols" - "lead"$ entries per updated row. B′: counts inherited from the equations, plus two status tests per entry the pivot row touches and one per row. B″: inherited counts and the Markowitz fill bound, at two operations per updated row. Lead scans are common to every rule and are emitted separately (sensitivity only).

== Measured
- *Identity (G2.124):* all 10,000 Stage A calls agree with the existing solver for every solver. Phase counts are equal (A counted = A, B counted = B′ = B), the B′ digests equal B's, and B′'s maintained counts never differ from a recount. All 30 end-to-end records keep every non-product field of E31 B's records (`folded.recovered` is a key added after E31). The old and new binaries give identical records for the four existing configurations. Both negative controls were detected on 32/32 calls. The in-crate unit tests type-check but were not executed (memory; see Limits).
- *Selection work per call* (Stage A; end to end agrees): B $1.049$–$1.057 dot 10^6$ (initial 176,064, comparisons 3,681–3,713, recounts $8.69$–$8.77 dot 10^5$); B′ $2.93$–$2.98 dot 10^5$ ($0.279$–$0.282$ of B); B″ 12,696–12,804. B″'s $C$ median is $1.205$–$1.222 times$ B's.
- *End to end* (20 rows), as ranges of $S_"E25"\/S_X$, $S_A\/S_X$ and $S_X\/"rho"$ at 0 / 0.1 / 1 GAE:
  - B: E25 $4.53$–$4.76$ / $0.274$–$0.276$ / $0.157$–$0.158$; A $1.82$–$1.86$ / $0.193$–$0.197$ / $0.149$–$0.152$.
  - B′: E25 $4.53$–$4.76$ / $0.678$–$0.683$ / $0.404$–$0.407$; A $1.82$–$1.86$ / $0.478$–$0.487$ / $0.384$–$0.393$.
  - B″: E25 $3.83$–$3.98$ / $1.457$–$1.497$ / $0.980$–$0.997$; A $1.538$–$1.558$ / $1.046$–$1.048$ / $0.9465$–$0.9480$.
  - Distance to the matched rho (walk (iii) of E27): at 0 GAE, B and B′ $3.45 dot 10^3$–$2.19 dot 10^5$ and B″ $4.12 dot 10^3$–$2.61 dot 10^5$; at 1 GAE, B″ $9.58 dot 10^4$–$7.01 dot 10^6$. Rho is ahead of every row at every weight.

#figure(image("figures/selection_parts.svg", width: 80%), caption: [Pivot-selection native operations per call by part, mean of the ten curves' Stage A means (1,000 calls each). B″'s comparisons (≈ 4,100) and updates (≈ 8,150), and B′'s initial counts (≈ 506), are too small to see at this scale; exact values are in `tables.md`. Source: `summary.json`.])
#figure(image("figures/stage_a_per_call.svg", width: 100%), caption: [Stage A mean per-call $S(g) = C + 97 g ("entries" + "selection")$ per curve, for E25, A, B (selection counted), B′ and B″, at 0, 0.1 and 1 GAE. At 0 GAE, B and B′ coincide (same pivots, same $C$). Source: `summary.json` (from `stage_a.json`).])
#figure(image("figures/band.svg", width: 100%), caption: [End to end, min–max over the 20 rows (marker at the geometric midpoint). $S_A\/S_X$ uses E27's solver-A records (Karatsuba); $S_"E25"\/S_X$ uses E25's records in schoolbook arithmetic; $S_X\/"rho"$ uses walk (iii). Above parity, X is cheaper. Source: `summary.json`.])

== Verdicts (pre-registered, §9.35)
G2.124 holds (the unit tests were not executed; disclosed). G2.125, G2.126, G2.127, G2.128, G2.129, G2.130, G2.132 and G2.133 hold. G2.131 is *falsified* against E25 at 1 GAE: B″ is behind E25 on all 20 rows ($0.981$–$0.997$). It holds against A.

== Ranks (only where the order holds at 0, 0.1 and 1 GAE on every row)
- B, B′, B″ against A: *unresolved*. Each is ahead at 0 GAE. B and B′ are behind from 0.1 GAE; B″ is ahead at 0.1 GAE and behind at 1 GAE. A remains the ranked solver.
- B, B′, B″ against E25: *unresolved* (each is behind E25 at 1 GAE).
- Rho is ahead of B, B′ and B″ across the band.

== What it says, and what it does not
Counting B's selection is accounting, not an improvement. It settles what §9.24 left open: B's advantage exists at 0 GAE only. Its pass reads the matrix about six times per call, which is far above the validator's lower bound of one read. B′ keeps B's choices at 28 % of the work and is still behind. B″ is cheaper than A with its own selection work counted at 0 and 0.1 GAE. At 1 GAE its $1.27 dot 10^4$ selection operations ($1.2 dot 10^6$ multiplications) exceed its $2.3 dot 10^5$ saving. It is therefore not ranked, and not claimed as an improvement across the band. The band at 1 GAE is set by the 176,064 Macaulay entries per call, which no pivot rule removes.

== Limits
- The four in-crate unit tests (E31's and E33's in `klein_v4::tests`) type-check (`cargo check --release --lib --tests`) but were not run. A release test build of the library needs about 9 GB of RSS. The first attempt pushed the shared container's memory cgroup over its limit, and the kernel killed it together with a parallel experiment's process (E32's `e32_ladder`). The solver-level test runs instead as `examples/e33_checks.rs`. The echelon-level test is covered only indirectly, by equal phase counts and outputs on 10,000 calls.
- E25's lead scans in the sensitivity are A's (same echelon, same rows). The existing solver's other unpriced traffic (normal forms, Hessenberg) is not emitted for any arm.
- Not validated: no replay certificate or validator report yet, so no leaderboard or `benchmarks/` row (validation first, per the task).
