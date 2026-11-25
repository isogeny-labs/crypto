# Batched exact signed-slope moments and a truncated-moment countermodel

2026-11-25 — PR #5131, parent 9abee312774275eb56cb3f8a390bc530b7ca8d08. Written producer derivation [pcc:unverified]. No new elliptic computation, native test, random draw, timing, paid job or signed proof.

## Inputs, scope, and sources checked

Use the inherited ordinary cofactor-one M31 source: p=2147483647, a=847514846, b=2087444724, r=2147446831, G=(625037715,2100627251), B=31 (parameters inherited, not recertified). Arithmetic below is over F_p, never F_r; scalar relation arithmetic remains over F_r. Let X=x((A+A) minus O), D=|X|, H(U)=product_(u in X)(U-u); archived D=961 is conditional. Let Q=-T=(q,v) be a supplied nonidentity rational target. As in the previous chord-slope gate, remove q from X if present; write X0, d0=|X0| and H0. Keep the exceptional splits O+T and -T+2T and target O separately, with all original signed buckets.

Read the actual preceding chord-slope proof (blob ffa8942e9a84004fa8f4ea6716b06ac990b3b7b8) and receipt (fa6730c3e0ed9ae84b31cc693f69df2dce5bfb6a), the exact-query problem and active root-qualification ownership #5128. #4977/#4979 merged; #5131 open draft at the parent above. Main and branch ACTIVE and 2026-11-25 logs read separately. Bounded default-branch code search "slope moments" found the earlier orientation-moment proof map at 969445bc449c7f8c86034eee736ff9ae7ebac908/research/prime_orientation_layers_20261114/moments/PROOF_MAP.md; its reciprocal orientation-layer arithmetic is a different object. The classical chord law and Newton identities are not novel. Read Somlai, A new proof of Rédei’s theorem on the number of directions, Arch. Math. 122 (2024), https://doi.org/10.1007/s00013-024-01979-x, especially sections 2–3 on projection counts/polynomial degrees; its hypotheses do not give a low-moment duplicate test for our pair spectra. Standard subproduct/remainder-tree evaluation as documented at https://flintlib.org/doc/algorithms/polynomials/subproduct-tree.html supplies the batch primitive. No novelty claim. KB/Conductor and native agent dispatch unavailable. Space read requires user input; no mutation/replay.

## 1. Exact Laurent recurrence on the actual signed spectrum

For u in X0 let t=u-q and y^2=f(u)=u^3+a*u+b. The two slopes z_+=(y-v)/t and z_-=(-y-v)/t have sum A(t)=-2v/t and product C(t)=(v^2-f(u))/t^2=-(3q^2+a)/t-3q-t, using v^2=f(q).

Put P_j(t)=z_+^j+z_-^j. Then
    P_0=2, P_1=A(t), P_j=A(t)P_(j-1)-C(t)P_(j-2).
Each P_j has Laurent support [-j, floor(j/2)]: by induction using the exponents {-1} of A and {-1,0,1} of C. Equivalently
    P_j(t)=2*t^(-j)*sum_(h=0..floor(j/2)) binom(j,2h)*(-v)^(j-2h)*f(q+t)^h.
The desired exact moment is s_j=sum_(u in X0) P_j(u-q). Thus all first K signed-slope moments reduce to sums of shifted powers and inverse shifted powers, with no per-target enumeration of the 2d0 slopes.

Write L0(U)=H0'(U)/H0(U). For e>=1 the coefficient of w^(e-1) in L0(q+w) is -sum_(u in X0)(u-q)^(-e). This follows by geometric-series expansion of each 1/(q+w-u). Positive shifted power sums through floor(K/2) follow from the source power sums via binomial expansion; if q belongs to X, its shifted power is zero for positive exponent and its contribution to exponent zero must be removed. Source power sums can be extracted via Newton identities from H's leading coefficients; they are not free but require at most O(K^2) classical arithmetic when K<=D (or extend the recurrence with missing coefficients zero when K>D).

Example:
    s_1=2v L0(q);
    s_2=-4v^2 L0'(q)-2(3q^2+a)L0(q)+4q*d0+2*sum_(u in X0)u.
These signs follow by expanding f(q+t); the direct signed moments are the reference.

## 2. Charged batch, including exceptional q values

For M supplied affine target points (possibly repeated q), form the product of (U-q_i)^(K+2) for distinct q_i and evaluate H modulo these powered factors by a polynomial remainder tree. Its total modulus degree is at most M(K+2). For each q, translate the residue into w=U-q. If its constant is zero, the simplicity of H permits exact removal of one w factor, yielding H0(q+w) to the needed order. Otherwise H0=H. In either case H0(q) is nonzero. Divide its formal derivative by the series (through w^(K-1)) to obtain inverse moments. This handles q in X without a fresh global product. Charge the separate point-split exception checks from the chord-slope proof. T=O uses the previous complete identity branch, not the generic moments.

With fast polynomial arithmetic the source product costs O-tilde(D); Hermite/remainder-tree batch evaluation and Taylor shifts cost O-tilde(D+M(K+2)) field operations for K+1<p (ordinary factorial-based shifts), with O-tilde(D+M(K+2)) storage including the tree. Positive shifted moments, Laurent recurrence and dot products cost O(K^2) per target by classical arithmetic (each recurrence stage uses O(j) coefficients and only three Laurent shifts). Source positive moments cost O(K^2), and each target requires a nonzero series-constant inverse, priced separately or batched. Thus a conservative total is O-tilde(D+M(K+2))+O((M+1)K^2) field operations plus the priced inversions and O(M) exception membership tests on the existing dictionary. Field multiplication/bit cost and pair-spectrum construction/bucket storage remain charged; no curve computation or performance test occurred. For fixed K this replaces O(D*M) direct slope processing for the *moment diagnostic* by near-linear batch work. For a single target with arbitrary q it does not remove the D term. It returns K moments, not a split, root, exact miss certificate or useful independent row.

## 3. Exact negative control: low moments cannot certify general split-polynomial misses

Let n=2d0. In the relaxed class of fully split monic polynomials of degree n, with root multiplicity at most two, suppose a nonzero guard polynomial Delta has degree <=4 (the chord-slope tangent guard has this degree). Let m divide p-1 with 2m<=n. If enough multiplicative cosets and unused field elements exist, choose two distinct cosets C1,C2 of the order-m subgroup of F_p^*, avoiding the at-most-four roots of Delta. Choose a common simple padding set R of n-2m distinct nonzero points disjoint from the cosets and guard roots.

    miss multiset = C1 union C2 union R (all simple)
    hit multiset  = two copies of C1 union R (m double roots).

For every integer j with 0<|j|<m the sum of jth powers on each coset is zero by the geometric-series identity (valid also for negative j since all points are nonzero). The zeroth moments match as integer lengths. Thus all positive and reciprocal power moments through m-1 agree, despite gcd(F,F' Delta)=1 for the miss and degree m for the hit. At j=m the two moments differ by m times the difference of the two distinct coset representatives' mth powers, hence nonzero. By Newton identities the first m-1 leading coefficients also agree. This is not a lower bound for arbitrary sketches, a statement about actual Dickson pair norms, or a real elliptic query counterexample: the relaxed spectra have not been shown to satisfy the signed-slope pairing constraint or to be induced by any actual E and X=A+A. A candidate must exploit that extra structure to avoid this control; it cannot infer exact misses merely from the number of moments and splitness/multiplicity promises.

At M31 with conditional D=961, m=906 is the largest divisor of p-1 not exceeding D, and p-1=906*2370291. Even with q in X (n=1920) the two cosets fit; generic n=1922 has 110 padding slots (108 when q in X). A quartic excludes at most four of 2370291 cosets; the padding is also available. The two relaxed instances match 905 positive and 905 reciprocal moments, but selector degree is 0 versus 906. This is a substantial deterministic *model* negative control, not observed curve behavior.

## 4. Validation and next gate

CHECK.py uses SymPy 1.14.0 over the integer rational-function ring for the recurrence and support at j=0..12 (13 residuals zero and 13 support checks), plus exact integer modular-power and divisor certificates for m=906. No elliptic point evaluations or randomized trials; no seeds, timings or benchmark claims. Representative algebra check only, not independent validation or a formal proof. The next discriminating step is to find an exact predicate on the actual signed pairing/Dickson leaf algebra that can be evaluated from a bounded batch summary, or to prove that a specifically defined such predicate fails on actual realizable spectra. The exact pair-complement scan and chord-slope gcd remain controls; output and useful-rank validation cannot be omitted. No new Rust work, #5128/#5157 duplicate execution, paid jobs, merge or Space change.

Impact score: 3/10 — exact near-linear-in-batch-size moment diagnostic for fixed K and a rigorous countermodel to general truncated-moment miss certification; no actual-Dickson miss predicate or faster extractor. [pcc:unverified]
