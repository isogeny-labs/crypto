#let make-run(version, source, checkpoint, start, method) = {
 set page(paper:"a4",margin:18mm)
 set text(size:10pt)
 [= #version: fixed-model cold IC run
 #image("flow.svg",width:100%)

 *Run identity.* Source #source; pre-run S3 checkpoint #checkpoint; driver stdout creation #start UTC on 15 November 2026. One requested/completed execution. Darwin arm64, Apple M4 Pro, L0/unisolated. Source y²=f(x), f ascending (31,89,73,48,91,41,123,1), p=127, genus3, ell=1028893; D1=2(0,44), B=2(3,21). Exact input is PUBLIC.txt.

 *Source recovery.* #method

 *What ran.* Driver requires runtime setup and checks expected rejection when it is absent. It constructs both27pointkernels,27normalizations, quotient geometry and pointcount/Serresign, then consumes those new files. The first degree18fiber pair calibrates a rank8matrix from12training/4withheld pairs. Positive control and both actual inverse systems satisfy all12equations exactly. Closed divisors and public adjacency orient both original images.

 *Outcome.* Fresh314split relations,81LPcolumns removed,232×47densecore rank46,129point logs and314residual checks. Three conic descents per original input yield34511and829912; ratio183049verifies on source AND destination. k+1rejects on source. All five positive stages and three solver cells exit0; expected missing-setup control exits101. No unexpected failure, dropped input or rerun. Positive stderr files empty.

 *Scope.* Target chart/anchors remain preconfigured and checked against the fresh quotient. No old source/image/point-log data supplied. One public pair, not independent curve samples. The source recurrence alone does not prove full-pipeline complexity; exhaustive controls and a dense finalcore remain. No wall-time,10×, memory or novelty claim. Diagram used because no comparable timing samples were collected.

 *Evidence.* research/hec_coldfiber13_20261115: RESULTS.md, input/binary hashes, protocols, DRIVER files and every stage stdout/stderr/status. This run's Markdown report links its raw directory. Prior scalar/degree Lean proofs retain explicit geometric hypotheses; they do not formalize the whole implementation. Primary comparator: https://arxiv.org/html/2007.03172 §4.2.2–4.2.3.
 ]
}
