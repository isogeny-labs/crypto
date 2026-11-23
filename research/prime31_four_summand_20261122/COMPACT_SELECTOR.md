# Compact membership for the actual hybrid Prime31 base

Date: 2026-11-22 Pacific. Parent tracking PR: #4938.

Status: source inspection, written algebraic derivation, and a passing
development-08 native setup certificate. The separately frozen development-07
check failed on an inherited source constant, as recorded in Section 2; its
failure is preserved. The circuit below has
not been qualified as a root extractor or compared for cost. It preserves the
original four-leaf, distinct-abscissa problem under the hypotheses stated here.
No novelty or speedup claim is made. [pcc:unverified]

## 1. What development-04 actually left open

The frozen output
`evidence/development-04/test.stdout` records two different obstructions:

- Both B8 controls built four remainder equations with 319 total terms and
  maximum degree ten. Their bounded F4 traversal found no usable univariate
  and refused enumeration. Neither control completed extraction.
- The actual B677 hybrid control stopped at the 20,000-term symbolic
  construction cap, before any F4 traversal or root extraction. Its counters
  record 4,378,124 symbolic additions and 4,148,594 symbolic multiplications;
  these are development counters, not a complete operation-price model.

The first obstruction concerns elimination. The second concerns expansion
of the input. Removing expansion does not establish that elimination will
succeed or become cheaper.

The actual base is the original 509 curve-liftable Dickson abscissae together
with the frozen 168 additional abscissae in
[`RUN66_Y7_EXTRA168.csv`](../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv).
The B677 control uses that construction, rather than the first 677 rational
abscissae on the curve.

## 2. Exact source parameters

The following committed sources were inspected:

- [`Base::build`](../prime31_ic_20261115/native/lib.rs), Git blob
  `42e1e7c1912be43190959c5be8f84cf59c867786` at the parent source snapshot:
  iterated `x -> x^2-2`, rejection of terminal `+/-2`, complete reverse
  square-root fibre, then curve-lift filtering.
- [`TWO_SUM_YIELD_GATE.md`](../prime31_ic_20261115/native/TWO_SUM_YIELD_GATE.md)
  and its [native example](../prime31_ic_20261115/native/examples/prime31_two_sum_gate.rs):
  the depth-ten fixture selects seed 3, terminal **1616683971**, 1024 distinct
  field abscissae, and 509 nonzero-y curve lifts. The example asserts these
  parameters. This note did not rerun that historical census.
- [`prime31_coset_xlift_f4.rs`](../../examples/prime31_coset_xlift_f4.rs),
  Git blob `9d52fd068c66e2c9b1d8f2ed7152ae6099cc1c54`: the RUN52 public
  `(c,x,y0)` anchors supply all eight `(c,y0)` pairs below.
- [`prime31_coset_cubic_factor.rs`](../../examples/prime31_coset_cubic_factor.rs),
  Git blob `8fd362be60b8cb134f07c28ba199180b6dbb11fa`: supplies the
  seven-cubic factorization, but its eighth `c` is mistyped as `1852955275`.
  The first seven pairs agree with RUN52; the eighth is corrected below to
  `1852957275`. The historical source file is preserved unchanged.
- [`RUN52_COSET_XLIFT_REPORT.md`](../prime31_f4_admission_20261121/RUN52_COSET_XLIFT_REPORT.md)
  and [`RUN53_CUBIC_FACTOR_REPORT.md`](../prime31_f4_admission_20261121/RUN53_CUBIC_FACTOR_REPORT.md):
  archived complete splitting, 21 abscissae per fibre, and the signed-lift
  identity. Their historical execution qualifications remain unchanged.
- `RUN66_Y7_EXTRA168.csv`, Git blob
  `b135333216ef7e72f4baa5da867167062a5daef9`: the actual extra-point input used
  by the present hybrid control.

The eight public pairs `(c_j,y0_j)`, where `y0_j^7=c_j`, are:

| j | c_j | y0_j |
|---:|---:|---:|
| 1 | 1970273222 | 391601180 |
| 2 | 1011118577 | 1045445284 |
| 3 | 784868900 | 1088714879 |
| 4 | 1902095001 | 839007678 |
| 5 | 131394216 | 1893426316 |
| 6 | 49444078 | 1215727561 |
| 7 | 53346144 | 1554347565 |
| 8 | 1852957275 | 136769428 |

All arithmetic below is modulo `p=2147483647`, on the original curve
`f(X)=X^3+847514846 X+2087444724`.

Before admitting a new compact implementation, preserve a native setup
certificate binding these constants to the **actual CSV**: check its points,
distinct abscissae, their membership in the eight polynomials, 21 roots per
fibre, absence of overlap, and equality of the canonical 509+168 union with
the existing B677 vector. The degree-168 monic product and 168 distinct roots
then establish its exact root set without scanning the field.

The first separately frozen native check, development-07, copied the erroneous
eighth constant from the cubic-factor example and failed its seventh-power
assertion: `136769428^7 mod p = 1852957275`, whereas the copied constant was
`1852955275`. The preserved [failure receipt](evidence/development-07/test.stderr)
is not a passing setup certificate. RUN52 supplies the corrected constant, and
the actual RUN66 CSV contains its anchor `(227959902,136769428)`. This note and
the new setup test now use that correction; the original sources and frozen
failure remain unchanged. No test was executed during the correction review.

The separately frozen development-08 repeat subsequently passed its one setup
test, with every pre-call source hash matching. Its
[raw output](evidence/development-08/test.stdout) records1024 distinct Dickson
roots,509 nonzero rational lifts,515 Euler-certified nonsquare RHS values,
zero zero-RHS values,eight disjoint21-root cosets,zero overlap with the full
Dickson fibre,1192 distinct selector roots,and exact equality of the canonical
677-point union to the original hybrid base. The receipt explicitly records
`circuit_evaluated=false`, `relation_extraction_performed=false`, and
`performance_measured=false`. This is a setup certificate only.

## 3. A compact selector before curve-lift filtering

Define the Dickson ladder

\[
 d_0(X)=X,\qquad d_{i+1}(X)=d_i(X)^2-2,
 \qquad H_D(X)=d_{10}(X)-1616683971.
\]

It is a monic degree-1024 polynomial. The selected fibre is fully split and
squarefree. Of its roots, exactly 509 lift to nonzero-y points on the original
curve; the other 515 have nonsquare `f(x)`.

Define

\[
 H_Y(X)=\prod_{j=1}^{8}\big(f(X)^7-c_j^2\big),
 \qquad H(X)=H_D(X)H_Y(X).
\]

Each factor in `H_Y` is monic of degree 21. Complete splitting in the frozen
instance is an archived source claim whose corrected native setup check is
still pending. Every root in `F_p` lifts to a point on the curve, because
`y=f(x)^4/c_j` satisfies `y^2=f(x)` and `y^7=c_j`. The stated source
distinctness conditions give a squarefree fully split `H` of degree 1192.
Its curve-liftable roots are precisely the original 677 abscissae.

Using `H` instead of the filtered degree-677 `h` preserves a short
composition. It does not pretend that all 1192 roots are factor-base points.
The distinction is handled by the RR5 geometry, as follows.

## 4. The straightforward field-rational equivalence

Keep the RR5 notation from [MATHEMATICS.md](MATHEMATICS.md): public
`Q=(r,s)`, three unknowns `b,c,d`, `V=X+b`, `L=cX+d`, and

\[
 U=(X^2+rX+r^2+A)V^2-(X-r)L^2-2sLV.
\]

For `b,c,d` in `F_p`, any solution of `U | H` gives four distinct roots in
`F_p`. The denominator argument applies because `H` is squarefree and
`H(r)!=0`. It reconstructs field-rational ordinates

\[
 y=-s-(x-r)L(x)/V(x).
\]

Therefore each root is a curve-liftable root of `H`, hence belongs to the
original B677 factor base. Thus `U | h`. The converse is immediate from
`h | H`. This proves exact equivalence **over F_p** once the root-set setup
certificate and target conditions hold.

For arbitrary arity or an unrelated selector, this argument alone does not
exclude extra geometric roots over `F_pbar`. Replacing `h` by `H` would then
invalidate the previous blanket all-geometric-roots-rational statement.
The exact four-leaf setting has a stronger argument.

## 5. Four distinct leaves exclude the extra geometric roots too

Additional hypotheses: the field has odd characteristic, `#E(F_p)` is odd,
the target `Q` is a nonzero rational point outside the **signed** factor base
(`Q != +/-P_i`, equivalently `h(r)!=0`), and all roots of `H` are distinct
elements of `F_p`. These hold for the intended ordinary prime-order fixture
once its setup and curve checks pass. The rational roots of `H` with square
curve RHS must be exactly the abscissae of that signed base.
Because `Q` is rational, `h(r)!=0` also implies `H(r)!=0`: a rational target
whose abscissa is a root of `H` would itself be a signed base point.

**Claim.** Under these conditions, every geometric RR5 solution of `U | H`
uses four rational leaves, and therefore already satisfies `U | h`.

Proof. The RR5 function has four finite signed leaves with distinct
abscissae in `F_p`, whose sum is `Q`. Each leaf belongs to `E(F_{p^2})`:
its ordinate either lies in `F_p`, or is a square root of a nonsquare in
`F_p` and is negated by Frobenius. Let `T` be the sum of the rational leaves
and `W` the sum of the nonrational leaves. Then

\[
 Q=T+W,\qquad Q=\operatorname{Frob}(Q)=T-W.
\]

Consequently `2W=O`. Also `Frob(W)=-W=W`, so `W` is a rational two-torsion
point. Odd group order forces `W=O`.

Let `k` be the number of nonrational leaves:

- `k=1` is impossible because a finite individual leaf is not the identity.
- `k=2` would make the leaves opposites, which gives the same abscissa twice.
- `k=3` leaves one rational base point whose sum `T` is `Q`, contradicting
  target exclusion.
- `k=4` would give `Q=W=O`, contradicting the nonzero target.

Hence `k=0`. All four leaves are rational and belong to the actual B677
base. Uniqueness of the normalized principal function then puts `b,c,d`
in `F_p`, as in the previous RR5 proof.

This proof does **not** generalize automatically to five or more summands:
three nonrational leaves can sum to zero while two remaining rational leaves
sum to a target outside the base. Nor does it cover repeated abscissae,
identity targets, target-in-base cases, or curves with rational two-torsion.

The finite reducedness argument also survives for the present selector.
All roots of `H` have nonzero curve RHS, because a zero would give a rational
two-torsion point on an odd-order curve. At each chart root, the same
infinitesimal argument forces the three coefficient variations to vanish.
Thus the filtered and compact-selector remainder ideals have the same finite
geometric roots and are both radical. Their ideals agree after passing to an
algebraic closure, and therefore over the original field. This is a written
argument, not a formal or executable certificate.

## 6. Evaluate the selector in the rank-four quotient

Work in

\[
 \mathcal R=F_p[b,c,d][X]/(U(X)).
\]

Every ring value has four coefficients in the basis `1,X,X^2,X^3`.
Monicity of `U` makes all reductions division-free. The test is simply
`H mod U=0`, calculated by the compact circuit:

1. Ten ring squarings and constant subtractions produce `d_10(X)`.
2. Four ring multiplications produce `f(X)^7`; for example
   `f^2`, `f^3`, `f^6`, `f^7`.
3. Form the eight shifted factors `f^7-c_j^2` and multiply them in a balanced
   tree: seven ring multiplications.
4. Multiply that result by `d_10-1616683971`: one ring multiplication.

This is **22 rank-four quotient multiplications**, with constants and affine
shifts also charged. A ring multiplication is not a single field operation.
Keeping its coefficient polynomials expanded would still recreate the
symbolic explosion; the useful input representation is an exact graph lift.

## 7. Exact quadratic graph lift, without coefficient expansion

Introduce four variables `u_0,...,u_3`, with four quadratic equations binding
them to the RR5 quartic coefficients. For each ring multiplication gate, let
its two degree-at-most-three input representatives be `A_i(X),B_i(X)`.
Introduce a degree-at-most-two quotient `Q_i` and a degree-at-most-three
remainder `R_i`, and impose the seven coefficient equations

\[
 A_i(X)B_i(X)=U(X)Q_i(X)+R_i(X).
\]

All equations have total degree at most two in the core and gate variables.
Each gate introduces seven variables and seven equations. For fixed inputs
and a monic `U`, polynomial division gives **one and only one** quotient and
remainder, over any extension field. Thus the auxiliaries describe a unique
arithmetic graph; they do not add branches or enlarge the projected solution
set.

The last gate can impose zero remainder directly and introduce only its
three quotient coefficients. With the conservative unoptimized 22-gate
layout this gives

\[
 3+4+21\cdot7+3=157\text{ variables},\qquad
 4+22\cdot7=158\text{ equations},
\]

all of degree at most two. Constants and the earliest low-degree gates permit
further eliminations, but these counts do not assume them. Constructing this
sparse input requires no field scan, leaf-tuple scan, pair-sum table, or
degree-677 symbolic remainder expansion.

The graph lift's projected solutions are exactly `H mod U=0`; Section 5
then identifies them with the original distinct-abscissa B677 relations.
Solving 158 quadrics in 157 variables can still be expensive. Eliminating
the auxiliaries blindly can reconstruct the original high-degree problem.
The compact description alone gives no F4 solving-degree, memory, root-cost,
or ECDLP exponent bound.

## 8. The smaller immediate cofactor-lift control

For an arbitrary explicit base, write

\[
 h(X)=U(X)W(X),\qquad \deg W=B_{\rm FB}-4,
\]

with `W` monic. Its coefficient equations use the three RR5 unknowns plus
`B_FB-4` cofactor coefficients: **B_FB-1 variables and B_FB equations**, of
maximum degree three. For every admissible `U`, `W` is unique. Thus this is
another exact graph lift of the existing remainder problem, and needs no
tuple enumeration. Its construction has sparse coefficient support even
when expanded `h mod U` does not.

At B8 this gives seven variables and eight cubic-or-lower equations, a
smaller next control than a general quadratic Horner-state lift. At B677 it
gives 676 variables and 677 equations; the input can be constructed without
the current remainder explosion, but extraction is a separate obligation.
Development06 constructed and checked the B677 cofactor input and retained an
inconclusive B8 cofactor extraction. The 22-gate
composed-selector route is a further representation to test after its
source-bound setup equivalence certificate, not a substitute for the
cofactor control's actual result.

## 9. Bounded next steps and acceptance conditions

1. Compare the B8 expanded and cofactor forms on the already frozen positive
   and empty controls. Keep the original F4/root stack and deny enumeration
   fallback. Compare complete signed row sets only when extraction completes;
   retain every cap or missing eliminant as inconclusive.
2. On the original B677 input, qualify input construction and known-answer
   graph evaluation separately from root extraction. Bind the terminal,
   eight cosets, 168-point CSV, 509-point source base, all source hashes, and
   the exact target. A supplied known-answer coefficient tuple may check a
   graph in a verifier; it must not enter the candidate extractor.
3. Implement the rank-four multiplication graph as a reusable native Rust
   polynomial constructor, retaining the unchanged original F4 and scalar
   root-factoring code. Compare gate evaluation to ordinary monic division
   on small public controls before attempting the full composed system.
4. Freeze a separate bounded solve protocol before running the large lifted
   system. Choose variable order explicitly, retain an external matrix/trace
   memory ceiling, and distinguish a missing univariate from an empty fibre.

All setup, input-construction work, algebraic branches, failed queries, root
factorings, signed replays, independent-rank work and target recovery remain
in the complete cost account. The larger polynomial has additional roots as
a univariate selector, but Section 5 is the specific reason they do not add
RR5 chart solutions here. That reason must travel with the implementation;
it must not become a general permission to use unfiltered selectors at other
arities.

## 10. Proof map additions

| Obligation | Current evidence | Remaining qualification |
|---|---|---|
| Constants and actual hybrid support | Development08 setup certificate passes on corrected RUN52 anchors and actual CSV; development07 failure retained | Independently executed certification; carry the exact source bindings into the circuit |
| F_p membership equivalence | Section 4 written norm/lift proof | Independent source review and bounded controls |
| No extra geometric four-leaf solutions | Section 5 written Frobenius/torsion proof | Faithful formalization; preserve every hypothesis |
| Quadratic multiplication-graph equivalence | Uniqueness of monic quotient and remainder | Native constructor/evaluator checks and formal polynomial-division lemma |
| Root extraction and complete cost | Not established by this note | Actual nonenumerative roots, signed independent rows, full solver and matched comparisons |

No Lean executable was available in the already recorded environment check.
No formal proof, compact-circuit root extraction, or cost advantage is asserted
here. The passing setup receipt has the narrower scope stated in Section2.
