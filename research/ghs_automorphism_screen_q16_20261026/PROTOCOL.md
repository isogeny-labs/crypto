# GF(16) cubic elliptic-quotient screen

26 October 2026, Conductor T-5399. This protocol was written after a
bounded exploratory screen of first-listed models. Results below this
protocol must be replayed from the verified local LMFDB public mirror,
not from the live web requests that began returning verification pages.

## Frozen inputs

- Source cohort and direct-B closure: the 128 order rows in
  [`../ghs_cubic_catalog_q16_20261026/certificate.json`](../ghs_cubic_catalog_q16_20261026/certificate.json).
- Curve catalog: `public.av_fq_isog` in
  `/Volumes/data/workspaces/lmfdb-public-mirror-20261019/resumable/`,
  whose `COMPLETE.json` dated 2026-10-25 reports 3,124 verified chunks.
  This is a cross-time mirror, not a transactionally consistent snapshot.
  The table plan names the `curves` column. Retain a hash of the extracted
  128 GF(16) rows and the archive completion manifest.
- Fields: Conway `k=GF(16)` and `K=GF(4096)`. The primary screen covers
  equations whose four hyperelliptic branch roots split over `K`.

## Replay and classification

1. Extract every catalog row labelled `3.16.*` from the local archive.
   Match the 128 labels and source orders in the frozen cohort; missing,
   duplicate, and malformed rows must be reported, not classified as
   negatives. Parse every listed hyperelliptic equation separately.
2. For each smooth genus-three equation `y^2+h(x)y=f(x)`, verify all four
   roots of `h` in `K` and nonzero simple-pole coefficients after
   Artin--Schreier reduction. If this gate fails, record an explicit skip.
3. Enumerate 24 permutations of the four branch points, solve for the
   unique projective linear transformation for the first three, and test
   the fourth. For each valid transformation `T=(Ax+B)/(Cx+D)`, compute
   `H=(Cx+D)^4 h(T)=e*h` and `F=(Cx+D)^8 f(T)`. Solve over `K` the
   `GF(2)`-linear equation `u^2+e*h*u=F+e^2*f` for polynomial `u` of
   degree at most four. Replay every reported lift by substitution.
4. Require a positive control in a known direct-B class with a
   nonidentity lift. A lift is only a curve automorphism; prove that an
   order-two lift has genus-one quotient and identify that quotient's
   elliptic curve before calling it a transfer map. A positive screen
   result still needs an explicit source isogeny and target-dependent cost.
5. Report counts by direct-B versus outside-direct, class and individual
   model. Unknown roots, model formats, or archive discrepancies remain
   separate from no-lift results.

## Prior work and claim boundary

[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
construct the direct-cover map.
[Menezes--Teske, Section 7](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
study isogeny walks.
[Nart--Ritzenthaler](https://arxiv.org/pdf/0905.0546) give explicit
characteristic-two genus-three elliptic quotients when there are enough
involutions. The latter is the positive-map lead tested here; the
class-B control prevents an all-negative parser bug from passing.
[Borodin--May](https://arxiv.org/pdf/2508.16885) address Jacobian
existence obstructions, which do not themselves evaluate a transfer.

An archive curve is preexisting. Keep separate labels for a new exact
source equation, a new eligible family, and a new attack method. A
vulnerability label requires a nonzero evaluated subgroup map, a complete
target solve, and measured total cost below same-target rho.
