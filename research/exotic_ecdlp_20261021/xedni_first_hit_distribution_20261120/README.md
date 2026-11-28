# Exact first-hit distribution for height-ordered rational lifts

Across every nonidentity target on the two measured reductions, the fixed
height order requires a mean of **7,637.724** candidate evaluations at
`p=1531` and **1,321.486** at `p=2287`. Those means are respectively
**9.836×** and **1.174×** the sharp sequential-search floor `q/2` for
`q-1` uniformly selected nonidentity targets. The full first-hit maps are
verified by an independent finite-field group-law replay. A checked Lean
theorem proves the floor for any positive, injective first-hit map, alongside
the prefix coverage bound.

## Frozen census and exact result

The [protocol](protocol.json) and [runner](run.py) were pushed in
`04f41fd1a18` before execution. They reuse the two first-full coefficient
radii and the same 128-bit height-pairing candidate order as the previous
construction and [six fresh-target comparisons](../xedni_fresh_target_pair_20261120/README.md).
The separate question here is the exact candidate-count distribution for a
uniform public point: each scalar `k=1,…,q−1` occurs once as a target. The
scalar labels are an audit oracle for the exhaustive census, not an input to
the online solvers.

| p | q | Radius | Targets | Exact sum of first-hit indices | Exact mean | Median | 90th percentile | Maximum | Mean / `q/2` |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1531 | 1553 | 65 | 1,552 | 11,853,748 | 11,853,748 / 1,552 ≈ 7,637.724 | 7,559.5 | 14,687 | 16,510 | 9.836 |
| 2287 | 2251 | 54 | 2,250 | 2,973,343 | 2,973,343 / 2,250 ≈ 1,321.486 | 1,125.5 | 2,267 | 10,522 | 1.174 |

The [result](result.json) retains exact numerators and denominators for the
mean and ratio, plus median-times-two, nearest-rank 90th percentile, source
digests, and the full-coverage index. All
[3,802 first-hit rows](first-hit-rows.jsonl.gz) are stored in deterministic
gzip, with hashes of both compressed and raw contents. The maximum indices
match the independently measured construction prefixes. All six prior
fresh-target first-match labels and indices also match this census.

The [independent audit](audit.py) sorts candidates with rational-number
scores, builds the actual finite-field `aQ̄+bR̄` point at every evaluated
label using a separate integer group law, maps each point back to its scalar,
and reproduces every target row and statistic. Its [result](audit-result.json)
is `PASS` for 16,510 and 10,522 candidate evaluations and all 3,802 target
first hits.

## Proof and cost interpretation

For `n` distinct target points, one target-independent candidate evaluation
can first hit at most one target. Thus its first-hit indices are distinct
positive integers. Ordering them gives `t_j≥j` for `j=1,…,n`, so their sum
is at least `n(n+1)/2` and their uniform mean is at least `(n+1)/2`. Here
`n=q−1`, giving the exact floor `q/2`. The floor is attained by an order
that visits every target once before any duplicate.

[FirstHitBound.lean](formal/FirstHitBound.lean) checks all three structural
steps: `prefix_hit_count_le` bounds distinct hits in a prefix,
`exists_unhit_before_n` supplies a missing target before `n` candidates,
and `first_hit_sum_floor` proves `n(n+1)≤2∑t_i` for positive injective
first-hit indices. The [pinned build](formal/build.sh) succeeds and
[#print axioms output](formal/build-output.txt) lists only `propext`,
`Classical.choice`, and `Quot.sound`. The mapping from the actual elliptic
curve reductions to scalar labels is checked by the independent finite-field
audit. `lean4checker --fresh` was unavailable on this host.

This is a secondary exact target-distribution analysis after the prior
single-target measurements. It counts candidates; it does not divide shared
setup wall time by target count or promote an ordinary-host IC/rho speedup.
The large gap at `p=1531` identifies a concrete next experiment: after the
verified `R̄=λQ̄` relation is prepared, skip duplicate reduced residues in
the height order and measure the resulting one-target candidate counts and
rational-coordinate costs. That optimization has a setup and bookkeeping
cost, which must be charged separately.

## Replay

From `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119`:

```sh
python3 research/exotic_ecdlp_20261021/xedni_first_hit_distribution_20261120/run.py > research/exotic_ecdlp_20261021/xedni_first_hit_distribution_20261120/result.json
python3 research/exotic_ecdlp_20261021/xedni_first_hit_distribution_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_first_hit_distribution_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_first_hit_distribution_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_first_hit_distribution_20261120/formal/build.sh
```
