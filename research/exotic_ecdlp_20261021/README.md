# Exotic routes to the prime-field ECDLP: GHS/Weil-descent extensions and six falsifiers

Status: pre-registered 2026-10-22. Owner: adam. Reasoning tier: Opus. Execution: Sonnet
runners on isolab (EKS/Karpenter, account 104186811093, us-east-2). Format:
`crypto.experiment/v1` (one spec per hypothesis under `specs/`).

This directory is a brainstorm turned into experiments. It asks whether any extension of
Weil descent / GHS, or any "exotic" substitute for a subfield, threatens the ECDLP on the
two 131-bit Certicom instances the lab is working:

- **ECCp-131** (prime): `p = 1550031797834347859248576414813139942411`, `#E = N` prime
  (131 bits), trace `t ≈ 2^64.7`, `t^2-4p` squarefree (conductor 1, class number ≈ 2^65),
  embedding degree `N-1` (maximal), 2-division polynomial irreducible, not anomalous. Generic
  in every computable invariant.
- **ECC2-131** (binary, `GF(2^131)`, `x^131+x^13+x^2+x+1`): `#E = 2N`, `N` prime (131 bits),
  `|t| ≈ 2^66.3`, `t^2-4q` squarefree, embedding degree `(N-1)/29`. `131` is prime and `2`
  is primitive mod `131`, so there is **no** proper intermediate field and **no** Frobenius
  sub-descent of GHS type; the only descent that exists is to the prime field `F_2` itself.

Both invariant tables are reproduced in `../../` session notes and recomputed by
`H*/verify_instance.py`.

## The organizing conjecture (H0)

Index calculus works in `F_p^*` because `Z` is a monoid with unique factorization and a
*linear* size function `log|n|`: smooth numbers are closed under multiplication and factor
recursively, so a factor base of `~L` small primes yields relations cheaply. The Néron–Tate
height on any abelian variety is *quadratic*, so no set of bounded-size points is closed
under the group law and nothing factors recursively.

> **H0 (conjecture).** There is no monoid `M` with a size function `s: M → R≥0` that is
> *subadditive* (`s(xy) ≤ s(x)+s(y)+O(1)`) together with a surjective monoid map
> `φ: M ↠ E(F_p)` such that `φ` has about `√p` "irreducibles" of size `O(1)` and a random
> point of `E(F_p)` is a product of `O(polylog p)` of them with non-negligible probability.

Every route below is either a candidate monoid `M` (H1 small integers, H2/H5 geometric
incidence, H3 p-adic digits) or a candidate way to dodge the quadratic-height obstruction
on a specific instance. A counterexample to H0 on either Certicom curve is the attack; a
measured obstruction is one more brick in the (still unproven) wall. The experiments are
written as **falsifiers**: each has a Win criterion that would threaten rho and a Kill
criterion that reduces it to a known wall, per `../ECDLP_EXPERIMENT_FRAMEWORK.md`.

## What is closed by theorem vs. open to measurement

`PROOFS.md` separates the two. In one line: subfield descent over `F_p` is *empty* by a
Lang–Weil point count (a factor base of size `p^{1/m}` is never a low-degree algebraic set),
and the generic-group lower bound holds for any algorithm using only the group law,
Frobenius and CM endomorphisms on these two curves. The six experiments probe exactly the
gaps those theorems leave: non-algebraic coordinate structure (H1, H3), the full
Néron–Severi lattice rather than just its sections (H2, H5), and the empirical
Gröbner/first-fall degree of the one descent that does exist in prime extension degree (H4).

| id | route | prior | isolab cost | the question |
|----|-------|-------|-------------|--------------|
| H0 | linear-size monoid on `E(F_p)` | — | tiny | the organizing conjecture; one closure-defect probe |
| H1 | symmetrized Semaev small-roots over `F_p` | low | small | does 4-torsion symmetry push Coppersmith past the naive box bound |
| H2 | Hermitian-surface Néron–Severi relation spectrum | very low | small | is the pushed-down relation matrix non-generic |
| H3 | canonical-lift p-adic digits as a size | very low | small | is any digit functional additive/closed |
| H4 | FPPR prime-degree descent to `F_2` on ECC2-131 | low | medium | does the first-fall degree stay bounded as `n → 131` |
| H5 | Ulmer Legendre at `d = p^f+1`, `f=2` | very low | small | is the specialization kernel computable from geometry |

## Layout

```
README.md              this file
PROOFS.md              theorems (closed) + conjecture statements (open), with proof sketches
H0_monoid/             PROTOCOL.md + runner brief
H1_structured_roots/   PROTOCOL.md + runner brief
H2_hermitian/          PROTOCOL.md + runner brief
H3_canonical_lift/     PROTOCOL.md + runner brief
H4_prime_descent/      PROTOCOL.md + runner brief
H5_ulmer_f2/           PROTOCOL.md + runner brief
specs/EXP-EXOTIC-*.yaml crypto.experiment/v1 pre-registration, one per hypothesis
RESULTS.md             written after the runs; per-hypothesis verdict against Win/Kill
```

## Execution model

`infra/` note: all compute runs on **isolab** via the helper
`~/ops/eks-isolab/run-sage-job.sh JOB PY CPU MEM SECONDS` on the harness host (the only host
the cluster's API allowlist admits). Image: `isolab/tasks@sha256:09b11b29b4caa46c92b36246…`
(pinned SageMath 10.10 + AWS CLI + boto3, built 2026-10-22). Each runner drops a Sage `.py`,
submits, polls `isolab_k8s_status`, and pulls results from `s3://isolab-results-…/runs/<job>/`.
Job success is a correctness/isolation check only; **no performance claim** follows from it,
per the isolab contract. The specs are the machine-readable pre-registration; the isolab run
is driven by the runners, not yet by an `expspec` batch backend (that backend is future work
per `docs/EXPSPEC_PROPOSAL.md`).
