# Benchmark admission gate: post-freeze evidence backlog

- **Status:** `investigating` (admission invariant restored; independent validation backlog open)
- **Kind:** validation evidence gap
- **Affected work:** [benchmark results table](../benchmarks/README.md), [unvalidated archive](../benchmarks/ARCHIVE.md), and [validation gate](../scripts/validation_gate.py)
- **Next action owner:** independent benchmark validators and benchmark archive reviewer
- **Last updated:** 2026-10-24

## Problem and disposition

At crypto `main` `5cb553810e5bbfbd8691ec614ffcaacefc625757`, 25 post-freeze benchmark rows lacked an admissible independent evidence bundle or a frozen digest. Two of those rows also appeared in the unvalidated archive. The rule-v6 and rule-v7 bundles now validate two rows. The two already archived rows were removed from the admitted table. The remaining **21 rows are preserved verbatim in the unvalidated archive**, with SHA-256 digests, a disposition, and source paths. Their studies, run archives, figures, and original row wording remain in Git; none is claimed as independently validated by its archive record.

The exact branch now passes `python3 scripts/validation_gate.py check --base 5cb553810e5bbfbd8691ec614ffcaacefc625757` and the 49 `scripts/test_validation_gate.py` cases. The gate was not weakened, the pre-gate freeze was not enlarged, and admitted measurements were not changed. This restores the admission invariant while leaving the scientific evidence backlog visible.

## Evidence and limits

- **Observed:** The gate has zero leaderboard, benchmark, and archive errors after the exact archival dispositions. [ARCHIVE.json](../benchmarks/ARCHIVE.json) preserves each of the 21 old row strings and its digest. The [archive index](../benchmarks/ARCHIVE.md) links the retained study records.
- **Observed:** Three additional rows (`20261022-strong-rho-batchinv`, `20261017-volcano-ecdlp-census`, `20261017-isogeny-volcano-cost-v2`) already had valid SHA-bound manifests on main. Their evidence checks pass after a sparse checkout materializes their raw dependencies. An earlier 24-row failure count was a partial-checkout artifact; the complete-tree backlog before v6/v7 and archival work was 25.
- **Observed:** Rule-v6 and rule-v7 benchmark-local manifests both pass the existing evidence checker. The v6 source report retains two targets where measured same-process rho was slower than the separate strong walk; its frozen size-median comparison passes. The v7 cold IC/rho ratios remain above one.
- **Derived or proved:** The admission gate and exact archival correspondence now hold for this branch. This does not prove the archived numeric claims or a general algorithmic speedup.
- **Missing capability:** Each of the 21 archived rows still needs row-specific independent scalar/output replay where applicable, headline recomputation from retained raw records, and a protocol-matched strongest-rho reference in the same unit before it can return to the admitted table.
- **What this evidence does not establish:** An archived row is not an invalid experiment. It is a historical observation with insufficient evidence for the repository's current result-admission rule. Lean proofs can cover mathematical lemmas and bounded executable contracts; they do not prove empirical timings.

## Re-admission conditions

- [x] Preserve exact old row text, digest, disposition, and source paths for every removed row; retain its raw study and figures.
- [x] Pass the repository validation gate and its unit suite without a new pre-gate freeze entry or `--allow-unvalidated`.
- [ ] For each archived row proposed for re-admission, inspect its raw source, protocol, host, metric, strongest comparison, and existing evidence. Missing raw data stays a source gap.
- [ ] Publish a SHA-bound independent replay, validator report, and matched reference that pass `scripts/validation_gate.py evidence` for the row's directory, then re-add the exact qualified row through a reviewed PR.
- [ ] Obtain exact-head hosted CI for this archival PR and each later re-admission.

## Next discriminating step

Classify the 21 entries in [ARCHIVE.json](../benchmarks/ARCHIVE.json) by the replay data they retain. The IC round archives contain protocols, analyses, and content-addressed run bundles; the other entries include stage diagnostics, a census, and publication records. Start with a small raw-record sample and a deliberately wrong scalar or result before building a row-wide validator. Stop on a missing source, mismatched digest, unpriced operation, or unqualified rho reference. Do not infer validity from the presence of files or backdate the freeze.

## Resolution and history

The admission defect is locally resolved; independent validation of the archived studies remains open. The 2026-10-24 sequence was: 25 post-freeze gaps with all pre-existing manifests present; v7 evidence reduced this to 24; removing two rows already in the archive reduced it to 22 and removed the archive overlaps; v6 evidence reduced it to 21; exact archival dispositions reduced the gate to zero. Earlier 28-to-24 accounting arose from a sparse checkout missing three already valid evidence trees and is superseded.
