#set page(paper: "a4", margin: 16mm, footer: context align(right)[#counter(page).display()])
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.5em)
#let s = json("../summary.json")
= The next fixed conductor candidate
18 November 2026 · ordinary binary curve · draft PR 4723

The published index *n=9551* gives a *#(s.conductor_bits)-bit* Frobenius
conductor, with #s.decimal_digits decimal digits, for
E: y²+xy=x³+1 over GF(2^9551).
Complete certificate: *accepted*. Fresh-process replay: *accepted*.
Changed-subject control: *rejected*.
This is arithmetic qualification; no isogeny crossing or timing was measured.
[pcc:unverified]

#image("sizes.svg", width: 100%)

== Exact conductor identity
The curve over F₂ has four points, hence Frobenius trace −1 and
π²+π+2=0. Fundamental discriminant −7 forces its geometric endomorphism
ring to be maximal. Base extension preserves that ring, so f(E)=1.

For U₀=0, U₁=1 and Uₙ=−Uₙ₋₁−2Uₙ₋₂, write πⁿ=aₙ+Uₙπ. Then

  tₙ=2aₙ−Uₙ,     tₙ²−4·2ⁿ=−7Uₙ².

Thus *f(Z[πⁿ])=|Uₙ|*. This is the Frobenius conductor, not the
endomorphism conductor of this same source. A different ordinary curve
and an explicit conductor-changing isogeny are still required.

#table(columns: (1fr,1.6fr), inset: 5pt, stroke: 0.4pt,
 [*Requested object*], [*Status*],
 [Prime conductor arithmetic], [Exact recurrence/matrix and C++ ring agreement; certificate/replay outcomes stated above],
 [Explicit lower endpoint and map], [Absent; no point-image evidence for this large prime],
 [Original minimum 80-bit crossing], [Original source/kernel retained; full quotient/non-kernel images remain open],
 [Paired scalar/ECDLP and IC times], [Not measured; existing paired planner component is separate],
 [World-record assertion], [Not made; fixed primary-source candidate, bounded review],
)

The complete integer, proof and exact source/executable identities are
retained with the raw attempt. All producer claims remain unsigned,
unverified and not_covered. [pcc:unverified]

#pagebreak()
= Qualification and provenance
The primary OEIS A101087 comment labels index 9551 and later entries
as probable primes. That label is an input lead, not certification.
The author's Numericana page gives the +1-leading recurrence; sign
conjugation yields the same absolute values as the trace −1 recurrence.
The earlier certified fixed index 7351 is retained unchanged.

== Independent arithmetic and negative control
PARI computes the recurrence and matrix power. A separately compiled
C++/GMP oracle uses binary powering in Z[π], multiplying

  (a,b)(c,d)=(ac−2bd,ad+bc−bd).

It checks the same full trace/conductor and discriminant identity, with
fixed n=4,7,571,7351 controls. The qualification compares exact native
output strings. Python only handles metadata equality and presentation.

A successful prime qualification requires all of: complete PARI certificate,
exact top-level subject, exit 0, empty stderr, explicit completion output,
a fresh GP process recomputing/accepting the subject, and rejection after
changing it. An interpreter error can leave a zero GP exit; that alone
is never accepted. Timeout or missing certificate remains incomplete.

== Execution boundary
One fixed published input was checked. Direct attempt A reached its
300-second cap without a certificate. Separately frozen staged attempt B
had a cumulative 600-second cap, saved four partial chains and produced
the full certificate. Both used one GP thread, seed 20261118 and a 512 MiB
stack. Oracle/replay caps were 30/60 seconds. This is local fixed-fixture
proof qualification, with no paid resource, search sweep or benchmark.
No certificate duration is used as an ECDLP or scalar-multiplication time.

The immutable README is the pre-call scope; RESULTS.md records the
outcome. The pre-check manifest binds sources and native executables;
post-check output establishes whether those bytes stayed unchanged.
A mathematical ECPP certificate and a same-host replay do not supply an
independent signed execution receipt or a compiled Lean theorem.

== Remaining original obligations
The original source still needs the complete quotient, non-kernel images,
endpoint conductor evidence and full construction/transport cost. Repeated
matched scalar/ECDLP measurements, wide-field IC and wall references
remain open. The paired scheduler's correctness checks do not supply them.
Cloud/image/budget admission and S3 publication remain separate blockers.
The configured store rejects access to the earlier wide-harness archive;
the existing draft PR and required gates are retained. [pcc:unverified]

Sources: OEIS A101087 (indexed primary entry; direct opens return 403),
Numericana sequence author, existing Sutherland ordinary-order derivation
and PARI primecert documentation. Exact URLs and access boundaries are
in sources.json. KB unavailable, wrapper exit 127.

Final claim: PCC-20261118-0282a8. This binds the complete certificate,
fresh replay, raw outcomes and immutable source/executable manifests.
The earlier direct timeout and all partial chains remain retained. [pcc:unverified]
