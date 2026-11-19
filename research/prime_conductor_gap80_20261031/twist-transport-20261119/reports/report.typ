#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10.4pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
= Explicit prime-conductor crossing, evaluated locally
19 November 2026. Original 80-bit objective remains open.

The actual ordinary endomorphism conductors are 1 and 33555419. This ratio is a 26-bit prime (log2 ratio 25.0000424361), over the 53-bit field prime 5629830687772387. It is the evaluated retained case in this report; no world-record statement is made. Arithmetic elliptic-curve conductors over global fields are not the quantity measured here. [pcc:unverified]

#table(columns: (1fr,2fr), inset: 5pt, align: left,
 [Parameter], [Exact retained value],
 [source equation a,b], [5620419248410339, 675862186879016],
 [quotient equation a,b], [2539833775737119, 1737207970637906],
 [prime map degree / conductor ratio], [33555419],
 [original prime subgroup order], [5629830654216971 (cofactor 1)],
 [source generator], [(2, 1401162638053187)],
 [transported generator], [(2975092980436849, 5333622322807887)],
)

#image("map.svg", width: 100%)

For normalized auxiliary map $(x,y) mapsto (f(x), y f'(x))$, the twist-d map is
$(x,y) mapsto (d f(x/d), y f'(x/d))$, here d=2 nonsquare. Its coefficient scaling is d²/d³. The auxiliary kernel is explicitly typed; it is not asserted to lie on E. Both horizontal maps descend to Fp, although the vertical isomorphisms use Fp². [pcc:unverified]

Fresh PARI counts give ordinary trace t=33555417 and discriminant t²−4p=−19 ell². The Frobenius/endomorphism order sandwich allows only conductors 1 or ell. PARI computes H−19(X)=X+884736 and class number 1. The source j is its root; the quotient j is different, excluding conductor 1 there. See CM_ARGUMENT.md and executable check-conductors.gp. This is a standard CM argument with arithmetic certificates, not a compiled formal proof. [pcc:unverified]

#pagebreak()
= Full construction, point transport and recovery
The native square-root Vélu schedule covers all 16777709 half-kernel representatives, with baby/giant counts 2896/2896. It emits the exact retained quotient only after its complete-coverage gate. 33 point inputs are original G plus 32 PARI-seeded known-answer targets; every image independently passes membership and the original prime-subgroup scalar identity. This is distinct from the later scalar timer's SHA-derived 32-base panel. [pcc:unverified]

#table(columns: (1.3fr,1fr,1.9fr), inset: 5pt,
 [Actual call], [Elapsed seconds], [Interval],
 [main-b], [8.661], [quotient construction plus 33 images],
 [follow-up repeat2], [8.664], [same exact input/output bytes],
 [follow-up repeat3], [8.457], [same exact input/output bytes],
 [auxiliary probe], [0.578], [quotient plus 1 independent torsion point],
 [normalized dual], [8.594], [dual construction plus 33 images],
)

These are Bash elapsed times under a watchdog of 120 seconds, address-space cap of 12 GiB and shared lock. The follow-up repeats were declared after the first call. Construction and per-point evaluation are not separately priced; RSS was not observed. No other build/audit/benchmark overlapped measured calls. Local L0 timing only; no PMU or reserved-core admission. [pcc:unverified]

The independent auxiliary torsion probe has nontrivial Weil pairing with the first kernel. Its nonzero image supplies the dual kernel, whose exact prime order is independently verified. The double quotient equation equals ell⁴/ell⁶-scaled E. All 33 compositions equal ell²/ell³-scaled [ell]P. With gcd(ell,r)=1, inverse scaling and multiplication by ell⁻¹ mod r recover the original subgroup. The general injectivity also follows because degree ell is coprime to r. Finite controls test implementation; they do not replace the isogeny theorem. [pcc:unverified]

Bounded qualification: 18 direct PARI kernel-polynomial maps,470 point-input evaluations over fields 71/11, square and nonsquare twists, three byte-identical legacy outputs and eight CLI/source rejection controls. Four further wrong-output/truncated-coverage controls reject. Immutable original evaluator and source/binary receipts remain preserved. [pcc:unverified]

Failed attempts are retained: initial build rejected external header macro/warnings; the include boundary was corrected. First control generator lacked its output directory. Initial map launcher lacked /usr/bin/time and exited127 before native execution; a separately named Bash-time launcher completed. No failed launch is counted as a constructed map. [pcc:unverified]

#pagebreak()
= Matched endpoint ECDLP and actual IC attempts
The existing word-field ecbench solver uses original prime r = 5629830654216971, one shared target and two measured rounds per endpoint. The quotient generator is the actual transported G. Four rho recoveries pass parent and independent PARI checks; all eight rho/IC deterministic attempts replay identically. [pcc:unverified]

#table(columns: (1fr,1fr,1fr), inset: 5pt,
 [Endpoint], [Rho mean seconds], [Outcome],
 [source], [7.637], [2 verified],
[quotient], [10.318], [2 verified],
)
#image("rho.svg", width: 100%)

Rho's coordinate hashing gives different walks/counts on the two models despite paired seeds. Canonicalization work is unpriced. Means are descriptive L0 observations; no endpoint ratio, conductor-related cause or speed ranking is admitted. This is four solve attempts of one target, not four independent targets or the full 32-target protocol. [pcc:unverified]

Index calculus actually ran on both word-field endpoints: ic.pipeline, prime-abscissa base size 8, rootfind-subtract m=2,10000 trials and 5 second solver budget. All four attempts exhaust 10000 trials, find zero relations and recover no logarithm. They are retained outcomes, not successful solve times. The four attempts take approximately11–12ms each; incomplete field costs and unpriced root/lookup/guard work remain in raw ledgers. This does not prove impossibility or a general complexity statement. [pcc:unverified]

The initial pilot plan attached IC parameters to a copied rho arm and was rejected before solving. The corrected spec selects by method ID. Inputs, order, seeds, caps, raw outcomes and replay receipts remain committed or in the complete raw bundle. Independent PARI checks confirm all eight target laws and all four recovered scalars. [pcc:unverified]

#pagebreak()
= 32-point scalar panel and requirement status
Existing scalar-timing CLI, counted four-limb library:32 bases per model, six measured rounds plus one warmup, batch64. All 672 batch outputs match independent PARI arithmetic. Native means/medians below exclude warmups. Bars/dots present native statistics; Python performs rendering and unit conversion only. [pcc:unverified]

#image("scalar.svg", width: 100%)
#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Model], [Mean µs], [Median µs], [Measured batches],
 [source], [132.4], [127.9], [192],
[quotient], [133.4], [127.5], [192],
[u=2 control], [131.6], [127.0], [192],
)

Timer includes multiplication, loop/black_box and ledgers; setup/expected-result construction/verification/output are excluded. Repeated batch throughput is not cold single-call latency. This adapter differs from the word-field ECDLP backend, so these microseconds are not rho-step costs. The initial scalar launch used a nonexistent copied binary path and exited127 before producing samples; corrected named launcher preserves binary/spec. [pcc:unverified]

Requirement status: the retained 26-bit-prime crossing has an explicit quotient,33 images, standard CM conductor argument, normalized dual controls and matched pilot ECDLP measurements.32-base scalar measurements are checked. Full32-target ECDLP and admitted performance remain open. The original 80-bit attempt still has no quotient/images after its earlier 60second cap; this milestone does not discharge it. The next larger local-family proposal is NEXT_LOCAL.md, unexecuted here. [pcc:unverified]

Complete raw results, qualification and negative-control directories form one 61700-byte deterministic archive with per-file extraction and byte-identical repack checks. Its SHA256 is e33b1259ac0710fb7a1b8a87bb5091a5c0b8477356276cbd489b485430eec259. It naturally fits the configured 64 KiB data threshold. S3 remains unauthenticated; no remote upload/readback is claimed. Benchmark records remain open for publication; older oversized archives remain separate obligations. [pcc:unverified]

Sources: actual native/ecbench contracts, retained CM kernel/catalog/CONSTRUCTOR.md, PARI/FLINT APIs; Galbraith ANTS introduction/section2 and Galbraith–Gilchrist–Robert section2.1 (links in CM_ARGUMENT.md). KB unavailable. Producer evidence PCC-20261119-3b9f6e/21ae00/1da80c/a266fd/3731f3 remains unsigned/not_covered. No formal proof, world-record or performance-leaderboard promotion is asserted. [pcc:unverified]
