# CT121 — Complete odd-prime Frobenius field degrees for the fixed binary destination

**2026-11-14 · Standing: [pcc:unverified].** This extends CT120's inert-prime theorem to **every odd prime order**, including split Frobenius eigenlines. It is not a map from arbitrary public inputs, a cost benchmark, or a claim of new Frobenius theory.

## Theorem

Let E over F2 be y²+y=x³+x². Its Frobenius π satisfies π²+2π+2=0 and π⁴=[−4]. Let ℓ be any odd prime and T be nonzero ℓ-torsion. Write d(T) for the least extension degree defining T, and k(T) for that defining x(T).

**Inert orders** ℓ≡3 (mod 4): CT120 gives h=ord_ℓ(−4), d(T)=4h, and k(T)=2h if h is even, otherwise 4h.

**Split orders** ℓ≡1 (mod 4): choose i in F_ℓ with i²=−1 and set λ₊=−1+i, λ₋=−1−i. The discriminant −4 is nonzero mod ℓ, so E[ℓ] is the direct sum of one-dimensional eigenspaces V₊ and V₋. Write T=T₊+T₋ and let I contain the signs whose component is nonzero. Set

\[
 d=\operatorname{lcm}\{\operatorname{ord}_\ell(\lambda_\sigma):\sigma\in I\}.
\]

Then the *exact* degrees are

\[
\boxed{d(T)=d,\qquad
 k(T)=\begin{cases}
 d/2 & d\text{ even and }\lambda_\sigma^{d/2}=-1\quad(\forall\sigma\in I),\\
 d & \text{otherwise}.
 \end{cases}}
\]

On a single eigenline, k=d/2 for even d and k=d otherwise. For a mixed point with both components, the halving criterion is equivalent to v₂(ord λ₊)=v₂(ord λ₋)≥1. Precisely 2(ℓ−1) nonzero points lie on the two eigenlines; the remaining (ℓ−1)² have mixed coordinates.

**Proof.** Frobenius on the two-dimensional ℓ-torsion is semisimple with distinct eigenvalues λ₊, λ₋. Its n-th power fixes T iff every present eigenvalue has n-th power one; the least such n is d. The x-map identifies T and −T and no other odd-torsion points. If the Frobenius orbit contains −T, it must be the unique element of order two in the cyclic orbit, first reached at d/2. Checking the action on each nonzero eigenspace gives the displayed condition. The two-adic valuation equivalence follows by examining whether each λ has the full 2-primary part of d. Counting vectors in the two punctured one-dimensional eigenspaces gives 2(ℓ−1), leaving (ℓ−1)² mixed. For inert primes, apply CT120's π⁴ scalar theorem. [pcc:unverified]

## Exact field-degree examples

| ℓ | Eigenline (point, x) | Other eigenline (point, x) | Mixed (point, x) |
|---:|---:|---:|---:|
| 5 | (1,1) | (4,2) | (4,4) |
| 13 | (6,3) | (12,6) | (12,12) |
| 17 | (16,8) | (16,8) | (16,8) |
| 29 | (28,14) | (7,7) | (28,28) |

At ℓ=5 there is an F₂-rational torsion eigenline yet generic 5-torsion needs F₂⁴. Consequently a single extension-degree claim for all nonzero torsion at split ℓ is **false**; assuming every split point benefits from the favorable eigenline is also false. CT115/120's inert ℓ=11 degree-40 point and degree-20 x-coordinate remain unchanged. These are unavoidable *field degrees of the classical coordinate elements*, not universal bit-operation or compressed-encoding lower bounds.

## Independent arithmetic controls

Two no-dependency programs check every odd prime 3≤ℓ≤211 (46 primes): Python uses direct Frobenius-vector recurrence and eigenline equations, while independently written JavaScript uses spectral projectors and full matrix multiplication. They each compare predicted degrees with explicit first-return periods for three vectors per prime and both eigenvectors at each split prime. At ℓ=5,13,17,29, every one of the ℓ²−1 nonzero vectors is separately checked, classified and counted by both algorithms. Both reject the zero input and check the ℓ=5 split-eigenline mutation. Four producer-local ordinary/optimized modes agreed with SHA256 7ed47a628626ea5180e8a4ea2014cf5f49f68053ccab3319cbaf184e08bc79e1. This is not independent signed PCC certification.

Reproduce from repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct121
    python3 -B "$D/ct121_frob_complete.py" >/tmp/ct121-py.json
    python3 -O -B "$D/ct121_frob_complete.py" >/tmp/ct121-pyo.json
    node "$D/ct121_frob_complete_independent.js" >/tmp/ct121-js.json
    node --no-opt "$D/ct121_frob_complete_independent.js" >/tmp/ct121-jso.json
    cmp /tmp/ct121-py.json /tmp/ct121-pyo.json
    cmp /tmp/ct121-py.json /tmp/ct121-js.json
    cmp /tmp/ct121-py.json /tmp/ct121-jso.json
    printf '%s  %s\n' 7ed47a628626ea5180e8a4ea2014cf5f49f68053ccab3319cbaf184e08bc79e1 /tmp/ct121-py.json | sha256sum -c

## Sources and governance

Reviewed [root PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), [CT120 #4165](https://github.com/isogeny-labs/crypto/pull/4165), CT115–119 reports, cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), [Stanford Frobenius/point counting](https://crypto.stanford.edu/pbc/notes/elliptic/count.html), and [Sage Frobenius polynomial documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html). This is an application of standard semisimple linear algebra; no novelty assertion. Research-KB MCP and checkout-local wrapper/Conductor were unavailable to this tool runtime; no claim about KB contents. Exact-head CI, policy gates, main-project tests and independent signed scientific PCC remain separate. No EKS/Spot, protected merge, leaderboard entry or speed claim.

**Next constructive gate:** choose a source-place-labelled *non-invariant* global x-conjugate from arbitrary public Q without scalar-table advice, certify characteristic-zero and both residue specializations, and charge global setup, factoring, online recognition, binary target field/DLP and sign against strongest source cold rho/IC. An enumerated scalar table or the CT118 interpolation is a control, not a claimed advantage. Genus-two/three, nonsplit Cantor/Richelot and Jacobian work on concurrent branches was preserved, not replicated. [pcc:unverified]
