# CT215 — measured negative toy BSGS transfer cost; keep full-solve gate open

Date: 2026-11-18. [pcc:unverified]. Existing draft [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on [#4697](https://github.com/isogeny-labs/crypto/pull/4697).

## New executed evidence (not a speedup)

The exact CT214 workflow [run #44386280870](https://github.com/isogeny-labs/crypto/actions/runs/44386280870), native [job #135091642965](https://github.com/isogeny-labs/crypto/actions/runs/44386280870/job/135091642965), **completed successfully on source commit `c3128076426a5b71149b652a53c5028bd82c0eef`**. The job compiled release Rust, passed 2/2 pre-existing native toy correctness tests, executed the new paired Rust test (1/1), and printed a successful CT214 raw-log validator receipt. The independent small-field oracle also passed. This supersedes CT214's unobserved timing status, but does not upgrade PCC.

Exactly 301 public known-answer **pairs**, seven repetitions of 43 distinct targets on the fixed group `E/F_31: y^2=x^3+3` of order 43, were recorded; both source BSGS and Jacobian BSGS recovered and verified the scalars. The Jacobian arm also pays fresh cover setup, two pullbacks, BSGS, pushforward norm and verification. The source Q construction is outside both clocks.

| Metric, summed/median within this single hosted CI run | Direct elliptic BSGS | Genus-2 pullback + Jacobian BSGS |
|---|---:|---:|
| Sum of 301 elapsed durations | **2,844,491 ns** | **175,600,609 ns** |
| Median per target | **9,304 ns** | **588,837 ns** |
| BSGS group additions | 4,690 | 4,690 |
| Baby entries per target | 7 | 7 |

**Measured direct/transferred elapsed ratio = 0.0161986397211; the transferred toy path is 61.7336× *slower*, not faster.** Direct was faster in every one of the 301 recorded pairs. The two equal operation counts refer to different groups and do not establish equal physical group-operation cost. The same Rust source algorithm uses a linear baby-vector scan; it is not the strongest optimized BSGS or an index-calculus comparison.

Independent arithmetic of the raw row durations recovered the aggregate totals and yielded the per-round direct/transferred ratios:
`0.0163352, 0.0166761, 0.0164035, 0.0164163, 0.0157719, 0.0160619, 0.0157680`.
There is measurable order sensitivity: for 151 direct-first pairs the ratio is `0.0188627`; for 150 Jacobian-first pairs it is `0.0135380`. Thus timing-order and cache effects remain, despite alternating order. A deterministic 12,000-replicate target-cluster resampling of the observed seven-round target clusters returns descriptive 2.5%/97.5% quantiles `0.0158729–0.0165240` for the ratio (equivalently `60.52–63.00×` slowing), **not** a population CI, isolated-runner uncertainty bound or prospective performance guarantee. Repeated targets and one hosted runner prohibit that interpretation.

## Independent accounting route and reproducibility

The previously committed `ct214_cost_log_validator.py` validated 301 pair rows, exact target schedule, 4,690/4,690 additions, aggregate totals and ratio. CT215 separately adds `ct215_paired_cost_audit.py`, an additional transparent producer-side calculation of order sensitivity, round sensitivity, 301/301 pair comparisons and target-cluster resampling, with synthetic tamper controls (local `--self-test` passed). It is wired into the bounded CT205 GitHub-hosted job after the Rust log is produced. **CT215's own new exact-head CI has no completed verdict at report publication**; the executed timings are from the earlier exact CT214 head, not a claimed CT215 rerun.

Reproduce from the original GitHub job log:

```sh
gh run view 44386280870 -R isogeny-labs/crypto --job 135091642965 --log > /tmp/ct214-native.log
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct214_cost_log_validator.py /tmp/ct214-native.log
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct215_paired_cost_audit.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct215_paired_cost_audit.py /tmp/ct214-native.log
```

Raw logs are held in GitHub Actions, not in an immutable independent signed PCC evidence package. A log auditor can check internal arithmetic and tampering but cannot establish clock integrity, input provenance or memory use.

## Scientific decision and next unblocked lead

1. **Negative at this toy scope:** transfer followed by generic BSGS should not be pursued as a performance lead. An injective DLP-preserving map of the order-43 subgroup does not itself shrink the √43 generic search; more expensive Jacobian group operations and transfer/setup make this measured path worse. This is not an obstruction to a different non-generic index-calculus solver, genus-4 Jacobian, or different cover.
2. **Original owner priority remains blocked:** actual norm-point reuse pricing for `C169`, both `C13` hops, `D` and `N2_actual` on the authenticated 695/701-record source model, followed by paired same-host *complete* cold single-target transfer + destination IC versus strongest rho/BSGS/IC. Repository notes still report original-model S3 HeadObject 403. This run did **not** renew or bypass credentials, and the toy result is no substitution for the original model.
3. **Avoid duplicate work:** [crypto #4215](https://github.com/isogeny-labs/crypto/pull/4215) owns prospective counter differences; [cryptanalysis #1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766) owns the independent full-solve adjudicator; [crypto #4697](https://github.com/isogeny-labs/crypto/pull/4697) and [cryptanalysis #1770](https://github.com/isogeny-labs/cryptanalysis/pull/1770) own general map arithmetic and source parity. The [genus-4 follow-up #4693](https://github.com/isogeny-labs/crypto/pull/4693) is separately benchmarking much closer to the rho frontier but is not a demonstrated transferable solver for prime-field ECDLP. The next substantial performance experiment should land in the existing relevant lane after authentic model access.
4. **Mandatory success gate:** identical verified logs and cold one-target targets, all setup/transfer/relation finding/LA/verification, working memory and uncertainty counted, optimized rho/BSGS/IC reference panel and independent PCC. No result here satisfies it.

Repository `AGENTS.md` reviewed; shared research-KB was not attached in this connector runtime and no novelty claim was made. No EKS/Spot paid job, protected merge, leaderboard promotion or full ECDLP advantage. **[pcc:unverified].**
