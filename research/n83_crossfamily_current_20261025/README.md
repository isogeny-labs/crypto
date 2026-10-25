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

The measured panel compares the current-source K3300 wide3, K3500 root4,
and K3596 root4 candidates on these same 16 points. The fixed panel has two
eight-point Tier 1 strata. Each candidate-point pair uses a fresh IC process
and an independently verified same-point rho process, with three fixed-point
rho calibrations before and after. The 384 scheduled children rotate arm
order and alternate IC/rho order. The IC child cap is 1,200 seconds, the rho
cap is 120 seconds, the per-stratum producer cap is 300 minutes, and the
host requires at least four visible CPUs and enough memory for the frozen
15.25 GiB child RSS cap. The runner and independent verifier preserve every
failed, incomplete, or noisy pair. The primary measure is each candidate's
verified one-target online wall time; fresh-process cold cost is
supplementary. A later fresh-point bridge will compare an admitted high-K
winner with the nearby-grid small-base selection after its confirmation.
The panel has not yet dispatched or selected a winner.

`run.py` freezes the event schedule, per-child caps and exact IC flags.
`verify.py` independently replays base construction, relations, scalar
answers, phase timing, child resources, Tier 1 telemetry and calibration.
`runpod_runner.py` binds each stratum to a Pod allocation and exact checkout;
`runpod_provider.py` requires two distinct machines with overlapping run
intervals. `aggregate_runpod.py` binds the raw archives to local replay and
applies `select.py`. The selector tests all 16 paired point differences,
uses an exact sign-flip test with a 0.05/3 pairwise threshold, and leaves the
online and cold selections unknown if any required row or host gate fails.
The allocated Pods, binary hashes and source head must be frozen in provider
receipts before running; no such allocation has been made for this panel.

Replay the sealed preparation in a full checkout with Python `blake3==1.0.8`
installed and the archived K3596 dump available:

```sh
python3 -m research.n83_crossfamily_current_20261025.generate_fixture
python3 research/n83_crossfamily_current_20261025/seal_k3500.py \
  /Volumes/data/cryptanalysis/n83_k3596_current_source_base_20261021_1208/base_dump.jsonl \
  --check
python3 -m research.n83_crossfamily_current_20261025.run --preflight --stratum 0
python3 -m research.n83_crossfamily_current_20261025.run --preflight --stratum 1
python3 -m research.n83_crossfamily_current_20261025.test_select
python3 -m research.n83_crossfamily_current_20261025.test_runpod_provider
```
