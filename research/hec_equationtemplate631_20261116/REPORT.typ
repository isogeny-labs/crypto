#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= Compile once, specialize inverse equations exactly
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr),[Gate],[Result],[Curve-dependent polynomials],[3 curve + 8 section],[Stored terms],[9680],[Target coordinates read at compile],[0],[D1 specialized equations],[Byte-identical to direct system],[B specialized equations],[Byte-identical to direct system],[Wrong-prime template],[Rejected before output creation])

== Target dependence made explicit
Actual quartic and sections → one serialized equation template → public target coordinates → specialized inverse equations.

Only the seven projective section constraints depend on the target vector, through lambda0 times section i minus lambda i times section zero. Curve, saturation and separating constraints are unchanged. Specialization rebuilds no section basis and expands no tensors.

Both positive executions exit zero with empty stderr. Independent equation files are read only afterward for byte comparison. No old eliminant, divisor representative or expected scalar enters compilation or specialization.

== Custody
Source11412d5e4482 and checkpoint20261116T040421Z precede one compilation. Serialized template freeze7438326ae4d1 and checkpoint20261116T040458Z precede both specializations and the expected negative control. No solver is rerun.

== Next mathematical obligation
This is an equation template, not a compiled inverse or reusable elimination schedule. Equal solver summary statistics on two inputs do not prove equal critical pairs or valid pivot reuse. A further elimination template must be target-independent and have exact specialization certificates or explicit failure handling.

No timing or novelty claim. Cold accounting must include template compilation, loading, specialization and all solve/reconstruction work; per-target reuse is not free construction. Earlier phase bounds rule out section/group-only tuning as a tenfold route with other costs fixed. The full objective, genus-two/four mechanisms and global correspondence remain open.
