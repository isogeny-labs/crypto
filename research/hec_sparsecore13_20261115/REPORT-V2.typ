#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Sparse original relation assembly
Genus 3 · F127 · 15 November 2026
#image("flow.svg",width:100%)

V2 removes dense original relation rows as well as the already removed dense reduced core. Two frozen gates passed once: standalone comparison and actual cold DLP execution. Sourcea00660d5e81e; checkpoint20261116T003417Z.

== Representation and exact checks
The314original relations store2378nonzero coefficients, with maximum support8. Sorted accumulation preserves coincident-point cancellation. Every reconstructed log needed by every original relation must be known; all314sparse residuals pass.

The232×47reduced matrix has2413entries and yields an exact nonzero kernel atattempt0. All129point logs match the independent dense reference up to common nonzero scale. Twelve planted controls at dimensions47,128,512pass; corrupted original matrices and zero vectors reject. The standalone dense reference is verifier-only; fullcold candidate has no dense fallback.

== Fullcold outcome
Fresh source/quotient setup, discovered chart, calibration, alignment, actual inverse systems and conic descent pass. Four D1descents and two Bdescents agree. k183049verifies by public scalar multiplication on source and destination; k+1rejects on source. All positive stages exit0; expected missing-loader101. No scientific rerun or dropped input; stdout, stderr, statuses and inputs are retained in run-v2/and cold-run-v2/.

== Next algebraic candidate
Current equations repeat the same canonical divisor h in every row: (H_i−h)x=0. Introduce z=hx and write H_i x−z=0, with h x−z=0 once. Projection and x↦(x,hx) are inverse linear maps of kernels, preserving nullity and adding exactly one to rank. Quartic rows have at most5coefficients instead of8, plus one defining row. Keeping z in the small core may reduce repeated hub fill; a gain is unmeasured and not guaranteed. No auxiliary-variable experiment was executed in V2.

== Scope
Engineering correctness and sparse representation, without new timing/RSS measurements or a tenfold runtime claim. F127, fixed48base points, family/kernel restrictions and exhaustive geometric controls remain. Wider-field portability, scalable construction, genus two/four and the mathematical tenfold objective remain open. The earlier failed coldcost comparison is unchanged.

See RESULTS-V2.md and PROTOCOL-V2.md for frozen execution scope; V1source/binaries/raw remain intact. Primary published cost references are recorded in RESULTS.md, not established by this finite correctness cohort.
