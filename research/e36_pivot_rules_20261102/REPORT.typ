#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E36: pivot rules whose selection work fits the §5a band

*Run report and results, 2026-11-02.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.41 (pre-registration, commit `cd7e571f7c`, before any code) and §9.43 (results). Code: `a86c6c846d` (module `src/cryptanalysis/chain_pivot_rules.rs`), `78add3ddbb` (`klein_v4` dispatch hook, its own commit), `df82b2e4b3` (harness: `examples/e36_stage_a.rs`, `e36_checks.rs`, `e36_tables.rs`, `run_e2e.sh`), control revision `ab560e5cf1`. Binaries built from an export of `df82b2e4b3` (no code of the parallel E35), pinned by sha256 in `experiments/40_pivot_rules/binary_sha256.txt`. Records: `experiments/40_pivot_rules/` (`stage_a.{json,log}`, `checks{,_v1}.{json,log}`, `identity_existing_solvers.json`, `identity/`, `e2e_<rule>_karatsuba_p*s*.json` and logs, `tables.md`, `summary.json`). Host: the 4-vCPU Claude Code container, runs on two cores. isolab-eks is not reachable from it (no `aws`, no `kubectl`). No isolation tier is earned: *operation counts only, no wall-time claim.*

== Question and budget
Can a pivot rule for the $k = 4$ chain solve (E31), with every native operation of its selection counted, beat solver A at 0, 0.1 *and* 1 GAE (GAE $= 97$ $FF_p$ multiplications; $S(g) = "F"_p" mults" + 97 g dot "native ops"$)? Every rule builds the same matrix, so per call $S_A(g) - S_X(g) = (C_A - C_X) - 97 g dot "sel"_X$. With B's pivot quality the budget at 1 GAE is $(C_A - C_B)\/97 approx 3,050$ operations per call, while the number of row updates is $approx 4,070$ per call: any rule that pays per row update or per extra candidate (B, B′, B″) cannot win at 1 GAE. Only $O("rows")$ or $O("columns")$ selection can.

== Rules (registered)
- *F, arrival order* (`chain-fifo`): each column's pivot is the bucket's first-arrived row (never-updated rows first). Selection work 0 (arrival order is A's own bucket bookkeeping).
- *S, static order* (`chain-static`): rows permuted once per call into blocks ordered by their equation's nonzero count, then A's rule. $approx 512$ operations.
- *H, hybrid* (`chain-fifo-static`): F where a never-updated row is present, otherwise S; one operation per contested column for the branch. $approx 846$ operations.
- *K, bounded Markowitz $k = 2$* (`chain-est-k2`): B″'s maintained estimate, comparing only the first two arrivals. $approx 8.6 dot 10^3$ operations (a test of the bound).
- *L, last arrival* (`chain-lifo`): negative control, 0 operations.

== What ran
- *Stage A:* E31/E33's calls exactly: E25's ten GLS curves ($p in {101,149,197,269,397}$, seeds 1, 2), the first 1,000 targets of arm A's stream, one thread, ten solvers per call (existing, A, A counted, B″, A's rule through E36's echelon, F, S, H, K, L): 100,000 solves, 0 failures.
- *End to end:* E27's command (Karatsuba $FF_(p^4)$ products) for the four carried rules F, S, H, K (carry rule: Stage A $S(0.1)$ below A's on every curve), ten curves, both arms, `RAYON_NUM_THREADS=4` on two cores: 40 runs, 80 arm records, 0 failures, every log and walk correct.
- *Identity and controls:* `identity.sh` (existing solvers at $p = 101$ seed 2) and `e36_checks` (32 calls).

== Identity and controls
- Every solver agreed with the existing solver on all 10,000 Stage A calls. Existing, A, A counted and B″ reproduced E33's per-call records on every call. A's rule through the new echelon reproduced A's phase counts, digest and lead scans on every call. All 40 end-to-end records equal E27's solver-A records except in the solve's multiplications and what derives from them. `macaulay`, `chain`, `chain-sparsest`, `chain --karatsuba` and `chain-sparsest-est --karatsuba` are byte-identical to E33's hashes and stored record.
- The perturbed-coefficient control changed every non-empty solution set (22/22) under each rule. F's and L's pivot digests differ from A's on every call.
- *The registered S/H control (inverse permutation wrong by one row) was detected on 0/32 calls.* A diagnostic added afterwards shows that the corruption moved different row contents on 32/32 calls but changed no output: the moved rows are pivots the normal forms never read (inferred mechanism). A post-hoc variant (the rows of the two last pivot columns) changed the output on 27/32 calls. *By G2.154 as registered, this stops E36*; the end-to-end data below are reported as measured, not as a pre-registered rank.

== Measured
#figure(image("figures/stage_a_ratios.svg", width: 100%), caption: [Stage A, per-call means over 1,000 calls per curve: $S_A\/S_X$ per curve at 0, 0.1 and 1 GAE for F, S, H, K, L and E33's B″ (reproduced). Above 1, X is cheaper than A. Source: `experiments/40_pivot_rules/summary.json` (from `stage_a.json`; binary `df82b2e4b3`).])

#figure(image("figures/budget.svg", width: 78%), caption: [Selection work per call against the per-call saving over A, $C_A - C_X$ (Stage A means, ten curves per rule). A rule beats A at weight $g$ only above the line $97 g dot "selection"$. F and L do no selection work (drawn at 1). K and B″ lie between the 0.1-GAE and 1-GAE lines: ahead at 0.1, behind at 1. Source: `summary.json`.])

Per call (Stage A, ranges over the ten curves): $C_A\/C_X$ is F $1.470$–$1.548$, S $1.411$–$1.460$, H $1.510$–$1.558$, K $1.602$–$1.716$, L $0.708$–$0.720$. Selection per call: F, L 0; S 512–513; H 846–847; K 8,529–8,755. $S_A\/S_X$ at 1 GAE: F $1.0115$–$1.0129$, S $1.0077$–$1.0086$, H $1.0076$–$1.0083$, K $0.968$–$0.970$, B″ $0.946$–$0.948$.

#figure(image("figures/e2e_band.svg", width: 100%), caption: [End to end, 20 rows (E25's ten curves × arms A, B), Karatsuba products. Top: $S_A\/S_X$ against E27's solver-A records at 0, 0.1, 1 GAE. Bottom: $S\/"rho"$ against walk (iii) of E27 (log scale), with A for reference. Source: `experiments/40_pivot_rules/e2e_*_karatsuba_p*s*.json` via `summary.json`.])

#table(columns: 5, align: (left, left, right, right, right),
  [*X*], [*quantity*], [*0 GAE*], [*0.1 GAE*], [*1 GAE*],
  [F], [$S_A\/S_F$], [1.464–1.528], [1.094–1.105], [1.0115–1.0128],
  [F], [$S_"E25"\/S_F$], [3.65–3.91], [1.524–1.579], [1.046–1.067],
  [F], [$S_F\/"rho"$], [$4.18 dot 10^3$–$2.71 dot 10^5$], [$1.92 dot 10^4$–$1.27 dot 10^6$], [$8.95 dot 10^4$–$6.56 dot 10^6$],
  [S], [$S_A\/S_S$], [1.407–1.445], [1.083–1.090], [1.0078–1.0085],
  [H], [$S_A\/S_H$], [1.504–1.538], [1.096–1.102], [1.0076–1.0083],
  [K], [$S_A\/S_K$], [1.594–1.688], [1.071–1.084], [0.968–0.970],
  [K], [$S_"E25"\/S_K$], [3.97–4.31], [1.492–1.548], [1.001–1.021],
  [A (E27)], [$S_A\/"rho"$], [$6.39 dot 10^3$–$4.06 dot 10^5$], [$2.13 dot 10^4$–$1.39 dot 10^6$], [$9.07 dot 10^4$–$6.64 dot 10^6$],
)
F, S and H are ahead of A and of E25 on 20/20 rows at every weight; K is ahead of A at 0 and 0.1 and behind at 1 GAE on 20/20 rows (unresolved), and ahead of E25 at every weight. Rho is ahead of every row of every rule at every weight.

== Verdicts (pre-registered, §9.41)
G2.154 *falsified*: the S/H inverse-map control was not detected, so the registered kill criterion fired; every other identity check holds. G2.155, G2.156, G2.158, G2.160 and G2.163 hold. G2.157 is falsified high (S helps more than predicted). G2.159 is falsified low on its quality band (K's $C$ is below B″'s); its bound part holds (K is behind A at 1 GAE). G2.161 is falsified: S was predicted to lose and did not, and under σ2 the negative control L moves ahead of A at 1 GAE (an accounting effect on A). G2.162 holds as measured, but is not admissible as a pre-registered rank.

== What it says, and what it does not
The budget argument holds: rules that maintain anything per row update (K, B″) cannot beat A at 1 GAE. Taking each bucket's first-arrived row, at zero selection cost, keeps 90.7–97.2 % of B″'s per-call saving over A. In these measurements it is ahead of A and E25 at every weight on all 20 rows, and S and H are too. *Status: not ranked.* The registered kill criterion fired on a control that applies only to S's and H's inverse permutation. F permutes nothing and passes every other check, but a pre-registered rank cannot be recovered after the fact. F, S and H need a re-registration (E36b) with a control that corrupts only rows the solve reads, or an independent validation, before any of them is ranked or called an improvement. *Distance remaining* (F as measured, best row $p = 397$ seed 1 arm B): $4.18 dot 10^3 times$ the matched rho at 0 GAE (A: $6.39 dot 10^3$), $1.92 dot 10^4 times$ at 0.1 and $8.95 dot 10^4 times$ at 1 GAE (A: $9.07 dot 10^4$). At 1 GAE the Macaulay entries ($approx 1.8 dot 10^5$ per call) are still almost all of $S$.

== Limits
- Kill criterion fired (above); no rank is claimed.
- The module's unit test type-checks (`cargo check --release --lib --tests`) but was not executed: a release test build needs about 9 GB of RSS, more than this shared container has. The solver-level checks run as `examples/e36_checks.rs`.
- The checks binary `e36_checks_v2` was built from `df82b2e4b3` with `chain_pivot_rules.rs` and `e36_checks.rs` taken from `ab560e5cf1`; the change touches control paths and a diagnostic field only.
- σ1–σ3 were evaluated in Stage A only. E24's curves were not rerun. Operation counts only; no wall time.
- Not validated: no replay certificate or validator report, so no leaderboard or `benchmarks/` row.
