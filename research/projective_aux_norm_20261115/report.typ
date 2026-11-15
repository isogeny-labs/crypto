#set page(paper:"a4",margin:(x:17mm,y:16mm),numbering:"1")
#set text(font:"DejaVu Sans",size:9.4pt)
#set par(justify:false)
#align(center)[#text(size:18pt,weight:"bold")[Auxiliary projective norm recovery]
#v(3pt)A complete fixed prime-field control • 15 November 2026]
#v(7pt)
The point-only candidate recovers scalar 11 for P=(4,3),Q=(18,1) on
E/F19:y²=x³+2. It constructs the factor base from a validated auxiliary
coset, obtains checked norm decompositions, builds signed rows and recovers
the scalar through a kernel. OpenSSL independently replays all queries,
relations, kernel arithmetic and [11]P=Q.
*PCC-20261115-1f50be:unverified.*

#image("pipeline.svg",width:100%)
#table(columns:(1.1fr,1.9fr),inset:5pt,stroke:0.4pt+rgb("b5c4ce"),
 table.header([*Fixed control*],[*Outcome*]),
 [Auxiliary curve],[A/F19:y²=x³+2x+2; T=(16,11), order 8; G=(12,5)],
 [Tower],[Three Vélu steps; terminal 15;8 distinct rational abscissas],
 [Norm source],[14 variables, cubic; 12 sets match 4 models],
 [Graph projection],[8 variables, quartic; 6 equations cancel; all sets still match],
 [Cold recovery],[13 attempts; 12 solver calls; 4 relations; 1 kernel; scalar 11],
 [Default regression],[18 targets; 72 relations; 18 kernels; 54 identical trace files],
)

Here 8 divides neither p−1=18 nor p+1=20. The auxiliary construction is
existing PKM work. This component implements its retained projective encoding
and one complete software recovery; it is not a general fast ECDLP result.

#pagebreak()
#text(size:16pt,weight:"bold")[What is checked, and where caps remain]

The projective step retains `N=P²−xi*P*Q+tau*Q²`, `D=P*Q−xi*Q²` modulo
monic U. The shared CommRing law preserves a coprime pair for unit tau. At
finite terminal N=cD, existing Mathlib laws imply D is a unit. Separate
denominator-inverse witnesses are therefore redundant. The exact F19
AdjoinRoot instance applies the three public steps; source/native binding and
program correctness remain separate obligations.

#table(columns:(1.35fr,1.65fr),inset:5pt,stroke:0.4pt+rgb("b5c4ce"),
 table.header([*Source-certificate control*],[*Outcome*]),
 [Six small ideals + Kummer D4],[Complete point sets / quotient dimensions match],
 [Original 14-variable PC D4],[12 capped unknowns],
 [Early 14-variable PC D4],[12 capped unknowns],
 [Projected 8-variable PC D6],[12 capped unknowns],
 [Indexed 8-variable PC D6],[6 checked unit ideals;6 step-cap unknowns],
 [Positive auxiliary targets],[msolve witnesses all verified; no PC positive certificate],
 [Lean],[Shared 102-theorem gate and 2-theorem instance gate pass],
)

The indexed producer uses integer leading-column lookup, sparse pivots and a
dense temporary row, retaining the same source DAG and independent symbolic
checker. Six other cases hit the one-million reduction cap at about 2295
pivots and 576k entries. No degree lower bound follows. Product/update counts
are partial native counters, not a complete cost model or a timing result.

Every msolve-returned assignment lifts to all original equations, yields
distinct main-curve points in the actual tower fibre, and sums to the query.
The independent OpenSSL census checks all complete target sets. The cold
collector accepts checked point witnesses and verifies its final answer; its
correctness does not need an unverified ideal-provenance assertion by msolve.

#pagebreak()
#text(size:16pt,weight:"bold")[Reproduction, custody and next obligations]

All research computation is native C++17 with strict compiler warnings.
PROTOCOL.md freezes six rounds/amendments and unchanged per-backend 20 s
CPU/wall,2 GiB address, row/node/entry/step/proof/quotient limits. Later wrappers
run as separately hashed frozen copies. No old failure is deleted, retried
under the same protocol or relabelled as a mathematical result.

#table(columns:(1fr,1fr,1.2fr),inset:5pt,stroke:0.4pt+rgb("b5c4ce"),
 table.header([*Whole round*],[*Bytes*],[*Call source*]),
 [qualification001],[4672],[62c7042e9a],
 [loader002],[3312],[5db5ecf7f7],
 [early003],[41332],[3fe2c55987],
 [graph004],[5204],[14f92aedbe],
 [indexed005],[688992],[8f9ecfff9a],
 [cold006],[18388],[626a4e0f91],
)

Each deterministic archive retains all native inputs, bases, proofs, errors,
exits and manifests, with a SHA256 file. CI run43704942686 reports the indexed archive uploaded and its entire remote
bytes replayed; commit379576ac63 installs the manifest and removes tracked
raw data. No evidence is split. The other whole rounds fit the existing policy.
README.md gives build, hydration, extraction and hash-check commands.

Initial failures are retained: missing libneogb at process startup; a final
wrapper syntax error from editing its on-disk source while bash waited in a
child; native format warnings; Lean field/import and numeral-cast bridges.
Both original native arms and replays had completed before the wrapper trailer
failed. Loader repair and later frozen execution copies are explicit amendments.
Hosted shared-library CI performs zero steps due an account billing lock;
local gates pass with standard axioms and no trusted-native proofs.

The source validates a preset auxiliary A,T,G and computes the actual base.
Generated auxiliary search, large-prime arithmetic, sufficient solving degree,
useful fixed-base yield and a complete growing-family cost argument remain
open. The tiny controls do not satisfy the supporting asymptotic parameter
conditions. No statistical performance dataset, rho comparison or T3 result
is claimed. The complete subexponential prime-field goal remains active.

Next setup candidate: fixed-degree rational point halving with exact-order
and trace checks. SETUP-REVIEW.md records the source review and the separate
sampling-law/accounting obligations; no implementation or run has begun.

Sources: prior auxiliary-curve/PKM and selective-oracle notes, native Vélu
implementations, the preceding PC component, and actual Mathlib coprimality,
AdjoinRoot and ZMod laws. KB unavailable; no novelty or exhaustive survey claim.

The root Rust run at source2fd16de7ca finishes6406 passed,48 failed,139 ignored.
Its failures report omitted sparse data/device sources in untouched tests.
The immutable partial snapshot remains; the complete final log is retained
by the generated-setup continuation. No full root-suite pass is claimed.
