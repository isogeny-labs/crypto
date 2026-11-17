# Geometric seven-kernel and cyclic-49 certificate proof

This completes stageC of EVEN_JACOBIAN_MAP_PROTOCOL_V2.md/6f1e93 with the
additional geometric-domain controls frozen in779dc2. Exact source code is
successor-preflight/src/even_kernel_certificates.rs. Selected19142f supplies
the diagnostic native data; full03/9f7c80 supplies complete38-test acceptance,
all625 unchanged hashes and byte-identical native JSON. All scientific
standing remains unsigned. [pcc:unverified]

## 1. Actual maps and the geometric domain

Let E0:y²=x³+x+46,E1:y²=x³+26x+58,E2:y²=x³+59x+27 over F101,with F0
the complementary y²=x³+44x+21 factor. Write alpha:E0→E1 and beta:E1→E2
for the existing normalized separable seven-isogenies with kernel
polynomials h1=[72,100,35,1] and h2=[99,94,75,1],respectively. The old
seven-reference constructor checks the exact denominator-cleared curve
identities,coprimality,psi7 divisibility and normalized dual identities;
these remain unchanged and are executed again by the new test.
[pcc:unverified]

Use C0=[56,0,13,0,6,0,1],C10=[89,0,93,0,8,0,61] and
C20=[40,0,45,0,75,0,18],coefficients ascending. Their actual optimal
degree-two cover pullbacks i_E,i_F and norms H=(N_E,N_F) satisfy
N_E i_E=[2],N_F i_F=[2],and both cross norms vanish. The written global
quotient proof128eb1 and its evaluator bridgeafd164 give

    [2] Phi7  = P10 (alpha,id) H0,
    [2] Phi49 = P20 (beta alpha,id) H0,

where Pj is the sum of the two actual target pullbacks. Phi7 and Phi49
are global separable isogenies of degrees7 and49. These statements do not
define a global division-by-two morphism. The quotient/Picard/degree
bridges remain written geometric mathematics,not axioms in the small Lean
modules. Bruin–Doerksen Lemmas2.4/2.6 provide the complementary-cover
identities used here. [pcc:unverified]

Since N_E i_E=[2],i_E is injective on every odd-primary torsion subgroup:
an element killed by both2 and an odd integer is zero. Thus i_E(E0[7])
has49 points,whereas the full geometric Jac(C0)[7] has2401. The native
test enumerates exactly this49-point first-factor slice,not the whole
Jacobian seven-torsion. It finds the24 roots of psi7 in the existing
F101^12 representation,both y signs at each root,and the origin. All
49 points are distinct,on E0 and killed by7;they exhaust E0[7] because
7 is prime to101. All2401 elliptic sums close in that exact set.
[pcc:unverified]

Every i_E(P) is computed as an actual reduced Mumford pullback in the
nonmonic-aware chart fromafd164. Direct divisor pushforward checks
H0(i_E(P))=(2P,O). The native Frobenius routine decodes support back to
the original sextic,applies coordinate Frobenius,and rebases the
Weierstrass point;it does not take coefficientwise Frobenius in a
nonrational chart. All49 source and target Frobenius commutators,and
all2401 source-pull and target-image sum comparisons,are checked.
[pcc:unverified]

## 2. Correct geometric half and the actual seven-kernel

On the seven-torsion slice,[4] is inverse to[2]. Hence the global cleared
relation determines Phi7(i_E(P)) uniquely as

    [4] P10(alpha(2P),O) = i_E1(alpha(P)).

The native test computes the left side from the input Mumford norm and
independently compares the right side using the actual target pull. The
two agree on all49 inputs and have exactly7 distinct outputs. There is
no rational-coordinate projection,corpus index or planted logarithm in
this evaluation. [pcc:unverified]

The rational half55 is wrong on this domain:55*[2] acts as[110]=[5]
on seven-torsion. Thus its output equals the correct one exactly when
[4] times the correct output is zero,equivalently the correct output is
zero. Alpha has a7-point kernel on the49-point E0[7] group and seven
fibres of size7. Therefore the wrong55 control agrees on the7 kernel
points and differs on all42 remaining inputs. These are preregistered
counts,observed directly;EvenKernels.geometric_half_coordinates/e1bd7f
and geometric_wrong_half_counts/f5bb27 certify the finite scalar and
count arithmetic. The actual finite-group identification is native/written.
[pcc:unverified]

The actual seven points killed by alpha are pulled into seven distinct
C0 Mumford classes. The data checks all49 sums,identity/nonidentity,
annihilation by7 and geometric Frobenius=[5]. Every nonidentity class has
exact order7 because7 is prime. The cleared formula kills them under
Phi7;their images have odd order and are killed by2,so their images are
zero. The global degree7 and separability then identify these7 points
with the entire geometric kernel of Phi7. This uses the global degree
proof as well as the native membership checks. [pcc:unverified]

## 3. The degree24 polynomial accounts for every composite-kernel point

Write the actual x-map of alpha as N/D with D=h1²,N and D coprime,and
degrees7 and6. With h2(T)=a+bT+cT²+eT³,e=1,put

    H(X)=aD³+bND²+cN²D+eN³,
    h49(X)=h1(X) H(X).

For D(X) nonzero,H(X)=D(X)³ h2(N(X)/D(X)). This exact denominator
clearing is the universal commutative-ring theorem
EvenKernels.cubic_homogenization/c9aa3b after N=xD substitution. Its
unchanged-checker record IDC1ha31d00e9a1842a62 checks32 issuer plus64
domain-separated points over the checker's fixed2^61-1 field;that IDC
does not by itself assert an F101 or all-characteristics theorem.
The Lean statement supplies the universal identity. [pcc:unverified]

When D(X)=0,h1(X)=0 and alpha sends both source points to infinity.
Coprimality gives N(X) nonzero,so H(X)=N(X)³ is nonzero at those roots;
there is no overlap or spurious denominator factor. When D(X) is
nonzero,H vanishes exactly when alpha sends the source point into the
nonzero beta-kernel. The old exact curve identity guarantees that the
actual rational isogeny values lie on E1. The origin is a separate
composite-kernel point. Thus h49 describes exactly the nonzero
geometric points of ker(beta alpha),up to the common ±x pairing.
[pcc:unverified]

The exported polynomial is

    [42,52,3,51,36,99,66,64,34,90,39,51,50,
     4,39,23,77,42,5,55,33,49,35,17,1].

Native checks establish monicity,degree24,squarefreeness and coprimality
with x³+x+46. Division of the full degree1200 psi49 has zero remainder;
the complete quotient is retained and multiplication reconstructs psi49.
Also gcd(h49,psi7)=h1. Galbraith corrected Chapter9 Definition9.8.5,
Lemma9.8.6 and Theorem9.8.7 justify the division-polynomial recurrence,
degree and torsion-root interpretation in characteristic101. The existing
division_poly implementation accepts composite49;the prime-only
kernel_polys routine is used only for7. [pcc:unverified]

The separable degree49 composition has49 geometric kernel points. The
gcd certificate identifies exactly six nonzero E0[7] points in this
kernel,plus the origin:its intersection with E0[7] has size7. An abelian
group of order49 is cyclic or is (Z/7)²;the latter would have49 points
in that intersection. Hence this actual composite kernel is cyclic.
Because i_E is injective on odd torsion,the49 classes i_E(ker(beta alpha))
are killed by Phi49 using its cleared relation and coprimality with2.
Global degree49/separability make them the entire Phi49 kernel. Thus
the Jacobian49-kernel is cyclic as well. The native program does not
enumerate these49 cyclic-kernel points over F101^42. [pcc:unverified]

## 4. Exact x-factor degrees and the full-point field boundary

The native repeated modular Frobenius certificate is

| d | gcd(h49,X^(101^d)-X) |
| --- | --- |
| 1 | 1 |
| 3 | h1 |
| 7 | 1 |
| 21 | h49 |

Every irreducible factor therefore has degree dividing21. The degree3
factor h1 has no degree1 factor and is irreducible. The remaining degree21
factor has no factors of degrees1,3 or7,the proper divisors of21,and is
irreducible. This establishes the exact3+21 x-coordinate factorization
without constructing a degree21 or42 point field. [pcc:unverified]

All coefficients and both isogenies lie in F101,so the cyclic49-kernel
is Frobenius stable. Frobenius acts as a unit scalar lambda modulo49;
its restriction to the order7 subgroup is5,as checked on the actual
kernel. The E0 Frobenius characteristic polynomial is T²+7T+101.
Consequently lambda is33:the exact finite implication is the accepted
CyclicFortyNine.first_unique_scalar,and first_exact_order gives order42.
The actual elliptic characteristic-polynomial and cyclic-kernel bridges
are the written inputs to those finite statements. [pcc:unverified]

For a primitive order49 point,its x coordinate is fixed precisely when
lambda^d is1 or-1. EvenKernels.first_primitive_x_period/7310b6 certifies
33^21=-1 modulo49 and excludes both signs for1≤d<21. Thus its x field
degree is21 and its full point field degree is42. The order7 eigenline
has analogous x-degree3/full-degree6;first_kernel_x_period/063ed6 checks
the finite5-power arithmetic. These agree with the independent native
polynomial gcd degrees,but are not F101^42 point enumeration evidence.
[pcc:unverified]

## 5. Reproduction,custody and remaining scope

The deterministic24,475-byte native JSON contains the actual extension
modulus,sextics/chart,elliptic models,h1/h2/N/D and second-map data,
h49/psi7/psi49/full quotient,Frobenius gcds,all49 E0[7] coordinates and
alpha-images,the7 kernel Mumford classes,all49 kernel sums and the
Frobenius permutation. It is arithmetic data,not a producer assertion
that an independent verifier must trust. Complete-suite custody requires
all625 input hashes unchanged and byte identity with selected output;
full03/9f7c80 passes both and retains129135 raw bytes as standardXZ11128,
with raw/archive hashes,exact roundtrip and complete process receipts.
Interrupted full02/eb1ebb lacks an original process-exit receipt and is
not promoted despite38-pass stdout;its whole data/late checks remain.
Algebra interrupted01/2cb5c1 is likewise separate;complete02/d7d12d
passes23 tests/eight outputs/all12 hashes,old7 outputs byte-identical.
[pcc:unverified]

Five new exactly bound local Lean passes are c9aa3b/e1bd7f/f5bb27/063ed6/
7310b6. Project census188 written declarations in37 modules;gate03 has
5 declarations and zero trusted native. Actual AV/Picard theories,
divisor/function-field identifications,division-polynomial evaluation and
cyclicity arguments remain written/native. Core local passes do not
confer an independent scientific signature. [pcc:unverified]

Sources checked first:the existing proofs128eb1/afd164 and native exact
seven/field/poly/Cantor implementations,Bruin–Doerksen0902.3480
Lemmas2.4/2.6,and Galbraith corrected Chapter9 §§9.8–9.9. KB MCP absent,
exact wrapper queries and theorem query failed because no executable is
available;the prior repository/primary-source fallback is recorded in
protocol779dc2. No novelty inference follows from that access gap.
Next:actual primitive-four/T20 Richelot targets and independent validation
of the external CT49 candidates,then admitted matched complete-cost
comparisons. This component reports no measured computational advantage.
[pcc:unverified]
