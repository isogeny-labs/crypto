#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set heading(numbering: "1.")
= Boolean F4/F5/F6 and native generalized roots
Instruction measurements, 2026-10-18. Native runtime qualification is separate.

#text("Correction dated2026-10-19: the old `random=true` rows are fixed generator
controls. The original raw flags and numeric results are preserved. See
[the correction](../f4_f5_boolean_runtime_20261019/TARGET_LABEL_CORRECTION.md)
and [the native continuation](../f4_f5_boolean_runtime_20261019/REPORT.md).

The opt-in single-write Gray-table builder reduces complete matrix-F4/F5
instruction work on the linked 22-variable fixture, with identical reduced
rows, rank and word-XOR accounting. These are Callgrind instruction results,
not admitted native runtime speedups. The production default is unchanged.

The missing bounded generalized-rootfinding execution path is now native Rust:
certified prime/binary/odd-extension fields; polynomial, affine-linearized and
partial rational-composition domains; complete enumeration, component MITM and
Boolean Gray dispatch; and a complete S3 separator retaining all original eight
coordinates. Ten independent tests pass, including poles, repeated and
inseparable roots, GF(2^83) extraction, a wide prime certificate, and literal
eight-coordinate separator enumeration. CLI fixtures return exactly two
Boolean roots and eight separator tuples. This completes these implementation
contracts; it does not establish the old hard-family 8x research target.

")

#pagebreak()
= Complete matrix step
ICV1 n23 (icv1-f2m23-tm5197-1f85e9e1), 22 Boolean variables, degree 4. Five alternating pairs; exact output and XOR accounting. Callgrind 3.19, merged workers, simulated AVX2. Fresh-seed and scaling records are retained in summary.json.

#image("matrix-instructions.svg", width: 100%)

#table(columns: 4, [Engine], [Reference Ir], [Candidate Ir], [Reduction],
[matrix-f4], [597,267,029], [487,814,530], [18.326%],
[matrix-f5], [607,964,549], [498,549,578], [17.997%],
)

#pagebreak()
= Every Boolean root
First complete call on the same n23 planted system. All original roots checked equal. The general API includes certification and normalization; direct Gray enumeration remains cheaper. Generic F4-basis profiling timed out on this fixture.

#image("complete-roots.svg", width: 100%)

#pagebreak()
= Cold same-target decomposition
One planted and one unplanted generator n23 target. Table setup is added once; online query data remain separate. F6 and the generic solver can return different verified witnesses for the same verdict. No IC/rho result follows.

#image("cold-decomposition.svg", width: 100%)

= Reproduction and limits
Source: REPORT.md, PROTOCOL.md, summary.json, analyze.rs and runtime.rs in research/f4_f5_boolean_perf_20261018. Raw member hashes and every failed attempt are retained. Ten native solver tests and CLI fixtures pass; full-library status is in VALIDATION.md. Magma, msolve, FGb, Singular and PolyBoRi were not available. Native wall, external fastest, hard all-roots 8x and whole-pipeline claims are not established.
