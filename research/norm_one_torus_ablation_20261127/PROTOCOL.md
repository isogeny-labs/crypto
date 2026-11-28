# Protocol (frozen before the recorded runs)

Follow-up to merged PR #5229. Policy: `isogeny-labs/experiment-seeds/v1`.

## Inputs
- Public input: `research/norm_one_torus_rust_mitm_20261126/public-precommit/manifest.json`
  (sha256 `21c171e847faca3bf01a99374ce2eaeec6c1061e2ea9ba7c7f4fc7e33f3597e3`), unchanged.
- Recall input: same master seed, fresh domain
  `isogeny-labs/norm-one-torus-mitm/ablation-recall/2026-11-27/v1`,
  4,000 random + 64 planted targets.
- Query block size 512, inherited from the manifest; not retuned.
- Run-order shuffle LCG seeds: `0x5229` (ablate), `0xabc` (xonly); bootstrap LCG seed
  `0x5262`; 2,000 round-paired resamples. All of these are fixed in `harness/src/main.rs`.

## Gate before any timing
`fidelity` must reproduce every op counter and digest in the library-produced
`public-precommit/results.csv` (36 rows) and every point, witness, digest and
manifest hash in `public-precommit/certificates.json` (40 targets). Every arm must
replay its witnesses with the affine law and return the PR-control hit mask.

## Arms
2×2×3×2 factorial = 24 arms (normalization × table × inversion × hasher), plus
the x-only affine query arm. Named comparisons:
- PR control: indiv + full + i128 EEA + SipHash (as in #5229)
- Fair control: indiv + full + i64 EEA + Fx
- PR candidate: batch + quotient + i128 EEA + SipHash (as in #5229)
- x-only: batch + quotient + i64 EEA + Fx, affine x-only query

## Primary endpoint
Median setup+query wall time on the public input, as a ratio to the fair control,
with a round-paired bootstrap 95% CI. Run: 41 rounds after one warm-up per arm,
order shuffled each round, pinned with `taskset -c 1`.

## Falsification targets
- The PR's lane is not faster than the fair control if the CI on the
  fair-control ratio includes 1.0.
- The x-only arm is not an improvement if its CI against the best PR-style arm
  (batch + quotient + i64 + Fx) includes 1.0, or if any hit mask or digest differs.

## Disclosed development runs
Before this freeze, an exploratory copy of the same harness (outside the repository)
ran the same arms on the same public input on 2026-11-27. Those numbers are not
the recorded result. Nothing was tuned after seeing them; the block size and arm
definitions are unchanged.

## Status
Host: 2 vCPU Intel Xeon @ 2.30 GHz, 7 GB RAM, not isolated. This is a local
diagnostic, not an admitted benchmark: there is no Iso Lab/EKS run, no matched rho
reference and no keyring signature. [pcc:unverified]
