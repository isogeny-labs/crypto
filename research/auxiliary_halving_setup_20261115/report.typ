#set page(paper:"a4",margin:(x:17mm,y:16mm),numbering:"1")
#set text(font:"DejaVu Sans",size:9.5pt)
#set par(justify:false)
#align(center)[#text(size:18pt,weight:"bold")[Generated auxiliary setup by point halving]
#v(3pt)Fixed prime-field recovery and conditional setup accounting • 15 November 2026]
#v(7pt)
The point-only control constructs its auxiliary curve, order-eight subgroup,
trace-good coset and main base, then recovers scalar 11 for P=(4,3), Q=(18,1)
on E/F19:y²=x³+2. OpenSSL replays setup decisions, every query/relation/kernel
and the final answer. *PCC-20261115-3ff0b8: unverified.*

#image("pipeline.svg",width:100%)
#table(columns:(1fr,1.8fr),inset:5pt,stroke:0.4pt+rgb("b5c4ce"),
 table.header([*Fixed qualification001 control*],[*Recorded outcome*]),
 [Complete doubling preimages],[14,754 cases over every nonsingular short model and finite point, p=5,7,11,13,17,19; OpenSSL sets agree],
 [Generated setups],[Eight fields, p=19,23,31,43,59,83,127,251; complete setup transcripts agree],
 [Legacy F19 cold],[Summary and three traces byte-identical to cold006; OpenSSL passes],
 [Generated F19 cold],[70 setup trials; 8 attempts/calls; 4 misses; 4 relations; native rank 2; 1 kernel; scalar 11],
 [Per-arm exits],[All 25 zero; source2ca55a9c9f],
)
The candidate uses no auxiliary point count, main point census, supplied base
logs, known target scalar or norm assignment. Enumerated reference domains
stay in the separate correctness-control/replay paths.

#pagebreak()
#text(size:16pt,weight:"bold")[Recorded setup controls]
#v(6pt)
#image("setup-counts.svg",width:100%)

Each bar is one fixed-seed functional control for d=8. These fields lie below
the large-parameter range of the counting lemma. The plot describes the actual
trial counts; it supplies no probability estimate, interval, timing, fitted
exponent or growing-family validation.

The unfiltered F19 setup reaches A:y²=x³+12x+10 after nine trials. The cold
filter requires exactly three main abscissas and reaches A:y²=x³+2x+2,
T=(16,8), G=(11,14), terminal 15 after 70 trials. It has the same eight-root
fibre and main base as the prior preset, with a replayed generation history.
It does not exercise a new cold fibre family.

The collection stream has seed 5678; the legacy stream has seed 1234.
Their eight-versus-twelve oracle calls are different controls and do not
establish a cost gain. No scientific timing, statistical performance result,
external-tool speed claim or canonical leaderboard value changes.

Source formula: halving abscissas satisfy
#raw("u^4-4*xp*u^3-2*a*u^2-(8*b+4*a*xp)*u+a^2-4*b*xp=0",block:true)
Every quadratic lift is checked by actual doubling. Native record
IDC1h6e44d7c75baf9f8d passes 32 author plus 128 extra evaluations over
the field of prime order 2^61−1, using the unchanged certificate module. Its null proof pointer
provides no formal integer identity or program-correctness proof.

#pagebreak()
#text(size:16pt,weight:"bold")[Conditional setup accounting]
#v(6pt)
Assume p>3, d=2^t≥8, p≥256d^4. The existing primary-source count gives
at least p/2 noncuspidal rational points on the fine moduli curve Y₁(d).
Conrad Theorem 4.2.1 and Remark 4.2.2 ensure genuine pointed Fp objects and
trivial pointed automorphisms in this range.

Each pointed short-model orbit under Fp× has p−1 elements. Thus the number M
of actual tuples (a,b,T) with exact-order-d T is at least p(p−1)/2.
For each tuple, the initial (a,x([d/2]T)) occurs with probability 1/p².
The unique ancestral halving path has probability at least
4^(−(t−1))=4/d², because every rational doubling fibre has at most four points.

Distinct terminal tuples are disjoint outcomes. Therefore
#raw("Pr(success) ≥ M*4/(p²*d²) ≥ 2*(p−1)/(p*d²) ≥ 1/d²",block:true)
The expected number of abstract subgroup trials is at most d². Fixed-degree
Las Vegas root finding, failed paths and final order checks give expected
#raw("O(d²*t*poly(log p))",block:true)
subgroup work. For d=exp(O(sqrt(log p))), this setup contribution is
subexponential. *PCC-20261115-4b68b3: unverified hand derivation.*

Uniform auxiliary-point proposals have constant expected rejection cost here.
At most 4d points satisfy 2G in H, so Hasse bounds give a trace-good probability
at least 1−8d/p≥1/2. Tower construction and listing the d abscissas are charged
separately by polynomial work in d,t,log p.

The exact-three main-base filter and fixed native 4096/64/128 caps are absent
from this abstract probability argument. Native fixed seeds implement finite
controls; the model above uses independent uniform choices. The sufficient
relation-solving degree, useful fixed-base yield, general-field program and
complete later cost remain separate obligations. No full ECDLP or T3 result.

#pagebreak()
#text(size:16pt,weight:"bold")[Evidence, source and reproduction]
#v(6pt)
PROTOCOL.md precedes implementation and qualification. The candidate and
reference are native C++; identity issuance/checking reuses native Rust.
Source2ca55a9c9f and distinct build/pre-call manifests bind every call.
All current source, binary and script bytes match those manifests. No seed,
field, target, degree, resource cap or replay rule was retuned.

The whole qualification archive is 99,856 bytes. It retains all 25 arms,
14,754-case census, setup histories, identity record, native inputs/bases,
legacy comparison, complete cold trace and exits. SHA256 and exact normalized
unpack/repack identity pass. The separate complete parent Rust-gate archive is
97,024 bytes. Artifact-autofix run43717663512 uploads both whole objects and
replays their complete remote bytes. Manifest commit ba62e3ae93 retains their
SHA256/length pointers; both local hydrated checks report zero problems.
No archive is split. README.md gives build, extraction and hydration context.

All new C++ binaries use strict -Wall/-Wextra/-Wpedantic/-Werror. Build001's
missing optional include and build002's missing Rust hex dependency are retained,
then corrected before fresh build003. No qualification ran before successful
source/build binding. Execution wrappers are separately copied and frozen.

The parent's root Rust suite at source2fd16de7ca finishes nonpassing:
6,406 passed, 48 failed, 139 ignored. Its output reports omitted sparse data
and device sources in untouched tests; the focused device replay confirms its
missing source. The earlier partial snapshot remains unchanged. The existing
shared law/exact-instance local gates keep their earlier scope; no new generator
or native program proof is asserted, and no signed execution standing is promoted.

Sources: existing auxiliary/counting review; Conrad Theorem4.2.1/Remark4.2.2,
author PDF hash f53e4ff819d534926e6ee59874cbce4e7bad4a315cbd6d05984b8405b11f15d8;
official MIT18.783 group law and finite-field root finding; FLINT API; actual
native interfaces. KB unavailable. No novelty or exhaustive prior-art claim.
