#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Product projection: partial result retained
2026-11-16 · PR3744 · unsigned producer evidence

#image("instructions.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 4pt,
 [Field/case], [v1 eager: whole Ir], [projected lazy: whole Ir], [Whole ratio], [Oracle ratio],
[83/planted], [97,366,933], [51,391,275], [1.895×], [2.033×],
[83/uniform], [165,786,662], [73,498,596], [2.256×], [2.463×],
[83/cancel], [61,520,467], [37,277,183], [1.650×], [1.629×],
[131/planted], [109,462,688], [56,940,626], [1.922×], [1.660×],
[131/uniform], [187,976,209], [81,842,368], [2.297×], [2.334×],
[131/cancel], [89,594,624], [45,652,289], [1.963×], [1.600×],
)

*Frozen goal: NOT MET.* Both uniform cells must have at least5× fewer complete-oracle instructions versus v1 with eager context fixed, and at least2× fewer complete cold-process instructions using projected-lazy versus v1-eager; all outputs must match and A/A median discrepancy must be at most1%. The goal was set before either panel and is unchanged. Every positive/cancellation cell remains shown [pcc:unverified].



*Design:* all210 planned queries complete: six disclosed d12 anchors ×seven arms ×five repeats. Each has a fresh native tally process and fresh Callgrind process. Full3algorithm ×2context crossing plus identical A/A. Error bars are recorded five-repeat min/max. This is a finite exposed control panel, not a population sample. All decisions, witness indices, models and common diagnostics agree; no invalid, missing, excluded or retried rows [pcc:unverified].

*Boundary:* setup + complete oracle + post/replay/serialization equals the entire fresh-process instruction total. Every arm parses the SAME compact query payload. Earlier all65-case parsing counts are not used as a new baseline. Simulated instructions establish work reduction; native time and fastest-external performance remain unmeasured.

#pagebreak()
= Algebra, implementation and validation

On E0: y²+xy=x³+1 over odd-degree binary fields, full rational trace-kernel membership uses both Tr(x)=0 and Tr(y+xH(x))=0. Here H is half-trace. This describes the exact finite rational kernel used by the solver; it is not a general scheme-chart completeness proof.

For an actual finite residual(z,y), put t=y+zH(z). Then t²+zt=1. For trace-zero u, the S3 equation v²+zv+z²u²+1=0 is equivalent to v=t+zH(u²)+epsilon*z with epsilon in F2. Project modulo W=span(VV): only d+1 bits remain, thirteen for d12 instead of the ambient thirty-five. The original ambient diagnostics remain alongside actual projected diagnostics.

Packed parity rows can reject inconsistency early. A full-rank solution is reconstructed and checked against the COMPLETE W remainder; deficient systems fall back to the original solver/model order. Monic roots, actual supplied signs, conservative limits and original-group replay remain required. Projection setup is lazy after the first residual. Numeric membership still uses the full kernel conditions; symbolic q is materialized on demand or explicitly forced by eager arms.

The first projected study reduces oracle work against v1 by2.334× at n131 and2.463× at n83 on uniform controls. This fails the frozen5× oracle criterion even though both cold-process criteria pass. The entire panel is retained; the next compact-workspace study uses the same criterion and inputs [pcc:unverified].

Eleven focused tests, full-variety10, prior20, terminal2, earlier-linearized3 and root library check pass. These include complete small-field S3 equivalence, inconsistent/unique/deficient projected systems, malformed shared-x signs and lazy symbolic model sharing. The integral certificate covers only the explicitly stated polynomial expansion [pcc:unverified].

*Cross-arithmetic audit:* a separate native checker imports no production field/group/solver code. Schoolbook polynomials and Euclid inversion recheck all65 input truths,19 negatives and all base admissions, then replay140 saved point tuples from this panel. Wide negative truths require inconsistency of every independent necessary symmetric system. Prior corrupt-point/false-refutation canaries are retained. This producer-written checker is not an independently signed verdict [pcc:unverified].

*Controls/resources:* planted and cancellation controls Found/replay; uniform controls Refuted. All28 largest-class pilot calls valid before freeze. Final phased/unphased counter calibration agrees within0.1%. Local CPU0,2GiB,600second combined calibration/pilot/freeze/primary cap, no paid launch or retry. The pilot estimate is planning only. Complete raw streams/counter dumps are archived with byte hashes and deterministic extraction/repack checks [pcc:unverified].

*Identity/custody:* E0 field moduli z^131+z^13+z²+z+1 and z^83+z^45+z²+z+1. Original workload SHA256 2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5; compact inputs retain exact point order. README links source/protocol/binary pins, rows, native summary, accepted export, audits, archive and reproduction. Earlier studies and their failed criteria remain immutable.

*Scope/sources:* this is the finite-base relation component, with fixed public controls. No Z[tau]-module advantage, asymptotic relation density, native-time or fastest-external claim. M4RI/FLINT/Magma comparison and independently signed review remain open. Existing scoreboard/timeline/runtime benchmark values stay historical. Sources checked: repository S3/Semaev/Diem, half-trace/Itoh–Tsujii and Montgomery batching. KB unavailable, wrapper127. No novelty assertion.
