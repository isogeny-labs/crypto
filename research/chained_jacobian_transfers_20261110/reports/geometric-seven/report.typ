#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set par(justify:false)
= Complete geometric 7-torsion action
2026-11-12 · fixed F101 Jacobian chain · scientific standing unsigned

PCCe78426 records30 passing standalone tests,with zero failures,ignores or
filters. Formatting,strict Clippy and the old reference/legacy guards pass.
All four Rust sources and nine fixture hashes remain unchanged. The whole
22,566-byte raw stream is retained as standard XZ5,008 bytes,with raw and
archive digests and exact roundtrip. These are correctness checks.
[pcc:unverified]

#image("method.svg",width:100%)

== Exhaustive source populations
For each of T0,T1,T2,the existing stable7 edges generate49 checked points
in each elliptic factor. Their2,401 product pullbacks are valid,distinct,
killed by7 and fixed by Frobenius⁶. The independent pointwise norms in
the reduced quintic chart equal four times every elliptic-product input.
The branch-point base divisor,finite poles,multiplicities,ramification
and signed original infinities are retained. [pcc:unverified]

The admitted identity HᵢKᵢ=[4] makes Kᵢ injective on7-torsion. Milne's
prime-to-characteristic torsion theorem gives exactly7⁴=2,401 geometric
points on a genus2 Jacobian. Under the written Picard and group-law
dependencies,the checked distinct population therefore exhausts Aᵢ[7].
This is stronger than testing the previously selected49-class kernels.
[pcc:unverified]

#image("coverage.svg",width:100%)

All six first,second and compressed forward/adjoint directions agree on
every2,401 inputs with the direct elliptic-product route. Each restricted
map has kernel49,image49,and every image is killed by7. The evaluator
uses the existing field,polynomial,elliptic,Kohel and separate legacy Cantor
engines;it does not import the producer Balanced,trace,Richelot or pipeline
implementation. Shared arithmetic dependencies remain explicit.
[pcc:unverified]

#pagebreak()
= Chain orientation and degree controls
On the shared T1 source,the first adjoint kernel and second forward kernel
intersect only in the identity. The compressed forward7-kernel equals the
first forward kernel;the compressed reverse7-kernel equals the second
adjoint kernel. Backtracking through either single hop and its actual
adjoint kills all2,401 source classes. [pcc:unverified]

#table(columns:2,stroke:0.4pt,
 [Checked control],[Recorded outcome],
 [All six directions:kernel / image],[49 /49 on each2,401 source],
 [Shared T1 kernel intersection],[1 identity],
 [Single-hop forward then adjoint],[2,401 zeros for each hop],
 [CT38 scalar4,976,069],[All2,401 images become zero],
 [CT38 scalar1,244,018],[2,352 differences from the map per direction],
 [Deeper scalar8,708,120],[Identity on all7-torsion controls],
)

== Conditional full compressed kernel type
Admit the old global separable degree7⁴ and norm49 identities. The full
geometric compressed kernel L then has order7⁴ and is killed by49. This
census gives |L[7]|=49. Finite abelian classification writes
L=(Z/49)ʳ×(Z/7)ˢ,so2r+s=4 and r+s=2 give r=2,s=0. Thus the abstract
kernel is (Z/49)² under those dependencies. The census does not enumerate
the full49-torsion or the full kernel over F101⁴². The earlier cyclic49
transport proof remains prior work. [pcc:unverified]

== Why the shadow controls remain necessary
CT38's first scalar is0 modulo7;the second is−1. Their restricted actions
are rejected on every nonzero image. However8,708,120=1+7·101·12,317 acts
as identity on the rational group,tangent module and7-torsion. Even this
exhaustive finite census cannot replace a global degree proof.
[pcc:unverified]

SevenCensusControls.nonzero\_seven\_shadow\_rejected/a6ff17 and
deeper\_shadow\_retains\_finite\_controls/90cebe faithfully formalize these
module consequences with explicit annihilation hypotheses. Both are
individually bound and locally pass;gate03 checks two statements with zero
trusted native proofs. The written census is149 declarations across30
modules. Actual varieties,torsion realization,Picard maps,Cantor correctness
and finite-group classification remain partial or written dependencies.
Failed proof attempts01/1a2d21 and02/a77458 retain their complete sources,
builds and gates;failed sorryAx output is never counted. [pcc:unverified]

== Sources and current checkpoint
Written proof d89ae5 maps every conclusion to its dependencies. Milne,
Abelian Varieties,Theorem7.2/Remark7.3 supplies geometric torsion. External
CT41 atca083268 and primary Howe–Lauter §2 are pinned by f07968;they motivate
the next actual4-gluing test. KB tools/wrapper and Conductor are unavailable;
local accepted proofs and primary papers were used. No novelty is claimed.
[pcc:unverified]

Frozen root67b exactCI43205484930 succeeds25native/29standalone/147written
29modules/full57d865. Priorb351 exact43196578049 succeeds21/14428/full3b94c3.
Consumer1508/c712 exact43202427032 is still running at the scheduled
observation;its accepted local scope is25native/98reference plus five
integrations. The new30/14930 root needs its own checkpoint and exact CI.
Existing frozen figures keep their historical scopes;these figures show
the added geometric census. [pcc:unverified]

Next:full elliptic4-torsion,ramified pullbacks and actual quartic gluing
anti-isometry on the three Tᵢ. Benchmark admission still lacks a live shared
balance;the atlas vault is locked. Native correctness and proof work continues.
[pcc:unverified]
