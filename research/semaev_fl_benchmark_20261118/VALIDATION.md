# Validation record

Frozen source revision:
`f37434af0ddbf23d7a415108b29067cfc72abfe5`.

Passed gates:

- `cargo test --release --lib faugere_lachartre_split_matches_the_full_width_cascade --locked`
  — 1 passed, 0 failed.
- `cargo test --release --lib the_f4_family_reports_its_whole_run --locked`
  — 1 passed, 0 failed.
- `cargo build --release --locked --bin ecbench --example semaev_fl_bench`
  — passed after applying the exact 18-line main-branch initializer repair
  required by the pre-merge source snapshot.
- `ecbench plan --spec research/semaev_fl_benchmark_20261118/spec.json --json`
  — `ECS1hf8be55a1046f`, 54 executions, no warnings.
- Local analysis — correctness gate passed, 16 measured pairs, no timeout or
  oversize rows, all input and reduced-basis hashes matched.
- PDF and chart were rendered from `local/analysis/summary.json` and visually
  inspected.

Infrastructure blockers:

- The trusted source checkpoint for the exact revision fetched the source and
  failed at AWS OIDC role assumption:
  [run 44262076729](https://github.com/isogeny-labs/crypto/actions/runs/44262076729).
- Recent trusted Iso Lab experiment workflows fail before source build because
  artifact hydration cannot recover required historical objects, e.g.
  [run 44257733755](https://github.com/isogeny-labs/crypto/actions/runs/44257733755).

No EKS job was launched after these preflight failures, and no local timing was
relabelled as admitted evidence.
