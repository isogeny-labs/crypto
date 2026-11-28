# Run 55 — exact Prime31 Dickson-2 relation-yield census

Date 2026-11-22 UTC. **Independent enumeration verified locally. Original Rust replay committed, CI status separately tracked. Classification: a negative feasibility gate for one existing index-calculus relation family; NOT an improved complete ECDLP solver.**

## What was computed, not estimated

Unchanged ordinary Prime31: `p=2^31-1=2147483647`, `E:y^2=x^3+847514846*x+2087444724`, `n=2147446831`, cofactor 1.

The lab's original *Rust* Dickson construction `research/prime31_ic_20261115/native/lib.rs` builds up to depth 10 because `2^10 | p+1`. Independently implemented in Python, its first admissible seed is **3** and depth-10's 1024 distinct Dickson x-values give **509 canonical signed-quotient points**. Their plus/minus lifts form **1018** distinct rational points.

An independent C++ group-law implementation (no native-Rust arithmetic reuse in this validator, modular inverse via Fermat) **enumerated every unordered signed pair**, including doubles and inverse pairs. Exactly:

| Object | Exact count |
|---|---:|
| Canonical factor-base points | 509 |
| Signed points (±) | 1,018 |
| Unordered two-point decompositions | 518,671 |
| Distinct E(Fp) sums, including identity | **518,085** |
| Colliding pair records | 586 |
| Inverse-to-identity pairs (trivial) | 509 |

For an independent uniformly random relation target `R=aG+bQ` (assuming `a` is uniform mod prime `n`), **the exact probability of at least one two-summand signed decomposition** is

```
518085/2147446831 = 0.00024125626419292708
mean independent uniformly random targets to first hit = 2147446831/518085 = 4144.97009
expected nontrivial unordered pair records per query
  = (518671-509)/2147446831 = 518162/2147446831
```

This is exact for this fixed support and uniform group distribution. It is not an inference from Poisson or heuristic smoothness.

## Independent holdout check against the existing ordinary curve

The archived run50/run54 4,096-point public hash-to-curve cohort (`adaptive4096.csv` SHA256 `07dfd381c1d069f4610e907ca3e7f9922ebb599ed473a18dca8e010652bb6e6a`) was checked against all **518,085** sums, after verifying every target satisfies the original EC equation.

- **2/4,096** public targets lie in this two-summand support, indices **666** and **2433** (zero-based).
- Expected value under independent uniform sampling: `4096*518085/n = 0.9882`; two observed is compatible with the expected rarity. No unproven uniformity inference from the empirical sample is required.
- Independent local C++ `g++ -O3 -std=c++17` reference census took about **0.15 seconds** for pair enumeration. This is a *reference enumeration timing*, NOT F4, a relation-collection throughput benchmark, or a qualified index-calculus solve.

Source: `run55_reference.cpp` in this directory; optional parameter points it at the archived `adaptive4096.csv`. Separate source-linked Rust parity: `research/prime31_ic_20261115/native/examples/prime31_dickson2_yield_census.rs`, with CI `.github/workflows/prime31-dickson2-yield.yml`. Rust uses **original** `Base::build`, `Arithmetic::add`, `Arithmetic::neg`, `check_curve` and verifies all expected counts. It does not duplicate or replace the F4 engine.

## Exact yield counts for every supported Dickson depth

`m` is the number of unsigned (canonical-y) base points in the earliest valid split seed. The right column is only the pair-count upper bound `(2m)(2m+1)/(2n)`, not a measured probability except at depth 10.

| depth | m | unordered signed pairs | upper bound on P(random hit) |
|---:|---:|---:|---:|
| 2 | 2 | 10 | 4.66e-9 |
| 3 | 4 | 36 | 1.68e-8 |
| 4 | 8 | 136 | 6.33e-8 |
| 5 | 13 | 351 | 1.63e-7 |
| 6 | 31 | 1,953 | 9.09e-7 |
| 7 | 64 | 8,256 | 3.84e-6 |
| 8 | 135 | 36,585 | 1.70e-5 |
| 9 | 256 | 131,328 | 6.12e-5 |
| 10 | **509** | **518,671** | **2.41529e-4** |

The exact 10-depth hit probability is slightly smaller due to 586 collisions.

## Fully rank-complete hypothetical budget (explicitly conditional)

If an implementation insists on **509 new nontrivial random relation occurrences** to learn its factor-base logs, even treating every occurrence as independent and every failed query as free apart from oracle invocation, a uniform i.i.d. random-shift stream requires **at least 509*n/518162 = 2,109,476.26 oracle calls in expectation**, by the expected record rate and Wald's identity. Real rank dependencies, false/no-progress attempts and F4 solve cost can only make this *particular full-rank plan* more expensive.

- Historical *cross-run*, fully charged 1024-target shared negation-BSGS baseline from #4532 and #4793: **170.113 ms**, not remeasured in run55.
- Under the above conditional plan, even allowing all other IC costs to be zero, the two-summand oracle would have to finish a public-only query in **under 80.65 nanoseconds on average** to beat that historical throughput over 1024 targets. This is a sensitivity calculation, not an observed time, and no same-host speedup is claimed.

**Critical limitations:** A graph cycle or an Amadori–Pintore–Sala one-relation method can solve target logs before complete 509-log recovery. Built-in geometric identities could lower required stochastic rank. Adaptive nonuniform relation targets or a different support also fall outside the uniform-stream calculation. Therefore this is **not a universal lower bound on ECDLP or all two-summand index calculus**. Its strongest unconditional finding is the exact fixed-support hit probability.

## Prior-work checks and concrete next experiment

| Lead | Current state | Decision |
|---|---|---|
| Original native Dickson 2-summand tower F4 | Genuine prime-field F4 with toy complete solve; exact Prime31 oracle support now found sparse | Keep as oracle correctness control; do not spend high-budget bulk relation collection until a new yield mechanism is found |
| #4901 run54 signed25 seven-leaf | 25-variable/27-equation chart; verified witnesses, no blind root | Preserve as algebraic input; low-degree admission CI remains cold-build-blocked |
| #4812 14-variable Horner F3 | Original-Rust correctness checks; no blind root extraction | Prioritize bounded public-only F4 against frozen 46+/18− corpus |
| #4822 4-variable quotient/RR9 Krylov | Rust tests pass on smaller registered cases, no complete Prime31 extractor | Highest-leverage public-only sparse kernel test; don't conflate predicates |
| #4692 owner for RR9/selector | Launch gate checkpoint missing; no registered solve row | Unblock approved checkpoint, then run pre-registered smoke before scaling |
| Genus-4 IC / Fp² IC | Different groups/fields, not ordinary Prime31 ECDLP | Keep positive controls separate |
| #4793/#4532 fast shared BSGS | Historical 170.113 ms/1024 unknown-target complete solving | Mandatory matched frontier, not a new run55 timing |

Literature first: Amadori, Pintore, Sala (2018), *On the discrete logarithm problem for prime-field elliptic curves*, DOI 10.1016/j.ffa.2018.01.009 (one-relation strategies specifically limit the scope of any naive full-rank counting argument). GitHub source paths listed above and PRs #4692/#4812/#4822/#4793 reviewed; KB connector unavailable in this runtime, so this is a bounded overlap audit.

**Next decisive milestone remains public-only extraction of unknown leaf indices from a genuine *larger-support* non-generic RR9 or Semaev system, integrated into complete original-Rust cold and amortized log recovery, with all negative attempts charged and same-runner BSGS/rho/index-calculus controls. This report makes no claim of such an implementation or a cost advantage.**
