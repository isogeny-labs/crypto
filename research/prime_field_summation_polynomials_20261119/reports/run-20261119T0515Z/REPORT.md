---
title: "Prime-field summation-polynomial structure lab"
subtitle: "Exploratory imported campaign — run 20261119T0515Z"
date: "2026-11-19"
geometry: margin=0.75in
fontsize: 10pt
---

# Evidence status

This report packages recorded measurements from source snapshot
`71b769857f99f57e0e3490ef7ced32d43b307a1d`. It is an exploratory import, not a
canonical repository benchmark. Every timing, correctness, and algebraic claim
is repository-unverified `[pcc:unverified]`. There is no `VALIDATION.json`,
independent replay, matched external-tool comparison, or end-to-end ECDLP
measurement.

The recorded environment is Linux x86-64 on an AMD EPYC 9V74 host, g++ 14.2.0,
`-O3 -march=native`, seed `6000285944033488934`. The producer recorded
`git_revision=unversioned`; isolation level, interval estimates, and run-to-run
spread were not recorded. Values below are aggregate means from one process.

# What was measured

![Recorded stage means. Raw nanosecond values, cases, repetitions, and source
rows are preserved in `stage-timings.data.csv`. Timed boundaries differ across
panels, so ratios are meaningful only within a panel.](stage-timings.png){ width=100% }

At degree 128, the recorded structure-aware all-roots paths took about
10.03--10.54 microseconds while the included coefficient-only generic
factorization control took 4.32--14.62 milliseconds `[pcc:unverified]`.
At degree 4096, the recorded subproduct-tree coefficient ratios were 3.248,
3.212, and 13.417 relative to quadratic expansion for the three fields
`[pcc:unverified]`. For a one-target degree-4096 workload at `p=1,000,003`, the
recorded all-roots/target-search ratios were 9.331 for a miss and 8.727 for a
hit; the all-roots/one-signed-sum ratios were 197.007 and 237.883
`[pcc:unverified]`.

These are stage-level comparisons, not whole-algorithm comparisons. The
structure-aware paths require the curve plus lifted point-origin inputs and do
not factor arbitrary coefficient vectors.

# Algorithm structure

![The experimental paths share lifted point-origin inputs but answer different
questions. The dashed boundary lists work excluded from the measurements and
claims.](root-finding-structure.png){ width=95% }

The imported implementation pursues separate all-roots, one-root, target-set,
coefficient-construction, fixed-reducer, workspace, and automorphism paths. The
automorphism quotient is a useful negative control: the recorded evaluator
regressed beyond tiny degrees even when the algebraic orbit count shrank
`[pcc:unverified]`. Explicit SIMD was investigated but not implemented.

# Local integration checks

The native `make test` command passed after import and reported 900 randomized
root-method checks, 900 one-root checks, 900 target hits, 900 target misses,
31 targeted degeneracies, 135 coefficient-method checks, 2,268,486 reducer
product checks, 1,225 inverse checks, and 84 automorphism checks
`[pcc:unverified]`. Both campaign shell scripts passed `bash -n`. Before the raw
payloads were excluded from Git, every entry in all seven imported SHA-256
manifests matched.

The nine compressed JSONL corpora and one 417,316-byte compiler report are not
committed because the repository artifact store was access-disabled during
integration. `results/OMITTED_ARTIFACTS.csv` preserves their paths, byte counts,
and SHA-256 digests; `scripts/run_campaign.sh` regenerates the corpus.

# Limits and next evidence

The campaign excludes point lifting, relation-system construction, sparse
linear algebra, and full ECDLP cost. The highest-value follow-ups are:

1. independent replay on repository-controlled hardware;
2. independent recomputation from the raw corpus after artifact-store recovery;
3. matched strongest generic-factorization and rho/external controls in the
   same process and cost unit;
4. interval estimates and multi-host sensitivity;
5. end-to-end relation-collection and linear-algebra accounting;
6. target-set crossover calibration for the intended caller workload.

The source campaign, exact receipts, negative controls, and prior-art boundary
are documented in `../../IMPORT_NOTES.md` and `../../research/`.
