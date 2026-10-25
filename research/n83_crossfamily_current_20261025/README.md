# N83 current-source cross-family comparison

The current-source K3500 root4 factor base has 581,000 usable subgroup points
and 3,500 folded columns. Its first 3,500 signed-Frobenius orbits reconstruct
the historical K3500 ordered base exactly: the BLAKE3 base hash, ordered-dump
SHA-256, and point-set SHA-256 all match. The source-updated candidate is
`IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0h0de90f01951d`.
The source update changes its candidate ID; this candidate has no
current-source timing yet. `seal_k3500.py` reproduces both the identity and
the receipt from the archived current-source K3596 base dump.

Sixteen new public points of `EC1N83Ckb1a1haacb69ee9f37` are sealed in
`targets.jsonl`, with scalar replay sidecars and 16 separate canonical
single-target workload IDs. `generate_fixture.py` deterministically
regenerates the points from the domain
`n83-crossfamily-current16-20261025-v1`. The exclusion audit checks all
35,172 earlier distinct public points in the prior inventory, nearby-grid
screen, and sealed nearby-grid confirmation; it reports zero overlap and zero
duplicates. Each point belongs to the subgroup of order `8569786107849059`.

The next measured panel will pair the current-source K3300 wide3, K3500
root4, and K3596 root4 candidates with the selected nearby-grid small-base
candidate on these same 16 points. The small-base arm will be bound only
after the 19-base screen and its confirmation pass their frozen gates. A
dispatch must freeze the arm IDs, source/binary hashes, exact point order,
host allocations, caps, runner, and verifier before target timings start.
The primary measure is each candidate's verified one-target online wall time
against independently verified same-point rho; cold fresh-process costs
remain supplementary. Alternate IC/rho order and arm order, record all
failed/noisy pairs, and require the Tier 1 host envelope in
`docs/IC_BENCHMARK_TIERS.md`. The three high-K candidate identities are
prepared, but this panel has not yet dispatched or selected a winner.

Replay the sealed preparation in a full checkout with Python `blake3==1.0.8`
installed and the archived K3596 dump available:

```sh
python3 -m research.n83_crossfamily_current_20261025.generate_fixture
python3 research/n83_crossfamily_current_20261025/seal_k3500.py \
  /Volumes/data/cryptanalysis/n83_k3596_current_source_base_20261021_1208/base_dump.jsonl \
  --check
```
