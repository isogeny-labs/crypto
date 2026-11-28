# A torus-shift lift obstruction for the Dickson leaf algebra

Date: 2026-11-23 UTC. Scope: disjoint written mathematics in the existing Prime31 S3/pair-spectrum lane, **not** a relation finder or a performance claim. The proof below is a corollary of the older torus-extension private-branch theorem, not a novel geometric lower bound. Standing: [pcc:unverified].

## Exact input, including the small-order and rationality conditions

Let `p>3`, `E/F_p: y²=f(x)=x³+a x+b` be nonsingular, and assume `#E(F_p)` is odd. Let `T` be the norm-one torus `{z in F_(p²)^*:z^(p+1)=1}`, and `H=mu_q <=T` a **power-of-two** subgroup. Fix a trace-good coset `gH`, so the values `x_w(g)=gw+(gw)^(-1)` are pairwise distinct. The rational affine base consists only of the indices `w` for which `f(x_w(g))` is a nonzero square in `F_p`; its two signs must still be retained. This mask need not be invariant under multiplying `w` by `H`.

The earlier proof in `research/prime_one_shot_20261029/continuation/theory/torus-extension.md` and its independent review `continuation/reviews/torus-and-reducedness.md` gives six simple finite branch zeros for each rational function

`b_w(Z)=f(wZ+(wZ)^(-1))`,

and **disjoint** zero sets as `w` ranges over distinct elements of `H`. The proof uses odd curve order (so `f` is an irreducible cubic) and the absence of 3-torsion in `H`; it is not asserted for an arbitrary subgroup or arbitrary curve.

## Single-cover model and rational Frobenius guard

For `w=1`, write `x=Z+Z^(-1)` and `Y=Z² y`. The exact identity is

```
Y² = Z⁴ f(Z+Z⁻¹)
   = Z⁷ + (a+3) Z⁵ + b Z⁴ + (a+3) Z³ + Z.          (1)
```

This is a genus-three hyperelliptic double cover of `E` when `f(2)f(-2) !=0`, as happens under the stated odd-order assumption. There are six distinct roots over nonzero `Z`, a simple zero at zero and an odd-degree pole at infinity: eight simple branch points and genus three. The map `(Z,Y) -> (Z+Z^(-1),Y/Z²)` has degree two; it does **not** identify elliptic addition with multiplication in `T`.

For `Z in T`, `Z^p=Z^(-1)`. If the mapped point has `y in F_p`, then its lift satisfies the exact guard

```
Frob_p(Z,Y) = (Z^(-1),Y/Z⁴).                       (2)
```

This is the deck involution on the chosen affine chart, not a claim that every `F_(p²)` point of the cover maps to a rational point. One must still enforce the actual square-root and point-lift conditions. A non-Frobenius-fixed branch cannot be silently admitted to the rational relation solver.

## The precise failed shortcut: no rational lift of a nontrivial torus shift

**Proposition (restricted obstruction).** Under the hypotheses above, for every nontrivial `w in H`, the substitution `Z -> wZ` does **not** lift to an automorphism of the single quadratic function field `F_pbar(Z)(sqrt(b_1(Z)))` which restricts to that substitution on `F_pbar(Z)`. Equivalently, there is no rational `h(Z)` over `F_pbar` with

```
b_w(Z)/b_1(Z) = h(Z)².                              (3)
```

**Proof.** Pick any of the six finite simple zeros `z0` of `b_w`. The older private-branch result states that `b_1(z0) != 0`; neither function has a pole there. The valuation of the quotient in (3) at `z0` is therefore 1, whereas a rational square has even valuation at every geometric point. Contradiction. Any extension of the base-field shift to the quadratic function field must send its square-root generator to `h(Z) sqrt(b_1(Z))`, giving precisely (3). This proves the claimed restricted no-lift statement. It does not rule out an isomorphism between *different* covers `Y_w²=b_w(Z)` and `Y_1²=b_1(Z)` after reparametrization.

**Corollary (representation size, not query lower bound).** The `q` distinct squareclasses `[b_w]` are independent in `F_pbar(Z)^*/squares`: in any nonempty product select one factor and its private simple zero, which occurs to odd order in that product. Therefore the **explicit compositum** of all leaf radical function fields over `F_pbar(Z)` has degree `2^q`. For an arbitrary selected subset of `B` indices it has degree `2^B`. This is an algebraic extension-degree statement about a particular expanded representation, **not** a lower bound on circuit size, pointwise arithmetic at a fixed seed, the exact pair-spectrum algorithm, or another succinct data structure. At one chosen `g` some selected square roots already lie in `F_p` by construction.

The tempting proposed shortcut “one genus-three cover plus a cyclic torus FFT automatically supplies all leaf signs or all pair-sum components” is therefore unjustified. Multiplication by `H` acts on the `Z`-coordinates and is useful for generating the **unfiltered** abscissa fibre; it is not a rational automorphism action on the single elliptic-lift cover. A Fourier routine on that orbit would additionally need an exact treatment of the liftability mask, signs and the nonlinear elliptic group law. The obstruction is **only** to that equivariant single-cover reduction, not to a wholly different indexed contraction.

## Costs and the remaining exact query gate

For a trace-good `q`-element full fibre, generating the `x_w` values by a torus recurrence and testing `f(x_w)` costs at least `q` evaluations, plus field setup/square tests/inversions; its selected rational base has `B<=q` abscissae. The original pair chart has `D<=B²` actual nonidentity abscissae. Building the explicit `B`-radical compositum grows as `2^B` over the function field and is excluded as a fast proposed implementation. Keeping a succinct polynomial/radical circuit avoids that expansion but supplies **no proven** sublinear empty-query evaluation.

The maintained control remains the exact `~O(D)` S3 pair-spectrum scan, with correct zero-pair branch, rational-root isolation, signed point replay, collisions, useful rank and complete output. A real improvement would need a concrete source-index construction with charged setup `~O(D)` and query `~O(D^theta+h)` for `theta<1` to beat that component's exponent; the stricter conditional full-pipeline target is `theta<1/2` for `D=Theta(B²)` and the recorded model. No such algorithm, source-frozen Rust candidate, native test, signed verification or full-cost result is produced here.

## Symbolic cross-check and provenance

A local symbolic-polynomial check over `Q[a,b,Z,Z^-1]` reduced both residuals to zero: (1), and the deck equality `F(Z^-1)=F(Z)/Z^8` for `F(Z)=Z^4 f(Z+Z^-1)`. This checks algebraic transcription only; the six-private-zero theorem and the valuation argument are supported by the prior written proof, not independently proved by this symbolic call. No field/group computation, cryptographic benchmarking, Rust build or paid compute was performed.

Sources read: the two prior October 30 torus papers named above, `iteration10/S3_PENCIL_CONTRACTION.md`, `iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md`, and the named `problems/prime31-exact-pair-spectrum-query.md`. Semaev (2015, arXiv:1504.01175) supplies the baseline S3 representation; special-resultant methods of Bostan–Flajolet–Salvy–Schost (JSC 2006) do not provide an elliptic signed-target index at the requested cost. Shared research-KB and native worker runtimes were not exposed by this connector session, so no exhaustive novelty claim is made.

### Independent validation / possible counterexample gate

To challenge this note, a separate reviewer should verify the odd-order/irreducibility implication, six simple nonzero roots, the private-zero no-ratio argument for each distinct `w`, and the exact Frobenius/deck convention. A small-field Rust fixture can then enumerate `H`, branch zero sets over a splitting field, and check valuation parity and selected point lifts; do not claim that fixture executed until source and outputs are frozen. Record a genuine counterexample if odd-order, 2-power or trace-good hypotheses are dropped. Future algorithmic work should seek a **different** method using leaf pair norm and swap symmetry, not repeat the rejected single-cover cyclic FFT assumption.

[pcc:unverified]
