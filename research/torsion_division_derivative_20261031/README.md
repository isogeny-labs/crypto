# H109 — torsion-local division-polynomial derivative oracle

Status: **executed; scoped null result** (2026-11-01).

H109 tested whether the invariant derivative of the order-`r` division
polynomial at a public target exposes scalar parity through a constant-size
two-power character formula. The mechanism is distinct from the paired
H107/H108 character-codeword spans; novelty is not established.

## Result

- 9,096/9,096 target rows passed every arithmetic and algebraic control.
- 412,328 non-base recurrence nodes, 10,674,040 field multiplications, and
  211,712 inversions were charged.
- All 8,832 coefficient tuples across 28 rows were enumerated.
- Exact formulas: **0/28**; therefore the frozen decoder gate did not open.
- Candidate best-match range: 0.500000–0.611111.
- Thirty-two symmetry-matched controls ranged 0.500000–0.629630 and likewise
  produced no exact formula.
- E101 candidate/control mean: 0.500000/0.500000, plus-one empirical p=1.
- E127 candidate/control mean: 0.574074/0.560185, plus-one empirical p=7/17.

This rejects only the frozen five-input constant-size phase family. It does
not rule out other uses of torsion-local data and does not establish a generic
ECDLP lower bound, a novel algorithm, or subexponential runtime.

## Evidence

- [Executed protocol and report](PROTOCOL.md)
- [Transfer/cost assessment](assessment.json)
- [Portable experiment](run.py)
- [Pinned H107/H108 baselines](baselines/)
- [Compressed full result](results/torsion-division-derivative.json.gz)
- [Standalone Lean interfaces](formal/H109TorsionDerivative.lean)
- [Editable evidence diagram](figures/h109-torsion-derivative-flow.svg)
- [Rendered four-page report](h109-torsion-derivative-preregistration.pdf)
- [Independent replay PR](https://github.com/isogeny-labs/cryptanalysis/pull/1342)

## Hashes

- full JSON: `940123c44c6194b247fab3596a9ed2e331ee7e01b2fb8830fc7e8465e354993a`
- scientific content: `3991b25d2a2ddac0ebb1d25370c8b110051a40acca64a6bda3dd3baba4191b69`
- deterministic gzip: `f4f815d9938794f814b6f289c78729e1b571bb2ff4c897829d6d84d8508b5bf4`
- runner: `bc54286d5e7245985d7637bf5e20d757627fcf87d3e39c0ee0b867f1a42c9687`
- Lean source: `70792ea2c95f6b035f299a9963728b2c49ccb492ba932d00d0a7434c42e14c0c`

See [REPRODUCTION.md](REPRODUCTION.md) for the exact replay gates.
