# Bounded interpolation states and an exact symbolic union

15 November 2026. Consumer: crypto PR4338 candidate A. Mathematical source
b94180fe11689d0af8b086d2dbe28b460bf58078, Lean4.35.0, actual Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Local source builds and the
generic/actual prime-field assembly probes pass. Final canonical gate003
audits 167 theorems, 0 failures, 0 trusted-native exceptions. Signed PCC standing
is a separate consumer obligation; no complete selector is supplied.

The checked finite endpoint separates every clean selected support of size
at most the u-degree cap by its actual bounded polynomial-evaluation kernel.
There are exactly b^t such kernels for t prefix colors with b choices each.
The same module gives an exact product representation of their union and
keeps nonzero witness extraction explicit.

## 1. The actual state and its separating witness

Let F be a field, x,y : I -> F, and assume x is injective. For a finite
selected support D, use the actual subset of Polynomial F x Polynomial F

    K_D = {(u,v): natDegree u <= dU, natDegree v <= dV,
           u(x_j) + y_j*v(x_j) = 0 for every j in D}.

No replacement curve or numerical state-count proxy is used. The nonzero
pair (h_D,0), with h_D = Lagrange.nodal D x, is a separating witness:

    natDegree h_D = |D|,
    h_D != 0,
    h_D(x_j) = 0 iff j belongs to D,
    (h_D,0) belongs to K_E iff E is a subset of D, when |D| <= dU.

The first three facts use Mathlib's existing nodal degree/nonzero/evaluation
laws, with injective x for the reverse zero implication. Equal K_D and K_E
therefore imply both support inclusions, and hence D=E, when both support
sizes are within dU. No condition on the ordinates is hidden in this argument.

For choices c : Fin t -> Fin b, let D_c = {(i,c_i): i in Fin t}. Each D_c has
size t; the support map is injective. The main exact endpoint is

    Nat.card (Set.range (c -> K_(D_c))) = b^t, for t <= dU.

Every prefix has a nonzero pair. Empty prefixes and empty choice domains
follow the actual Fin/empty-product conventions in the generic statement.

## 2. What the state count says about one enumeration architecture

Take the consumer caps dU=floor(m/2), dV=floor((m-3)/2), m>=3, and put
t=floor(m/2). An architecture that constructs the *entire prefix level*,
stores one explicit entry for each exact kernel, identifies entries only
when their kernel sets are equal, and prunes only when the currently
selected constraints have no nonzero pair, needs b^t distinct entries.

For b=ceil(r^(1/m)),

    b^t >= r^(t/m) >= r^(1/2 - 1/(2m)).

At m=ceil(sqrt(n)), n=ceil(log2 r), the logarithm of this explicit entry
count is at least n/2-O(sqrt(n)). For standard input families with input
length polynomial in n, materializing that level cannot meet the proposed
2^o(n)*poly(input_length) attempt cap. This conversion is hand accounting
of the stated storage model, not a formal runtime theorem.

The result counts a whole state family. It does not establish a lower
bound on states visited by a lazy or early-stopping program, on selective
searches and their useful yields, or on pruning that uses future colors.
It does not exclude coarser states with a separately justified lifting
interface, symbolic unions or dedicated algorithms. Canonical bases of
the same actual linear subspaces cannot merge these distinct kernel sets.

## 3. A positive symbolic representation of the same union

Define the field-valued products of evaluation linear forms

    P_i(u,v) = product_(j in Fin b) (u(x_ij) + y_ij*v(x_ij)).

The shared module proves, with the original degree caps retained, [pcc:unverified]

    [all P_i(u,v)=0] iff [there exists c with (u,v) in K_(D_c)].

This is exact existential choice within each color. It uses the existing
product-zero law and finite choice; it does not replace a disjunction by
simultaneous vanishing at every record. Injectivity and a curve equation
are unnecessary for this logical equivalence. The nonzero-existence
endpoint carries (u,v)!=0 on both sides. Returning the homogeneous zero
pair cannot satisfy that interface. [pcc:unverified]

At the consumer caps the coefficient vector has m entries. The symbolic
input consists of t products with b linear factors each. Straight Horner
evaluation gives a hand membership-check budget O(t*b*(dU+dV+1)) field
operations for a supplied pair. This is representation/evaluation
accounting, not a measured implementation or a witness-search bound.

After a nonzero pair is supplied, scanning each color's factors selects
zero factors without expanding the prefix state family. Actual group-sum
and final scalar checks remain part of the consumer contract. The curve
divisor interpretation, mixed-label recovery law, and effective capped
nonzero-extraction algorithm have their own scopes. This module formalizes
the polynomial state/membership components, not the full elliptic reduction.

The next algorithmic question is thus explicit: find a nonzero point of
this factored homogeneous system on the clean random elliptic input law,
with a known per-attempt cost and useful-yield bound. A compact membership
predicate alone does not answer that question.

## 4. Exact proof map

| Statement | Shared declaration | Scope |
|---|---|---|
| Actual bounded state | kernel / mem_kernel | arbitrary Field F and finite support |
| Nodal zeros recover clean records | nodal_eval_zero_iff | injective x |
| Witness membership iff support inclusion | nodal_pair_mem_iff | support size <= dU |
| Nonzero feasible prefix witness | nodal_witness | same |
| Equal kernels recover supports | supports_eq_of_kernels_eq | both sizes <= dU |
| Actual colored support/cardinality | prefixSupport_* | Fin t x Fin b |
| Kernel map injective | prefix_kernel_injective | injective x, t <= dU |
| Exactly b^t actual kernel sets | prefix_kernel_count | same |
| One zero factor per color | colorProduct_zero_iff | arbitrary field |
| A colored choice of zero factors | colorProducts_zero_iff_choice | actual function choices |
| Symbolic set equals the kernel union | symbolic_mem_iff_exists_kernel | both degree caps retained |
| Nonzero extraction carried exactly | symbolic_nonzero_iff_exists_kernel | no zero-witness substitution |

The final assembly applies these endpoints unchanged to nonsingular
Mathlib Weierstrass coordinates over ZMod p, with p prime, p>3 and m>=3.
The clean-abscissa condition is explicit where needed. The structural
statements need no point-sampling distribution or subgroup-order hypothesis;
those belong to the consumer's separate probability/cost argument.

Prospective plans are interpolation-kernels-20261115-plan.md at1d7dc7c
and interpolation-symbolic-union-20261115-plan.md atf7a287b. Floor probes
0e381fb and41218f4 precede the respective new source families. No theorem
statement was weakened and no proof limit was increased.

## 5. Receipts and gate dependency

Complete source/build/probe/audit material is under
docs/receipts/interpolation-kernels-20261115/. Build001 atf51eb1f and
build002 atb94180f pass. Both floor probes and the final generic/actual-point
assembly pass; explicit axiom prints cover all 18 named laws and list only
propext, Classical.choice and Quot.sound. No fresh lean4checker executable
is present on the restored PATH, so no --fresh result is asserted.

The initial canonical gate001 failed before axiom auditing because the
gate recognized Lake's old unregistered-target diagnostic but not its
"unknown module source path" diagnostic. The one-line owner-tool recovery
atf319f7a still directly compiles every tracked auxiliary source and audits
its environment. The standard axiom policy and source enumeration are
unchanged. All 11 existing self-test controls pass, including the expected
rejection of omitted/bad, user-axiom, sorry and native fixtures. A fixture's
intentional trusted-native exception is not a library audit exception.
The failed gate001 is retained. Gate002 compiles all sources, but its audit
argument parser rejects the diagnostic module paths containing hyphens.
A direct String.toName diagnostic reproduces that failure. The three
probes were renamed byte-for-byte to valid module paths under
docs/receipts/InterpolationKernels20261115/. They remain in the complete
tracked source inventory. Gate 003 passes 167 theorems / 0 failures / 0 trusted
native exceptions, with the same source-enumeration and axiom policies.
All earlier failures are retained and the gate dependency hash is pinned.

Sources checked: exact Mathlib nodal/finite-cardinality/product-zero laws,
the consumer's frozen plan and prior selector assessments; Jeannerod,
Neiger, Schost and Villard, sections1.1--1.2/Definition1.1 of
[Computing minimal interpolation bases](https://cs.uwaterloo.ca/~eschost/publications/jsc_final.pdf).
Their relation module imposes the supplied linear interpolation constraints.
The colored condition here keeps existential root choices. KB remains
unavailable in the inherited consumer runtime; no novelty claim is made.

No native numerical case, timing, fitted exponent, paid job or frozen
qualification run was started. The existing PR4338 proposal diagram and
measured canonical charts are unchanged: the new diagram explains this
component and adds no measured or completed-selector edge.
