# Lead stub: low-degree embeddings with small value set for the DLP with auxiliary inputs

Proposed id: `LEAD-RHO-cheon-value-set-embedding`. Proposed family: `generic`. Proposed disposition: `open_unexamined`.
This is a stub for the atlas owner to file. It is written here because the atlas in `crypto-autoresearcher` is vault-encrypted and
could not be read or written from this session. Every statement is `[pcc:unverified]`.

## Question

Cheon's `r ± 1` algorithms for the DLP with auxiliary inputs (given `G, [α]G, …, [α^d]G` in a group of prime order `r`) cost about
`√(r/d)` plus a term in `d`, and only when `d` divides `r − 1` or `r + 1`. Cheon and Kim (LMS J. Comput. Math. 2016; ePrint 2012/609)
recast the method as a choice of polynomial `f` of degree `d` over `F_r`: the cost is `Õ(√(r/τ_f) + d)`, where `τ_f` counts the
absolutely irreducible factors of `f(x) − f(y)`. `x^d` recovers the `r − 1` case and Dickson polynomials the `r + 1` case.

**Open in the literature:** whether, for `d` not dividing `r ± 1`, some `f` of degree `d < r^{1/3}` has a value set of size much
smaller than `r` (equivalently large `τ_f`). Cheon–Kim show none exists for `d | Φ₃(r)`, and Kim–Cheon–Lee (Math. Comp. 2014) close
embeddings into `F_{r^k}`, `k ≥ 3`.

## Scope

- Applies only when the protocol leaks the powers `[α^i]G`. It does not touch the plain ECDLP: `LEAD-RHO-cheon-aux-acquisition`
  (closed, theorem) shows the powers cannot be produced from `G, [α]G` without solving square-DH.
- Its worth is a parameter statement for schemes that publish powers of a secret (`q`-SDH, KZG setups), as with
  `LEAD-RHO-cheon-deployed-applicability`.

## What is already settled here

- **Elliptic-curve and torsion embeddings** (an auxiliary curve `E'/F_r` with `d | #E'(F_r)`, or its `[d]`-map in place of `x^d`):
  closed by measurement. The canonical label is a quotient of polynomials and cannot be formed in the exponent, so the comparison
  falls back to about `r/d` pairwise tests (`research/notes/ecdlp-general/RESEARCH_TORSION_AUXILIARY_INPUTS.md` §6; honest rows
  500–12,000× rho at 14–20 bits, oracle rows at 33–139× the `√(r/d)` floor).
- **The `r ± 1` cases** are measured in ecbench (`research/cheon_ecbench_wide_20261106/`, PR #3932 intervals).

## Novelty check (partial)

- Read: the 173-lead index of `research/lead_sweep_20261110/leads.json` (no value-set or Cheon–Kim lead); the 13 new leads of
  PR #3938 (none); `crypto` and `cryptanalysis` full-text search for "Cheon–Kim", "value set" and "elliptic embedding" (only the
  torsion note cites Cheon–Kim, as literature).
- **Not checked:** the vault-encrypted atlas itself, and research-kb (no server or local catalog in this session). File the lead only
  after both are checked.

## Cheapest discriminating next step (desk)

Read Cheon–Kim 2016 §§3–5 and record (a) the exact theorem linking `τ_f` to cost, (b) the exact non-existence statement for
`Φ₃(r)`, and (c) whether any later paper (search citations of ePrint 2012/609) has resolved the existence question for degree below
`r^{1/3}`. No experiment is proposed before that reading.
