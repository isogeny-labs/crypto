#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Exact LFD on paired n83 m2 factor bases
Observations only. Eight original polynomial systems receive complete degree-three closure and producer replay. This measures last fall degree, not degree of regularity. Independent scientific review remains pending.

== Recorded outcome
#table(columns:(2fr,1fr,1fr,1fr),inset:5pt,
 [Curve / target],[Baseline LFD],[Spike40 LFD],[Prior FFD],
 [Koblitz planted],[2],[3],[2],
 [Koblitz natural],[2],[3],[2],
 [General binary planted],[2],[3],[2],
 [General binary natural],[2],[3],[2])

#image("lfd-comparison.svg",width:100%)
Eight exact input-specific certificates from n83-degree3-paired-20261028-r1. No statistical uncertainty is estimated. Both arms have 2^27 candidate x-values and identical targets. Planted tuples lie in the common low26 support; they are not uniform over either full base.

All eight producers, eight fresh-process replays, three runtime controls and four unit tests passed. No production timeout, retry, failure or exclusion. All 200 raw-manifest entries pass. The baseline uses coordinate positions 0..26; spike40 uses 0..25,40. Every system has 54 Boolean variables.

Each spike40 certificate has a degree-at-most-two polynomial in V3 with nonzero remainder against the complete V2 space, proving a strict fall at three. Natural systems contain one in V3. Planted systems have 53 independent linear constraints and both remaining affine assignments satisfy all original equations. These give upper bounds of three. Baseline retains exact LFD two.

V3 rank is 26288 for planted and 26290 for natural systems in both arms. Spike40 linear rank increases from 17 in V2 to 53 or 54 in V3. The LFD certificates rely on strict nonmembership and global upper checks, not rank increase alone.

#pagebreak()
== Definition, preservation and reproduction
The computation uses ordinary-degree closure with Boolean field equations. A row may be multiplied by a variable only when its ordinary degree is below three, before Boolean normalization. Replay checks the complete prior V2 trace, source insertion, every reduction, queue exhaustion, immutable row bytes, nonmembership witness and upper certificate.

Definition source: Caminata and Gorla, #emph[Solving degree, last fall degree, and related invariants], definitions 1.1 and 1.5; https://arxiv.org/html/2112.05579. Prior FFD two is certified on the same source hashes in evidence/pdp/paired-summary.json. Producer replay is not independent review.

Run: n83-degree3-paired-20261028-r1. Started 2026-10-28 07:02:11 UTC; completed 07:17:39 UTC. Existing Spot host, one native process, 8 GiB virtual memory, 600 seconds per producer/replay, 10800-second outer limit. Existing-host fallback follows the recorded EKS shared-bucket denial. Wall timestamps are diagnostic; no performance comparison. CPU/RSS and intermediate solving-degree fields remain null.

#text(size:8pt)[
Source: 3b4cca71a0f3df5ab0b5da202c10af1e741095b7 Frozen protocol commit: 4955822af2f805eb6046dcfe2fb775bdf52a0293 Protocol SHA-256: 471f53d00cdab630bb3a1ab2f1ad1e637eeceb17549ed08fad0dab21dd13031f Native SHA-256: fd0393eaacc36513d835abb362bf63729300088c339cd12e82cb26f41305b574
]

Seed 20261027 and original targets are retained. Launch interface: #raw("run-degree3.sh BINARY PRIOR_PDP_RAW FRESH_OUTPUT_ROOT"). Per-case commands: #raw("degree3 run INPUTS ID V2DIR OUT") and #raw("degree3 replay INPUTS ID V2DIR OUT"). The frozen launcher supplies the resource limits and exact eight-case order.

Nine archives preserve the whole raw tree: eight case archives with binary rows and JSONL operations, plus metadata with inputs, V2 snapshots, executable, controls, replay/status logs, environment and manifest. Extract all nine into a fresh directory; run #raw("sha256sum -c manifest.sha256"). Byte layouts are in DEGREE3_FORMAT.json. Archive hashes and URIs are in evidence/degree3-storage.json. The protocol-bound records are in experiment-degree3.export.json. Pre-production compile failures are retained.

#text(size:8pt)[Raw S3 prefix: #raw("s3://setec-astronomy-artifacts/runs/n83-degree3-paired-20261028-r1/local/")]

== Remaining scope
FFD two and LFD three are certified for these four candidate inputs. No universal or asymptotic conjecture verdict, elite-base improvement, regularity reduction or direct solving-degree measurement follows. Prime and m3..9 LFD remain open. The original fixed-base m2..9 top-form bounds are preserved separately and cannot be assigned to spike40.
