# openai/math — relevance scan (2026-11-24)

Clone: `/Volumes/SSD990-2/openai-math` (shallow, 3.9 GB, Lean toolchain v4.34.1).
Corpus: 730 unrefereed model-generated manuscripts, 375 families, ~42% of top-line results Lean-formalized.
Full family list: `families.tsv` (id, title, blurb). Treat unformalized results as unverified.

No family addresses ECDLP, index calculus, summation polynomials, Gröbner/solving degree,
Weil descent, isogeny-based crypto, or lattice reduction. Nearest hits:

| Fam | Result | Why it matters to us | Lean |
|---|---|---|---|
| 142 | Deterministic complete factorization of degree-n polynomials over F_p in poly((n+1) log p) | Removes randomness from root-finding; touches the genroot IC oracle (crypto#4755) and the Semaev root-finding step. If it holds, it changes nothing in asymptotic IC-vs-rho cost (root finding is already cheap randomized) but is a citable deterministic bound | **none** |
| 279 | Exact quantum factoring over a fixed finite gate set | PQ framing (pqscan reports, threat-model text) | yes (`lean/docs/279.md`) |
| 032 | Rational Hodge conj. for CM abelian varieties ⇒ Tate conj. for all abelian varieties over finite fields | Background for HEC/abelian-variety point counting and endomorphism arguments | yes (`lean/docs/032.md`) |
| 002, 006, 030 | Full BSD at low Selmer corank; Goldfeld densities; modularity over imaginary quadratic fields | EC arithmetic over number fields; LMFDB-adjacent, not ECDLP | — |
| 019 | Local p-adic section conjecture | Curves of genus ≥2; background only | — |
| 105, 106, 112 | 2-to-1 games, 3-coloring hardness, depth-3 circuit bounds | Complexity background for SAT/ETH discussions (ic-prime-eth-leads) | — |

## Suggested follow-ups
1. Read family 142 (`preprints/Deterministic-Polynomial-Factorization-over-Prime-Fields-October-4-2026/`) closely; check the claimed algorithm against Shoup/Evdokimov lineage before citing.
2. Optional: ingest 142/279/032 into research-kb.

Disk: SSD990-2 had ~1.1 GB free after clone. Delete with `rm -rf /Volumes/SSD990-2/openai-math` if space is needed.

---

# Review: family 142 vs the genroot oracle (2026-11-24)

Source read: `preprints/Deterministic-Polynomial-Factorization-over-Prime-Fields-October-4-2026/build/sections/` (intro, analytic section; other sections skimmed by title only — proofs NOT checked).

## What it claims
- Thm 1.1: deterministic complete factorization over F_p in O(((n+1)log p)^(10^12)) bit ops. No GRH, no randomness.
- Thm 1.2 (the real content): a deterministic reduction, polynomial in B=(n+1)(L+1) and in the *numerical size* E of supplied auxiliary primes ℓ_q (ℓ≡1 mod 12q, p not a q-th power mod ℓ), for each prime q ≤ n.
- Mechanism: Berlekamp → split a totally split squarefree F. Even degree: tournament orientation of root differences using a 2-primary generator (Rónyai 1988-style). Odd degree: curve Y^q=F(X), λ=1−σ acting on the Jacobian; lifting ramification classes through λ and a lattice-of-infinity-divisors obstruction forces a label difference. Divisions done via norm equations / maximal orders in split cyclic algebras.

## Where the weight sits
- Unconditional status rests entirely on the companion "Primitive roots for every admissible integer base" (Thm 1.2 there): **every** finite-order Hecke L-function of every cyclotomic field ⊇ μ_12 is zero-free in Re s > 1−10⁻⁶, uniformly in conductor and height. That is a uniform zero-free strip — it would kill Siegel zeros and is far beyond anything known. Same claim family as result 003 (quasi-RH). Treat as unverified until the analytic companion is independently checked; the 142 result is only as good as that.
- Conditional on GRH, the reduction alone would give polynomial time in both n and log p, improving Evdokimov's (n^{log n} log p)^{O(1)}. That part is the plausible, checkable contribution — it needs a careful read of sections 04–07 (divisor arithmetic, λ-division, odd splitter).
- No Lean formalization.

## Relevance to genroot (crypto#4755, commit f0671e10b5c)
- None in practice. genroot is an exact meet-in-the-middle solver for the Semaev S3 chain over small fields (prime r~2^20, F_2^13..17); its root-finding field work is already reported-but-unpriced, and the measured gap is 21-635x rho from structure/wall time, not from randomness in root-finding.
- Randomized Cantor–Zassenhaus / Frobenius-gcd root finding is Õ(n·log p) field ops with negligible failure probability; a deterministic algorithm with exponent 10^12 (or even a GRH-conditional polynomial with large constants, cyclic-curve Jacobians and maximal-order computations) cannot be competitive and does not change IC-vs-rho asymptotics.
- Only use: a citation if a writeup ever needs "deterministic polynomial-time root finding" as a complexity-theoretic aside — cite as unrefereed and conditional on the companion analytic theorem, or prefer citing Evdokimov/GRH.

## Verdict
Do not integrate. No code change to genroot. Low-priority reading item at most.
