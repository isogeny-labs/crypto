#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set par(leading:0.4em)
= H2 loses the matched rejection-cost comparison
24 November 2026 · Rust · Local component study · Producer evidence UNVERIFIED

The opt-in template/root/materializer adapter is implemented and qualified, including both F4 fallbacks and deadline handling. The degree-7 H2 basis is exact on the exposed pair, but its dense elimination and exact checks cost far more than the existing stopped six-coordinate F4 paths. Keep the existing default. [pcc:unverified]

#image("results/analysis/counts.svg",width:100%)

#table(columns:(1.2fr,1.4fr,1fr),align:(left,right,right),
[Arm],[Partial field products],[Ratio to traced],
[Plain F4],[1,834,438],[1.2843],
[Traced F4],[1,428,387],[1],
[Degree-7 H2],[1,017,832,355],[712.5746])

Five samples per arm have identical recorded counts. The unit sums measured Weil/F4/root/postprocessing products and charges H2's fresh development F4 and compilation. Context/factor-base setup and uninstrumented bookkeeping are excluded from that partial count. They are included in wall intervals. Overlapping context counts are not added twice; there is no complete native total. [pcc:unverified]

#table(columns:(1.1fr,1fr,1fr,1fr),align:(left,right,right,right),
[Local seconds],[Minimum],[Median],[Maximum],
[Plain F4],[0.007664],[0.007887],[0.009495],
[Traced F4],[0.008277],[0.008449],[0.008871],
[Degree-7 H2],[10.846889],[10.869866],[11.102457])

The process is warm, algorithm caches cold per interval. A/A max/min 1.0904338 passes the frozen 1.10 gate. The plain/traced median difference is inside that spread; no wall ranking is supported. H2's median is 1286.5 times traced here. The native subgate reports uncontended, with PSI unavailable. These are local observations, not full L2 or signed admission. External tools were not compared. [pcc:unverified]

== Correctness and useful-relation scope
All 18 planned intervals run: one warm-up, two traced A/A and five interleaved three-arm rounds. All saved root sets, resolved statuses and cross-arm base/materialization fingerprints match. There are 54 repeated root occurrences, three distinct roots and two exposed residuals. *Every arm materializes zero decompositions.* Divisor checks in this panel are vacuous. Positive materializer behavior is covered separately by the fixed planted unit control; useful-relation yield and graph-rank gain remain unmeasured. [pcc:unverified]

#pagebreak()
== Why this implementation loses
H2 obtains a fresh full F4 basis on residual 0, compiles degree 7, materializes residual 0 from that basis, then applies the template on residual 1. Exact checking accepts without fallback. The latter costs 1,004,673,424 elimination products and 12,005,619 checker products. Checker work alone exceeds the entire traced pair's 1,428,387 recorded products. Free dense elimination would not rescue the current checking design in this unit. This is a construction-specific result, not a general limitation on templates. [pcc:unverified]

Traced F4 learns on residual 0 and replays on residual 1. Its F4 subtotal is 1,020,198 versus 1,426,249 plain; root-stage products are 398,639 in each arm. This is a deterministic partial-count reduction on two exposed rejection cases. The wall comparison between those two methods stays unresolved inside measured noise. [pcc:unverified]

== Frozen method and source
Published source 99e80565ce7c3e72637bba7f2b73c8b1fb35278a precedes the sole registered scientific invocation. Public fixture hash cca80f4d1443f8f750f72a46f8678da945a93ad4c0c9ce07ae4ca6aedeb6f4fa. Expected roots are separate and never passed to the solver. Worker hash f7d6ad8ae02736953bc6a1a2a3405385667ba129ae719b102e1e85de6291d8d5.

All arms rebuild spec/context/factor base with seed 13, use root RNG 389 per residual, grevlex/maxdegree 24, Fast roots and the same Nagao materializer. Plain/traced stop at staircase 64; H2 learns a complete development basis. Checker counters are enabled, partial native bitop hooks uniformly disabled. The 600s watchdog and 8GiB limit bound the complete worker. AMD EPYC 9V74 x86_64, reserved CPU 0, one Rayon thread. Native/launcher/posthash statuses 0 and empty stderr; native wall 54.7655s, max RSS 77,136KiB. [pcc:unverified]

== Evidence and limitations
PROTOCOL.md and WORKER_DESIGN.md fix the schedule, inputs, limits and stop rules. results/raw.tar.xz contains the complete raw-data tree (7128 bytes), including all samples, full summary, statuses, native conditions, hashes and inputs. Executables remain separate read-only LOCAL objects with published hashes. The read-only Rust analyzer checks the complete order, source/input bindings, counts, statuses, roots and zero-materialization scope. Archive replay regenerates identical derived JSON and SVG. This is producer replay, not independent signed verification. [pcc:unverified]

Unsigned claims: PCC-20261124-61c400 (qualification/registration), 8ed801 (raw result), 773135 (analysis). Raw archive SHA256 3792993f536152116c19215ccea4c17e55770c0491f0f190a352b21a65b2a8eb.

S3 checkpoint gate lacks profile/token; PSI is unavailable. Canonical append-only PCC reconciliation and current whole-root compilation remain open. No useful-relation, full IC/DLP, fresh-population, strongest-tool, rho or leaderboard conclusion is admitted. [pcc:unverified]

== Decision and next discriminating work
Close this dense H2 registration as consumed. Preserve its exact API and failures; do not raise the dense bound or retry unchanged. A viable successor must lower certificate cost as well as elimination, then demonstrate positive useful relations at the full cold graph boundary. Review sparse Frobenius/field-equation filtering and actual-curve rho applicability before registering a new finite test.

Sources: prior H2 primary packet (Larsson–Åström–Oskarsson 2017 and Larsson et al. 2018), Joux–Vitse 2012, existing F4/root/materializer/trace APIs. KB unavailable; no novelty or universal negative claim. #link("https://github.com/isogeny-labs/crypto/pull/5094")[Lane PR #5094].
