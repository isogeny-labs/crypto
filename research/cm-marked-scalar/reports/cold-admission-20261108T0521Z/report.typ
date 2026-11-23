#set page(paper:"a4",margin:16mm)
#set text(font:"Libertinus Serif",size:10pt)
#set par(leading:0.5em)
#align(center)[
 #text(size:18pt,weight:"bold")[Independent point replay and native admission]
 #linebreak()
 CM-CACHE-COLD-V2 · attempt-001 · 2026-11-08T05:21Z · PR 3674
]
= Completed component and checks
The independent GMP field implementation replays all 60 frozen expected $[mu]P$ outputs and all 60 subgroup checks $[n]P=O$. It uses affine field/group arithmetic without OpenSSL or the producer's point functions. Three altered sign, coordinate or scalar controls reject. The native analyzer passes 24 synthetic decision/corruption checks; all fixture times are fabricated software data. Its complete fixtures each contain 10,500 rows, never experimental clock observations.

The closed oriented relation and scalar label remain those of fixtures-v2. P-192 is primary; P-224, P-256, P-384 and P-521 remain included. The executable cached method preserves the original scalar, retaining labelled prefix/table points for shared digit columns. The existing closure proof supplies provenance; this replay does not expand the global high-degree quotient.

#image("flow.svg",width:100%)

= Recorded launch outcome
The one calibration launcher returned exit3 at native admission, before fork/exec of the point-action worker. The receipt records 1.8 other CPU-seconds during the two-second preflight; the frozen ceiling is 0.2. CPU and memory PSI avg10 are both zero. These are admission measurements, not point-action times. No calibration rows file exists. No held-out worker, retry, cloud submission, controller restart or generator restart occurred. Worker wall/CPU time, phase times, RSS, A/A ratios and speedups are absent.

Two panels were planned conditionally: 3,500 calibration rows and 7,000 held-out rows, 25 configurations, one warmup and six paired rounds. Actual measurement workers and rows are zero. Each panel had a 120-second deadline, one thread and a 2 GiB address-space ceiling. The local/cloud incremental cloud cost is zero; qualification/build CPU costs are not assigned invented dollar values.

= Boundary, publication and custody
The full cold function includes serialized request parsing, fresh curve/point construction, plan preparation, arithmetic/table normalization, symmetric output verification and destruction. The deterministic primary uses no installed plan. Installed methods remain secondary. The frozen analysis compares every held-out point with the fastest of eleven reference configurations beyond the paired A/A envelope; it was not run on absent data.

Source/checker/launch 9f59b9428989 and frozen record d4b2f1be961c were verified hosted before admission. Canonical protocol digest: 0bbaa17b83ae883e2b72c62c611177b4d4f134e9099c102933a082ba054cbf4f. Publication receipts and the schema-valid export are retained. The raw tree archive is 1,232 bytes; fresh extraction verifies all members and deterministic repacking is byte-identical. No experimental attempt was repeated for custody checks. Precise process start/end timestamps were not recorded by the refusal receipt and remain null in the export.

The preflight refusal is retained without a performance inference. A subsequent timing attempt requires a distinct authorized amendment and a quiet admission environment. The active e2e objective is preserved. Existing EKS budget/quota restrictions remain unchanged.

#text(size:8pt)[Evidence: ../../cold-v2/admission-receipt.json, summary.json, export.json, runs-manifest.sha256 and runs.tar.xz.sha256; ../../protocol/cache-cold-v2/; ../../qualification/cache-analysis-v2/ and cache-points-v1/. KB unavailable as previously recorded; this continuation implements the published CACHE_COLD_CONTRACT.md and its primary references. No new novelty claim or official scientific promotion. No Rust implementation changed; no root Cargo test result is claimed.]
