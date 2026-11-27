# Canonical-row scan in native certification

Both native checkers used to compare each monic row against a cloned, sorted and rescaled normalization. The new private `canonical_row::is_canonical_monic` checks canonical coefficients and strict monomial order directly on the bounded uncounted path. Input reductions, all pair obligations, graph bounds, original-ideal proof replay and public verification are unchanged.

The scan is limited to primes 2147483647/9007199254740881, uniform 3..16-variable rows and exponents at most 64. Empty or nonmonic rows retain the old rejection. Unsupported shapes and calls with active Visit or bitop accounting retain the original normalization predicate. The general public `normalise` function is unchanged. This is an implementation optimization, with no new algebraic or novelty claim.

All three candidate differential/fallback unit functions pass against actual production normalization, including 15,804 fixed boundary rows. In the tested supported monic row, the uncounted helper makes zero normalization calls; the accounted helper calls the reference once. An additional default-discovered unit imports the live production helper directly and passes 31,500 predicate comparisons under both accounting flags, preventing drift from the preserved candidate copy. This is a branch/operation check, not a timing result. [pcc:unverified]

Complete baseline and candidate matrices each execute 432 constructions and fresh public reverifications over both primes, n=3/4/6, seed7 LAST-star, six wire forms, three kernels, two native checkers and bitop off/on. Every original wire matches a frozen reference. All 1,729 output files match byte for byte, including complete proofs/bases, native receipts and per-phase enabled-bitop primitives. No counter credit is claimed. The helper's accounted flag/fallback is tested; no separate full Visit-ledger campaign was executed. [pcc:unverified]

The unchanged original36-case and108-pair native integrations also pass on the integrated source, with1,116 mutation assertions and six unsupported refusals. Both unchanged separate auditors replay all144 proofs and reject their2+5 semantic sabotage views. Their181/433 payload files match the preceding qualified production scope byte for byte. Results remain unsigned same-host evidence. [pcc:unverified]

## Source contexts and reproduction

`BASELINE_SOURCE.sha256` and `BASELINE_PRECALL.json` precede baseline execution. Production source was unchanged while the helper was qualified outside it. Old `reuse.rs`/`pair_certificate.rs` preimages are retained under `baseline-source/`. `CANDIDATE_SOURCE.sha256`, `INTEGRATION_PLAN.json` and `CANDIDATE_PRECALL.json` bind the integrated source and fresh workers. The promoted helper is byte-identical to `pending-source/canonical_row.rs`.

Use the candidate source manifest at the new PR source. Earlier instrument/native manifests deliberately belong to earlier compiled contexts; they are not relabelled as current. No frozen report, source index, key, gate, original test or earlier measurement is rewritten.

Follow the supplied build/run scripts with NEW log/receipt/worker/output paths. They refuse replacement. Builds use Rust1.100.0, opt0/LTOoff/debug0/incrementalfalse/codegen16/default-features=false, CPU0/jobs1 and one numerical thread. This is correctness qualification, not an optimized performance build or whole-library unit pass. Prior full-libtest allocation failures remain historical and were not repeated unchanged.

Rebuilding the baseline requires a separate checkout with the saved two production preimages; do not replace the current candidate or historical outputs. The additional live-file test is `cargo test --release --locked --offline --manifest-path research/relation_canonical_scan_20261127/harness/Cargo.toml --test live_row_scan --no-run` under the same build flags, followed by a fresh frozen libtest executable with `--test-threads=1`. Default test discovery includes both `row_scan` and `live_row_scan`.

`qualification-data.tar.gz` contains both complete432-case matrices and the fresh36/108 native cohorts. `qualification-workers.tar.xz` contains six copied read-only executables, including old/new snapshot/unit workers and the native integrations. The additional live production-file unit worker is retained in live-unit-worker.tar.xz. Archive SHA sidecars use repository-relative paths. Inspect source/custody records before executing local ELF archives; hashes are not signed execution attestations.

For mathematical proof replay without root rebuild, use the unchanged original/pairgraph auditors from the earlier qualifier archive with the new native cohorts and frozen reference inputs/pins. The baseline/candidate primitive receipts are reported execution data, not independently attested operation histories.

`BASELINE_OBSERVER.json` retains the outer observer's exit1 while both captured unit/baseline exits are0 and Rust summaries/source postchecks pass. No silent baseline rerun hides this discrepancy. Candidate/unit/native/auditor captured exits are all0. The additional live-unit raw exit is also0 while its outer observer reports1; its separate receipt retains that discrepancy. Focused shell exit probes both return0 and do not explain it. [pcc:unverified]

## Remaining performance work

No comparative timing ran and15× remains unachieved. Intrinsic regularity and actual F4 solving degree remain unknown. This changes the native row-form check, not the polynomial space, solver degree or proof format. [pcc:unverified]

The separate canonical timing-document choice is still pending; Conductor and checkpoint/S3 remain waived. Before timing, qualify a new performance source/control roster that retains the previous root worker—including its pair-graph variants—as eligible controls alongside both newly optimized native checkers. The earlier four-worker/8,640-call instrument remains a frozen qualified draft, not an authorization or prospective pin set for this new production version. Preserve the original complete call boundary, strongest matched controls, repetitions/A/A, six fixed inputs and every proof/failure gate.
