# Projective-law formal proof boundary

This directory pins and builds a Lean packet for algebraic obligations used by
the weighted genus-2 projective-law implementation.  It is a proof-boundary
artifact.  It does not claim a complete formal proof of Jacobian addition.

The [simple-branch zero-order packet](BRANCH_ZERO_ORDER.md) proves that an
actual local numerator of the form `N=(x-a)t` has `ord(N-By)=1` at a rational
simple branch when `B` is a unit. Its [diagram](branch-zero-order.svg),
[gate table](branch-zero-order.csv), and [PDF](BRANCH_ZERO_ORDER.pdf) keep the
explicit factorization hypothesis and the still-open global `Pic⁰` law visible.

The [finite-branch weighted-Proj stalk packet](FINITE_BRANCH_STALK.md) places
a rational branch point in the actual `Z` basic open and proves its
structure-sheaf stalk is isomorphic to the affine branch localization. Its
[diagram](finite-branch-stalk.svg), [CSV](finite-branch-stalk.csv), and
[PDF](FINITE_BRANCH_STALK.pdf) separate this local-ring transport from the
still-open global divisor-class and compiled-law obligations.

The [simple finite-branch DVR packet](FINITE_BRANCH_DVR.md) uses that stalk
isomorphism to prove the actual projective stalk is a DVR and that pulled-back
`y` and `x-a` have local orders one and two when `a` is a rational simple
root. Its [diagram](finite-branch-dvr.svg), [CSV](finite-branch-dvr.csv), and
[PDF](FINITE_BRANCH_DVR.pdf) retain the open global divisor, `Pic⁰`, and
compiled-law gates.

The [actual projective branch zero-order packet](PROJECTIVE_BRANCH_ZERO_ORDER.md)
then proves `ord(N-B*y)=1` in that projective stalk under the explicit local
hypotheses `N=(x-a)t` and unit `B`. Its [diagram](projective-branch-zero-order.svg),
[CSV](projective-branch-zero-order.csv), and [PDF](PROJECTIVE_BRANCH_ZERO_ORDER.pdf)
also record the checked composition from a supplied polynomial norm identity
and the proof-side common 2+2 source coefficient model to that local order;
the actual weighted-Proj quintic's evaluation and normalized head are now
checked in the same chain. They keep compiled-law identification and the global
divisor/Pic⁰ equation open.

The [branch norm-factor packet](BRANCH_NORM_FACTOR.md) proves that a polynomial
norm identity at a rational branch root on its norm support forces `(X-a) ∣ n`,
then transports that factor through a checked polynomial evaluation map to
the actual weighted-Proj branch stalk.
Its [diagram](branch-norm-factor.svg), [CSV](branch-norm-factor.csv), and
[PDF](BRANCH_NORM_FACTOR.pdf) distinguish this algebraic factor from the still
open compiled-numerator and global-divisor bridges.

The [weighted-Proj head and chart packet](WEIGHTED_PROJ_HEAD_VERIFIED.md)
checks homogeneity of the proposed sextic and its principal ideal, plus the
two dehomogenized equations obtained at `Z=1` and `X=1`. Its [CSV](weighted-proj-head-verified.csv),
[graph](weighted-proj-head-verified.svg), and [PDF](weighted-proj-head-verified.pdf)
mark the quotient grading, chart isomorphisms, projective completion, and
`Pic⁰` transport as open.

The [weighted-Proj cover packet](WEIGHTED_PROJ_COVER.md) proves in Lean that
a prime containing the sextic equation and `Z` contains `Y`, and that adding
`X` forces containment of the coordinate ideal. A native identity certificate
independently checks the underlying sextic factorization; the [PDF](weighted-proj-cover.pdf),
[dependency graph](weighted-proj-cover.svg), and [CSV](weighted-proj-cover.csv)
keep the still-open graded-quotient and actual `Proj` cover steps visible.
The separate [ordinary-quotient packet](WEIGHTED_PROJ_QUOTIENT.md) proves a
surjective specialization map at `Z=1` and that `Z-1` lies in its kernel;
the [exact-kernel packet](WEIGHTED_PROJ_KERNEL.md) proves equality with that
principal ideal and an ordinary affine-algebra equivalence. Its [PDF](WEIGHTED_PROJ_KERNEL.pdf),
[graph](weighted-proj-kernel.svg), and [CSV](weighted-proj-kernel.csv) keep the
graded quotient and localized degree-zero `Proj` chart open.
The [componentwise quotient packet](weighted-proj-components/README.md)
checks finite weighted-homogeneous lifts and equality degree by degree in
pinned Lean. Its [PDF](weighted-proj-components/REPORT.pdf),
[graph](weighted-proj-components/componentwise-dependencies.svg), and
[CSV](weighted-proj-components/componentwise-status.csv) mark construction of
the quotient `GradedRing` and localized degree-zero chart as open.

## Pinned environment

- Lean `v4.34.1`, pinned by `lean-toolchain`.
- Mathlib commit `d13f23b723b8a846827a245b89c10fc7d3f11612`, pinned by
  `lakefile.toml` and `lake-manifest.json`.
- Runtime source and native certificate inputs are bound by SHA-256 in
  `source-bindings.json`.
- Four exact Rust function fragments are brace-scanned and bound separately in
  `runtime-bindings.json`; `verify.py` recomputes those fragment hashes.
- The [coefficient-slice packet](COEFFICIENT_SLICE.md) derives seven common
  2+2 coefficient expressions from the pinned Rust and Lean source, compares
  their expanded integer polynomials, and rejects operator mutations on both
  sides. Its [certificate](coefficient-slice.json), [CSV](coefficient-slice.csv),
  [graph](coefficient-slice.svg), and [PDF](COEFFICIENT_SLICE.pdf) are reviewable
  separately. This check covers only those seven expressions.
- The [cross-scaling packet](CROSS_SCALE22.md) compares the eight power
  intermediates, eight weighted common-branch input expressions, and seven
  composed coefficients with their Lean definitions. Its kernel-checked theorems prove the common
  `Z=z1*z2`, `W=w1*w2` weights when the weighted numerator equations hold,
  derive `B=Z⁸B_aff`, `A=Z⁹WA_aff`, `C=Z⁷WC_aff`, and preserve the nonzero
  `B,C` gates for nonzero weights.
  The packet includes a [certificate](cross-scale22.json),
  [CSV](cross-scale22.csv), [graph](cross-scale22.svg), and
  [PDF](CROSS_SCALE22.pdf) and a [coefficient-weight table](cross-scale22-weights.csv);
  it does not prove the returned Jacobian class.
- The [common residual packet](COMMON22_RESIDUAL.md) proves the exact cleared
  ordinate congruence in Lean and checks five source-to-theorem output
  obligations. Its [certificate](common22-residual.json),
  [summary CSV](common22-residual-summary.csv), [graph](common22-residual.svg),
  accompany a [PDF](COMMON22_RESIDUAL.pdf) and term-level CSV in the artifact
  store. Pull both from the repository root with:

  ```sh
  python3 tools/artifacts/artifacts.py pull research/projective_law_followup_20261022/formal
  ```
- The [residual-support head packet](COMMON22_SUPPORT_HEAD.md) proves the
  unit-weight common branch's `X⁶`, `X⁵`, and `X⁴` coefficients are `0`,
  `B²`, and `0`. Its source checker binds `B,A,C,Q,Rbar` to Rust and rejects
  both-side `Q` sign mutations. The [certificate](common22-support-head.json),
  [CSV](common22-support-head.csv), [graph](common22-support-head.svg), and
  artifact-stored [PDF](COMMON22_SUPPORT_HEAD.pdf) retain the bounded result.
- The [coprime-support uniqueness packet](COMMON22_UNIQUENESS.md) proves that
  nonzero source-style determinant `B` makes the two quadratic supports
  coprime by an explicit Bézout identity; the supports' degree-two and
  nonzero properties are also checked. Conditional residue lemmas derive
  the two support divisibilities from interpolation and valid-input curve
  congruences. Separate theorems derive the strict degree bound from a
  sextic upper bound with zero sixth coefficient, matching quintic heads,
  and nonzero quadratic supports; the bound then
  forces the cleared residual difference to vanish. The runtime interpolation
  link remains open. Four generic polynomial
  theorems now expand `N²-u₁u₂U` and read its degree-six, five, and four
  coefficients. Four source-style theorems substitute the proof-side
  coefficient formulas and recover `0,B²,0`; they also equate the canonical
  cubic with its interpolation form and prove its second-support
  factorization. The proof-side residual is at most
  sextic, and its difference from a monic quintic with zero fourth
  coefficient has degree below the support product. A further theorem
  combines these source-style ingredients into the cleared curve equation
  under valid-input assumptions. Runtime semantic correspondence remains
  open. The [CSV](common22-uniqueness.csv),
  [JSON](common22-uniqueness.json), [graph](common22-uniqueness.svg), and
  [PDF](COMMON22_UNIQUENESS.pdf) distinguish the kernel-checked implication
  from source and geometric premises still to establish.
- The [affine point-allocation packet](POINT_ALLOCATION.md) evaluates the
  source-style norm identity at curve points. Its Lean theorems isolate the
  overlap at `y=0`, prove that exactly one conjugate factor vanishes at an
  unramified norm root when `2,B≠0`, and show the interpolant and support
  degree drop when `C=0`. Zero-divisor multiplicities, infinity valuations,
  output Mumford reduction, and the `Pic⁰` equation remain open.
- The [infinity-chart packet](INFINITY_CHART.md) proves the normalized
  quintic chart equation after `z=1/x,w=y/x³` and the exact source-style
  cleared interpolant on that chart. It identifies the local-unit residue
  and the `C=0` linear `w` coefficient. A formal smooth local-ring/DVR
  construction and its valuation remain open.
- The [local-parameter packet](INFINITY_LOCAL_PARAMETER.md) proves the
  infinity-chart `z` partial derivative is `-1`, its `w=0` section has a
  simple root at `z=0`, and, conditional on a supplied local ring whose
  maximal ideal is `(z,w)`, the chart relation makes that ideal `(w)`.
  It does not construct the actual projective infinity stalk or DVR.
- The [conditional DVR valuation packet](LOCAL_VALUATION.md) proves
  `addVal(z)=2` and source-shape cleared-interpolant orders zero and one
  in an abstract DVR with the stated chart and unit hypotheses. The actual
  projective infinity stalk, rational-function pole orders, divisor and
  `Pic⁰` transport remain open.
- The [conditional fraction-field packet](FRACTION_POLE.md) proves in Lean
  that `w²=z*u`, `v(u)=1`, and `z³*g=G` imply `v(g)=v(w)⁻⁶` when `v(G)=1`,
  or `v(g)=v(w)⁻⁵` when `v(G)=v(w)`. It does not construct the projective
  infinity stalk or identify its valuation and uniformizer; that geometric
  specialization, principal divisor, and `Pic⁰` transport remain open.
- `branch-coverage.json` gives the standardized runtime-label, Lean
  constructor, certificate-schema mapping and lists the five rejected or
  exceptional labels separately.

Build from this directory:

```sh
MATHLIB_NO_CACHE_ON_UPDATE=1 lake update
lake build
```

For the dependent source-head file, build its two local imports before the
read-only kernel check:

```sh
lake env lean -o .lake/build/lib/lean/Common22SupportHead.olean Common22SupportHead.lean
lake env lean -o .lake/build/lib/lean/Common22PolynomialHead.olean Common22PolynomialHead.lean
lake env lean -o .lake/build/lib/lean/Common22Coprime.olean Common22Coprime.lean
lake env lean -o .lake/build/lib/lean/Common22Degree.olean Common22Degree.lean
lake env lean Common22SourceHead.lean
```

`MATHLIB_NO_CACHE_ON_UPDATE=1` avoids unpacking the complete Mathlib binary
cache.  It does not alter the pinned dependency graph or the proof checked by
Lean.  The successful build output is retained in `build.log`; the axiom-audit
lines are copied verbatim to `axiom-audit.log`.

## Kernel-checked scope

`ProjectiveLawBoundary.lean` proves:

1. equal squared ordinates over a field are equal or opposite;
2. the same-`u`, zero-slope-delta mismatch collapses to the inverse branch,
   with characteristic two explicitly excluded;
3. `B=0` and `dq=0` force equal quadratic coefficients, excluding the
   `shared_zero_q_delta` label for distinct inputs;
4. the cleared output-ordinate identity used by the generic 1+2 formula;
5. both exact factorization identities expressing that the generic 2+2 cubic
   interpolant agrees with the two input ordinates modulo `u1` and `u2`;
6. a total, manual mapping from the reachable runtime label inventory to the
   eight native certificate schemas or the definitional inverse case.

`RuntimeSemantics.lean` adds a manual executable abstraction of the decision
order in `add12_weighted_complete` and selected proof-side intermediates
associated with the generic 1+2 and common 2+2 calculations. Its five
kernel-checked theorems are conditional on their supplied hypotheses:

1. a valid equal-square ordinate pair cannot select the 1+2 mismatch result;
2. the selected 1+2 abstraction satisfies the proved cleared ordinate identity
   when its two defining intermediate equations are supplied;
3. the proof-side 2+2 polynomial witness has the stated exact remainders modulo
   both input support polynomials; and
4. through an explicit principal-divisor interface, a residual divisor has
   the inverse class of the input whenever their sum is principal.

The algebraic items are theorems over abstract fields or
commutative rings.  They are not finite-field samples.  The runtime mapping is
a manually transcribed finite inventory; `source-bindings.json` pins the Rust
source it was read from. The fragment bindings identify four exact function
bodies associated with the manual abstractions, but Lean does not parse or
compile Rust and no Rust-to-Lean derivation is proved. In particular, the 2+2
`N` in `generic22Runtime` is a proof-side congruence witness; the Rust function
does not expose that variable.

`AxiomAudit.lean` rejects dependencies beyond `propext`,
`Classical.choice`, and `Quot.sound`.  The actual audit found only subsets of
those accepted Lean/Mathlib foundations.  The mapping theorem and the explicit
bridge implication use no axioms.

`verify.py` also runs all four source-derived checkers in read-only mode; a stale
or altered certificate fails verification. It requires exactly four distinct Rust functions and their exact Lean
declaration sets, parses declarations after removing nested comments and
strings, and cross-checks the status count, schema, binding kind, and audited
theorem set. Every verifier run also confirms rejection of a duplicate row, a
hash-valid unexpected Rust function, an unexpected but real Lean declaration,
and a name present only in a Lean comment.

The additional five weighted 2+2 theorems and their published weight table are
checked by the native Rust binary `cargo run --release --bin
validate-cross-scale-weights` from the parent study directory. It checks the
Lean declaration and kernel receipts, exact CSV/JSON values, explicit open
runtime link and null cost change, and the report artifact inventory.

`WeightedSource22.lean` composes the cross-scaled B/A/C identities with the
ordered common-branch B/C guards. The native `verify-weighted-source22` binary
matches every Rust statement in the represented prefix and rejects source
mutations. Its [certificate](weighted-source22.json), [CSV](weighted-source22.csv),
[graph](weighted-source22.svg), and [PDF](WEIGHTED_SOURCE22.pdf) mark the exact
boundary. This source-shape check does not prove compiled Rust field semantics,
the output coordinate formulas, or the divisor-class map.

The separate [affine coordinate-ring packet](COORDINATE_RING_BRIDGE.md)
kernel-checks the source-style common 2+2 norm identity in
`K[X,Y]/(Y²-f(X))`. Its geometric zero/pole divisor and `Pic⁰` class
obligations remain open.

## Remaining theorem bridge

The packet does **not** define the smooth hyperelliptic curve, Mumford
reduction, concrete principal divisors, or `Pic^0`.  The structure
`DivisorClassBridge` makes the missing implication visible:

```text
literal formula identities
  + valid reduced Mumford inputs
  + smooth odd-characteristic quintic
  + branch composition and degree-drop arguments
  => returned representation is the sum in Pic^0
```

`formula_to_divisor_class` only applies a supplied bridge.  It does not
construct or assume one globally.  Therefore this packet strengthens the
mechanized algebraic boundary but does not close universal correctness of the
Rust implementation. `PrincipalClassInterface` and
`residual_represents_input` additionally prove the final abstract group step,
but construction of that interface for the intended smooth curve remains
open. The full remaining acceptance condition is tracked in
`problems/genus2-projective-transfer-backend.md`.

The standardized reviewer summary is available as [REPORT.md](REPORT.md),
with the same proof boundary rendered in [formal-status.svg](formal-status.svg)
and [REPORT.pdf](REPORT.pdf).

## Failed setup attempt retained

The first environment setup invoked a full `lake exe cache get`.  Extraction
failed with `No space left on device` while unpacking the complete Mathlib
cache.  No theorem was checked in that attempt.  Only that attempt's untracked
`.lake` directory was removed, and the build was restarted with Mathlib's
documented no-auto-cache environment setting and targeted imports.  The
failure and recovery are recorded here because they change how the successful
build was produced; they do not change its theorem scope.

# Explicit-chart local ring

The [explicit-chart packet](EXPLICIT_CHART.md) constructs the affine
infinity-chart quotient and its localization at the origin in pinned Lean.
It proves the origin maximal ideal is generated by the local coordinate
`w` in a Noetherian local ring. The quotient's integrality, DVR structure,
projective-stalk identification, divisor transport, and complete-law
correctness remain open. Its [CSV](explicit-chart.csv),
[SVG](explicit-chart.svg), and PDF retain the checked/open boundary.

The later [domain and DVR packet](DOMAIN_DVR.md) proves the chart quotient is
integral and its origin localization is a DVR with local uniformizer `w`.
The projective infinity-stalk identification, divisor and `Pic⁰` transport,
and independent Rust-law correspondence remain open. Its
[CSV](domain-dvr.csv), [JSON](domain-dvr.json), [SVG](domain-dvr.svg), and PDF
preserve the separate theorem and open-gate inventory.

The [affine-stalk packet](AFFINE_STALK.md) identifies the structure-sheaf
stalk of this explicit **affine** chart at its origin with that localization.
Its [CSV](affine-stalk.csv), [SVG](affine-stalk.svg), and artifact-stored PDF
record the theorem audit. This does not identify the stalk in a smooth
projective completion.

The [chart-overlap packet](CHART_OVERLAP.md) proves the affine and infinity
equations are equivalent on the nonzero-coordinate overlap under
`z=1/x, w=y/x³`, together with four reciprocal-coordinate round trips.
Its [CSV](chart-overlap.csv), [SVG](chart-overlap.svg), and artifact-stored
PDF record checked and open gates. A principal-open scheme isomorphism,
two-chart gluing, smooth projective completion, infinity-stalk transport,
divisor/`Pic⁰` bridge, and Rust-law correspondence remain open.

The [principal-open equation packet](PRINCIPAL_OPEN_EQUATION.md) proves in
pinned Lean that, after adjoining an inverse for `x` in one ambient ring,
the transformed infinity equation is a unit multiple of the affine equation.
It checks vanishing equivalence, principal-ideal equality, a same-ambient
quotient-ring equivalence, and an instance in `Localization.Away X₀` of
`K[X,Y]`. Its [CSV](principal-open-equation.csv),
[SVG](principal-open-equation.svg), and artifact-stored PDF expose the
checked and open gates. That packet alone does not construct maps between
the **separately presented** chart localizations; the following packets
address that ring and scheme overlap.

The [separate-chart map packet](SEPARATE_CHART_MAP.md) defines the affine
quotient and both principal opens independently. Pinned Lean checks that each
defining equation maps to zero, constructs algebra maps in both directions
between the localized coordinate rings, and proves both compositions are
identities. This yields an algebra equivalence of the separately presented
principal-open coordinate rings. Its [CSV](separate-chart-map.csv),
[SVG](separate-chart-map.svg), and artifact-stored PDF record the result and
remaining gates. The following packet checks the corresponding
scheme-open identification; smooth projective gluing and infinity stalk,
divisor/`Pic⁰` transport, and Rust-law semantics remain open.

The [principal-open scheme packet](PRINCIPAL_OPEN_SPEC.md) applies that
coordinate-ring equivalence to `Spec`. Pinned Lean proves that both
localization maps are open immersions onto the expected basic opens and that
the spectra of the localized chart rings are isomorphic. Its
[CSV](principal-open-spec.csv), [SVG](principal-open-spec.svg), and
artifact-stored PDF record the result. The later gluing construction is
described below; identification with a smooth projective completion, its
infinity stalk, divisor/`Pic⁰` transport, and Rust-law semantics remain open.

The [gluing gate packet](GLUING_GATE.md) records the pinned Mathlib
`Scheme.GlueData` requirements after the checked overlap. Its
[CSV](gluing-gate.csv), [SVG](gluing-gate.svg), and artifact-stored PDF
describe the obligations at that stage. The subsequent
[two-chart construction](TWO_CHART_GLUE.md) checks the full datum and the
resulting glued scheme in Lean, with open chart maps that jointly cover it;
its [CSV](two-chart-glue.csv), [SVG](two-chart-glue.svg), and artifact-stored
PDF record the completed gate. Identifying this scheme with a chosen smooth
projective completion, its infinity stalk, divisor/`Pic⁰` transport, and
Rust-law semantics remain open. Neither packet changes measured cost ratios.

The [transition-cast packet](GLUE_CAST.md) additionally checks the two
off-diagonal overlap-object equalities and the affine-to-infinity transition
equality inside the constructed gluing datum, including the equality casts
to the separately presented overlap schemes. Its [CSV](glue-cast.csv),
[SVG](glue-cast.svg), and [PDF](GLUE_CAST.pdf) keep the checked and open gates
separate. It does not identify the glued scheme with a chosen smooth
projective completion or prove divisor/`Pic⁰` or Rust-law correspondence.

The [exhaustive small-field gate](../exhaustive_small_field/README.md)
checks all valid reduced-divisor pairs over the specified `F_7` and `F_11`
models against independent Cantor addition under two weight settings. Its
zero-mismatch finite result is recorded in a PDF, SVG, CSV, and source-bound
JSON; it is not a universal Rust-law or `Pic⁰` theorem.

The [explicit infinity-point packet](COMMON22_COMPLETION_BRIDGE.md) proves
in pinned Lean that the complement of `D(z)` in the infinity chart has
exactly one point, the origin ideal `(z,w)`, over any field and for arbitrary
coefficients. Its [four-row CSV](common22-completion-bridge.csv),
[graph](common22-completion-bridge.svg), and
[PDF](COMMON22_COMPLETION_BRIDGE.pdf) carry the compiler and axiom audit.
Projective completion, properness, smoothness, divisor/`Pic⁰` transport,
and Rust semantics remain separate obligations.
`WeightedOverlapScheme.lean` now checks that the Spec isomorphism induced by
the canonical weighted overlap-ring transition is the transition morphism
used by the separately glued two-chart scheme. This is an equality of
principal-open overlap morphisms under the chosen chart identifications;
identifying the glued scheme itself with weighted Proj and transporting
its local stalks remain open.
`WeightedOverlapEmbedding.lean` checks that both localized chart Spec maps
into weighted Proj factor through the canonical `D₊(XZ)` immersion.
`WeightedInfinityJacobian.lean` proves that `z` and the `z`-partial generate
the unit ideal in the explicit infinity chart for arbitrary coefficients.
The [weighted quotient proof packet](WEIGHTED_QUOTIENT_COMPONENTS.md)
includes these declarations, their [status CSV](weighted-quotient-components.csv),
[graph](weighted-quotient-components.svg), and
[PDF](WEIGHTED_QUOTIENT_COMPONENTS.pdf). Global scheme smoothness, the
isomorphism to the separately glued scheme, stalk transport, and `Pic⁰`
remain open.
The later [cross-chart inverse packet](CROSS_CHART_INVERSE.md) closes the
glued-scheme-to-weighted-Proj isomorphism gate with two kernel-checked
theorems. Its [gate CSV](cross-chart-inverse.csv),
[dependency diagram](cross-chart-inverse.svg), and
[PDF](CROSS_CHART_INVERSE.pdf) record the exact scope. The global divisor
and `Pic⁰` equation and the compiled Rust semantics bridge remain open;
the new proof adds no target measurement or cost ratio.
The [bounded weighted-Proj attempt](WEIGHTED_PROJ_HEAD.md) records the
specific quotient-grading and `D₊(Z)` chart-isomorphism obligations, available
pinned APIs, and three failed/timed-out compiler commands. Its
[standardized API-gap rows](weighted-proj-head-api-gap.csv) are an open
proof record; no additional theorem passed the compiler.
The later [weighted-head proof packet](WEIGHTED_PROJ_HEAD_VERIFIED.md) checks
in pinned Lean that the proposed sextic has weighted degree six and that its
principal ideal is homogeneous. Its compiler exited 0 with standard axioms;
the [PDF](weighted-proj-head-verified.pdf),
[graph](weighted-proj-head-verified.svg), and
[seven-row CSV](weighted-proj-head-verified.csv) show the two checked and five
open obligations. The graded quotient and chart isomorphism still have no
checked proof, and the earlier compiler/copy failures remain recorded.

The [source-bound F17 exceptional dispatch packet](../exceptional_source_bridge/README.md)
checks one valid 2-torsion divisor against the pinned author expressions,
compiled Rust dispatcher, and Cantor arithmetic. Both public dispatchers
return identity. A direct author doubling subroutine returns invalid zero
weights on this input, but the public inverse branch makes that path
unreachable here. The packet retains the initial failed assertion, corrected
replay, [PDF](../exceptional_source_bridge/REPORT.pdf),
[graph](../exceptional_source_bridge/GRAPH.svg), and
[CSV](../exceptional_source_bridge/ROWS.csv); it makes no cohort-cost or
universal-law claim.

The [public author-source parity packet](../public_source_parity/README.md)
extends the source-evaluated comparison to all 34,704 weighted ordered-pair
rows on the specified `F7` and `F11` fixtures. All author/Rust outputs and
Rust/Cantor controls matched; the checker recorded no invalid weights or
undefined divisions. Its [PDF](../public_source_parity/REPORT.pdf),
[graph](../public_source_parity/GRAPH.svg),
[summary CSV](../public_source_parity/ROWS.csv), branch matrix, and
[raw artifact manifest](../public_source_parity/.artifacts.json) preserve the
result. Magma was unavailable and was not executed; this remains a finite
source-translation result, not a universal law or revised cost ratio.

The [projective branch zero-order packet](PROJECTIVE_BRANCH_ZERO_ORDER.md)
proves the proof-side common 2+2 numerator has order one on the actual
weighted-Proj stalk at a rational simple branch under its Mumford congruence,
nonzero-determinant, and norm-support hypotheses. The separate
[nonbranch stalk packet](NONBRANCH_STALK_ORDER.md) proves the actual stalk
equivalence and that pulled-back `y` is a unit at a rational point with
`y ≠ 0`. Its [PDF](NONBRANCH_STALK_ORDER.pdf),
[dependency graph](nonbranch-stalk-order.svg), and
[standardized gate CSV](nonbranch-stalk-order.csv) distinguish four checked
items, one conditional norm-order theorem, and seven open gates. The
nonbranch domain/DVR construction, formula-specific norm transport, global
divisor/Pic⁰ equation, and compiled Rust semantics remain open. Neither
packet changes a target or cost row.

The [prime-factor domain packet](AFFINE_PRIME_FACTOR_DOMAIN.md) gives a
separate sufficient condition for the affine ring and its actual
weighted-Proj stalk to be domains: the quintic must have an explicitly
supplied prime factor occurring once. Its [PDF](AFFINE_PRIME_FACTOR_DOMAIN.pdf),
[dependency graph](affine-prime-factor-domain.svg), and
[gate CSV](affine-prime-factor-domain.csv) state that hypothesis and mark the
nonbranch DVR, norm transport, global Pic⁰ equation, and compiled semantics
open. The subsequent [separable-quintic packet](SEPARABLE_AFFINE_DOMAIN.md)
derives that factor witness from the actual monic quintic's separability and
thereby proves the affine ring, rational-point localization, and weighted-Proj
point stalk are domains. Its [PDF](SEPARABLE_AFFINE_DOMAIN.pdf),
[dependency graph](separable-affine-domain.svg), and
[gate CSV](separable-affine-domain.csv) state the checked hypotheses and
remaining DVR/Pic⁰/runtime gates. The later [nonbranch DVR packet](NONBRANCH_DVR.md)
proves that the actual weighted-Proj point stalk is a DVR when the quintic is
separable, `y²=f(a)`, and `2*y≠0`. Its [PDF](NONBRANCH_DVR.pdf),
[dependency graph](nonbranch-dvr.svg), and [gate CSV](nonbranch-dvr.csv)
retain formula-specific norm transport, the global Pic⁰ equation, and
compiled Rust semantics as open gates. The complementary
[derivative-point DVR packet](DERIVATIVE_POINT_DVR.md) proves the actual
stalk is a DVR under separability, `y²=f(a)`, and `f'(a)≠0`, including in
characteristic two. It identifies `y-y₀` as a local parameter and supplies
a [PDF](DERIVATIVE_POINT_DVR.pdf), [dependency graph](derivative-point-dvr.svg),
and [gate CSV](derivative-point-dvr.csv). A combined theorem and the norm,
Pic⁰, and compiled-runtime gates remain open. These packets do not change a
target or cost row.

The [nonbranch norm-transport packet](NONBRANCH_NORM_TRANSPORT.md) then maps
the source-style common 2+2 polynomial identity into the actual weighted-Proj
rational-point stalk under the stated Mumford congruences and nonzero
determinant. Its [PDF](NONBRANCH_NORM_TRANSPORT.pdf),
[dependency graph](nonbranch-norm-transport.svg), and
[gate CSV](nonbranch-norm-transport.csv) separate the checked identity from a
chosen-factor valuation theorem. The latter assumes a separable quintic,
`y²=f(a)`, `2*y≠0`, a nonzero coefficient, and a nonunit chosen factor.
Characteristic-two order allocation, global Pic⁰ transport, and compiled Rust
semantics remain open. This packet changes no target or cost row.

The [smooth-point DVR packet](SMOOTH_POINT_DVR.md) combines the two rational
point cases: `2*y≠0 ∨ f'(a)≠0` is equivalent to one of the explicit affine
partials of `y²-f(x)` being nonzero, and under the point equation and a
separable quintic the actual weighted-Proj point stalk is a DVR. Its
[PDF](SMOOTH_POINT_DVR.pdf), [dependency graph](smooth-point-dvr.svg), and
[gate CSV](smooth-point-dvr.csv) leave the scheme-theoretic `Smooth`
equivalence, global Pic⁰ transport, characteristic-two norm-order allocation,
and compiled Rust semantics open. No target or cost row changes.

The [global divisor/Pic⁰ gap inventory](GLOBAL_PIC0_GAP.md) identifies the
first missing construction after these local results: a function-field
element and its finite principal divisor on all closed points of the actual
curve. Its [PDF](GLOBAL_PIC0_GAP.pdf), [dependency graph](global-pic0-gap.svg),
and [gate CSV](global-pic0-gap.csv) distinguish nine previously checked
declarations from four open construction gates. It adds no theorem or cost row.

The [function-field gate packet](FUNCTION_FIELD_GATE.md) records the missing
whole-curve integrality and closed-point valuation bridge. Its exact F3
[diagnostic script](global-divisor-counterexample.py) checks valid common 2+2
data whose two quadratic supports have no F3-rational points. The
[PDF](FUNCTION_FIELD_GATE.pdf), [dependency graph](function-field-gate.svg),
and [gate CSV](function-field-gate.csv) mark the closed-point interpretation,
global divisor, and Pic⁰ result as open. No new Lean theorem or cost row is
claimed.

The [infinity-chart integrality gate](PROJECTIVE_INTEGRAL.md) checks `zbar≠0`
and supplies a [PDF](PROJECTIVE_INTEGRAL.pdf),
[graph](projective-integral.svg), and [CSV](projective-integral.csv). The
[actual X/Z overlap packet](PROJECTIVE_OVERLAP_NONEMPTY.md) then constructs a
point in both weighted-Proj basic opens, with its own
[PDF](PROJECTIVE_OVERLAP_NONEMPTY.pdf),
[graph](projective-overlap-nonempty.svg), and
[CSV](projective-overlap-nonempty.csv). Finally, the
[whole-curve integrality packet](PROJECTIVE_IS_INTEGRAL.md) transports the
two chart-domain results and uses the overlap and open cover to prove the
actual weighted-Proj scheme is integral under separability of its quintic.
Its [PDF](PROJECTIVE_IS_INTEGRAL.pdf),
[graph](projective-is-integral.svg), and [CSV](projective-is-integral.csv)
separate this checked geometric result from the still-open function-field
element, all-closed-point principal divisor, Pic⁰ equality, and compiled
runtime link. None of these packets changes a target or cost row.

The [source function-field packet](SOURCE_FUNCTION_FIELD.md) maps the actual
affine chart injectively into the integral curve's function field and defines
the source-shaped `N/B-y` expression there. It checks that `B ≠ 0` keeps the
denominator nonzero. Its [PDF](SOURCE_FUNCTION_FIELD.pdf),
[graph](source-function-field.svg), and [11-row CSV](source-function-field.csv)
record the hypotheses and open gates. All-closed-point orders, a finite
principal divisor, Pic⁰ equality, and compiled runtime correspondence remain
open; no target or cost row changes.

The [local Noetherianity packet](LOCAL_NOETHERIAN.md) proves that both actual
weighted-Proj chart rings are Noetherian and that their two-basic-open cover
makes the whole curve locally Noetherian over any field, without a separability
hypothesis. Its [PDF](LOCAL_NOETHERIAN.pdf),
[dependency graph](local-noetherian.svg), and
[10-row status CSV](local-noetherian.csv) distinguish this checked prerequisite
from the open coheight-one identification, all-closed-point source-function
orders, principal divisor, Pic⁰ equality, and compiled Rust correspondence.
No target or cost row changes.

The [source-order packet](SOURCE_ORDER.md) specializes Mathlib's
order-of-vanishing operation to the already embedded source `N/B-y` function
on the actual weighted-Proj curve. Pinned Lean checks the definition and the
two conditional API boundaries: the defined zero outside coheight one, and
agreement with the stalk valuation at coheight-one points. Its
[PDF](SOURCE_ORDER.pdf), [dependency graph](source-order.svg), and
[12-row gate CSV](source-order.csv) retain closed-point coheight,
nonvanishing, finite support, the infinity pole, the global principal divisor,
Pic⁰ equality, and compiled Rust correspondence as open. No target or cost
row changes.
