# N83 high-K distribution screen: 192 verified point pairs, selector unresolved

The Tier 1 RunPod screen admitted all five factor-base arms and independently
replayed 192 public targets, their four-point sums and the paired rho children.
K4405 wide3 had the lowest descriptive median online time, 5.9855 ms, but
its pointwise comparison with K4300 wide3 was 96–96 (exact two-sided sign
test `p=1.0`). The frozen ten-comparison threshold was `0.005`, so the
selector recorded `selected_candidate_id: null` and `BOUNDED_UNKNOWN`.
The screen holdout was therefore not dispatched. The conditional fresh-point
tie-break is the next measurement.

| Arm | Exact candidate ID | Median online ms | Geometric mean online ms |
| --- | --- | ---: | ---: |
| K3072 root11 | `IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e` | 13.7366 | 13.2984 |
| K3500 root4 | `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea` | 9.9805 | 9.0052 |
| K3300 wide3 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` | 11.0512 | 10.1105 |
| K4300 wide3 | `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77` | 6.8222 | 6.1721 |
| K4405 wide3 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7` | 5.9855 | 5.9607 |

These are secondary shared-setup per-point intervals. The primary metric
remains one previously unseen target's verified online IC time against rho
on that same point. The screen's null selection does not supply that primary
measurement. The exact curve is `EC1N83Ckb1a1haacb69ee9f37`, an `a=1`
model with a 53-bit subgroup. The source commit was
`e7634a81c22b60ad37e5d6bdb817a3b84867ae86`; the executed IC source
SHA-256 was `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
This identity is separate from the full-width `a=0` N83 campaign.

The admitted host was RunPod Pod `yajulte7twpr9p`, machine
`p3ho0s1tf65b`, an Intel Xeon Silver 4214 with frozen affinity CPUs
`[0,1,2,3]`. Its provider termination receipt records the Pod absent at
`2026-10-20T04:31:24.216912+00:00`. The result was replayed locally by
`aggregate_runpod.py` from the archive and provider snapshot; it reported
`SCREEN_BOUNDED_UNKNOWN` and `PASS` admission.

The original Pod-side verifier invocation failed with
`ModuleNotFoundError: No module named research` because it ran the Python
file directly. The same retained child tree passed package-module
verification with `PYTHONPATH=/workspace/crypto`; both logs and the repair
receipt are retained here. The launcher and local aggregator now use module
invocation. The repaired raw archive SHA-256 is
`4fec165637a10a632fbc85597124d4ccc9ed48eecfdbcffa45680153decf3340`,
and the independently produced `aggregate.json` SHA-256 is
`117cc0ab8f919bad5ab44aab1c918ce8cd277c4805cdf811f30c8d1171d7fa59`.
The 144 MB raw archive, all child receipts and provider snapshot are retained
under `/Volumes/SSD990-2/tmp/n83_distribution_results_20261020/`; the
compact aggregate and failure/repair receipts are versioned beside this note.
