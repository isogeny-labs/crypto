#set page(paper:"a4",margin:17mm)
#set text(size:9.5pt)
= Paired n83 top-form certificates
Observations only. The exact 96 saved baseline/spike40 systems receive original-top regularity certificates. No exact finite degree inside an interval, elite-base improvement or universal FFD/LFD verdict is asserted.

All 96 producers, 96 fresh-process replays and 16 prime paired isomorphism checks passed. Ten native unit controls passed. No production failure, timeout, retry or exclusion.

#table(columns:(0.4fr,1.7fr,1.7fr),inset:3pt,[m],[Koblitz and binary],[Prime],
[2],
[28 to 55],
[29 to 55],
[3],
[111 to 165],
[Positive dimensional],
[4],
[194 to 275],
[Positive dimensional],
[5],
[277 to 385],
[Positive dimensional],
[6],
[360 to 495],
[Positive dimensional],
[7],
[443 to 605],
[Positive dimensional],
[8],
[526 to 715],
[Positive dimensional],
[9],
[609 to 825],
[Positive dimensional])
Each entry applies to both bases and both retained target kinds. Finite entries are certified lower-to-upper intervals. Positive dimension means no finite graded vanishing degree in this convention; it is not an infinite CM regularity claim.

#image("bounds.svg",width:100%)
Run n83-paired-top-20261028-r1. Each plotted interval represents eight saved cases (two bases, two binary families, two targets). All have identical bounds. These exact certificates carry no sampling interval; the shaded degree range remains unresolved.

The binary projection retains 27 base selectors and 83(m-2) intermediate variables. Every checked original top generator vanishes modulo retained squares. A surviving squarefree product gives the recorded lower bounds. This strengthens certificates on unchanged inputs. Original linear-top rank is zero in all 64 binary cases, so the upper bound stays nvars+1.

#pagebreak()
== Prime map and metric scope
All 16 prime pairs have an invertible diagonal map: scale the last selector in each block by 16384, leave the other variables unchanged. The inverse modulo the saved prime is 4835408130856784033218561. Checked selector weights match; highest S3 forms map onto their counterparts and selector squares acquire nonzero multiples. The two original top quotients therefore have identical graded Hilbert functions and regularity under this convention. The affine Boolean-selector ideals are not asserted equivalent.

Top-form regularity is the first degree where the quotient by highest parts of the original saved generators vanishes. No affine row reduction or derived equations are added. Six binary projections are checked, including two deterministic greedy maps; failed projections keep their surviving monomials. Greedy maps are not claimed optimal. XOR certificates in original linear top generators justify upper bounds when applicable. Replay recomputes complete source checks, projections, linear ranks and prime maps.

Baseline uses coordinates 0..26, spike40 uses 0..25,40. Each curve has 2^27 candidate x-values and fixed point manifests for m2..9. Natural targets are paired; planted tuples come from the common low26 support, not uniform sampling over whole bases. No saved equation or target changes. Candidate roots still require curve lifting and sign verification.

== Reproduce and preserve
Run: n83-paired-top-20261028-r1. UTC 2026-10-28 15:40:16–15:45:17. Existing Spot host fallback after recorded EKS shared-bucket denial; one process, 8 GiB virtual memory, 900 seconds each producer/replay, 1900 seconds outer. Times are diagnostic; no speed claim, per-case CPU/RSS or qualified wall time.

#text(size:8pt)[Source: 94b5400d9cbd670a048307809e439c6859fa24a0 #linebreak()
Frozen protocol commit: 5af325f18cc #linebreak()
Protocol v2: 5b0a75feb54abe9be95f384be778d804e985656e608a11c2684c40ea9312edb7 #linebreak()
Native: 5bbcafb370fea6310cd78c377aacc613cad3d8e4ff9b0f322d79c8f987322e4c]

A predata amendment corrected a premature source pin. No affected production ran under version1; both freezes and development errors remain. The original seed 20261027 is retained. Launch: #raw("run-top.sh BINARY PRIOR_PAIRED_RAW FRESH_OUT"). The frozen launcher records exact hash checks, limits and stable paired order.

The complete raw archive retains original inputs, all 112 certificates, failed maps, executable, logs, controls, partial/final summaries, replay, host and manifest. Extract into a fresh directory; run #raw("sha256sum -c manifest.sha256"). File semantics: TOP_FORMAT.json. Archive URI/hash: evidence/top-storage.json. Bound records: experiment-top.export.json and protocol-top.json.

#text(size:8pt)[Raw: #raw("s3://setec-astronomy-artifacts/runs/n83-paired-top-20261028-r1/local/")]

Definitions: Caminata–Gorla, https://arxiv.org/html/2112.05579. S3 and first-fall caution: Kosters–Yeo, https://arxiv.org/html/1503.08001. Producer replay is not independent review. Exact finite regularity, larger-m/prime LFD and an elite-base improvement remain open.
