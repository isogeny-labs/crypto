# Integrated continuation claims review

Reviewer: `/root/continuation_integrity`, 2026-10-30 UTC. Read the current
continuation README; auxiliary-curve, selective-oracle and small-positive-degree
theory notes; their four named independent reviews; and
`problems/prime-divisor-uniform-solver.md`. This is a publication-consistency
review, not a new proof certification, scientific computation or independent
rereading of every primary source.

## Verdict

No material contradiction or newly promoted sub-exponential ECDLP claim was
found. The integrated report correctly distinguishes the following results:

- The auxiliary construction removes the p-1/p+1 divisibility condition from
  the **supporting reduction** for every sufficiently large prime field while
  retaining the main curve's odd prime whole-order hypothesis. It requires
  d=2^t>=8 and p>=256d^4 at the finite level. The construction is described as
  mathematically specified and charged, with no implemented auxiliary pipeline.
- Exact rational order-d torsion is proved available and then checked on an
  actual point. Divisibility of an auxiliary cardinality is expressly not used
  as a cyclicity certificate. The claimed expected d^2 poly(log p) setup agrees
  with the conservative 1/(24d^2) model probability and at-least-half point
  extraction probability in the reviewed notes. This is a setup bound only.
- A complete fast decomposition/refutation oracle is one sufficient route.
  The selective alternative allows false negatives and capped timeouts, needs
  an effective cap and verified yield on each base of the good set, and charges
  failures and restarts. Its positive-instance mean includes all oracle seeds,
  including infinite unsuccessful runs, rather than conditioning only on
  eventual successes. Main-group cancellation still requires the prime-order
  subgroup/whole-order assumption.
- The positive-target result bounds the **final quotient degree** on at least
  rho0/2 of all targets of each simultaneous good base. It does not give an
  initial Gröbner basis, a bound on intermediate elimination or solving degree,
  or a way to recognize the low-degree targets in advance. The README and
  independent review repeat this distinction. The statements about subsequent
  polynomial-in-degree conversion and root extraction are conditional on
  having a suitable initial basis/multiplication representation.

The opening and closing report statements retain that the requested novel
sub-exponential algorithm has not been achieved. The tiny implemented solver
is still explicitly exponential. Earlier frozen reviews have historical scope
and need not be rewritten to claim they had already reviewed later additions.

## Minor current wording refinements

1. The problem record title still says **“A uniform complete solver ... is
   missing.”** Its body correctly permits the weaker selective route. A title
   such as “A sufficient efficient divisor oracle is missing” would make that
   broader unresolved obligation visible immediately. The current title is
   true but emphasizes the stronger contract after it has ceased to be
   necessary.
2. The Kummer introduction says primes without compatible d are outside
   **“the quantitative family below.”** Prefer **“this Kummer family”** so this
   sentence does not appear to govern the later auxiliary extension too.
   Likewise the torus paragraph's **“no ... all-prime-field construction
   follows”** is clearer as **“none follows from the torus construction
   alone.”** The following auxiliary section then removes that restriction
   with explicit additional work and hypotheses. These are scope clarifications,
   not a mathematical contradiction in the existing local reading.
3. The README's **“no such oracle or computable cost cap”** and problem record's
   **“no such oracle, computable cap ...”** should explicitly mean a cap/yield
   pair satisfying the sub-exponential contract. The current code has
   computable exponential/development caps. Prefer **“no computable
   sub-exponential cap with proved sufficient verified yield”** where the
   selective contract is intended, or qualify a complete solver's cap as
   sub-exponential. This avoids suggesting that computability alone is the
   remaining hurdle.

None of these refinements changes raw results, the conditional proof, or the
remaining algorithmic gap. They were sent to the parent before publication.

## Remaining claim boundary

The report may claim a reviewed auxiliary construction and supporting bounds
under their exact assumptions, a broadened sufficient oracle contract, and a
small final quotient on a sufficiently large positive-target set. It cannot
claim an efficient verified-yield oracle, initial-basis bound, implemented
auxiliary pipeline, all-prime-curve attack, certified novelty, complete corpus
review, or achieved sub-exponential ECDLP. Current integrated prose preserves
that boundary.

No repository files, solver sources or raw records were modified by this
reviewer. No scientific computation was performed. The separately pending
unbound native-CAS unit result was outside this requested review.
