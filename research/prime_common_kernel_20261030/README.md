# A common kernel: gauges, exterior lifts and coded operators

2026-10-30 UTC. **An essentially new sub-exponential prime-field ECDLP
algorithm remains unproved.** This stage tests specific constructions for
the [common-kernel problem](../../problems/correlated-packet-common-kernel.md).
It follows the [packet-variant study](../prime_tensor_variants_20261030/README.md)
without modifying its frozen evidence. There is no speed measurement,
implemented general contraction algorithm or mathematical priority claim.

The selected objective is unchanged: a single deformation with compatible
two-axis local factorizations, giving sufficiently few modes at every
valuation level. Matched-level recovery already has the desired precision.
The missing construction must preserve the actual jets and include all
recursive entry, degree, padding and precision costs.

## What was checked

The Boolean kernel used by the primary manuscript is

```
K(W)[Y,X] = product_j ell_(Y_j)((W X)_j),
ell_0(z)=1-z, ell_1(z)=z.
```

A change of physical port coordinates is not automatically a change of
basis on this Boolean-index space. For
`W=[[1,1],[0,1]]`, `V=[[1,0],[1,1]]`, the `(11,11)` entry of `K(W)K(V)`
is4, whereas the same entry of `K(WV)` is6. This exact difference survives
modulo4. A modulo2 check would miss it. The actual definitions and both
values have a faithful Lean proof, separate from the general written
[gauge analysis](reviews/boolean-gauges.md).

True finite-index gauges do preserve one common deformation when their
inverse changes are carried into all attached tensors. Group-preserving
invertible gauges preserve the flattening's determinantal ideals over the
coefficient ring. That leaves possible representation improvements open,
but supplies no automatic rank, valuation or mode-density saving. One-step
entry evaluation fits the source allowance under explicit compactness
hypotheses; recursively repeated evaluation still needs a proof.

The exterior alternative is a genuinely compositional kernel:

```
Gamma(W)[A,B] = det W[A,B] when |A|=|B|, and0 otherwise.
Gamma(UV)=Gamma(U)Gamma(V), Gamma(I)=I.
```

It has a correct origin and integral local charts. Its one-particle block
is exactly `W`, however, so exterior lifting alone preserves passive-axis
crossings. Rank-one updates become squarefree, but the resulting subset
enumeration still has the square-root counting envelope. Generic fermionic
regrouping can also require many ordinary Boolean separation states. These
are scoped written arguments and controls, not a general impossibility
theorem. See the [independent review](reviews/exterior-independent.md).

A proposed global-even-parity slot code provides a useful exact diagnostic.
For `L=4`, both `(0,0)` and `(2,1)` are code positions, yet the actual Cauchy
product has a nonzero one-axis chart coefficient `-200/9` between them.
The code detects individual bit flips; integer translation and carries are
different errors. This closes that particular disappearance argument.
The coefficient has valuation3; it is not claimed to have valuation1.

The independently reviewed generalization has normalized passive rank
`2(L-1)` over `F2` for this particular code and Cauchy kernel. Finite exact
checks at `L=4,8,16,32` give ranks `6,14,30,62`. The general theorem has a
written proof; the separate Lean project checks the actual finite rational
witness, not this full rank family.

## A coded operator gives a genuine positive identity

Assign distinct nonzero binary syndromes to the bit flips on either address
register. The code projector `P` then gives

```
W(T) = (P (A(T) tensor A(T)) P)|C
     = f_*(T)^2 I_C + sum_j f_j(T)^2 U_j,
A(T)=f_*(T) I + sum_j f_j(T) X_j.
```

Here `U_j` flips the matching bit in both registers. Mismatched pairs vanish
exactly. This is one common family with the correct origin. Lean checks the
underlying general projector and weighted-pair laws under explicit syndrome
hypotheses; the binary instantiation and Kronecker-product bridge remain
written. It is ordinary coefficient-matrix algebra; no quantum computer is
assumed.

The explicit integral-chart Lagrange selectors still fail the requested
weighted linear-mode template. At an odd node chart, the crossing block is
`2^(2a+2) (c I+4D)` with `c` odd. It has `K/2` nonzero Smith factors, even
at precision `2a+3`, where the proposed constant-family two-index template
would supply only `O(a^2)` retained rank-one terms. This is a specific
selector failure with a written DVR argument, not an invalid transfer of
rational rank to a truncated ring. The code also pays a density factor
of order `a`. See the [construction](reviews/coded-kernel.md) and
[independent review](reviews/coded-kernel-independent.md).

A stronger [packing argument](reviews/exact-filter-packing.md), independently
[reviewed](reviews/exact-filter-packing-independent.md), exposes the scope
of that density cost. Over a field, suppose linear error maps `E_i:C->V`
and recovery maps `R_j:V->C` have `R_j E_i=0` for unequal labels, while
every `R_i E_i` is injective. Applying each recovery map to a sum of error
images shows that sum is injective. Hence

```
number_of_labels * dim(C) <= dim(V).
```

Here dimensions count position-basis modes: `dim(V)=L^2`, `dim(C)=K`.
With `u+1` labels this exact template loses at least a factor `u+1` in
slot density. It invalidates the constant-density premise for that template;
it is not a runtime lower bound or a statement about all degenerate codes.
Singular matched maps and other common deformations remain outside it.
The source's actual local Cauchy maps are partial translations, so dropping
matched injectivity is a concrete remaining possibility, with all jets,
precision and mode costs still to be proved.
The [separate packing project](lean-packing/PROOF_MAP.md) formally proves the
full abstract injection and dimension bound under its stated field hypotheses;
the application and asymptotic substitution remain written corollaries.

| Construction | Positive fact | Remaining issue or scoped failure |
| --- | --- | --- |
| Physical-coordinate substitution in `K(W)` | Correct origin | General composition is false:4versus6 |
| True Boolean-index local gauges | Exact common-family identity | No demonstrated improvement of weighted modes; recursive costs need proof |
| Exterior lift `Gamma(W)` | Composition, degree and chart integrality | One-particle passive channels remain; child/sign identity missing |
| Syndrome-coded operator family | Exact mismatch elimination and common kernel | Explicit selector has full-rank low-valuation crossings and density loss |

## Native verification

The [prospective development protocol](native/PROTOCOL.md) fixes rational
inputs and expected identities. The strict C++20/GMP unit checks:

- 4080 exterior-composition entries and4080 monomial Boolean controls;
- the `4 versus6`, `-200/9` and intermediate-state `1 versus0` witnesses;
- 1364 adjacent-swap sign entries, with rational ranks2 through32 certified
  by `H H^T=2^n I`;
- 560 minors for affine dependence on a rank-one update.

A separate [parity-kernel unit](native/parity/PROTOCOL.md) constructs the
actual puncture polynomials and checks their origins, coefficient witness
and normalized passive matrices, rather than inserting their expected ranks.
The [coded unit](coded-native/README.md) checks30796 projected operator pairs
and21 selector valuations. Its first GMP crash was a noncanonical-rational
implementation defect; failed sources/logs are retained, and corrected strict
and sanitizer runs agree. A separately implemented
[a3 unit](coded-native-independent/README.md) computes both8-by-8 crossing
blocks, exact determinant valuations and two discriminating negative controls.

The [independent replay and semantic review](reviews/native-independent.md)
passes. These are development correctness checks. No wall-clock comparison,
scaling fit or ECDLP execution is inferred. CPU contention cannot cause an
exact rational disagreement; no earlier timing result is dismissed here.
The [current affected Rust checks](receipts/ROOT-VALIDATION.md) pass429 tests
with one existing default ignored case. Their initial sparse-fixture failure
and the earlier full-library result are preserved separately.

From this study directory, replay all units and original package bindings:

```sh
bash check.sh /tmp/common-kernel-all
```

Individual development units are also available:

```sh
bash native/check.sh /tmp/common-kernel-check
bash native/parity/check.sh /tmp/common-kernel-parity-check
bash coded-native/native/check.sh /tmp/common-kernel-coded-check
bash coded-native-independent/replay.sh
```

The helper requires the frozen
expected output and performs an exact diff. Proof checks and their narrower
formal scope are recorded in the [proof map](proof-map.md).

## Decision and remaining work

Keep the common-kernel route open. Neither unrestricted physical-coordinate
conjugation nor exterior lifting by itself supplies its missing identity.
The next sufficient object is an explicit fixed-order Boolean-index
factorization whose actual level-wise jets agree with one common family,
with all sign/intermediate states and entry costs charged. A different
basis or coded kernel is still possible; none is established here.

The [requirement mapping](requirements.md) retains the original request,
including the incomplete six-repository historical audit and missing new
ECDLP algorithm. Passing finite controls or merging this work does not
complete those requirements.
