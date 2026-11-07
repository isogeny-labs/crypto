# H110 — canonical p²-lift defect cocycle

Status: **executed; scoped low-degree bridge rejected** (2026-11-01).

H110 measured the first formal-layer defect of a deterministic Hensel section
into `E(Z/p²Z)`. The lift, reduction-kernel path, and formal coordinate are
executable, but the candidate scalar polynomial had maximum possible degree on
every frozen row.

## Result

- 9,096/9,096 canonical targets passed all transfer/arithmetic checks.
- 28/28 scalar words had degree `r-2`: 81 through 945.
- Frozen degree gates were only 49 through 100; therefore 0/28 rows passed.
- All 32 random-section/permutation controls also had maximum degree.
- Both coordinate-isomorphic controls retained maximum degree.
- Every solved torsion lift had `tau=0`, confirming that the homomorphic lift
  kills rather than exposes the formal observable.
- No root/EC decoder opened.

This rejects only the frozen coordinate-section scalar-polynomial bridge. It
does not establish a generic ECDLP lower bound or rule out other lift-derived
invariants.

## Evidence

- [Executed protocol](PROTOCOL.md)
- [Transfer/cost assessment](assessment.json)
- [Portable experiment](run.py)
- [Compressed full result](results/p2-lift-defect-cocycle.json.gz)
- [Standalone Lean interfaces](formal/H110P2LiftDefect.lean)
- [Editable typed correspondence](figures/h110-p2-lift-defect-flow.svg)
- [Four-page report](h110-p2-lift-defect-preregistration.pdf)
- [Independent replay PR](https://github.com/isogeny-labs/cryptanalysis/pull/1346)

## Hashes

- result JSON: `7d6c390d0156a4ed101bddd0969132a85505fa884282ef505dc4621b7852124e`
- scientific content: `3397823fe3f55fe008797bb87b31324b7ae33efb5699fe479e8e58f5832db4de`
- deterministic gzip: `a69d06462068b9ad5ac16d46d3d6e29b7e05cbad0bff7a75b3dce59c5cbee7d8`
- runner: `8188b197ede45f994856e20c2e78214faa5d28fa79ef2c250e89927043ae73c8`
- Lean source: `fc24d97a72e060574882e23b12705a4693e5323ad0b5c5b8f25b036dc124ef5e`

See [REPRODUCTION.md](REPRODUCTION.md).
