# Genus three Weil recovery screen

## Which parts are established, and which are derived here?

### Takeaway
Hasse–Witt coefficient recovery and integer lifting are established methods. The exact CRT/root-interval uniqueness certificate below is a derivation for this screen, not a novelty claim. It can replace source F_p² enumeration when its candidate count is one; its computation and validation costs remain charged.

### Cited Findings
- For odd p and squarefree degree-seven f, use the smooth projective curve, including its single point at infinity. Put
  L(T)=1+a1 T+a2 T²+a3 T³+p a2 T⁴+p² a1 T⁵+p³ T⁶,
  χ(X)=X⁶ L(1/X).
  With c_k=[x^k]f(x)^((p−1)/2), W_ij=c_(pi−j), 1≤i,j≤3, one has L(T)≡det(I−TW) (mod p). Thus a1≡−tr W, a2≡sum of principal 2×2 minors, a3≡−det W. Transposing W leaves this determinant unchanged over F_p. — Harvey–Sutherland, *Computing Hasse–Witt matrices …*, §1, equation (1.1): [2014 primary paper](https://doi.org/10.1112/S1461157014000187)
- N1=#C(F_p)=p+1+a1; N2=p²+1−a1²+2a2; N=#J(F_p)=L(1)=A+(p+1)a2+a3, A=p³+1+(p²+1)a1. In particular a1 is minus the Frobenius trace, not that trace. — Zeta-function convention and genus-three polynomial in Shi, *Lifting L-polynomials of genus 3 curves*, §§1,5.3: [Primary paper](https://arxiv.org/html/2602.00965v1#S5.SS3)
- The earlier lifting literature already uses residues, Weil bounds and Jacobian group computations. Shi gives a Las Vegas general lifting method with expected worst-input p^(1/2+o(1)) time, and heuristic average-input p^(1/4+o(1)); ambiguous cases require further work. Its auxiliary extension-field group operations must not be confused with enumerating C(F_p²). — Shi, abstract, §§5–6: [Primary paper](https://arxiv.org/html/2602.00965v1)
- Source construction previously used N1=129 and N2=16043, obtaining L=[1,1,−43,−1224,−5461,16129,2048383]. — Local primary producer record [coldbuild13 RESULTS](../../research/hec_coldbuild13_20261115/RESULTS.md).
- Coldcost13 records sourceWeil median 4.234041 ms within complete candidate median 1780.974708 ms, versus best native median 0.797166 ms; producer evidence is local ARM L0/unisolated and independently unvalidated. — Local primary record [coldcost13 RESULTS](../../research/hec_coldcost13_20261115/RESULTS.md).

### Inferences
The Newton identities give the N1,N2 expressions directly from this L convention. Hasse–Witt lifting removes the need to obtain N2 by enumeration if the following proof certificate succeeds. It does not remove construction, mapping, relations, descent or verification.

### Gaps
No Hasse–Witt coefficients were computed from the fixture during this source/algebra review. The residues used below are the values implied by the recorded L-polynomial, so the p=127 result is conditional on an independently obtained determinant 1+T+84T²+46T³ modulo 127. No science execution, timing, implementation or independent replay was performed.

Sources checked (KB): revision 9141e386254340a9bdba3036f2bb706a, last_refresh 2026-11-14T21:44:48Z, lexical current, hybrid unavailable. Queries: “genus 3 Hasse Witt Weil polynomial recovery”; “Hasse-Witt matrices Harvey Sutherland”; “genus 3 Weil polynomial subgroup recovery”. Opened source:d6ba1459be09a038e4bb059706a89914, crypto-autoresearcher:knowledge/literature/KN-LIT-3054.md, metadata and bounded context. It is an unverified literature pointer, so original papers were read instead. The index predates today's local results; those were read directly. No memory update. Researcher instructions were read first.

## What are the exact uniqueness conditions without F_p² enumeration?

### Takeaway
The conditions below are necessary and sufficient for uniqueness among integral reciprocal p-Weil polynomials consistent with the recovered residues, exact a1 and known ℓ-divisibility. They do not assert that every surviving Weil polynomial belongs to the specified curve. Since that curve's actual polynomial is among the survivors, exactly one survivor certifies recovery.

### Cited Findings
The elementary bounds are |a1|≤6√p, |a2|≤15p, |a3|≤20p^(3/2). Thus p>144 suffices for unique a1 from its residue; the precise condition is that its residue class intersects [−6√p,6√p] in exactly one integer. Shi §5.2 prints “12√p>p” in this argument; the required inequality for p>144 is 12√p<p. — [Primary paper](https://arxiv.org/html/2602.00965v1#S5.SS2)

### Inferences — proof and exact finite criterion
Assume p odd prime, squarefree f of degree 7, r_i the actual coefficient residues, exact a1, and a proved nonidentity ℓ-torsion divisor for prime ℓ≠p (hence ℓ|N). A supplied integer ℓ without a subgroup-order justification is insufficient.

For each integer b≡r2 (mod p) with |b|≤15p, define
  D=a1²−3(b−3p), s=2√p.
Reject D<0. Otherwise set
  u=(−a1−√D)/3, v=(−a1+√D)/3,
  g_b(x)=x³+a1 x²+(b−3p)x.
Reject unless −s≤u≤v≤s. Define the exact real bounds
  L_b=max(−2p a1−2√p(p+b), 2p a1−g_b(u)),
  U_b=min(−2p a1+2√p(p+b), 2p a1−g_b(v)).

Proof of the interval:
χ(X)=X³ h(X+p/X), with
  h(x)=x³+a1 x²+(b−3p)x+c−2p a1.
All six roots of χ have absolute value √p iff the three roots of h are real and lie in [−s,s], counting multiplicity. Indeed each root x of h gives X²−xX+p; its two roots have modulus √p exactly when x is real in that interval. For a monic cubic with critical points u≤v, all three roots lie in [−s,s] iff the critical points lie there and
  h(−s)≤0, h(u)≥0, h(v)≤0, h(s)≥0.
Monotonicity on the three intervals proves sufficiency; Rolle's theorem and the signs prove necessity. This remains valid for repeated roots, including D=0. Substituting h gives precisely L_b≤c≤U_b.

Now CRT uniquely specifies R_b modulo m=pℓ by
  R_b≡r3 (mod p),
  R_b≡−A−(p+1)b (mod ℓ).
Let l_b=ceil(L_b), q_b=floor(U_b), and
  kmin_b=ceil((l_b−R_b)/m),
  kmax_b=floor((q_b−R_b)/m).
The exact number of consistent polynomials is
  C=Σ_b max(0,kmax_b−kmin_b+1).
Recovery is unique iff C=1. C=0 falsifies the inputs/conventions/certificate; C>1 means this information alone does not certify uniqueness. Use exact algebraic comparisons for ceilings and endpoint signs. If a1 is ambiguous, sum over all admissible a1 lifts too. If ℓ=p, replace CRT by the compatible common-modulus congruence; no pℓ precision gain is available.

Useful sufficient conditions, not necessary conditions:
1. If an enclosing N interval [nlo,nhi] contains exactly one ℓ-multiple, namely ceil(nlo/ℓ)=floor(nhi/ℓ), then N is known exactly.
2. With N known, each b determines c=N−A−(p+1)b. Adjacent b lifts differ by p, so the associated c values differ by p(p+1). If their admissible c values all lie in a common interval of width strictly less than p(p+1), at most one b survives. The coarse interval yields the sufficient inequality 40p^(3/2)<p(p+1), but the exact cubic interval can establish uniqueness far earlier.
3. Without first fixing N, U_b−L_b<pℓ guarantees at most one c for that particular b. It does not imply uniqueness across different b values.

### Inferences — p=127 certificate
Take a1=1, ℓ=1028893 and expected residues r2=84, r3=46.
The r1=1 residue itself has just one lift in [−6√127,6√127], even though the uniform p>144 sufficient threshold does not apply.

Write h's roots x_i∈[−2√p,2√p]. Since Σx_i=−1,
  b=3p+(1−Σx_i²)/2.
On this cube slice the maximum Σx_i² is 8p+1, attained at (−2√p,2√p,−1) up to permutation, and the minimum is 1/3. The maximum follows because a convex function on the slice attains its maximum at a vertex, where two coordinates are endpoints; both cannot have the same sign for sum −1. Hence −127≤b≤381+1/3. The residue leaves exactly b=−43,84,211,338.

A=2064514. Since 20·127^(3/2)<28625, use the safe enclosing c bound |c|≤28625. The resulting order interval
  2019633≤N≤2141907
contains only the ℓ-multiple 2ℓ=2057786. Therefore
  c=−6728−128b,
giving respectively −1224,−17480,−33736,−49992.
The last two violate |c|≤28625. For (b,c)=(84,−17480),
  h(2√127)=−17226+422√127<0,
contradicting the required endpoint sign. For (−43,−1224),
  h(x)=x³+x²−424x−1478.
Its signs at (−20,−19), (−4,−3), (21,22) are respectively (−598,80), (170,−224), (−680,326). Thus it has three roots in those disjoint intervals, all inside [−2√127,2√127]. Exactly one polynomial survives.

Consequently
  a2=−43, a3=−1224,
  N=2048384+16130−5504−1224=2057786=2·1028893.
The implied counts are N1=129 and N2=16130−1−86=16043, agreeing with the recorded counts. These are hand algebra checks of supplied data, not new point counts.

### Gaps
This certificate needs the independently computed residues. It makes no universal uniqueness claim for other primes, curves or subgroups. A Weil polynomial need not be a Jacobian polynomial of the specified curve; uniqueness is sufficient here because the true curve polynomial is included. More than one survivor does not by itself prove that multiple curves realize them.

## What is the cost and what would falsify the proposed saving?

### Takeaway
For fixed genus, candidate filtering is inexpensive relative to many coefficient algorithms, but obtaining the nine matrix entries is real work. This is a source-recovery component opportunity; it does not establish the full 10× genus-2-or-higher objective.

### Cited Findings
Harvey–Sutherland II §1 gives a single-prime algorithm costing O(g² p M(log p)) bit operations, and Remark 2.1 discusses the g^O(1) p^(1/2+o(1)) Bostan–Gaudry–Schost approach. Theorem 1.1 is an all-primes bound for reductions of one integral curve; its average per-prime polylogarithmic cost is not a standalone cost for one arbitrary curve/prime. — [Primary paper](https://arxiv.org/html/1410.5222v1#S1) ; [Primary paper](https://arxiv.org/html/1410.5222v1#S2)

### Inferences
- A transparent but inefficient coefficient computation repeatedly multiplies by the degree-seven f for (p−1)/2 steps: O(p²) field operations and O(p) stored coefficients. Binary powering with fast polynomial multiplication gives O(M_poly(p) log p) field operations and O(p) polynomial storage. Neither makes the coefficient step free. The published linear-in-p fixed-genus method, or an appropriately implemented square-root method, is a better asymptotic candidate. Recurrences dividing by k fail directly at multiples of p; HS II Remark 2.2 addresses this issue.
- There are at most 31 b lifts in [−15p,15p]; the cubic bounds can be tested in constant algebraic degree per lift. Thus uniqueness counting with CRT and exact comparisons uses a bounded number of operations on O(log p+log ℓ)-bit quantities, apart from obtaining residues and validating public inputs/subgroup order. Emitting every survivor can cost more if ℓ is small; counting them does not require enumeration.
- Any N1 enumeration retained to fix a1 costs O(p) evaluations/character tests (and exponentiation cost if used), which must also be charged. The residue interval can avoid that enumeration only when it yields unique a1. No F_p² point enumeration is present in the certificate.
- Coldcost13 reports a 2234.13-fold candidate/native cost ratio and an approximately 22341.33-fold reduction needed for the full 10× gate. Even zero cost for source Weil recovery leaves the other measured phases. Phase medians are not additive medians, so 1780.974708−4.234041 is only illustrative arithmetic, not a measured residual median. The source-recovery optimization cannot alone supply the missing full-cost result. — [Primary local results](../../research/hec_coldcost13_20261115/RESULTS.md).

Falsifiers / next proof obligations, with no execution authorized here:
1. An independently obtained det(I−TW) differing from 1+T+84T²+46T³ modulo 127 invalidates the fixture certificate's residue premise.
2. A non-squarefree f, wrong model, wrong prime, wrong sign convention, or unproved ℓ-divisibility invalidates the candidate-inclusion argument.
3. Any second CRT/root-interval survivor falsifies uniqueness from these data; random annihilation checks alone do not repair a deterministic uniqueness claim.
4. Any failure of exact boundary/multiple-root handling falsifies the interval implementation. In particular use non-strict inequalities at repeated roots and exact algebraic rounding.
5. A charged coefficient-and-filter recovery cost at least as large as the replaced source-counting/recovery phase falsifies a component saving for that implementation and workload.
6. A later full-cost experiment below the frozen 10× threshold rejects the full performance hypothesis even if coefficient recovery improves. Construction, maps, relations, descent and verification remain in that test.

### Gaps
No replacement latency, memory measurement or speedup was measured. General ambiguous cases require additional certified information or established lifting machinery. The bounded screen supplies an exact p=127 conditional recovery certificate and a general uniqueness test; the full 10× genus-2-or-higher objective remains the acceptance criterion.
