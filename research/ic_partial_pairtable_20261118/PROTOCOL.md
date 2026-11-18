# Frozen preregistration — partial pair tables, toy prime-field ECDLP

Date: 2026-11-18. ID: EXP-IC-PARTIAL-TABLE-20261118-R2. Status: `PREREGISTERED BEFORE ANY RUN`. [pcc:unverified]

## Boundary and previous work
- This is a classical generic pair-sum table engineering experiment, **not** a new ECDLP algorithm. It uses an actual toy elliptic curve, not synthetic random rows.
- Closure baseline: `research/ic_mega_pass_20261031/brief/BRIEF.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md` (generic pair-table known and asymptotically constrained), `research/prime_ic_lead_heatmap_20261114/ATLAS_DELTA.md` (yield conservation), `research/prime_ic_collection_frontier_20261117/README.md` (pair-table build O(B²), relation collection O(p/B²), best batched engine).
- Immediate previous iteration: toy graph elimination, `H-GRAPH-PAIR-001`, 48 solves, frozen hypothesis failed; graph-vs-dense LA was a component-only result. We do **not** compare unrelated cold estimates as if a matched experiment.
- Primary literature: Semaev 2004 (https://eprint.iacr.org/2004/031); Shoup 1997 (https://www.shoup.net/papers/dlbounds1.pdf); the tested operation is classical generic advice vs online work, not Semaev solving.

## Hypothesis and frozen controls
H1: At B=32, a **half-sized** uniformly preselected subset of the full unordered pair table reduces **median cold, single-target counted group additions by at least 10%** vs the full table on the same seeded factor bases and sequential point streams, and >=22/24 independently replayed seeds complete within one group-order sweep. If either fails, H1 is falsified. Selection is deliberately *independent of secret logs and target Q*.

Toy curve: E/F_1009: y²=x³+4x+10, prime group order 1069, generator first lexicographic affine point. Factor bases B=32 primary; B=16 sensitivity. 24 primary seeds from 0x1c2026111840 + [0,23]; 8 sensitivity seeds from 0x1c2026111850 + [0,7]. Fixed seed creates 32 distinct known-answer factor-base points and target Q=kG. Full table = all i<=j, half = ceil(0.50*M), three-quarter = ceil(0.75*M), quarter = ceil(0.25*M), with one uniform shuffled pair-index order. A/A = repeat full table deterministically. All costs start cold including factor-base setup. Matrix use signed-free two-summand rows with odd-cycle rank test.

Use a deterministic **cyclic sequential** relation target walk `a0+t (mod 1069)` common to all arms and one point addition per step (first P=a0G generated with double-and-add, then P+=G), not a fresh scalar multiplication per trial. For descent, scan `Q+tG` from the fixed target Q with identical t order until a table hit. Keep exact witness scalars, pairs, group points, rank progression, and failure rows.

## Cost and verification
Report and preserve separately: factor-base EC group additions; pair-table EC additions and unique keys; collection EC additions, attempts, accepted rows, dependencies, duplicates, rank; matrix construction wall and dimensions; graph linear-algebra modular additions/multiplications/inversions and wall; descent EC additions/attempts; full cold IC EC additions and sensitivity cost `EC_add + w*F_r_ops` for w in {0, 0.1, 1}. Separately run the same public toy target through a matched seeded Pollard rho with all startup and restart additions charged and compare to heuristic sqrt(pi*1069/2). Time in unisolated Python is secondary, not a publishable speed benchmark.

Correctness: independent affine point arithmetic and `rG=O`, every relation `F_i+F_j=aG`, full modular rank and solved logs, all FB points, `recovered*G=Q`, rho recovery, A/A identity. Keep every failure and censoring count. No leaderboard promotion or theorem claim; no asymptotic inference.

Budget: <=32 seeds x four arms (plus A/A), <=1069 attempts/arm, <=60 seconds CPU run, <=50 MiB raw outputs; zero cloud spending. Stop on a verification mismatch rather than silently skipping it. False negative: no independent replay, <22 successes, or median half/full group-add ratio >0.90. No tuning after frozen tests; exploratory outcomes must be labeled.

## Repository/coordination constraints
Required path: `/workspace/isogeny-workspace`; route then `need crypto` (blobless shallow only). The official trunk was not mounted in this runtime. An explicitly labeled offline subset was used to route before the `need` attempt; GitHub DNS failure must be recorded, never represented as successful checkout. Repository instructions read over the GitHub connector. Conductor coordination and pcc signed standing unavailable; this PR contains only [pcc:unverified] diagnostic evidence until independent CI verifies it.
