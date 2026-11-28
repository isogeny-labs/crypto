#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Trace-zero oracle: frozen work-reduction target met
2026-11-16 · PR3744 · unsigned producer evidence

#image("instructions.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 4pt,
 [Field/case], [v1 eager: whole Ir], [kernel lazy: whole Ir], [Whole ratio], [Oracle ratio],
[83/planted], [96,758,357], [31,440,436], [3.078×], [5.238×],
[83/uniform], [165,060,581], [40,566,277], [4.069×], [6.154×],
[83/cancel], [60,972,930], [24,772,243], [2.461×], [5.086×],
[131/planted], [108,446,027], [39,756,339], [2.728×], [3.747×],
[131/uniform], [186,817,738], [51,773,074], [3.608×], [5.043×],
[131/cancel], [88,613,529], [32,624,740], [2.716×], [4.882×],
)

*Frozen goal: MET.* Both uniform cells must have at least5× fewer complete-oracle instructions versus v1 with eager context fixed, and at least2× fewer complete cold-process instructions using kernel-lazy versus v1-eager; all outputs must match and A/A median discrepancy must be at most1%. The goal was set before all three panels and is unchanged. The preceding projected and compact panels fell short and remain preserved. Every positive/cancellation cell remains shown [pcc:unverified].

*Against the immediately preceding compact oracle:* n131: 1.321×, n83: 1.583×. All contrasts and min/median/max values are in summary.json [pcc:unverified].

*Design:* all210 planned queries complete: six disclosed d12 anchors ×seven arms ×five repeats. Each has a fresh native tally process and fresh Callgrind process. Full3algorithm ×2context crossing plus identical A/A. Error bars are recorded five-repeat min/max. This is a finite exposed control panel, not a population sample. All decisions, witness indices, models and common diagnostics agree; no invalid, missing, excluded or retried rows [pcc:unverified].

*Boundary:* setup + complete oracle + post/replay/serialization equals the entire fresh-process instruction total. Every arm parses the SAME compact query payload. Earlier all65-case parsing counts are not used as a new baseline. Simulated instructions establish work reduction; native time and fastest-external performance remain unmeasured.

#pagebreak()
= Algebra, implementation and validation

On E0: y²+xy=x³+1 over odd-degree binary fields, full rational trace-kernel membership uses both Tr(x)=0 and Tr(y+xH(x))=0. Here H is half-trace. This describes the exact finite rational kernel used by the solver; it is not a general scheme-chart completeness proof.

For an actual finite residual(z,y), put t=y+zH(z). Then t²+zt=1. For trace-zero u, the S3 equation v²+zv+z²u²+1=0 is equivalent to v=t+zH(u²)+epsilon*z with epsilon in F2. Project modulo W=span(VV): only d+1 bits remain, thirteen for d12 instead of the ambient thirty-five. The original ambient diagnostics remain alongside actual projected diagnostics.

Packed parity rows can reject inconsistency early. A full-rank solution is reconstructed and checked against the COMPLETE W remainder; deficient systems fall back to the original solver/model order. Monic roots, actual supplied signs, conservative limits and original-group replay remain required. Projection setup is lazy after the first residual. Numeric membership still uses the full kernel conditions; symbolic q is materialized on demand or explicitly forced by eager arms.

The final path specializes reduction only for the two exact public moduli, with a general fallback for every other tail. It builds projector multiplication maps by successive multiply-by-x and batches at most256 residuals in original order; unused tail work is charged. Private compact storage, numeric admission, signs, complete W checks and original replay remain.

Twenty-three focused release tests pass: every product-basis monomial at n83/n131, every multiply-by-x basis vector, same-degree/different-tail fallback, independent wide random operands, exact65-case decisions/indices across six algorithm/context modes and prior small admission/group/projection checks. Root library check also passes. Retained compact solver AND field sources execute alongside v1/current kernels in the same final binary [pcc:unverified].

*Cross-arithmetic audit:* a separate native checker imports no production field/group/solver code. Schoolbook polynomials and Euclid inversion recheck all65 input truths,19 negatives and all base admissions, then replay140 saved point tuples from this panel. Wide negative truths require inconsistency of every independent necessary symmetric system. Prior corrupt-point/false-refutation canaries are retained. This producer-written checker is not an independently signed verdict [pcc:unverified].

*Controls/resources:* planted and cancellation controls Found/replay; uniform controls Refuted. All28 largest-class pilot calls valid before freeze. Final phased/unphased counter calibration agrees within0.1%. Local CPU0,2GiB,600second combined calibration/pilot/freeze/primary cap, no paid launch or retry. The pilot estimate is planning only. Complete raw streams/counter dumps are archived with byte hashes and deterministic extraction/repack checks [pcc:unverified].

*Identity/custody:* E0 field moduli z^131+z^13+z²+z+1 and z^83+z^45+z²+z+1. Original workload SHA256 2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5; compact inputs retain exact point order. README links source/protocol/binary pins, rows, native summary, accepted export, audits, archive and reproduction. Earlier studies and their failed criteria remain immutable.

*Scope/sources:* this is the finite-base relation component, with fixed public controls. No Z[tau]-module advantage, asymptotic relation density, native-time or fastest-external claim. M4RI/FLINT/Magma comparison and independently signed review remain open. Existing scoreboard/timeline/runtime benchmark values stay historical. Sources checked: repository S3/Semaev/Diem, half-trace/Itoh–Tsujii and Montgomery batching. KB unavailable, wrapper127. No novelty assertion.
