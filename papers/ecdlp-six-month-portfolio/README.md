# A Six-Month ECDLP Research Programme

**Research draft, 19 October 2026. Anonymized; not peer reviewed, submitted, or cleared for submission.**

[Read the paper](portfolio.pdf). Editable source: [portfolio.tex](portfolio.tex), [sections](sections/), and [figures](figures/). The review covers 19 April–19 October 2026 across 19 pinned `aburan28` repositories, with a separate current-branch metadata inventory and selected unmerged updates.

The paper measures the accessible research record, explains the implementations and experiments by field and mechanism, shows concrete improvements, preserves corrections, distinguishes proof and evidence levels, and assesses the ten open leads. It extends the earlier fourteen-week index-calculus programme manuscript to the full six months and the wider portfolio.

The pinned snapshots contain 305,367 tracked entries and 34,276 window commits; current branch metadata covers 5,144 heads and 49,936 distinct window commits. These include merges, imports and upstream history and are **not counts of discoveries or independent experiments**. The catalogue indexes 4,075 study directories and 16,587 safely parsed ledger files; 24 loader failures remain explicit.

## Evidence and boundaries

- [Source register](evidence/sources.json): 72 exact source entries, including pinned main files, observed open-PR heads, a prototype branch and the linked Drive checkpoint.
- [Recorded figure inputs](figures/recorded-data.json): copied historical values and metadata; plotting does not run cryptography.
- [Frontier review](evidence/frontier-record-review.json): fresh hashes and outcome counts for 19 session record files, 3,178 verified and 54 exhausted measured executions. Stored solver replays are distinguished from this review's byte checks.
- [Volcano archive review](evidence/volcano-archive-review.json): all 29,409 manifest entries pass; the outer archive hash also agrees with the pinned repository delivery receipt. This is integrity verification, not independent solver or mathematical validation.
- [Repository inventory](evidence/repository_inventory.csv) and [branch summary](evidence/all-repository-branch-summary.json): exact counting boundaries and revisions.
- `evidence/catalogue.tar.xz`: the full machine-readable metadata catalogue, with a separate SHA-256 file and manifest. It contains a directory index, not an assertion that every run was independently replayed.
- `evidence/review-validation.json` and [validation logs](evidence/validation-logs.tar.xz): review execution and its limits. The archive preserves the original output bytes and a member-hash manifest, including failed environment attempts.

The manuscript reports engineering and methodological progress. It establishes no general prime-field sub-rho ECDLP method, no cryptographic-scale IC break, and no global ranking of historical research efforts. It does not promote a stage ratio, retained-preparation online result, model extrapolation, finite enumeration, or author-reported open-PR result into a complete-method theorem. It separately identifies the unavailable historical 51-GB-class mirror and private ten-lead packets.

No new cryptographic performance experiment was run for this synthesis. Fresh work consisted of repository metadata collection, source review, native correctness tests, byte/record checks and document generation. The entire historical corpus and Lean dependencies were not rebuilt. Full branch histories are indexed; branch **content** review is selective. The paper's coverage table keeps the literal “everything” requirement partial where raw access or exhaustive independent replay is absent.

## Build and check

Requires a normal TeX installation (`pdflatex`, `bibtex`, `pdfinfo`) and standard packages. Rendering figures additionally needs Python, NumPy and Matplotlib; these scripts lay out recorded values only.

```sh
make
make review-check
make check
```

`make check` rebuilds the PDF and rejects LaTeX errors, unresolved references/citations and overfull boxes. `make review-check` checks source anchors and figure/count consistency; when sibling checkouts are present, it also rechecks their cited source bytes. It is not a scientific replay certificate. The normal shared paper build also supplies submission-bundle and de-anonymization targets; those do not constitute permission to submit this draft.

To browse the catalogue:

```sh
sha256sum -c evidence/catalogue.tar.xz.sha256
mkdir -p /tmp/ecdlp-catalogue
tar -xJf evidence/catalogue.tar.xz -C /tmp/ecdlp-catalogue
```

`catalogue/INDEX.txt` defines each file, the sparse-materialization boundary and overlapping topic tags. The catalogue contains no downloaded raw executables or private ten-lead packets. The complete historical Autolab mirror is not bundled. Scripts used to collect the review metadata are retained in the catalogue for reproduction against new checkouts, with the original pinned outputs preserved.

## Canonical graphs and coordination

Checked `docs/ic/progress-timeline.json`, `docs/ic/LEADERBOARD.md`, `docs/ic/leaderboard.json`, `docs/bounds/frontier.json`, the earlier programme paper and the named study reports. They are unchanged: this retrospective adds no cryptographic measurement, admitted speed claim or official hypothesis transition. The paper's figures are a new sourced retrospective view, with unsuccessful outcomes retained.

The Conductor command is unavailable in this runtime (exit 127); no reservation is claimed. Work is confined to this new paper directory and the paper-index row. The root Rust library check was unavailable because Cargo is not installed; this change alters no Rust implementation. Native C tests and manuscript checks are reported separately.

## Before external submission

- Complete an independent novelty/literature audit; external bibliography entries were inherited from the earlier manuscript, not freshly audited publication by publication.
- Reconcile or explicitly retain the open-branch addendum at the selected submission revisions.
- Independently review the load-bearing measurement interpretations and proof-to-report bridges; build the actual Lean targets if making a new checked-proof claim.
- Obtain missing raw artifacts for any stronger replay claim, and rerun complete matched comparisons for any new performance claim.
- Restore the intended author block, then rebuild and inspect every page. This draft has not been submitted or posted to a research venue.
