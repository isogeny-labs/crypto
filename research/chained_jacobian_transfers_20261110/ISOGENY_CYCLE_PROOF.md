# Actual gluing cycles and the fixed F101 quotient catalog

The complete changed native suite passes33 tests, with no failures, ignores
or filters; oldguards, formatting, strictClippy and all621 input hashes pass.
Full75,828-byte output→standardXZ9,336, exact roundtrip/source/status are
PCC-20261112-70a16a. The targeted1/32filtered diagnostic367c83 is separate.
Native data give elliptic periods4/3, joint model period12, and actual
two-/four-gluing graph period36. Closed36 rational action on all12,317 oldT0
classes equals public CRT scalar12035, not a new unknown logarithm.
Scientific standing is unsigned. [pcc:unverified]

This proof identifies what those finite data imply using stated standard
geometric results. Its later catalog corollary was pursued after the accepted
cycle census; it is a written classification, not a native construction of120
surfaces or72 new hyperelliptic equations. The native census and all global
bridges retain their distinct scope.

## Sources and exact objects

Use the actual covers/Picard maps and embedded factors from
QUARTIC_GLUING_PROOF.md/933d35 and TWO_PRIMARY_QUOTIENT_PROOF.md/492496.
E0=(1,46),E1=(26,58),E2=(59,27),E3=(5,62) have order109,trace-7 and
j-invariants12,87,47,91. F0=(44,21),F1=(69,50),F2=(23,7) have order113,
trace-11 and j58,92,89. All equations are short Weierstrass over F101.
Their geometric CM fields are Q(sqrt(-355)) and Q(sqrt(-283)); cross Hom=0.
The native census tests every generic j/both quadratic twists and all nonzero
a or b models at j1728/0; no exceptional model has either required order.
[pcc:unverified]

Primary sources read before implementation: Galbraith, corrected Chapter25,
§25.3.4–9 (ordinary ideal action/cycles/eigenline orientation), §25.4
(endomorphism orders) and §25.6 (isogeny DLP reductions), and Chapter26
(pairing functoriality). Bruin–Doerksen arXiv0902.3480, Theorem1.1,
Lemma2.4/2.6, Definition2.7, Lemma3.1 and Theorem3.2 supply polarized gluing
and genus-two existence over the present field. Theorem1.1 assumes char(k)
does not divide6 and #k>5, both true here. Their Proposition2.8 assumes
characteristic0 and is not used. Downloaded PDF SHA256
ef02fb57d0274c897ea4ee4b3ddd4ef7d23f0155a03e04d1b273425294120102.

Howe–Lauter arXiv1202.6308 §2 supplies the gluing-exponent bound; Milne,
Abelian Varieties, I7/I8/I13 and IV2/IV4.2 supply torsion, isogeny degrees,
polarization/Tate compatibility and rational Torelli. Their cached primary
PDF hashes are recorded in the earlier933d35/f07968 proofs.
Fresh external CT43/44 at cryptanalysiscc5d5541 classify abstract mod4
intertwiners/anti-isometries and covariance; exact original Git blobs
a3081f47ce6b7e4c89ac1b0ae31eef7ecbde4bee and
2ea292129f6aecfe9678402fbc560b87e37669a8 are retained as
evidence/ct43-external-cc5d5541.md and ct44-external-cc5d5541.md.
Their abstract results are credited, not presented as new here; no external
Python/JS oracle was executed. KB MCP/wrapper and encrypted atlas are
unavailable, so no novelty/absence inference is made.

## 1. Actual oriented walks and quotient descent

On each factor choose the7-kernel with actual Frobenius eigenvalue5 for E
or6 for F. The unchanged constructor checks the full denominator-cleared
Kohel curve identity, division-polynomial inclusion, coprimality and both
dual compositions. Every repeated target model is normalized by a checked
u in F101: a_target=u^4 a_raw,b_target=u^6 b_raw; coordinates multiply by
u²,u³. This is a field-defined isomorphism, not merely equal j.

The E cycle is E0→E1→E2→E3→E0, with kernel cubics
[72,100,35,1],[99,94,75,1],[35,76,69,1],[52,69,39,1].
The last raw model(78,29) uses u13 to return to E0. The F cycle has length3.
The native transcript records every raw target, kernel, eigenvalue and scale.
Different eigenlines identify the dual direction; selecting one fixed
eigenvalue at every step supplies the nonbacktracking oriented ideal power.
[pcc:unverified]

Let B_n=E_(n mod4)×F_(n mod3). Extract Q0=Ker(K0) from all256 actual
elliptic4-product pullbacks into the old Cantor Jacobian A0=Jac(T0).
This has16 elements and bijective factor projections. Transport

    Q_(n+1)=(alpha_n×beta_n)(Q_n),
    Gamma_n=[2]Q_n=Q_n[2].

The native checker verifies all16 coordinates, Frobenius stability, all256
group sums and inverse factor Weil pairings at every step; Q1 and Q2 agree
with separately extracted actualT1/T2 kernels. Odd-degree7 maps are
isomorphisms on4-torsion. Weil functoriality raises each factor pairing to7,
so the transported product pairing stays1. Gamma_n is the corresponding
order4 graph on the factor2-torsion. [pcc:unverified]

Define A_n=B_n/Q_n and D_n=B_n/Gamma_n. These quotients exist over F101
because their finite étale kernels are Galois stable. By the universal
property, alpha_n×beta_n descends to F101 isogenies on both quotient families.
Since the source and target quotient degrees agree, each descended isogeny
has degree7²=49. The degree is coprime to109 and113, so both prime-order
DLP subgroups are preserved injectively, with the corresponding dual/public
recovery normalization. This supplies prime-field quotient transfers;
native coordinates are elliptic-product/graph presentations, not newly
evaluated Mumford maps on36 new sextic equations. [pcc:unverified]

## 2. Why factor return12 does not close gluing

The ordinary discriminants -355 and -283 are fundamental; hence every factor
has geometric endomorphism ring Z[pi] and automorphisms ±1. The oriented
four-hop E endomorphism has degree7^4=2401 and the three-hop F endomorphism
has degree7^3=343. Ordinary CM ideal theory identifies their kernels with
powers of the selected prime over7; they are cyclic and exclude scalar
endomorphisms with both7-adic components nontrivial.

For E, N(a+b*pi)=a²-7ab+101b² and
4N=(2a-7b)²+355b². At N2401 this bounds |b|≤5. Enumerating these11 integers
gives only ±49, ±(31+5*pi), ±(4+5*pi). The scalar cases have noncyclic
7-primary kernel. Evaluating on the selected eigenline5 excludes4+5*pi
and selects deltaE=±(31+5*pi). For F,
4N=(2a-11b)²+283b² bounds |b|≤2 at N343; the only solutions are
±(22+pi), ±(-11+pi). Eigenvalue6 selects deltaF=±(22+pi).
These are global identifications using End/degree/ideal theory; finite point
agreement by itself would not establish them. [pcc:unverified]

In a companion basis of geometric4-torsion, both Frobenius matrices are

    F=[[0,3],[1,1]], F²-F+I=0, F³=-I.

Such a basis exists because X²+X+1 is irreducible modulo2: any nonzero
primitive vector and its Frobenius image form a basis modulo2 and therefore
modulo4. The actual geometric module identification remains a written bridge
to the finite Lean model. Native actual-coordinate checks corroborate it.

Modulo4, deltaE is ±(F-I), whose cube is I up to sign. DeltaF is ±(F+2I),
whose fourth power is -F and twelfth power is I. The exact integer identities

    (x-1)^3-1=(x²-x+1)(x-2),
    (x+2)^4+x=(x²-x+1)(x²+9x+32)+4(14x-4)

have universal core Lean proofs and native IDC records below. Thus after12
paired hops the relative gluing action is F up to sign; after24 it is F²
up to sign. Neither is ±I. Surjectivity of the kernel projection implies
that a noncentral target action cannot fix the graph up to relative sign.
After36 hops it closes. Reducing modulo2 gives the same exact order3
monodromy, so Gamma also first closes at36. The distinct j cycles imply
that any earlier equality must differ by a multiple of12; this excludes
every earlier possibility. Native checks compare every pair of the36
states and independently confirm both graph periods. [pcc:unverified]

## 3. Actual finite graph census

For each of the12 factor pairs, the native basis-image enumeration checks
every candidate4-torsion isomorphism and retains exactly12 commuting with
Frobenius. Six reverse Weil pairings, with three orbits under relative sign;
the other six give three non-anti sign orbits. The36 walked Q_n exhaust the
anti sign orbits once each. Their reductions give all three2-graphs per pair:
the centralizer of irreducible order3 Frobenius in GL2(F2) has order3, so no
additional Galois2-isomorphism is omitted. The36 Gamma_n exhaust these too.
[pcc:unverified]

Every factor's16-element4-group has15 additive subgroups and exactly three
Frobenius-stable subgroups, of sizes1,4,16. Native checks the2^16 subset
universe using a full addition table; subsets omitting the identity are
excluded before the closure loop. Algebraically Z4[F] is the unramified
quadratic chain ring: its only submodules in the rank-one module are
0,2R,R. The native complete-coordinate result and this standard ring
argument have the same scope. [pcc:unverified]

## 4. Catalog corollary:120 AV classes,84 principal,72 Jacobians

This is a written global consequence of the accepted finite inputs and the
following standard bridges, not a second native AV-isomorphism algorithm.
For any A in the F101 isogeny class of E0×F0, its two distinct rational
isotypic components give unique intrinsic embedded elliptic curves E_A,F_A.
Their canonical sum K:E_A×F_A→A is an isogeny, and its kernel Delta projects
injectively to both factors: each factor inclusion has trivial kernel.
Thus Delta is a graph of an isomorphism between finite Galois-stable
subgroup schemes of those elliptic curves. The factor curves occur in the
complete four/three-class census. Cross geometric Hom=0 guarantees uniqueness
of the factors and prevents any interchange between the two CM types.

Howe–Lauter's gluing exponent divides the resultant of the real Weil
polynomials S+7 and S+11, namely4. Equivalently Frobenius/Verschiebung
separates the factors by4, including characteristic101 as group schemes.
Consequently Delta is killed by4 and is étale. The actual subgroup census
therefore forces its projections to be0, full2-torsion, or full4-torsion.
For each of the12 factor pairs there is one product quotient, three2-graph
quotients and six4-graph sign-orbits. Conversely each stable graph supplies
such a quotient with the two canonical factor embeddings.

An isomorphism of two quotients restricts to actual isomorphisms of their
unique intrinsic elliptic factors. Their j values must agree; the remaining
factor automorphisms are only±1. These are exactly the relative signs
already divided out. Thus there are

    12*(1+3+6)=120

unpolarized F101 AV isomorphism classes, also geometrically distinct within
this list. No alternative ledger lineage or conjectural native isomorphism
test enters this count. [pcc:unverified]

To determine which admit any principal polarization, write |Delta|=n²,
n∈{1,2,4}. Since cross Hom=0, the pullback of a polarization along K is
diagonal(a*lambda_E,b*lambda_F), with a,b positive integers. If the
polarization is principal, its degree gives ab=n². Delta lies in the kernel
of the pullback, and each projection is injective, so n²≤a² and n²≤b²
as finite group-scheme lengths. Hence a≥n,b≥n and ab=n² force a=b=n.
This implication is formally proved in principal_pullback_forced; the
geometric degree/kernel-length inputs are written, not assumed as a
formalized abelian variety.

It follows that any principal polarization must pull back to n times the
product polarization. Bruin–Doerksen Lemma2.4 says it exists exactly when
Delta is isotropic, and it is unique since pullback under a surjective
isogeny is injective on Hom after cancellation. For n2 every graph is
anti-isometric; for n4 exactly the six anti-isometries/three sign-orbits
qualify. The other three4-graph orbits admit no principal polarization at
all, not merely no chosen inherited one. Thus84 of the120 are principal:
12 products,36 two-glued and36 four-glued;36 are not principally polarizable.
[pcc:unverified]

The36 nontrivial two-glued principally polarized surfaces are Jacobians
over F101 by Bruin–Doerksen Lemma3.1/Theorem3.2: the exceptional a=infinity
case would make the two elliptic factors geometric twists/isomorphic,
excluded here. The36 anti-four-glued surfaces are Jacobians over F101 by
Theorem1.1: all product/Weil-restriction exceptions require equal, twisted,
or3-isogenous factors, also excluded by cross geometric Hom=0. Rational
Torelli identifies a unique genus-two curve for each principal Jacobian.
Therefore exactly72 classes in this fixed class are genus-two Jacobians.
The native walk supplies their quotient presentations, not new curve
equations or full Mumford-coordinate arithmetic on every one. [pcc:unverified]

## 5. Closed rational action and scope

The actual normalization gives E four-cycle -(31+5*pi) and F three-cycle
22+pi. Native compares all16 geometric4-points and all109/113 rational
points, not only a generator. On rational points pi=1, so these cycle
scalars are73 modulo109 and23 modulo113. After36 paired hops they are45
and56. The final graph equals Q0 after negating the F factor, giving45/57
and the public CRT scalar12035 modulo12317. The separate outer route
K0*factor-walk*H0/4 agrees with [12035] on every oldT0 rational class;
scalar12036 differs on all12,316 nonidentity classes. [pcc:unverified]

Globally the oriented closed endomorphism is not integer multiplication:
each factor kernel is supported on one prime above7, whereas an integer
endomorphism has equal valuations on both. Its global kernel is of type
(Z/7^36)^2 and its degree is7^72, using the same CM/quotient bridges; those
huge kernels were not enumerated natively. Agreement with [12035] is scoped
to the rational group. This preserves the rational-shadow negative result
without identifying the global correspondence with scalar multiplication.
[pcc:unverified]

## Formal and certificate map

IsogenyCycles.lean has nine exactly bound local-pass statements; gate03
passes9/zero trusted native,169 written declarations across33 project modules.
The full compiled count remains a separate bulk-CI observation. [pcc:unverified]

| Declaration | Claim suffix | Scope |
| --- | --- | --- |
| quadratic_norm_multiplicative |3175ff| Universal quadratic CM norm polynomial |
| first_cycle_polynomial |8c6bed| Exact integer cycle reduction |
| second_cycle_polynomial |77c7ba| Exact integer cycle reduction with4-error term |
| cm_cycle_norms |c03a03| Exact2401/343 norms |
| exact_finite_monodromy |5e7c11| All16 vectors, no native evaluation |
| no_early_relative_sign |353d0a| 12/24 return actions are neither sign |
| noncentral_action_does_not_close_graph |243a61| Surjective graph projection blocks closure |
| public_closed_walk_crt |a91e55| Exact rational cycle powers/CRT |
| principal_pullback_forced |84fce9| Degree and both kernel-length bounds force equal diagonals |

Native algebra02/8d3d02 passes all23 tests and all10 source guards; old two
certificates replay byte-identically. The three new identity.certificate/v1
records are norm IDC1hcf1c09671f79d47d, first IDC1hdc15986d66227afa,
second IDC1h93a8cf047d0ece53. Each uses32 issuer+64 domain-separated checker
points over2^61-1 and the exact final Lean03 source digest; they are finite
evaluated records, not universal or signed proofs. Initial formal586ae2,
stylecf3e19 and algebra5f671c failures stay immutable. [pcc:unverified]

The actual varieties, complete global Hom/CM classification, scheme quotients,
polarization descent, Torelli and the formal-to-actual module bridge are not
implemented as Lean varieties here. The written proof retains those standard
dependencies explicitly. The feasible polynomial, finite and structural
implications are formalized without adding research axioms or native checks.

Next executable construction: unbalanced(7,1) on two-gluing and(49,1) on
four-gluing, with actual new models/maps and whole rational/exceptional
controls. CT44 already supplies the(7,1)four-glue polarization obstruction;
it is a negative control, not an untried claim. Full cold solver comparisons
still require live budget/S3 admission; this correctness/catalog result is
not timing evidence. [pcc:unverified]
