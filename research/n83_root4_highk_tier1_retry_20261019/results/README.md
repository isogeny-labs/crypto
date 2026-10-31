# N83 high-K Tier 1 eight-point result

The three exact wide3 bases completed all eight fresh same-point IC/rho pairs
on one RunPod V100 SXM2 allocation. The archived producer result, local
independent arithmetic replay, allocation proof, source/binary binding,
resource gate, and 20% A/A gate passed. The paired online screen remains
`BOUNDED_UNKNOWN`, with `selected_candidate_id: null`. The separate cold
process selector selected K3300.

| Base | Usable points | Folded columns | Geometric mean online IC, ms | Mean cold process, s | Median paired rho/IC online |
| --- | ---: | ---: | ---: | ---: | ---: |
| K3300 wide3 | 547,800 | 3,300 | 10.009 | 605.775 | 207.28 |
| K4300 wide3 | 713,800 | 4,300 | 4.407 | 1,080.899 | 589.24 |
| K4405 wide3 | 731,230 | 4,405 | 6.781 | 1,146.875 | 371.36 |

Every one of the 24 single-target online IC intervals was shorter than its
verified same-point rho interval. The online pairwise exact sign-flip p-values
were 0.1015625 (K3300/K4300), 0.390625 (K3300/K4405), and 0.4453125
(K4300/K4405); each exceeds the prespecified Bonferroni threshold 0.05/3.
These eight points therefore do not select an online factor base. The
geometric-mean fastest arm, K4300, is descriptive only. K3300 is the admitted
cold-process choice for this exact source and resource envelope.

The immutable raw archive is retained locally at
`/Volumes/SSD990/llm/tmp/n83_retry_results_20261019/raw_results.tar.gz` with
SHA-256 `d4f41f87706b6ea433bb4d3b07037a71cd390bb0ebfb905df5677590f70674f3`.
It contains 1,384 files; the local replay exited 0 and produced the
[`validated.json`](validated.json) and [`selection.json`](selection.json)
records here. The source commit is
`e7634a81c22b60ad37e5d6bdb817a3b84867ae86`, and the IC source SHA-256
is `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The provider receipts record the single allocation, stop, termination, and
post-termination absence. The next experiment compares five exact bases over
192 additional public points, then performs a fresh one-target holdout if
its screen selects a unique arm.
