#set page(paper: "a4", margin: (x: 18mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9.4pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
= Literal certificate: explicit ring normalization
Verification v3 · 2026-10-25 · terminal allocation failure

The same sealed p37 seed0 phi system-0 certificate is exported after a
passing 13-test native gate. Source freeze bc818088c1af precedes the unique
07:10:27 UTC managed launch, PID 3851339. Only the instance proof tactics
change to explicit grobner; all coefficients, balanced sums, four product
lemmas, three final statements and ten axiom audits remain required.

#figure(image("figures/proof-v3-flow.svg", width: 100%), caption: [Actual terminal verification gates. No theorem, axiom audit or IDC is inferred from a passing export.])

== Actual outcome and resource evidence
The export exits 0 and retains the same exact integer-lift statement.
All 2,197 dense coefficient positions and the 454 nonzero R coefficients
are retained. The intended identity is ∑ hᵢfᵢ = q² + 37R; its complete
literal commutative-ring proof and characteristic-37 consequence remain open.
Statement SHA-256:
#text(size: 7.5pt)[e7dd6d7622171f18c1d80ea9c1d26d182b08dbf664c3a3046115eb6715e5eb45]

The Lean phase starts at 07:20:28 UTC and terminates at 07:32:28 UTC with
exit 134, signal 6: libc++abi reports an uncaught std::bad_alloc. Maximum
recorded RSS is 3,772,392 KiB under the unchanged 8 GiB address-space bound.
These are different memory measures. The one aggregate 1,200-second bound
was not increased. Shared-host durations are diagnostic and unadmitted.

GNU time reports signal 6 and then an “Exit status: 0” footer. That footer
does not indicate success. The actual phase and terminal receipts both
record 134. The driver stops before the axiom audit and IDC. No accepted
instance theorem comes from this run or either earlier memory-failed encoding.

== Exact remaining proof obligation
Prove the literal integer identity in a commutative ring, then its
characteristic-37 q² consequence and the repeated-abscissa conclusion
under the explicit no-zero-divisor hypothesis. No certificate identity may
be assumed. Permit only propext, Classical.choice and Quot.sound in all ten
axiom sets; no sorry, custom axiom, native evaluation or kernel bypass.
Lean is pinned to 4.34.1; lean4checker was not available.

These failures do not disprove the original finite-field certificate and
do not locate the memory failure within elaboration or normalization.
Before another large attempt, independently verify the serialized integer
statement and isolate small definition/product blocks. Preserve the full
statement, explicit hypotheses and aggregate execution bound.

== Scope and retained artifacts
No relation collection, matrix rank, target recovery, new complete-cost
comparison or general group/family theorem is established here. The native
parser, field/group/decoder bridges remain separate. The original 189+21
study and completed twelve-configuration diagnostic are not rerun.

See PROOF-V3.md, native/src/main.rs, PROTOCOL.md and the source-pinned
run driver. Raw parent runs/: certificate-instance-native-gate-v3,
certificate-instance-proof-v3 and certificate-instance-v3-layout-v1/v2.
Compact failed receipts remain under results/certificate-instance-proof-v3/.
The deterministic runs-v3.tar.xz and checksum preserve this run without
replacing the v1/v2 archive. All actual PDF pages are inspected. No canonical
IC/rho graph, leaderboard or benchmark index is promoted by this failure.
