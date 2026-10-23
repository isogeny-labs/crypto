# Round17: integral map existence, odd theta lifts and survivor exclusions

Client date22 October2026, America/Los_Angeles; execution23 October UTC.
Scientific parent dcc9802e717465708a3e69562f4498bd7046def4. The base merge
preserves all earlier evidence and both problem-index entries. This round
continues the three requested tracks with exact native experiments and
relevant Lean proofs. No IC timing, rank gain or efficiency verdict is supplied.

The main new lead is a written prime-to11 injection existence argument for
the level77 carrier, alongside explicit integral lattice controls. The odd
cover now has actual kernel generators and factored normalized theta lifts.
All four Q22 survivors are excluded by source-bound quartics; four new Q24
survivors keep full saturation open. Canonical character matching, target
section extraction and an evaluated modular correspondence remain open.

## Integral level77 map: what is established

CM18 checks that each actual degree8 n=5 source Frobenius factor has eight
simple roots modulo11. The trace-zero augmentation matrix has a cyclic
basis of determinant-64 for both signs, giving an isomorphism with its
companion lattice at11. An integral16-by8 intertwiner into the cyclic
mixed-sign lattice preserves the fixed11 line. The natural reverse map
kills that line. Generated Lean proves the actual intertwiner, fixed source,
nonzero image, reverse cancellation, root counts and derivative checks.

Two integral controls with the same Frobenius polynomial and det(F-1)=121
have different11-primary cokernels: the split control gives(11,11), the
cyclic glued control gives(121). Their fixed11 dimensions are2 and1.
These controls do not determine the actual modular Jacobian's integral lattice.

The individual squarefree mod11 source factor gives a further written
argument. For an abelian subvariety B isogenous to the source A, its
11-adic Tate lattice and A's decompose into eight rank-one Z_11 components
under the Hensel idempotents. They are isomorphic. Tate's integral Hom
theorem and approximation modulo11 therefore give an actual A→B isogeny
of degree prime to11. Composing B→J gives a map injective on geometric
11-torsion, whenever J contains the source isogeny factor. CM.md gives
our deduction and all hypotheses, using the primary proof in SOURCES.md.

Applied to the smooth compact level77 modular normalization, this uses the
frozen written modular/Jacobian bridge. It is an existence result, not an
evaluated correspondence or divisor map. The old denominator704 still
obstructs the simple coprimality projector; it does not obstruct every
integral subgroup-preserving map. Full Tate/abelian formalization remains
open; no axiom encoding that conclusion is introduced.

## Odd cover: kernel and actual theta lifts

G17 computes U*B*V=diag(1,1,3,105) in17 Smith iterations for the actual
odd cover, with explicit two-sided integral inverses for U,V. Thus its
split-product kernel is E[3]×E[105], of order99225. Four actual generators
have orders3,3,105,105. Independent3/5/7 torsion bases and primitive Weil
pairings verify completeness; B kills each generator. Diagonal Theta and
3Theta commutators are all trivial, with actual finite congruences proved
in Lean.

| trace model | Frobenius orders on3/5/7 torsion | common coordinate field |
|---|---|---|
|E0|8,24,42|F_(2^168)|
|E1|8,24,21|F_(2^168)|

The certified field modulus has low bits95. Combining these coordinates
with the F32 split-cover identification needs F_(2^840); that identification
and F2 compatibility remain unevaluated.

G18 constructs exact factored translation operators for all four kernel
generators on the diagonal bundle of degrees[9,63,315,855], including
the nontrivial Pic2 corrections. The exported functions use full Miller
addition chains and explicit lines through T and-a. Period scalars agree
at three independent tuples. Exact3/5/7 roots normalize each operator to
its required order; all roots are checked. All18 commutator probes per
graph pass. Independent point addition checks the correction-square identity
at48 coordinates per graph. A universal integer residual and its rational
characteristic2 consequence are proved in Lean.

Under the written Miller/divisor bridge, the constant period and commutator
ratios make these normalized operators a geometric kernel linearization.
They need not be the canonical character of the original mixed divisor.
That matching and F32/F2 descent are still open. The factored averaging
operator is an idempotent invariant projector; Lean proves the general
statement for any supplied representation of this odd order.

| descent stage | section dimension under written geometry |
|---|---:|
|diagonal cover|8704314675|
|after odd kernel|87723|
|after19² quotient|243|

The functions and projector descriptors do not materialize dense matrices
or extract an invariant basis. The final243 target sections and proper
embedding equations remain unconstructed. THETA.md records exactly which
linearization and divisor obligations have been checked.

## Saturation: the four requested survivors are excluded

Q23 normalizes actual source cones independently for inputs0,2,4,6.
Sixteen quartic separators have membership depth6 in h. The second-factor
separator rejects the unit cubic of each actual Q22 model, excluding all
four points for arbitrary ramification under the exported source semantics.
Generated Lean binds the actual units/gates and checks the scalar separators.

Q24 retains every original and prior repair constraint, adding only the
valid radical boundary consequence that all four ordinary cubic nodes
vanish. Four new models survive, each found in nine search visits:

| graphs | nonzero nodes |
|---|---|
|0,1|59,61,64,65,66|
|2,3|58,60,63,64,65|

They lie in the supersingular companion subcircuit and are not yet classified
as false or genuine closure points. Lean checks their complete finite prior
constraints and cubic zeros. Full saturated ideal equality and proper
quotient embeddings remain open. SATURATION.md gives the source bindings.

## Validation, failures and remaining work

There are69 new named Lean results. The full project build and transitive
axiom audit cover553 project theorems with0 failing and0 trusted;
VALIDATION.md and the raw gate receipts give the exact scope. Six
new JSON outputs and five generated Lean modules have exact replay checks;
three new source-bound IDC records cover the correction residual, CM norm
product and T translation. PROOF_MAP.md distinguishes kernel statements,
finite native checks and written global geometry.

The root release suite passes4487 tests with119 ignored; native tests
pass115 with1 ignored. Failed Rust type/serde builds, the stale-binary
invocation, and the Lean cast/denominator-clearing attempts remain in raw
logs. Earlier G17/G18 outputs are retained before stronger inverse and
square-identity controls. No failed run is promoted as evidence.

Next construction obligations: an explicit integral level77 correspondence
and its actual subgroup image; canonical mixed-divisor character matching,
F32/F2 compatibility and invariant section extraction; a companion separator
or full arc for the new Q24 models. Round14 ranks and signed divisors and
Round15 norm source-fiber gaps remain frozen. The user retains IC assessment.

New identity certificates: Pic2 correction IDC1h31bc950ebd40dee8; CM norm
IDC1h9a53998afa83f290; T translation IDC1h9e3d3e3bc4edfd8a. Their JSON files
and exact Lean source hashes sit beside this report. The native ledger
pins153 sources and96 raw files, including retained failed attempts.
