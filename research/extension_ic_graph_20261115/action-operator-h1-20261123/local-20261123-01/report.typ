#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.4em)
= Local H1 result: retain the existing coordinate
23 November 2026 · Once-only local component control · Producer evidence UNVERIFIED

The fixed selection rule retains coordinate 5, the incumbent last coordinate. Every alternative has a higher reported multiplication count on both halves of the saved prefix. All 192 candidate root-set comparisons match; 32 systems contain 34 roots total, with zero dense fallbacks. [pcc:unverified]

#image("held-out.svg",width:100%,height:73mm,fit:"contain")

#table(columns:(0.65fr,1.2fr,1.2fr,1fr),align:(left,right,right,right),[Coordinate],[Development 16],[Held-out 16],[Held/default],
[0],[8,012,329],[8,008,581],[2.513],
[1],[7,511,312],[7,508,605],[2.356],
[2],[6,072,607],[6,069,868],[1.904],
[3],[5,802,646],[5,800,408],[1.820],
[4],[5,927,060],[5,924,159],[1.859],
[5 default],[3,188,499],[3,187,180],[1.000])

Unit: existing partial `lin_muls`, including attempted fast work and dense fallback. These ratios describe this counter, not full arithmetic, runtime or complete IC cost. The best alternative reports 1.820 times the incumbent on held-out inputs. [pcc:unverified]

All development candidates total 36,514,453 reported multiplications; selected held-out extraction brings this to 39,701,633. The incumbent across both halves totals 6,375,679. Selection is therefore not free setup. Shared basis generation and other operations/phases remain outside these totals. [pcc:unverified]

== Execution and evidence
User-authorized local backend; one thread, 8 GiB address-space cap, 120-second outer bound. Exactly 32 F4 basis calls, 32 incumbent and 192 candidate extractions; seeds 389. Native exit 0, empty stderr, all saved completion checks pass. A separate Rust analyzer recomputes every counter sum, root binding and selection decision. [pcc:unverified]

The execution tool reported exit 1 despite the saved native exit 0 and completed records. Cause unresolved; both statuses and original outputs are retained, with no retry. All pre/post executable hashes match. S3 checkpoint upload remains unavailable; the local plan uses published Git plus hash custody, without S3/isolation admission.

== Scope and decision
This is an exposed same-fixture 16/16 holdout over F1009, with no new target, natural-yield, elapsed or fresh-instance evidence. Retain the default; the coordinate-only H1 panel does not lower the reported multiplication metric. Other fields and linear forms remain untested. H2's elimination-template construction is the next distinct lead. [pcc:unverified]

Original and derived summaries, complete compact cases, native analyzer, source/executable pins and LOCAL raw manifests accompany this report. Large raw cases/bases await artifact-store publication. Follow-up #link("https://github.com/isogeny-labs/crypto/pull/5094")[PR #5094] after merged #4986. The pre-execution source/plan is commit 9fe79b2003. Full cost, signed validation and rho parity are not established.
