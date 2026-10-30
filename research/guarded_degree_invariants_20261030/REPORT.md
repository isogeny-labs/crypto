# Proof report: guarded degree invariants

## Definitions and prior work

Before choosing this approach we read the existing fall-degree ledger,
`guarded_fall_degree_20261028/{README.md,SOURCES.md,STATUS.md}`, the actual
`GuardedExactPair`, `GuardedTops`, `FilteredClosure` and `GuardedLastFall`
Lean modules, and the binary native-checker contract. We then checked the
primary sources:

1. Caminata–Gorla, [Solving degree, last fall degree, and related invariants,
   arXiv:2112.05579v2](https://arxiv.org/html/2112.05579v2), Definitions
   1.1/1.4/1.5 and Theorem 3.1. Their degree of regularity assumes eventual
   fullness of the homogeneous top ideal; their solving-degree equality is
   for degree-compatible term orders. No equality between first fall and
   regularity is assumed.
2. Semaev, [arXiv:1504.01175](https://arxiv.org/html/1504.01175), Section 2:
   the odd and ordinary-binary third summation polynomial formulas. Its
   first-fall complexity assumption is not imported as a theorem.
3. The previously reviewed Huang–Kosters–Yang–Yeo closure definition and
   coefficient/Boolean presentation boundary are retained from
   [the study sources](../guarded_fall_degree_20261028/SOURCES.md). No new
   priority or literature-novelty claim is made for these corollaries.

Let `V(F,d)` be the actual smallest coefficient-linear subspace containing
inputs of degree at most `d`, closed under multiplication whenever the
product has degree at most `d`. Degree falls are iterated. The Lean module
uses the already published `FilteredClosure.Derivable` relation.

For a monomial order, a set `G` satisfies the standard Gröbner-basis
criterion if `G` lies in `(F)` and every nonzero `p` in `(F)` has leading
monomial divisible by that of a nonzero member of `G`. The new `SolvesAt`
predicate requires such a `G` inside `V(F,d)`. `IsClosureSolvingDegree`
expresses attainment and minimality. These are explicit algebraic objects;
no solver output, basis-degree guess, presumed unit ideal or circular
five-degree premise appears in the public prime theorem.

## Proposition 1: sharp prime-field solving degree

Keep all original hypotheses of `GuardedExactPair.exact_pair_prime`.
The published argument constructs `1` in `V(F,5)` and excludes it from
`V(F,d)` for every `d<5`.

On a unit ideal, `G={1}` is a Gröbner basis for every monomial order.
Conversely, any Gröbner basis contained in `V(F,d)` must contain a nonzero
constant: apply the leading-monomial criterion to `p=1`. Scaling that
constant gives `1` in the same space. Thus `SolvesAt(F,d)` is equivalent
to refutation at degree `d`. The sharp closed-Macaulay solving degree is 5.
This equivalence is checked directly in Lean.

For the classical solving degree of Caminata–Gorla, the reduced Gröbner
basis of the unit ideal is `{1}`, of maximum degree zero. Their Theorem 3.1
therefore gives `sd_sigma(F)=max(d_F,0)=5` for every degree-compatible
order. This use of an established theorem is the written classical
corollary; the new module does not formalize the general equivalence of
a particular matrix construction and the filtered closure, nor an F4/F5
implementation or arbitrary non-degree-compatible classical solving degree.

## Proposition 2: raw top regularity has no finite value

The actual prime tops are `x^3,y^3,z^3,T1,z^2*u^2,v*(x-y)`, with
`T1=(x-y)^2*u^2-2*(x+y)*x*y*u+(x*y)^2`. The actual binary first top is
`(x*y+x*u+y*u)^2`; the remaining five tops are the same.

At `u=1,x=y=z=v=0`, every top is zero in both presentations. Hence the
whole top ideal lies in the kernel of evaluation at this point. For every
`d>=0`, `u^d` is homogeneous of degree `d` and evaluates to 1; it cannot
belong to the top ideal. No homogeneous component is full. These proofs
hold independently of SAT/UNSAT, distinctness, rationality or curve
nonsingularity. The binary formulas are bound to the actual polynomial
inputs and their top forms, not to an odd-characteristic formula reused
as a purported binary model.

Consequently the BFS degree-of-regularity finiteness hypothesis fails for
the raw systems. One may use an extended-value convention `infinity`, but
must not report a finite regularity from a truncated Hilbert table or
identify this statement with Castelnuovo–Mumford regularity.

## Proposition 3: explicit field equations are a real control

Adjoin `t^q-t` for each of the five variables, with `q>1`. Lean verifies
that each equation has degree `q` and actual homogeneous top `t^q`.
Now `u^d` belongs to the augmented top ideal for every `d>=q`; the prior
evaluation point no longer annihilates that ideal. This is a checked
control against transferring the raw-system obstruction to another encoding.

Every monomial of degree greater than `5(q-1)` has an exponent at least
`q`. Such a monomial is divisible by one of the five field tops. Lean
checks this pigeonhole argument for every homogeneous polynomial, giving
`d_reg <= 5(q-1)+1`. This is an upper bound, not an exact regularity
measurement, and does not identify a solver's running time.

For a prime `p>3`, in fact `p>=5`. A derivation at degree below five cannot
use an explicit field equation of degree `p`. The degree-below-five raw
and augmented closures therefore coincide. The raw five-degree refutation
still works after adjoining inputs. Both the exact last fall and the sharp
closed-Macaulay solving degree remain 5. The public Lean wrappers retain
all original prime-field curve hypotheses and construct their unit ideal.
The same classical degree-compatible solving-degree corollary follows as
in Proposition 1.

## Validation and remaining work

New source, toolchain/configuration hashes, actual build/audit/kernel-replay
outcomes and retained development failures are in `validation/`. All earlier
proof-study source/certificate bytes remain unchanged; the new Lake target
configuration is recorded separately. The complete source-bound repository library check passed **5,430 tests, zero
failed, 123 ignored**, with actual exit zero and all 5,553 entries retained.
The native draft-card check also passed after restoring the exact tracked
backlog schema; its original exit-two missing-schema failure is preserved.
[The separate completed receipt](validation/final-validation.json) binds source,
binary, raw/archive hashes and terminal counts without rewriting the earlier
formal-check snapshot. No nested subprocess result substitutes for the outer suite.

The exhaustive prime/binary campaign is **not executed**. Its designed arms,
controls and exact certification requirements are in `STUDY_DESIGN.md`.
Direct AWS STS fails for missing credentials in this environment, and the
reviewed GitHub launch path builds an ecbench image rather than the new
instrument. No substituted integration fixture, local scientific sweep,
backend bypass, frozen evidence change, speedup or security claim is made.
The remaining campaign, binary exact first/last formalization, growing-base
formalization and Boolean/E12 encoding certification remain open.
