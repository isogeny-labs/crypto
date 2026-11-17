#set page(paper: "a4", margin: (x: 18mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9.5pt)
#set par(justify: false, leading: 0.58em)
#set heading(numbering: none)
#show link: underline

= Literal certificate verification
Twist index calculus · bounded verification round · 2026-10-25

== One fixed system, two checked constructions
The precommitted degree/closure workload selects p37, seed 0, phi arm,
system 0. First executions, independent coefficient/root audits and
deterministic A/A checks pass for both constructions. This is one public
synthetic polynomial system; the three presentations of scalar 271 are
not independent secret samples.

#table(
  columns: (1.1fr, 1fr, 1fr, 1.3fr, 1.3fr), inset: 5pt,
  [Method], [Working degree], [Product degree / power], [Search multiplications], [Sparse-check multiplications],
  [raw16], [12], [12 / 2], [33,609,631], [15,211],
  [closure8], [7], [12 / 2], [916,980], [15,983],
)

All counts are recorded Fp37 modular operations. Search additions are
33,300,873 and 896,786; inversion calls are 1,562 and 177. Inversion
exponentiation products are included in multiplication counts. Both
certificates independently check q squared; their multiplier polynomials
need not be equal. The raw degree-4 through degree-8 prefix exactly matches
the old record.

#figure(image("figures/first-pair.svg", width: 100%), caption: [
  Recorded values from the two first records. Old F4 is a partial reference
  for the different task of finding all roots. No complete-cost ranking,
  admitted timing or fastest-tool claim follows.
])

== Accounting boundary
The old F4 record has 125,327 row-reduction multiplications for this system.
Its other work is unpriced. Even the closure search alone exceeds this
partial count. Neither search ratio nor sparse verification cost establishes
a complete filter or DLP improvement. Setup, hashes, allocation, memory,
journaling, full F4 work, calibration and matched strongest IC/rho remain
open. Callgrind is installed but no profile was executed in this round;
client instructions alone would not close these obligations.

#pagebreak()
= Exact identity and actual proof failures

== Independent native integer lift
The exporter binds the original public input, equation packet, complete
enumeration reference, passing prior audit, pair, peeled abscissa and raw8
prefix. It checks the retained finite root and the absence of an accepted
physical relation in this source system. Fresh dense integer arrays check
all 2,197 positions, independently of the constructor's sparse verifier.

For q = a³ − a²s₁ + as₂ − s₃, every coefficient has zero remainder in

#align(center)[∑ hᵢ fᵢ = q² + 37R.]

R has 454 nonzero coefficients. This integer lift, rather than a bare
mod-37 assertion treated as a characteristic-zero identity, is the
statement intended for the standard IDC checker. Statement SHA-256:
#text(size: 7.2pt)[e7dd6d7622171f18c1d80ea9c1d26d182b08dbf664c3a3046115eb6715e5eb45]

== Two attempts, both terminal
The native gates v1 and v2 each pass 13 tests: four instance controls and
nine unchanged IDC tests. Format, build and source seals pass. Both actual
exports pass and emit the same statement hash.

#table(
  columns: (1fr, 2.3fr, 1fr, 1.1fr), inset: 5pt,
  [Attempt], [Encoding], [Lean exit], [Maximum RSS],
  [v1], [Flat polynomial expressions], [1: memory panic], [3,775,120 KiB],
  [v2], [Balanced sums; four product lemmas], [1: memory panic], [3,772,400 KiB],
)

Each actual Lean phase retained one aggregate 1,200-second limit and an
8 GiB address-space limit. RSS differs from address space. Durations are
diagnostic on the shared host and are not admitted performance evidence.
No third proof attempt was launched for this report. Neither attempt
produced an accepted instance theorem, an axiom audit or an IDC. Failed
proof sources and resource receipts remain preserved.

== The theorem still owed
The literal integer identity must hold in an arbitrary commutative ring.
Characteristic 37 must then give the q² identity; explicit absence of zero
divisors and the unchanged generic cubic lemma must imply a = x, y or z
at every common zero. No certificate identity may be assumed. All ten
axiom sets must contain only propext, Classical.choice and Quot.sound.
No sorry, native evaluation, custom axiom or kernel bypass is accepted.
Lean is pinned to 4.34.1. lean4checker was not available.

The readable argument is conditional on the literal identity: common zeros
annihilate every hᵢfᵢ, hence q² = 0 in characteristic 37. Absence of zero
divisors gives q = 0. The cubic identity gives
(a−x)(a−y)(a−z) = 0, so one factor is zero. Formal compilation of the
concrete identity remains incomplete; the native exact check is not a
replacement for that proof.

#pagebreak()
= Scope, evidence and remaining work

== Relation finding and multiplicities
The current collector requires four distinct ordinary abscissae and one
disjoint image abscissa. It rejects cubic roots that repeat internally or
contain the peeled ordinary ID. The certificate targets rejection in that
family. A repeated factor might encode a signed multiset or lower-arity
relation after opposite signs cancel. Its folded row and contribution to
rank have not been determined by this proof. An expanded audit must cover
all 42 arms, preserve the old distinct set as a subset, combine
multiplicities before deduplication and compare phi with doubling.

== Original requirements retained
#table(
  columns: (1.4fr, 2.6fr), inset: 5pt,
  [Requirement], [Actual evidence / remaining gap],
  [189 + 21 original cases], [Complete; not repeated. No consistent twist-specific complete cold gain.],
  [42 distinct zero-fiber arms], [Complete first collection and independent audits. Factor-log rank remains the relevant output.],
  [12 degree/closure configurations], [Existing managed owner continues its full workload. Historical progress-v1 snapshot has 3 verified rows and 9 unverified/null rows.],
  [42-arm certificate panel], [Existing owner continues. Bounded failures are not nonexistence.],
  [85 target configurations], [Existing owner continues. No full recovery/cost conclusion is inferred here.],
  [Concrete instance formal proof], [Both bounded encodings failed from memory; original theorem remains open.],
  [Complete end-to-end comparison], [Unpriced work, matched strongest IC/rho, calibration and external comparison remain open.],
)

The historical snapshot includes system-1 raw16: no certificate for powers
1,2,3 through degree 16 after 142,494,025 search multiplications. It exits
2 as partial. Later live records do not change this frozen snapshot.
This is neither a completed panel nor a nonexistence proof.

== Reproduction and provenance
Study: research/twist_index_calculus_20261020/zero_fiber_collector/
certificate_instance_proof. See REPORT.md, PROTOCOL.md, native/src/main.rs,
run.sh, render.py and the retained metadata. Raw directories under the
parent runs/: certificate-instance-native-gate-v1/v2 and
certificate-instance-proof-v1/v2. The paired source records are under
degree-closure-probe-panel-v1/cases/p37-seed0-phi-l0-active-sys0-raw16
and the corresponding closure8 directory. Original F4 reference:
panel-v2/cases/p37-seed0/phi-l0-active/first/record.json.

Workload SHA-256:
#text(size: 7.2pt)[254581dfebfcb992e8ce0c6b7896cbd7cb402259b0a5b72e34fd8c711cc0c88a]
Candidate SHA-256:
#text(size: 7.2pt)[35d1890ef4aa04d3a7568dca950f85274097079a4813db64e8022025a57ba2ec]
Source freezes before exports: 379d5880d0 (v1), 7cc1b72419 (v2).
The generic Lean prefix is copied unchanged. No source in any live
collector was edited for this verification.

This report and its benchmark copy are unlisted. Canonical IC/rho graphs,
leaderboard and benchmark index retain their previous data because complete
price, reference and VALIDATION gates remain unmet. Literature/atlas matches
place the method in known Semaev and degree-closure work; no novelty or
atlas saturation is claimed. Tool documentation:
#link("https://valgrind.org/docs/manual/cl-manual.html")[official Callgrind manual].
