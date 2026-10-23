# Round18: canonical characters, explicit correspondences and boundary exclusions

Client date: 22 October 2026, America/Los_Angeles. Execution: 23 October UTC.
Scientific parent: 76e409c1e31ff5b4a51e901bcda7e44529b732a5.
This round continues all three requested tracks. Native arithmetic, failed
attempts, exact replay and relevant Lean proofs are retained. No IC timing,
rank gain or efficiency assessment is supplied.

The canonical theta characters can now be determined by symmetry under the
written pullback-class bridge. The chosen diagonal cover has a precise
rotation obstruction, and an explicit scalar refinement avoids it. All four
Q24 points and the next four Q26 points are excluded. The integral factor
map remains open after an evaluated deck correspondence and a complete
bounded search with independently refuted false positives.

## Explicit integral level-77 map: actual correspondence, bounded negative search

CM19 gives the diamond-2 action by re-normalizing the Tate curve at
2P11=(b,bc). With A=1+c, K=b+Ac, u=c³/K and s=K/c, the new parameters are
b'=bc/u³, c'=1+A/u, lambda'=(lambda+s)/u and
nu'=(nu+lambda*b+bc)/u³. All ten positive F32 incidence points are permuted;
the fifth iterate is the old diamond involution and the tenth is identity.
There are 10,240 exact coordinate-residual checks and 210 curve-point controls.
Lean proves the universal integer residual and its Tate-form consequence.
Under the written normalization/moduli bridge, this is an integral degree-one
Jacobian correspondence. It is not the missing factor inclusion.

CM20 binds the actual two source root sets to joint CM Frobenius/diamond
systems modulo 11. The signs still share (F,D)=(1,6) on rational 11-torsion.
The inverse conductor character gives the shared unramified Hecke system
1+epsilon_-7; all ten units, 100 nonzero coordinate pairs and 23 primes
through 97 pass the exact controls. This rules out scalar separation of
that common residual system. Arbitrary integral geometric maps remain open.

The target models are K_a: y²+xy=x³+a*x²+1, a=0,1, over F32 with
polynomial basis z⁵+z²+1 (native modulus 37). The intended source is
A5(K_a)/F2 and its rational prime-11 line; no factor image is evaluated.

CM21 exhausts 33,554,432 affine-linear x-functions per sign in
span_F32(1,b,c,lambda,nu), then solves for every affine-linear y. Positive
identity-map controls pass. The ten original affine points admit false fits:

| target a | x-valid linear systems | constant fits | nonconstant finite fits |
|---|---:|---:|---:|
|0|792646|43|90|
|1|805|21|12|

CM22 constructs eight new F1024 incidences from actual order-7 points over
F2^30, with independent order-11, QR-triple and incidence checks. Every one
of the 102 nonconstant fits fails with an explicit nonzero residue; all
constant controls survive. Lean independently recomputes all 102 residues by
binary polynomial convolution and long division. This excludes this exact
affine-linear ansatz under the written incidence bridge and native exhaustion.
Functions with denominators or higher degree remain untested. CM.md gives
the complete formulas, controls and map obligations.

## Canonical theta matching and the F2 descent obstruction

For a raw lift F_a and its old scalar c_a, inversion gives the constant
Delta_a=c_a²*F_a(-P)*F_a(P-a). Multiplication by its unique inverse square
root produces the unique symmetric lift. The actual old-to-canonical
character exponents are [35,0,50,23] in the checked 105th-root basis, on
both graphs 0 and 2. Three probes per generator agree; the corrected periods,
18 commutator probes and 12 standard Frobenius controls per graph pass.
Lean proves scalar/character uniqueness and the actual finite congruences.

The original corrected mixed bundle is symmetric, and G16's diagonal class
is its actual pullback. Rigidification at the origin identifies their
symmetric structures. The canonical pullback action is therefore the unique
symmetric action just computed. This matching uses written line-bundle and
Miller geometry; it does not require a guessed trivializing function.
That rational trivialization and explicit invariant sections remain absent.

Standard split Frobenius is not the actual trace-zero descent rotation.
G20 finds denominator 105 in B⁻¹RB. A modulo-81 annihilator certificate,
proved in Lean, excludes an integral lift on this selected diagonal cover.
It does not exclude descent of the quotient or the symmetric bundle.

G21 constructs C=105B⁻¹ as an integral CM map and verifies BC=CB=105I.
The composite scalar cover commutes with R. Its degree is
14,774,554,437,890,625; C has degree 148,899,515,625. Actual small-prime bases
give torsion Frobenius of order 168 and trace-zero Frobenius of order 840.
These are algebraic degrees and module orders, not execution costs. The
refined pulled-back polarization is mixed. Its action, coherent F2 invariant
basis, second 19²-kernel action and 243 target sections remain open.
THETA.md separates the finite certificates from the geometric argument.

## Saturation: Q24 and Q26 excluded, Q28 unclassified

Q25 finds the actual companion cube at node 64 or 63 and a quartic at
node 320. Their depth-2 source relations give a depth-6 separator, excluding
all four Q24 points for arbitrary ramification under the source bridge.
The possible companion-infinity interpretation fails for these complete
circuit points. Q26 retains every prior constraint and finds four new models,
each supported only on the companion square, in nine visits per graph.

Q27 binds the square/cube residual on all five x inputs. Its source depth
is 5, and it excludes all four Q26 points. Both universal residual and depth
implication are proved in Lean. Q28 then retains every prior constraint plus
the five radical square-zero consequences. In 51 visits per graph it finds
four complete nonzero models, all supported on [95,97,123]. Their genuine
closure status is unknown. Full h-saturated ideal equality and proper
polarized quotient embeddings remain open. SATURATION.md gives the exact
source bindings and distinguishes radical boundary consequences from ideal
membership of individual nodes.

## Validation, retained failures and next obligations

There are 94 new named Lean results across 11 modules. Every module builds;
the full transitive audit covers 651 theorems with zero failures and zero
trusted native results; VALIDATION.md gives the exact scope.
Root tests pass 4,487 with 119 ignored; native tests pass 118 with 1 ignored.
The exact replay compares 11 JSON outputs and 9 generated Lean modules.
Three new source-bound IDC records check universal identities at 128 extra
parsed points each. PROOF_MAP.md records what remains written or native.

The missing Rust import and the first Lean extensionality/under-specified
rewrite failure are retained. The first refinement output precedes stronger
CM block and pointwise composition controls; the first Hecke output precedes
explicit binding to the actual source root sets. The false finite fits are
retained alongside their counterexamples. No rejected attempt is promoted.

Next obligations: an integral factor correspondence with controlled poles,
a function-field identity and an evaluated 11-primary image; a mixed refined
theta action and coherent F2 invariant basis; a separator or full ramified
arc for Q28, followed by complete saturated ideals and proper equations.
Round14 ranks and divisors and Round15 norm-source-fiber/image gaps remain
frozen. The user retains the IC assessment.

The three new identities are IDC1h0c6e29317a135107 (Tate coordinate),
IDC1hee24ebe64bfee106 (cleared elliptic map), and IDC1hb0f2e139dc231b5a
(square/cube source). The native ledger pins 176 sources and 112 raw files.

Evidence: [diagram](routes.svg), [ledger](ledger.json), [proof scope](PROOF_MAP.md),
[integral-map notes](CM.md), [theta notes](THETA.md), [saturation](SATURATION.md),
[primary sources](SOURCES.md) and [exact replay](REPLAY.sh).
