# N83 five-base Tier 1 tie-break result

The 1,536-point fresh screen admitted all five exact factor bases on one
RunPod Tier 1 instance. K4300 wide3 had the smallest geometric mean online
point interval, 3.889450 ms. The frozen all-pairs selector returned
`BOUNDED_UNKNOWN` with `selected_candidate_id: null`: K4405 wide3 won 782 of
their 1,536 same-point comparisons, against 754 for K4300, with exact
two-sided sign `p = 0.4908866` (required corrected threshold `0.005`).

| Base | Actual usable points | Folded columns | Geometric mean online ms | Median online ms |
| --- | ---: | ---: | ---: | ---: |
| K3072 root11 | 509,952 | 3,072 | 6.961543 | 7.998202 |
| K3500 root4 | 581,000 | 3,500 | 5.832362 | 6.778854 |
| K3300 wide3 | 547,800 | 3,300 | 6.064993 | 6.976049 |
| K4300 wide3 | 713,800 | 4,300 | 3.889450 | 4.282397 |
| K4405 wide3 | 731,230 | 4,405 | 3.945845 | 4.442195 |

The curve is `EC1N83Ckb1a1haacb69ee9f37`, over
`F₂[z]/(z⁸³ + z⁷ + z⁴ + z² + 1)` with subgroup order
`8569786107849059`. Exact candidate IDs and the complete ten-pair table
are in `screen_selection.json`. The screen is a shared-setup, per-point IC
diagnostic. Its timed intervals exclude reusable base/index/log preparation.
The conditional one-target IC/rho holdout was not dispatched because its
frozen selector gate requires a single selected candidate.

The provider-side verifier and independent local archive replay both passed.
The local aggregate records `admission_status: PASS`, `PASS_TIER1`,
`PASS_INDEPENDENT_REPLAY`, zero disqualifying resource telemetry, and
`BOUNDED_UNKNOWN` for selection. The executed IC source file SHA-256 was
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`;
the pinned source commit was `e7634a81c22b60ad37e5d6bdb817a3b84867ae86`.
The raw 513-file archive SHA-256 is
`352766bd9610615b10b0aa8f6a10d28a4880f0cb8851c33ee300a199bb9e6ece`.
It and the extracted run tree are retained under
`/Volumes/SSD990-2/tmp/n83_tiebreak_results_v2retry_20261020/` because the
145 MB archive is not part of this Git commit. Reproduce the aggregate with
`aggregate_runpod.py` using the pinned provider snapshot, this raw archive,
the preceding distribution aggregate and archive, and the sealed source
commit as specified in `PROTOCOL.md`.

The Pod requested provider stop at `2026-10-20T08:57:30Z` but remained
reachable afterward; provider absence was recorded at
`2026-10-20T15:58:56.317004Z`. The conservative allocation-to-absence upper
charge is $1.433076 for this Pod and $5.710216 across the campaign, exceeding
the approved $5 upper cap by $0.710216. The actual invoice is unknown.
`spend_audit.json` and `provider_post_terminate.json` preserve the calculation
and observed absence. The allocation command now rejects any new Pod under
this campaign. The archived dispatch-time allocator bytes remain separately
hash-pinned so the original provider snapshot can still be replayed.

The next factor-base step is a capacity and full-pipeline comparison beyond
K4405 using local resources or a separately authorized, budgeted compute
envelope. Any primary performance result requires a fresh single-target IC
online interval, an independently verified same-point rho interval, and the
Tier 1 paired host receipts.
