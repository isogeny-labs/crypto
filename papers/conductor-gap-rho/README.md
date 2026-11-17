# Pollard Rho Across a Large Prime Conductor Gap: A Large-Sample Measurement and a Formal Invariance

`conductor_gap_rho.tex` writes up
`../../research/conductor_gap_rho_large_sample_20261018/`: the frozen-protocol
measurement of Pollard rho (`ecbench rho.negation`) on the crater (End = O_K)
against the floors (End = Z + ℓ·O_K) of prime conductor gaps at
ℓ = 8209, 131101, ≈2^20 (hard kernels) and ≈2^25 (easy kernel), with
isomorphic-model negative controls, a √2 positive control, two pre-registered
follow-ups, and the Lean theorem `RepresentationHashInvariance.lean`.

**Status**: research draft, anonymized, not peer-reviewed, not submitted, not
posted, not cleared for either until the items under "Before submission" are
closed. Restore authors with `make deanonymize` after creating an `AUTHORS`
file (see `../AUTHORS.example`).

**Change class** (per `AGENTS.md`): *null result on a measured question*, with
one observation about the harness's partition hash. Not an advance, not an
engineering gain, not a relabelling, not an accounting change. No security claim
about any deployed curve.

## What it says

- In the hard-gap regime (kernel field degree 2736 … ℓ−1, where crossing the
  gap costs more than the DLP): floor/crater S ratio 0.998 [0.985, 1.011] at
  ℓ = 8209 (12,288 paired solves), 1.005 [0.960, 1.052] at ℓ = 131101,
  0.982 [0.940, 1.028] at ℓ ≈ 2^20. The earlier 1.22 [1.005, 1.46] lead at
  ℓ = 8209 (`conductor_gap_cm_pairs_20261010`, 4 targets, seed 1) is closed.
- Positive control rho.plain/rho.negation = 1.36–1.45 in every S session
  (expected √2); twin negative controls 1.004 [0.990, 1.019].
- Easy-gap tier (ℓ ≈ 2^25, 53-bit `cm_orbits` pairs): floors 8 % [3 %, 13 %]
  cheaper over 48 pair-samples, reproduced under fresh targets, but between
  re-coordinatised endpoints the same gap measures 1.040 [0.965, 1.119]; the
  shift belongs to the coordinate models under the representation hash, not to
  End(E). The earlier post-hoc 1.17 on these pairs is closed the same way.
- Theorem (Lean 4.34.1, no Mathlib, axioms `propext`, `Quot.sound`): along any
  isomorphism of groups, an r-adding walk whose partition hashes the
  representation has a transported walk with the same trajectory and ρ-length,
  and the transport is a bijection of the walk space. Representation-hashing rho
  is conductor-blind in distribution; a fixed partition is the only channel.

Every number cites a committed artefact (`crypto:research/conductor_gap_rho_large_sample_20261018/…`)
in its table caption or text.

## What it does not say

- Nothing about non-generic algorithms beyond the transfer bound (cost of one
  ℓ-isogeny, ≈ ℓ^2.6 for a non-rational kernel); no proof of equal ECDLP
  hardness, which exists for no curve.
- Nothing about index calculus (not measured here), binary or extension
  fields, other discriminants, or other walks.
- No wall-time result: the host was shared and every run is L0.

## Build

No TeX Live is installed on the measurement host; the committed PDF was built
with Tectonic 0.15.0 (`tectonic conductor_gap_rho.tex`), which runs the
pdflatex-compatible engine and BibTeX. With TeX Live, `make` and `make check`
use `../paper.mk` as for every other paper here. Figures are the study's
`figures/*.pdf`, written by `plot.py` from `results/summary.json` and the
session records.

## Reproduction pointers

- Study directory: `research/conductor_gap_rho_large_sample_20261018/`
  (PROTOCOL.md, AMENDMENTS.md, FROZEN.sha256*, specs/, sessions/,
  sessions_packed/, audits/, results/, figures/, reports/run-*/, report.pdf).
- Lean: `lean/RepresentationHashInvariance.lean`, checked with
  `~/.elan/bin/lean RepresentationHashInvariance.lean`; `lean/build.log` is the
  `#print axioms` output.
- Harness: `ecbench` binary sha256 `ee0af723cf6c1064…`, source at or before
  commit `42ab2c57`.

## Before submission

- [ ] Independent replay of at least one session per tier on another host
      (`ecbench-independent-runner`); the audits here are same-host.
- [ ] A Lean axiom-audit CI job for `RepresentationHashInvariance.lean` as for
      the earlier conductor-gap Lean files (the kept `build.log` is a local run).
- [ ] Literature check of the bibliography entries against the publishers'
      records (volumes and pages were written from memory).
- [ ] Decide whether the tier-L partition-hash observation deserves its own
      follow-up (a hash with a per-session salt would test it directly).
- [ ] De-anonymize (`make deanonymize`) and run `make check` under TeX Live.
