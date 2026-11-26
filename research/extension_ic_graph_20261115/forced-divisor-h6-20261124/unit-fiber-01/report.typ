#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set par(leading:0.4em)
= H6 unit elimination: exact outputs, higher cost
24 November2026 · Rust · Local count study · Producer evidence UNVERIFIED

All18 registered six-variable calls complete and reproduce the frozen seven-variable full-reference roots, normalized functions, support outcomes and resolution flags. The exact substitution works, but loses the solver-count comparison. No unchanged baseline is re-solved. [pcc:unverified]

#image("results/analysis/solver-products.svg",width:100%)

#table(columns:(1.6fr,1.2fr,1fr),align:(left,right,right),
[Arm],[Solver products],[Ratio to prior],
[Prior seven-variable plain],[16,414,425],[1],
[New unit plain],[23,582,716],[1.436706799],
[Prior seven-variable traced],[12,480,689],[1],
[New unit traced],[19,237,258],[1.541361859])

Unit compilation/specialization/lift add1,076 coefficient products and1,045 additions per arm, reported separately. Native coverage is incomplete, and legacy Fq inversion pricing differs. No full native total or wall ranking. [pcc:unverified]

== Exact mechanism and yield
The highest equation is −2·b+2·r·s+c=0, where b=c0.im, r=t2.re and s=t2.im. Constant −2 is a unit throughout the odd-prime domain. Substitute b=r·s+c/2 and fix c0.re: six equations in six variables, degree at most four. Every root lifts and passes the original norm/support/Jacobian boundary. The opt-in module preserves the production default.

Both formulations report quotient dimension70 and solving degree5 on every case. Traced mode replays all eight development cases. Each arm finds ten norm functions but only the retained known relation has admissible support. No new development graph row is obtained. Fewer variables and unchanged solving degree are insufficient to predict cost here. [pcc:unverified]

#pagebreak()
== Frozen method and source qualification
Eleven final native controls pass: exact substitution at off-variety points including nonzero imaginary constants, retained positive lifting, existing support/group controls, malformed-pivot/coordinate refusal, six-variable known roots/trace replay and expired budgets. Optimized worker and strict worker/test Clippy pass. Source788b8f005f and registrationfe68282737 were published before the sole scientific call. [pcc:unverified]

Order: nine plain then nine traced fibers, values369(control),0..7(development). Grevlex/maxdegree24, staircase128, Fast roots, seed0x94700000 XOR case index. Fresh family/unit compilation/trace scope per arm; learning and all attempted work retained. Frozen reference answers are validator input only. Every original function and resolution flag matches. Positive-control failure would stop remaining calls. [pcc:unverified]

One local CPU0 on AMD EPYC9V74 x86_64, one Rayon thread,8GiB virtual memory,360s hard watchdog and10s shared cooperative budget per case. Native/launcher/posthash0; empty stderr; uncontended receipt; unavailable PSI. This count study has no A/A or wall ranking. [pcc:unverified]

== Evidence and replay
PLAN.md, PROTOCOL.md, source-worker-inputs.sha256 and validation/ bind exact methods/source/logs. Worker SHA256:a14ac218f704079c2aa34f4ed8d57e8c81034b2261de5c99f972ca2207e08827.

results/raw.tar.xz is9,580bytes with the complete raw-data tree and frozen reference. SHA256:39b542c6c807f3cd78b2fe76c88f338de1075756765a37adbd3be4889df9754e. Executables remain separate read-only LOCAL objects. Read-only Rust analyze.rs checks all18 calls/36 events, original outputs/flags and independently lifts coordinates. Archive replay reproduces derived JSON/SVG byte-for-byte; strict analyzer Clippy passes. [pcc:unverified]

Claims:b82fc5(qualification),ceb374(registration),dababd(raw),f74bc0(analysis), all PCC-20261124, unsigned and UNVERIFIED. S3 upload, canonical append-only reconciliation, signed admission and whole-root compilation remain open. No external-tool or admitted IC/rho comparison. [pcc:unverified]

== Decision and successor
Reject a solver-count gain on this panel. Close the registration without retry and retain the seven-variable traced incumbent. Exact substitution preserves support yield; improving it requires a distinct mechanism.

Next review: force a quadratic divisor already represented by a checked factor-base witness, and cancel it to turn a three-small/three-quadratic principal function into a two-endpoint graph edge. First audit whether such anchors exist in retained data and whether original group/projection witnesses compose. This is a hypothesis requiring source controls and its own registration.

Sources: H6 and prior fast-unit implementation; primary Joux–Vitse/Sparse FGLM review. KB unavailable; no novelty claim. #link("https://github.com/isogeny-labs/crypto/pull/5094")[PR #5094] · #link("README.md")[Source-linked evidence]
