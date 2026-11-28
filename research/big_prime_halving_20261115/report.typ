#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:false,leading:0.55em)
#text(size:18pt,weight:"bold")[Multiprecision auxiliary setup]
#v(5pt)
#text(size:12pt)[Native prototype, semantic identities, pending qualification]
#v(7pt)
The GMP/FLINT candidate and separate OpenSSL replayer build with strict C++17
warnings. Four generic ring/field curve identities pass the shared canonical
local proof gate. The native source is frozen and pushed; no numerical
qualification case has run. This is a component source/proof report, without
a relation oracle, DLP target, recovered scalar or performance result.
*PCC-20261115-261591 remains unverified.*

#image("pipeline.svg",width:67%)

For a finite point with abscissa xp, the candidate solves the cleared doubling
quartic, lists all rational ordinate lifts and checks actual doubles. A rational
abscissa alone is insufficient. It constructs a degree-d auxiliary tower and
checks the coset-root polynomial. The old word-prime code is unchanged.

The separate reference derives the quartic from the differentiation/doubling
equation. Its complete-root certificate is
#raw("product(X−root) = monic gcd(quartic, X^p−X)",block:true)
It uses its own BN polynomial arithmetic and OpenSSL group operations, without
including the candidate, GMP or FLINT. Native program correctness and prime
admission retain their separate trust boundaries.

#pagebreak()
#text(size:16pt,weight:"bold")[What has actually been checked]
#v(7pt)
Source f5b318b482 freezes the native code before numerical calls. Build001
failed for missing FLINT declarations and strict layout diagnostics; the full
log is preserved. Fresh build002 succeeds with -Wall/-Wextra/-Wpedantic/-Werror.
Source, binary, library/header and wrapper bindings are retained or specified
before launch. The copied wrapper is read-only and has its own pathname.

Shared crypto-lean source b38b6b4712 provides the cleared doubling quartic,
the exact degree-two curve residual, its kernel-root specialization and the
affine codomain equation with a nonzero denominator. Its local canonical gate
audits106theorems,0failures,0trusted-native exceptions. PR9 is merged; hosted
run43728954205 had zero executed steps because of the account billing lock.
No hosted pass is asserted. Additional shared source7364d8 proves the semantic
finite-field root certificate; its canonical local gate passes108theorems,
0failures and0trusted-native exceptions. No native program proof is asserted.

*Historical positive control.* Parent setup source2ca55a9c9f has a preserved
p19 receipt with A=(12,10), T=(1,17), G=(2,17), terminal15 and eight distinct
coset abscissas. This is earlier word-prime evidence, not a run of this port.
Parent admission controls rejected infinity, off-curve/noncanonical targets,
singular curves and a composite modulus. The new equivalents remain pending.

*Pending panel.* The frozen protocol requests14754finite preimage cases,
18setups with seed1234, eight legacy comparisons and four native adverse
transcripts: missing root, altered codomain, numerator coefficient and RNG
word. Native limits are one thread,2GiB,120s for census,30s for each setup or
replay and1500s for the wrapper. No fields, seeds or caps may adapt after calls.

*Observed launch blocker.* The required checkpoint gate cannot upload: there
is no AWS profile, ambient key pair or checkpoint token. AGENTS.md601 says
not to launch after this gate fails. Its diagnostic is retained; no wrapper
or numeric case was launched. Restore the existing upload access and pass
the same gate before invoking the frozen wrapper once. Source review and
proof checking continue independently.

Q: Does a complete x-root list certify point halves? A: It still needs all
ordinate lifts and actual doubling checks.

Q: What does a curve equation proof provide? A: Semantic algebra under its
hypotheses; native parsing, group-map construction and complexity are separate.

Q: Is the implementation a growing-family algorithm theorem? A: No. The
software caps, fixed-seed controls and abstract counting argument are distinct.

Primary sources: MIT18.783 Fall2023 Lectures2--5 (Lecture5 Theorem5.13);
FLINT upstream v3.1.3 root, sqrt and primality code; pinned mathlib field
division/finite-field laws. KB access is unavailable. Exact links/hashes and
scope are in PROTOCOL.md, SOURCE-AUDIT.md and checks/primary-sources.sha256.

*Relation-solver checkpoint.* Re-read PKM2016 sections3.1--3.5/5 and the
repository's selective-oracle and code/guessing reviews. The auxiliary-kernel
framework is prior work. A positive-only solver still needs a known cost cap
and effective useful yield on enough bases; small quotient dimension or the
coding predicate alone supplies neither. No new solver or numerical result
is claimed. The exact obligation and source hash are in SOURCE-AUDIT.md and
checks/oracle-review-sources.sha256.
