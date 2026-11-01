# N83 current-source root4 frontier reserve

This reserve compares the K3596 root4 standard factor base with the exact
current-source K3500 root4 and K3300 wide3 controls. Its winner, if admitted,
will enter a later fresh same-point bridge against the selected online
candidate from the extended finalist holdout. The comparison uses 16 new public points of
`EC1N83Ckb1a1haacb69ee9f37`: polynomial-basis `GF(2^83)`, modulus terms
`[0,2,4,7,83]`, curve `y² + xy = x³ + x² + 1`, subgroup order
`8569786107849059`, and cofactor `1128547018`. The
[fixture](fresh_manifest.json) fixes every point before either upstream
panel reports a selection. Its target-file SHA-256 is
`fadc5caf48c0241c1bae6c7059de93abbb3cdc14e8900c4d3152b200b2e5ea25`.
The scalar rule is
`1 + little_endian(SHA256(domain || u32_le(i))) mod (r-1)` for `i=0..15`,
with domain `n83-root4-frontier16-batched-roots-20261018-v1`.
The answer sidecar is reserved for independent scalar replay.

The [exclusion audit](fresh_exclusion_audit.json) reports zero overlap with
31,504 prior public N83 points, including the six-arm and extended fixtures,
and zero overlap with the separately archived 32-point finalist fixture.
The exact IC producer source SHA-256 is
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.

| Fixed arm | Usable points | Folded columns | Candidate ID |
| --- | ---: | ---: | --- |
| K3300 wide3 | 547,800 | 3,300 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` |
| K3500 root4 | 581,000 | 3,500 | `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea` |
| K3596 root4 | 596,936 | 3,596 | `IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h3da4e8b31c3f` |

K3596 is at the fixed 29-bit root4 state-ID boundary:
the implementation's state bound is
`83*K*(K-1)/2 + 42*K`, which is `536647262` at K3596 and exceeds
`2^29` at K3597. The checked declarations and exact source bridge are in
the [proof map](PROOF_MAP.md). The candidate retains the historical K3596 ordered base
digest and the current K3500 root4 standard layout. An exposed-point
full-process pilot must independently replay its base, rank, four-point
relation, and scalar before dispatch. The pilot does not enter timing
selection.

The ordered panel is exactly K3300 wide3, K3500 root4, K3596 root4. Freeze
these exact manifests, public points, source, runner, verifier, selector,
workflow and resource caps in a dispatch file before any reserve-point
timing. Do not add or remove an arm using the prior mirrored online means.
An admitted high-K winner does not itself answer the wider N83 comparison:
the later bridge must pair it with the independently selected extended
online candidate on new public points and the same calibrated resources.

Use two distinct overlapping Tier 1 cloud allocations, eight points per
allocation. Rotate arm order by global point index and alternate IC/rho
order by point and fixed-arm parity. Each candidate-point pair starts a
fresh IC process with empty caches and a separate same-point rho process;
run three fixed-point rho calibrations before and after the pair. Preserve
timeouts, OOM, noise and partial receipts. Require at least four visible
CPUs, enough memory for a 15.25 GiB child RSS cap, cgroup and affinity
telemetry, zero observed steal, and at most 10% before/after calibration
median drift for every pair. Cap each IC child at 1,200 seconds, rho and
calibration children at 120 seconds, each producer at 300 minutes, and each
cloud job at 330 minutes. The three-arm schedule has 48 IC, 48
same-point rho and 288 calibration children.

The primary measure is verified single-target online IC wall time after
reusable setup, paired with independently verified rho online wall time for
the same public point. Fresh-process cold cost is a separate selector.
Charge every target-dependent attempt and verification phase. For each
unordered candidate pair, compute the mean of 16 paired
`log(second/first)` online times and the mean of paired
`second-first` cold milliseconds. Enumerate all `2^16` sign flips for each
two-sided test. Select the fastest geometric-mean online candidate only if
it beats every other arm and each relevant pairwise test passes
`0.05/3`; use arithmetic means and paired cold
differences for the separate cold selection. Any missing, capped, failed,
unverified or noisy row leaves both selected IDs null. Report same-point
rho/IC ratios by host, setup/rank/root-index cost, base and relation
identity, pointwise outcomes, and canonical `IC1...W...R1` rows.
