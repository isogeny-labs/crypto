# Proper C20→T20 correspondence and the degree196 chain

The complete native invocation41/6cecba accepts the proper map on all12317
rational C20 classes and the actual C0→C20→T20 chain. This note supplies
the written geometric argument and identifies the formal boundary. All
scientific standing remains unsigned. [pcc:unverified]

## Source, target and inherited global isogeny

Work over k=F101. The actual raw models are

    C20: y²=18x⁶+75x⁴+45x²+40,
    T20: Y²=76z⁶+62z⁵+52z⁴+63z³+69z²+3z+25.

PRIMITIVE_FOUR_T20_GEOMETRY_PROOF.md/b24be8 constructs K20=P20(Q20)
from the actual sixteen-point anti-four graph. It proves that the selected
quadratic splitting has common leading coefficient11, determinant95,
and normalized factors F_i,G_i satisfying productF_i=f_C and productG_i=f_T.
Its cubic kernel algebra has square discriminant class. Bruin–Doerksen
0902.3480 §4, especially Proposition4.3, consequently supplies the prime-field
separable polarized (2,2)-isogeny R:J_C→J_T, degree4 and kernelK20.
No twist is selected from point counts. Those hypotheses and the written
cover/Picard bridges are inherited explicitly, rather than proved by this
new finite computation. [pcc:unverified]

The normalized correspondence in that primary source is

    F1(x)G1(z)+F2(x)G2(z)=0,
    yY=F1(x)G1(z)(x−z).

The existing Lean RichelotAlgebra.fundamental_identity, using exactly the
derivative-first bracket convention, gives

    Σ F_i(x)G_i(z)=−(x−z)².

Its existing correspondence_target_cleared lemma proves the cleared
target equation. On y≠0 cancellation is legitimate; thus U,V below define
a degree-two target divisor on the dense open set. These are old universal
identities and old certificates, not new algebra issuer outputs. Cyclically
rotating paired F_i,G_i preserves orientation and the induced Picard map.
The inherited prime-field descent identifies that induced map even though
the individual factors and chosen Weierstrass charts are not over k.
[pcc:unverified]

## Proper divisor map and exceptional fibres

Fix source and target chart Weierstrass points W,W'. For a source point P,
write A(P) for the class of its completed degree-two target fibre minus2W'.
Then, on a reduced divisor class D=Σ_{i=1}^d P_i−dW, the map is

    R(D)=Σ A(P_i)−d A(W).

This is pullback to the proper correspondence followed by pushforward and
therefore respects linear equivalence and addition. The subtraction is
forced by the representative of the degree-zero divisor, including at d=1.
It is not optional merely because one selected chart point maps to infinity.
Moving W changes the formula's coordinates, not the Picard homomorphism.
The implementation rotates the paired arrays cyclically so W is on F1;
then A(W) is the class (G2,0), which is nonzero in these charts.
The omitted-basepoint mutant differs on every prescribed degree-one
control:122 in the source direction and120 in the reverse. [pcc:unverified]

For finite y≠0 use the full polynomials

    U(z)=F1(x)G1(z)+F2(x)G2(z),
    V(z)=F1(x)G1(z)(x−z)/y mod U.

If degU=2, retain the quotient algebra with multiplicity and apply the
existing sextic-to-odd Chart transform. No splitting of this target U is
needed. If U contains W', remove its single branch factor and retain the
remaining original point class; W' itself contributes zero. Native checks
reject a duplicated ramified chart root, rather than silently dropping it.
For repeated nonbranch source support, the support list retains each
occurrence; pushforward of2P is2A(P). Explicit actual Mumford doubles
exercise122/120 such inputs and their additivity/dual/Frobenius checks.
The diagnostic recorded zero repeated *target fibre* U values, so that
different target-fibre special case is not represented as observed coverage.
[pcc:unverified]

If degU=1, its finite root z contributes (z,V(z)) and the second point is
the signed original target infinity. Comparing the z³ coefficient in the
ordinate equation gives

    rho_target=−F1(x) leading(G1)/y,   Y/z³→rho_target.

The raw leading-coefficient square is checked. Keeping only the linear
finite part would lose one point of the completed fibre. The planned
leading-coefficient equation forces four signed inputs in each direction;
all have positive dual norm2 and commute with Frobenius. Constant or zero
U is not assigned an invented fibre: the implementation fails explicitly.
None occurs in the accepted complete rational domain or planned geometric
controls. This finite-domain outcome does not prove that no other algebraic
point could require another chart. [pcc:unverified]

For either signed source infinity, y/x³→rho_source, take the x-leading
terms of the same equations:

    U=leading(F1)G1+leading(F2)G2,
    V=leading(F1)G1/rho_source mod U.

Both resulting U have degree2 in the fixture; both infinity signs are checked
in each direction. At y=0, specialization in a local parameter gives
(G2,0) on F1 and (G1,0) on F2: use the first or second ordinate expression
respectively so its numerator vanishes quadratically in the local parameter.
At F3=0 the fundamental identity gives U=−(x−z)², and the two completed
points form a hyperelliptic conjugate pair over z=x. Its class relative
to2W' is zero; it is not a doubled point with an arbitrary sign. All six
branch points, every branch case, target chart corners and both signed
infinities are checked on the actual raw curves. [pcc:unverified]

## Positive adjoint, recovery and nonuniform chain norm

Interchanging source and target arrays in the ordinate equation replaces
x−z by z−x. Thus the raw swapped correspondence has the opposite sign
from the transpose. Its induced map must be negated to obtain the positive
Rosati adjoint R†. The maximal isotropic kernel and principal polarizations
give both global identities

    R†R=[2] on J_C,   RR†=[2] on J_T.

The native evaluator implements that negation after the proper reversed
basepoint correction. Both identities are checked on all12317 rational
classes and on all16 geometric two-torsion classes in each direction.
The latter checks do not by themselves prove the global identities;
their geometric proof is the inherited Richelot polarization argument.
[pcc:unverified]

Both rational Jacobians have order12317=109·113. Multiplication by2 is
invertible, with inverse6159. Hence

    D=[6159] R†(R(D)).

For a rational class, the wrong uncorrected transpose agrees with R† exactly
when[4]D=0. Since gcd(4,12317)=1, this is precisely D=0. Therefore the
whole-domain control has12316 disagreements and one equality. The new
T20Whole module proves the abstract recovery and sign lemmas under the
explicit hypothesis[12317]D=0; the native group order and actual Picard map
remain separate inputs. [pcc:unverified]

Let Φ49:J_C0→J_C20 be the already accepted two-step elliptic-seven-induced
Jacobian isogeny. Its global degree49 and adjoint norm diag(49,1) in the
source E0/F0 coordinates are established in128eb1/afd164/063773. Thus

    Ψ=R Φ49 has degree196,
    Ψ†Ψ=Φ49†[2]Φ49=diag(98,2).

Recovery can apply the existing nonuniform Φ49 recovery after R†, yielding
[2]D, and then multiply by6159. Equivalently the two source coordinates
use public inverses99 modulo109 and57 modulo113. This norm is not uniform
doubling on the source product. The accepted test evaluates actual maps
on all12317 C0 Mumford classes, obtains distinct target classes, checks
the actual adjoint against diag(98,2), and recovers every D. It checks all
109 and113 scalar images and exact orders for both the direct R map and
the composed Ψ. Evaluation uses physical Mumford-coordinate keys, never
source enumeration indices or known logarithm labels. [pcc:unverified]

The two geometric kernels have2-power and7-power order, respectively,
so the nonzero prime groups109 and113 meet neither. The k-defined
homomorphisms preserve their DLP scalars. This states the transport
property; complete correctness tests do not price map evaluation or an
ECDLP algorithm. [pcc:unverified]

## Prime-field domain and the geometric parity obstruction

Raw equations are over k, but their monic odd charts choose branch anchors
in k3 and k6. For a k-rational Jacobian class, its reduced chart coefficients
are in k6; quadratic support therefore fits k12. The generic support helper
retains multiplicities and applies actual geometric Frobenius with the
required changed-basepoint correction. All rational input and output classes
are individually checked Frobenius fixed. This is a coordinate consequence
of k-defined maps, not coefficient projection without rebasing.
[pcc:unverified]

The forced degree-drop x coordinates lie in k6 and y in k12. Frobenius^6
therefore fixes or negates each point and its class, since both anchors lie
in k6. The k-defined Richelot map preserves that relation. For reduced
Mumford coordinates, negation preserves u, so image u has coefficients
in k6; its quadratic roots fit k12. The implementation checks this
condition. Arbitrary k12 Jacobian classes can require k24 support and
are outside the stated evaluator domain. No such broader support claim
is inferred from the complete k-rational enumeration. [pcc:unverified]

The standard genus-two two-torsion description identifies its15 nonzero
classes with unordered pairs of the six Weierstrass points. With canonical
labels, source Frobenius is(012)(345), target Frobenius is(012345).
The cube is the identity on source branches and swaps three opposite
target pairs. Exactly the pairs{0,3},{1,4},{2,5} are fixed on the target.
Including zero gives

    #J_C[2](k3)=16,   #J_T[2](k3)=4.

These canonical permutation facts are the new finite Lean lemmas; the
identification with the actual root action is checked by native geometric
Frobenius on all16 classes. Each map has geometric two-torsion kernel4
and image4, with all256 group sums and all256 map additions checked,
and kernels equal the actual F_i/G_i matching classes.
[pcc:unverified]

An odd-degree k-isogeny would induce a Galois-equivariant isomorphism on
geometric two-torsion: its kernel meets J[2] trivially and both sides have
dimension4 over F2. It would preserve the Frobenius^3 fixed counts, which
these models do not share. Hence a direct odd-degree k-isogeny between
C20 and T20 is excluded. The constructed degree4 bridge satisfies this
constraint. This independently applies the external CT49 parity observation
at the actual C20/T20 scope; it does not invalidate the prior odd-degree
maps between C0 and the two-glue models. [pcc:unverified]

## Faithful formal boundary, evidence and reproducibility

T20Whole.lean has six exact local bound passes, gate03/0trusted:

| Declaration | Claim suffix | Exact scope |
|---|---|---|
| rational_half_recovery | d01c21 | abstract IntModule, explicit order hypothesis |
| wrong_transpose_only_identity | 2de44b | abstract IntModule, same hypothesis |
| recovery_and_chain_constants | 507bb7 | explicit natural-number arithmetic |
| branch_permutation_cubes | f2b3ce | canonical Fin6 permutations |
| actual_target_fixed_pairs | 87bce9 | explicit15-pair filtered list |
| fixed_two_torsion_counts | b7ad54 | cardinalities of those explicit lists |

The project census is198 written declarations in39 modules;224 compiled
declarations is the next focused-CI expectation, not yet an own-CI observation.
No new axiom asserts a curve, correspondence, polarization, group order,
or Weierstrass identification. The two earlier failed proof builds45889b
and7fc8a6 retain their source/logs/zero audited status; compiler recovery
sorryAx was rejected. Additive clarification76b1b9 identifies the stray
placeholder in formal02 without editing its immutable claim.
[pcc:unverified]

Complete full04/6cecba:41 passes, zero failed/ignored/filtered, all old39
tests retained, old reference+legacy hashes/fmt/strictClippy pass, all627
native source hashes unchanged, and both prior JSON certificates byte-identical
(geometry6141 bytes, kernel24475). Whole155310-byte output is standardXZ12028,
with raw/archiveSHA256, exact roundtrip, three source snapshots, original
start/finish/exit, complete afterhashes and both comparisons in
evidence/t20-whole-full-04. Point-selected02/19c509 is a separate1/40
diagnostic with explicit repeated-input and map-addition controls.
[pcc:unverified]

Whole-selected03/f776e2 printed a passing test but lacks its original
outer exit/finish/afterhash receipts. It is preserved as incomplete,
not accepted. A late627-hash check is labeled late, not substituted for
the missing original receipt. An archive caller failed to stop after the
archive helper rejected that missing exit; the subsequent full04 was
already launched and then completed all original acceptance requirements.
No exit was reconstructed from stdout and no criterion was relaxed.
[pcc:unverified]

Sources checked before implementation: Bruin–Doerksen0902.3480 §4
(primary local PDF SHAef02fb57d0274c897ea4ee4b3ddd4ef7d23f0155a03e04d1b273425294120102),
the existing native Richelot exceptional-fibre/dual implementation,
RichelotAlgebra universal identities, b24be8/128eb1/afd164/063773,
protocola2c024 and prospective notesb0c5f6, and the external CT49 primary
note at cryptanalysisf7d4c6be340c6b94e316fa4c851a60121758211c.
KB unavailable: actual query receipts are in t20-primary-review-01 and
t20-whole-primary-01; repository/primary-source fallback was used.
No unsuccessful search is treated as novelty evidence. [pcc:unverified]
