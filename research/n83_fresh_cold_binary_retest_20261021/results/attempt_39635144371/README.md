# N83 five-base retest: K1107 selected for cold process cost

The sealed [GitHub Actions run 39635144371](https://github.com/aburan28/crypto/actions/runs/39635144371) measured five exact factor bases on 16 fresh public N83 points across two different Tier 1 cloud instances. Both hosts used the same source and executed IC/rho binary pair. Independent local arithmetic replay admitted all 80 IC arms and all 80 same-point rho references; both host strata passed Tier 1 telemetry and every A/A calibration pair. The selector was recomputed from [validated rows](validated_rows.json), and its sorted, indented JSON output matches the [saved selection](selection.json) byte for byte.

| Base | Usable points | Folded columns | Mean cold process | Geometric mean one-target online | Cold selector |
| --- | ---: | ---: | ---: | ---: | --- |
| K992/root11 | 164,672 | 992 | 127.179 s | 48.184 ms | — |
| K1024/root11 | 169,984 | 1,024 | 123.411 s | 42.968 ms | — |
| K1107/root11 | 183,762 | 1,107 | **116.326 s** | 61.108 ms | **SELECTED** |
| K2304/root11 | 382,464 | 2,304 | 222.180 s | 18.719 ms | — |
| K3300/wide3 | 547,800 | 3,300 | 524.301 s | 10.070 ms | — |

K1107 reduces mean cold process time by 7.086 s (5.741%) versus the next-fastest K1024 arm, a 1.0609× ratio of mean costs. K1107 beat each other arm in cold process wall time on all 16 paired targets. Each of its four two-sided paired sign-flip tests gives `p = 3.0517578125e-05`, below the frozen Bonferroni threshold `0.005`. The cold selector is `SELECTED` for `IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0he423fcbc5668`. The one-target online selector remains `BOUNDED_UNKNOWN` with a null selected ID; K3300's online mean is descriptive; its paired online comparison with K2304 has `p = 0.075469970703125`, above the frozen threshold. The table reports the two different accounting intervals separately. Online begins after reusable preparation; cold process includes fresh factor-base, index and log preparation as well as target work. The per-point rows retain stage costs, exact candidates, correctness and paired rho timing.

The [custody receipt](custody.json) binds the two independent runner identities, raw ZIP digests, replay analyses, frozen source and targets, and aggregate hashes. [Provider jobs](provider_jobs.json) retain their overlapping allocation times and [binary hashes](executed_binaries.sha256) identify the shared executable pair. [Provider archive custody](provider_archive_custody.json) records provider-matching S3 download hashes and the two raw ZIP URIs. The provider measurement head was `48501c90100223ebf9597ed223692c0f24ce1f8f`; the added evidence commit did not change the measured executable. The previous [compiler-drift attempt](../../../n83_fresh_cold_finalists_20261019/results/attempt_39282996293/README.md) remains a distinct failed record with null selections.

This selection covers the five frozen bases on this source, target law, and Tier 1 envelope. The [nearby-K current-source comparison](https://github.com/aburan28/crypto/pull/2396) and [K5500 full-process path](https://github.com/aburan28/crypto/pull/2394) extend the search; their results are separate experiments.
