# Adaptive quadratic chains: capture, support, and contamination

Research cycle: 20261125. This note independently reviews the quadratic-selector
count and extends it to several fibers of one shared chain. The mathematical
arguments below are hand proofs. The numerical bounds are deterministic integer
calculations, independently reproduced by the study's Rust checker. No randomized
experiment, new elliptic-curve benchmark, or paid job is part of this review.

The main new deliverable is a finite-partition capture bound that accounts for
adaptively chosen fiber constants without paying for every possible field-valued
constant. A second bound controls total admitted support. A tail argument then
turns that support bound into a statement about actual unlabeled contamination.
These are scoped bounds for a particular selector architecture and sampling law;
they are not lower bounds for ECDLP or for all relation-finding algorithms.

## 1. Objects, sampling law, and admissible selectors

Let `p` be an odd prime, let `k = F_p`, and regard rational maps as morphisms
`P^1 -> P^1` defined by reduced, coprime homogeneous numerator/denominator pairs.
Each nonconstant step has degree one or two. The budget `s` counts quadratic
steps. Define

\[
 A_s=\sum_{j=0}^{s}p^{2j},\qquad D=2^s.
\]

Postcomposition by a Möbius transformation changes output coordinates but does
not change the partition of the input into fibers. A selector chooses one chain
and a union of at most `t` of that chain's fibers. The chain and the output values
may be chosen after inspecting the entire sampled pool.

For the curve application, let `H` be an odd-order subgroup of `E(F_p)` of order
`r`. Exclude the identity and identify `P` with `-P`. The resulting universe

\[
 \Omega=(H\setminus\{O\})/\{\pm1\},\qquad M=|\Omega|=(r-1)/2
\]

is represented injectively by its affine x-coordinates. This uses the ordinary
elliptic-curve x-coordinate quotient: two affine points have the same x-coordinate
exactly when they are negatives, and an odd-order subgroup has no nonidentity
2-torsion. The counting theorems themselves need only a finite universe, not a
curve or group.

The pool `L` is a uniform `N`-element subset of `Omega`, with `N <= M`. One way to
obtain this law is independent uniform subgroup-point sampling, rejecting the
identity and previously seen negation orbits. Merely calling a pool pseudorandom
does not prove that law. A fixed seeded replay is an instance; the probability
statement is about the stated random-pool model. [pcc:unverified]

Every inspected point must be included in the pool budget. Selecting 32,768
survivors from a much larger inspected pool does not justify using `N=32768`.
Correlated walks, specially constructed labeled point families, or adaptive
nonuniform point generation require a separate distribution argument.

For an admitted union `U`, write

\[
 B=|U|,\qquad H_L=|U\cap L|,\qquad C=|U\setminus L|=B-H_L.
\]

Here support and contamination count subgroup negation classes. A selector that
also admits points outside `H` can have additional operational costs that this
count does not include. Public labels and sign-reconciled relations must still be
validated separately.

## 2. Quadratic maps modulo output Möbius transformations

### 2.1 There are exactly p² projective involutions

In odd characteristic, every nonidentity involution in `PGL_2(k)` has a trace-zero
matrix representative

\[
 A=\begin{pmatrix}u&v\\w&-u\end{pmatrix},\qquad
 \Delta=u^2+vw\ne0.
\]

Indeed, a noncentral matrix whose square is scalar has zero trace by its
degree-two characteristic equation. Conversely, the displayed matrix satisfies
`A^2 = Delta I`; it is nonsingular and is not scalar when `Delta != 0` and the
characteristic is odd.

The projective parameter space `[u:v:w]` has `p^2+p+1` points. Its singular conic
`u^2+vw=0` has `p+1` points: in the chart `w=1` there are `p` choices of `u` and
`v=-u^2`, and the remaining point has `w=u=0`. Removing the conic leaves exactly
`p^2` involutions. This direct count avoids assuming a classification of
conjugacy classes.

### 2.2 A degree-two map determines its output-equivalence class

If `f` has degree two, the function-field extension `k(x)/k(f)` is separable
because the characteristic is odd. It is quadratic, hence has a unique nontrivial
deck automorphism. Every `k`-automorphism of `k(x)` is a Möbius transformation, so
the deck automorphism is one of the `p^2` involutions above.

If two degree-two maps have the same involution, their generated subfields both
equal that involution's fixed field. Two generators of the same rational
function field differ by an output Möbius transformation. Consequently there are
at most `p^2` degree-two maps modulo output Möbius transformations.

Choose a representative `q_sigma` for each class that occurs. Starting at the
input of a chain, write its first quadratic step as `alpha o q_sigma`. Absorb
`alpha` into the next step before choosing the next representative. Continue
inductively. The final output Möbius transformation does not affect any fiber
partition. Degree-one steps are absorbed in the same manner.

Thus chains with exactly `j` quadratic steps induce at most `p^(2j)` partitions;
chains with at most `s` steps induce at most `A_s` partitions. Coincidences between
different words only reduce this number. A nonconstant chain has degree at most
`D`, so every fiber has at most `D` distinct geometric points and at most `D`
members of `Omega`.

### 2.3 One fiber, multiple fibers, and exceptional formulas

There are at most `(p+1) A_s` single-fiber subsets. For exactly `t` output values
there are at most `binom(p+1,t) A_s` subsets. For an "at most t" capture event,
one may pad output values to `t`; this enlarges the union and is adequate for a
capture-only upper bound. Do not use that padding to preserve a support cap.
The anchor proof below handles support caps without this problem.

The following scope conditions are necessary:

- Infinity is an ordinary projective output value. Poles belong to its fiber;
  they must not be silently discarded or treated as arbitrary exceptions.
- A formula with a common numerator/denominator factor must first be reduced.
  Cleared-denominator equations with unremoved basepoints are a different
  selector model.
- Constant maps have the single nonempty fiber `Omega`. They cannot satisfy a
  nontrivial support cap `b < M`; when `b=M` their capture probability can be one.
  They do not obey the degree-based fiber bound for nonconstant chains.
- A different map chosen separately on each branch is a tree, not one shared
  chain. Its additional maps must be counted.
- Arbitrary sample-dependent filtering inside a fiber is not a fiber union.
  Its description and work must be accounted for separately.

The earlier one-fiber number `2^-628` used another supplied order,
`r=2147487637`, for which `(r-1)/2 > 2^30`. It follows safely from
`binom(N,h) <= (4N/h)^h`, a coarse geometric-sum bound, and `D/M < 2^-20` for the
then-specified parameters. It is not an arithmetic error. It is also not the
canonical fixture used below: the current native fixture has `r=2147446831` and
`M < 2^30`. The current bounds use exact integers instead of transferring that
rounding assumption.

### 2.4 Scope of the curve-automorphism lifting obstruction

For a generic elliptic curve, meaning `j(E)` is neither `0` nor `1728`, a curve
automorphism over the algebraic closure is a translation followed by `+1` or
`-1`. If it lifts an automorphism of the x-coordinate projective line, it
normalizes the deck group generated by negation. Writing the translation point
as `T`, this forces `2T=O`. If the projective-line automorphism is defined over
`F_p`, its value at infinity is `x(T)`; a nonzero 2-torsion point with rational
x-coordinate is itself rational in odd characteristic. Therefore an odd-order
**full group** `E(F_p)` leaves only `T=O`, whose induced x-coordinate action is
the identity.

An odd target-subgroup order alone does not prove that the full curve has no
rational 2-torsion. Translation by such an outside point can map the subgroup to
another coset, which does not provide subgroup labels for free. This argument
also does not exclude correspondences, higher-degree maps, or auxiliary curves.
The incidence mechanism in Section 8 acts on coordinates as a selector; it does
not assume that its involution lifts to a curve automorphism. [pcc:unverified]

## 3. Adaptive capture by at most t fibers: the Stirling bound

Let `Pi` be any finite family of at most `A` partitions of a universe of size
`M`, with every block of size at most `D`. Let `L` be a uniform `N`-subset.
For integers `1 <= t <= h <= N <= M`,

\[
 \Pr[\exists\pi\in\Pi:\text{ at most }t\text{ blocks capture at least }h
       \text{ elements of }L]
 \le
 \min\!\left(1,
 A\binom Nh\sum_{j=1}^{t}
 {h\brace j}\left(\frac{D-1}{M-h+1}\right)^{h-j}
 \right). \tag{1}
\]

Here `{h brace j}` is a Stirling number of the second kind. Apply this theorem to
quadratic chains with `A=A_s` and `D=2^s`.

**Proof.** Fix a partition and `h` sample indices. If their values occupy at most
`t` blocks, partition those indices into `j <= t` nonempty groups according to
their common block. There are `{h brace j}` possible index partitions. Reveal
one sample value from each group first, then reveal the other `h-j` values.
Each additional value must lie in a block already containing its group's
representative. At most `D-1` elements of that block remain available. At every
reveal, at least `M-h+1` universe elements remain. The conditional probability is
therefore at most `(D-1)/(M-h+1)` per additional value. Ignoring requirements
that different groups have different blocks only enlarges the event. Multiply
these conditional bounds, and union-bound over index partitions, the
`binom(N,h)` choices of indices, and the `A` partitions. This already permits the
chain and the occupied blocks to be selected adaptively from the whole pool.

For `D=1` and `h>t`, the right side is zero as it should be. For `h=t`, the bound
can be vacuous; selecting one singleton per desired point is permitted by the
identity partition. The convention `0^0=1` gives the expected endpoint.

## 4. Adaptive capture with bounded total support: the anchor bound

This result does not require a bound on individual block sizes. Let `Pi` again
contain at most `A` partitions of a universe of size `M`, and let `L` be a uniform
`N`-subset. Assume

\[
 1\le t\le h\le N\le M,\qquad h\le b\le M.
\]

Then

\[
 \Pr[\exists\pi\in\Pi,\ U\text{ a union of at most }t\text{ blocks}:
 |U|\le b,\ |U\cap L|\ge h]
 \le \min(1,Q_h(b)), \tag{2}
\]

where, writing `(z)_k = z(z-1)...(z-k+1)` and `(z)_0=1`,

\[
 Q_h(b)=A\binom Nh\binom ht
          \frac{(b-t)_{h-t}}{(M-t)_{h-t}}. \tag{3}
\]

### 4.1 Anchor padding is legitimate

Choose `h` captured elements `W subset L intersection U`. Let `j <= t` be the
number of blocks meeting `W`. Select one representative from each such block and
pad those representatives, using other elements of `W`, to a set `T` of exactly
`t` anchors. This is possible because `t <= h`.

The union `U_pi(T)` of blocks meeting `T` is precisely the union of blocks
meeting `W`. In particular it is determined by the anchors, is contained in the
original `U`, and has size at most `b`. Padding anchors adds no new blocks.
After revealing the `t` distinct anchors, at most `b-t` available universe
elements remain in `U_pi(T)`. The chance that the remaining `h-t` selected
indices all hit that union is at most the falling-factorial ratio in (3).
Union-bound over the choices of partition, `h` indices, and `t` anchors.

The size condition on `U_pi(T)` is evaluated after the anchors are revealed.
Anchors for which its size exceeds `b` cannot witness the event; no conditioning
on a favorable future sample has been inserted.

### 4.2 Equivalent finite counting proof

The probability statement follows from a purely finite count:

\[
 |\mathrm{Bad}_N|
 \le A\binom Mt\binom{b-t}{h-t}\binom{M-h}{N-h}. \tag{4}
\]

For each partition choose the `t` anchors. Complete them to `W` by choosing
`h-t` elements from `U_pi(T) minus T`, which has at most `b-t` elements. Complete
`W` to the pool by choosing `N-h` elements from its complement, of size `M-h`.
Every bad pool has at least one such witness, so overcounting is harmless.
Dividing (4) by `binom(M,N)` and cancelling factorials gives exactly (3).
This finite-cardinality formulation is the preferred formalization target.

If `b<h`, the event is empty. The displayed theorem excludes `t=0`, whose
positive-capture event is also empty. For larger fiber budgets use
`t=min(t,h)`; the bound need not be informative. For nonconstant quadratic chains
one may also use `b <= min(M,tD)` and take the minimum of (1) and (2).

## 5. Actual unlabeled contamination: a necessary tail correction

A bound on total support `B <= h+c` implies contamination at most `c` when at
least `h` labels are captured. The converse is false: a selector could capture
`H_L>h` labels and admit support `H_L+c>h+c` while still having only `c`
unlabeled elements. Consequently a lower bound on total support alone is not an
unconditional lower bound on actual contamination.

There is a clean correction. Assume `0 <= c <= M-N`, and retain
`1 <= t <= h <= N <= M`. For `k=h,...,N`, put

\[
 R_k(c)=A\binom Nk\binom kt
          \frac{(k+c-t)_{k-t}}{(M-t)_{k-t}}.
\]

If a selector captures `H_L=k` labels and has at most `c` unlabeled elements,
its total support is at most `k+c`, so (2) applies. Therefore

\[
 \Pr[\exists\text{ admissible selector}:H_L\ge h, C\le c]
 \le \min\left(1,\sum_{k=h}^{N}R_k(c)\right). \tag{5}
\]

Direct factorial cancellation gives

\[
 \frac{R_{k+1}(c)}{R_k(c)}
 =\frac{N-k}{M-k}\frac{k+c+1-t}{k+1-t}.
\]

Both factors are nonincreasing in `k` when `M>=N` and `c>=0`. If

\[
 \rho=\frac{N-h}{M-h}\frac{h+c+1-t}{h+1-t}<1,
\]

the useful corollary is

\[
 \Pr[\exists\text{ selector}:H_L\ge h, C\le c]
 \le\min\left(1,\frac{R_h(c)}{1-\rho}\right). \tag{6}
\]

The cases `M=N` or `M=h` need their direct finite interpretation rather than a
division by zero. They are not part of the numerical application. The restriction
`c<=M-N` ensures `k+c<=M` throughout the sum; a larger contamination budget
requires capping support at `M` or reverting to (5) with that cap.

## 6. Exact canonical M31 evaluation

The input fixture used here is

| Parameter | Value |
|---|---:|
| `p` | 2,147,483,647 |
| subgroup order `r` | 2,147,446,831 |
| orbit universe `M=(r-1)/2` | 1,073,723,415 |
| inspected pool size `N` | 32,768 |
| required captured labels `h` | 128 |
| quadratic depth `s` | 10 |
| per-fiber degree cap `D` | 1,024 |

The subgroup order is an input from the canonical native fixture. This note does
not independently recount the curve or certify its order. Exact-integer
comparisons, rather than floating-point logarithms, certify the powers below.

### 6.1 Capture-only bound

For (1), the following are conservative certified upper bounds:

| Fiber budget `t` | Probability at most |
|---:|---:|
| 1 | `2^-716` |
| 4 | `2^-405` |
| 8 | `2^-207` |
| 10 | `2^-133` |
| 12 | `2^-66` |
| 13 | `2^-35` |
| 14 | `2^-5` |
| 15 | bound is vacuous |

Thus even unrestricted adaptive choice of a shared ten-step chain and fourteen
fibers succeeds on at most one pool in 32 under this model. A vacuous result at
fifteen does not show that such selectors exist often or can be found efficiently.

### 6.2 Total-support bound

Each entry `k` in the following table means that (2) is at most `2^-k`.
An em dash means the bound is vacuous. The last column uses a different support
cap for each row.

| `t` | `b=128` | `b=256` | `b=512` | `b=1024` | `b=t*1024` |
|---:|---:|---:|---:|---:|---:|
| 1 | 1270 | 1019 | 861 | 721 | 721 |
| 4 | 1184 | 937 | 782 | 645 | 388 |
| 8 | 1075 | 832 | 681 | 548 | 177 |
| 12 | 969 | 729 | 583 | 453 | 27 |
| 14 | 916 | 679 | 534 | 407 | — |
| 16 | 864 | 629 | 486 | 361 | — |
| 32 | 458 | 241 | 115 | 7 | — |
| 64 | — | — | — | — | — |
| 128 | — | — | — | — | — |

For example, a 32-fiber selector of total support at most 1,024 captures 128
labels with probability at most `2^-7`, even after unrestricted adaptive chain
selection. This statement is about the simultaneous capture and support event.

### 6.3 Actual-contamination thresholds remain a follow-on calculation

The symbolic tail correction (6) is part of the written proof. The frozen Rust
certificates cover (1) and (2), not a search for the smallest contamination
allowance at a specified success probability. Numerical threshold tables are
therefore outside this cycle's retained native evidence and are not reported
as verified results here. A later native extension can compare (6) exactly with
one half, retaining both a threshold and its predecessor as certificates. [pcc:unverified]

Such a threshold would be necessary only for the probability of **both**
capturing at least 128 labels and having contamination at most that allowance.
It would not establish a construction at the threshold or imply that every
successful selector must have that contamination.

### 6.4 What was checked

Independent scratch BigInt arithmetic was used during mathematical review;
it is not retained as an executable research result. Every numerical bound
reported in the tables above is covered by the separately compiled Rust
checker, which records four passing tests: arithmetic against machine
`u128`, independent combinatorial identities, exact power-of-two rounding, and
exhaustive finite-partition examples. See
`../capacity-checker/evidence/tests.txt`, `bounds.tsv`, and `certificates.json`.
These tests validate arithmetic and finite examples. They do not replace the
general proofs above or a Lean formalization.

## 7. What this forces into complete-cost accounting

The finite bound gives no free construction algorithm. A candidate must charge
generation of all `N` labeled input points, processing of their coordinates,
selector fitting, root recovery, validation, and failed attempts. Evaluating a
new length-`s` candidate at all `N` inputs involves `Ns` map evaluations unless a
specific reuse algorithm reduces that work; shared prefixes can help, but their
cost remains visible. The count does not prove a universal lower bound of `Ns`
for every possible synthesis algorithm. [pcc:unverified]

A one-relation APS recovery may only substitute known public labels. Admitted
unlabeled points cannot be treated as labeled for free. A method must either
filter them, obtain labels with charged work, or collect enough relations to
eliminate their contributions. A full-rank block of `c` unknown columns needs
more than `c` rows for a nonzero left-kernel combination; special structured
elimination can behave differently and must be measured. Merely counting `c`
does not establish a universal relation-count lower bound.

Materializing a support of size `B` requires at least its output/storage work.
An implicit solver need not enumerate every contaminant, but it must supply and
validate the needed query and witness-recovery operations. Support bounds do not
establish a solving-degree, LFD, or complete ECDLP runtime bound.

## 8. A concrete mechanism that remains open: involution synthesis

The capture bound is not a reason to repeat blind selector search. It suggests
an explicit synthesis problem. For distinct finite coordinates `x,y`, the
involution

\[
 \sigma(x)=\frac{ux+v}{wx-u},\qquad u^2+vw\ne0
\]

folds `x` and `y` into the same quadratic-quotient fiber exactly when

\[
 wxy-u(x+y)-v=0. \tag{7}
\]

Each candidate pair gives a line in the projective parameter space
`[u:v:w]`. Two distinct pair-lines meet at a parameter; a high-incidence point
off the conic `u^2+vw=0` gives an involution that folds many pairs together.
The determinant exclusion is essential: pair-lines sharing an endpoint meet on
the singular conic and can otherwise create large, completely invalid signals.
For `w!=0`, (7) is equivalently

\[
 (x-\alpha)(y-\alpha)=\kappa,\qquad
 \alpha=u/w,\quad\kappa=(u^2+vw)/w^2\ne0.
\]

For `w=0`, it becomes a fixed pair-sum/reflection condition. The homogeneous
version is `wXY-u(XW+YZ)-vZW=0`, so infinity can be included without ad hoc
exceptions. At a nonsingular involution, distinct folded pairs are disjoint.

This identifies a testable primitive: report high-incidence nonsingular
parameters, propagate original-point witnesses through successive quotient
layers, and count the actual final support and contamination. No implementation
or speedup of that primitive is supplied here. At `N=32768`, explicitly creating
all pair-lines already means 536,854,528 records. Enumerating intersections of
pairs of those lines is worse. An output-sensitive or compressed implementation
must account for every inspected pair; naming an incidence problem does not
provide such an algorithm. [pcc:unverified]

A discriminating next step is a deterministic replay on an existing frozen
public-label pool: produce a concrete nonsingular chain and its recovered
witnesses, report all inspected-pair/map work, and certify total support by
projective preimage recovery. Then compare the complete cost with the current
APS/PKM extractor on the same pool. This is a proposed next step, not a run
performed in this review.

Correlated public-label constructions are another way to leave the uniform-pool
model. They require a fresh useful-relation analysis: relations that cancel the
construction's original coefficient vectors are tautologies and do not recover
the target logarithm. Extra selector descriptions, mixed-degree maps, branching
trees, and partial unknown-label elimination likewise need their own count and
cost contracts. None is ruled out by the present theorem.

## 9. Lean instance gate, inheritance, and formalization boundary

The repository's `lean-instance-gate` skill was read in full before any proposed
Lean edit. No Lean source was written. Read-only environment checks found no
`lean`, `lake`, or `elan` executable in PATH and no usable local Mathlib or
crypto-lean checkout/toolchain under the inspected workspace, home, `/opt`, and
`/usr/local` locations. Therefore no assembly probe was typechecked and no
kernel-verification claim is made.

The remote shared library was inspected at
`aburan28/crypto-lean@8eb8e502b92a4c1caefa781672347c1d78fa8e59`. Its pins are Lean
`v4.35.0` and Mathlib `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.

### 9.1 Six-field proof map

1. **Structure above.** A finite universe, a finite family of equivalence-relation
   partitions (`Setoid`), and existing `Finset`/fixed-cardinality-subset APIs.
   Curve geometry is a later instance obligation, not the primary object.
2. **Theorem used as stated.** Existing finite-union and subset-counting laws
   support the proof. No assembly-ready theorem giving (4) was located in this
   bounded audit. The missing generic anchor-count law belongs in the shared
   combinatorics layer before any M31 endpoint.
3. **Object as an instance.** Fiber equivalence relations of actual reduced
   rational maps, restricted to the real x-coordinate orbit universe. A syntax
   of selector words is only a presentation used to bound the family size.
4. **Assembly probe.** First instantiate the generic finite theorem with abstract
   partition-family and cardinality hypotheses; then instantiate those hypotheses
   with the rational-map family and the actual subgroup. This probe is blocked by
   the missing compiler/dependency environment and is not reported as checked.
5. **Missing components.** The generic anchor witness-cover law; its factorial
   normalization; the Stirling occupancy law; the contamination-tail ratio;
   the reduced quadratic-map/output-equivalence count; the rational-map degree
   fiber bound and Möbius absorption assembly; and the subgroup x-orbit instance.
   Each requires searching the full owning library before implementation.
6. **Inheritance.** Reuse the laws in the following table. Do not replace the
   geometric and sampling obligations by assumed numerical inequalities and call
   the resulting endpoint the full theorem.

| Component | Existing source or proposed owner | Status |
|---|---|---|
| Fixed-size finite subsets | Mathlib `Finset.powersetCard`, `Finset.mem_powersetCard`, `Finset.card_powersetCard` | Exact declarations inspected at the pinned Mathlib revision |
| Union cardinality upper bound | Mathlib `Finset.card_biUnion_le` | Existing use inspected in shared `FiniteIsolation` |
| Anchor witness surjection/cover | Generic shared combinatorics law | New obligation; not typechecked |
| Uniform sample probability | Cardinality ratio on all N-subsets | Follows from the finite model; no probability-library instance assembled |
| Fiber partitions | Existing semantic equivalence relations of maps | Requires source-level assembly |
| Quadratic involution and chain count | Generic projective/function-field layer | Hand proof above; no kernel receipt |
| Subgroup negation-orbit cardinality | Existing finite-group and elliptic x-coordinate laws | Exact consumer assembly not completed |
| Integer specialization | One generic checker plus per-instance certificate | Rust arithmetic checked; no Lean soundness theorem supplied |

Dependency order is: generic finite anchor law; numerical normalization and tail
law; generic map-family count; subgroup/orbit instance; then the M31 endpoint.
The combinatorial and map-family branches can be developed independently before
their final assembly. A candidate module name such as
`CryptoLean.Combinatorics.PartitionCapture` is a proposal, not an existing module.

### 9.2 Existing work and why it is not silently reused

The inspected `CryptoLean.Combinatorics.FiniteIsolation` supplies useful finite
union-counting patterns. The shared selector-tree report supplies a different
shortlisted-target theorem for surjective additive homomorphisms with independent
right inputs. Its hypotheses do not identify the adaptive partition event here.
Likewise, `CryptoLean.Curve.LineIncidence` proves fixed-slope secant bounds on
Weierstrass curves, not high-incidence synthesis of projective involutions.
These are relevant components and prior work; they do not constitute a proof of
the new claims. [pcc:unverified]

| Claim | Current evidence | Missing formal evidence |
|---|---|---|
| At most `A_s` shared-chain partitions | Hand function-field/projective proof | Faithful generic theorem and instance assembly |
| Adaptive Stirling bound (1) | Hand conditional-count proof, exact arithmetic, finite tests | Kernel proof of the general counting theorem |
| Anchor/support bound (2)-(4) | Hand finite witness proof, exact arithmetic, finite tests | Kernel proof of the generic witness cover |
| Actual-contamination bound (5)-(6) | Hand tail proof and independent exact evaluation | Kernel proof and dedicated checker receipt |
| Specific powers of two | Replayed integer certificates | Generic Lean checker soundness and per-instance application |
| Faster complete M31 ECDLP | No such result | A new extractor and matched complete-cost validation |

The acceptance boundary is explicit: no `sorry`, added axiom, `native_decide`,
raised proof limit, or conditional arithmetic lemma has been used to simulate a
full formalization. Once a toolchain is available, the faithful theorem still
needs a focused build, the repository proof gate, and `#print axioms` receipts.

## 10. Sources and relationship to the literature

The bounds in this note are elementary arguments presented here with their full
scope; no literature-novelty claim is made. The wider research objective remains
the dedicated relation-extraction problem in PKM and the useful-single-relation
goal in APS. Chained summation equations or public point labels alone do not
establish novelty or speedup. [pcc:unverified]

- Petit, Kosters, Messeng, *Algebraic approaches for the Elliptic Curve Discrete
  Logarithm Problem over prime fields*, PKC 2016:
  <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>.
- Amadori, Pintore, Sala, *On the discrete logarithm problem for prime-field
  elliptic curves*, 2018, institutional record:
  <https://research.tue.nl/en/publications/on-the-discrete-logarithm-problem-for-prime-field-elliptic-curves/>.
- Exact inspected Mathlib finite-subset API:
  <https://github.com/leanprover-community/mathlib4/blob/cfc55fc58714a59dc06e6e79edc6d1d95140d3c2/Mathlib/Data/Finset/Powerset.lean>.
- Exact inspected shared finite-isolation module:
  <https://github.com/aburan28/crypto-lean/blob/8eb8e502b92a4c1caefa781672347c1d78fa8e59/CryptoLean/Combinatorics/FiniteIsolation.lean>.
- Existing selector-tree scope and its independent-input limitations:
  <https://github.com/aburan28/crypto-lean/blob/8eb8e502b92a4c1caefa781672347c1d78fa8e59/docs/selector-tree-20261116.md>.
- Inspected Lean gate, blob `7d3ffa79b65a92cdcbe1df02043a15e426f3b5c4`:
  <https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/lean-instance-gate/SKILL.md>.
