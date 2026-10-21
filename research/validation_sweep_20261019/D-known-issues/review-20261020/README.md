# Validation sweep D review qualification — 2026-10-20 UTC

This qualification accompanies audit revision `67eade206bbbd0d9a2cebc8d75254cfb4a351371`. The original report, PDF, figures, verdicts, native KS source and archived TSV retain their bytes; [original-artifacts.json](original-artifacts.json) records all 11 files. The original runner is retained as [original-run.sh](original-run.sh). This review repairs the live runner and qualifies evidence scope. It does not perform a new cryptanalytic or performance campaign.

## Runner failure propagation

The previous POSIX shell pipeline returned the status of `awk` even when the native executable failed. The runner now captures each native invocation before labeling rows. A nonzero native status stops the script. `ks_recompute/test_run.sh` checks failure status 42, absence of rows from the failed invocation, paths containing spaces, row labels and cleanup; the regression and ShellCheck pass. Earlier directories may already have printed rows if a later directory fails, so consumers must require a successful process status before accepting a complete audit.

## Native KS replay

The original Rust calculation was replayed against the 16 frozen trial/result pairs at crypto PR #2186 source `7855d66ef59d5d55836b582ed870edb3f0a1093c`; [replay-inputs.json](replay-inputs.json) binds all 32 files. Both runner versions produce byte-identical output on this review host. The [native comparison program](verify_ks_replay.rs) and [comparison output](replay-comparison.txt) record the checks.

All 2,963 record identities, sample counts, recorded values and recomputed D statistics match the archive exactly. Twelve p-value fields differ in final floating-point bits, with maximum absolute difference 2.220446049250313e-16. Six historical tie-blind p-values therefore differ from the recorded values by those final bits; D has zero mismatches. Byte-for-byte reproduction of archived p-values is not established on this host. The archive is preserved without rounding or replacement.

Tie handling changes 49 tests (33 generator, 13 shift, three class). Maximum absolute D change is 0.0010000000000000009 and p change is 0.015913766910500726. There are zero crossings of p < 0.05, 0.01 or 0.001. These remain the original asymptotic p-value approximation for discrete data; replay does not validate that approximation or settle covariance and rank-test questions.

## Item 7 chronology and scope

The claim that the solver and GPU paths are absent from every main branch is now stale. Cryptanalysis [PR #968](https://github.com/aburan28/cryptanalysis/pull/968) merged at 2026-10-19T20:28:08Z as `9b5cf83d4ac58bddb19fefb50bf59388c4f98713`. This review checked that the merge tree contains `suite/src/cryptanalysis/certicom_dlp/` and `gpu/certicom/`. Code presence does not establish physical GPU qualification or demonstrate that every historical output was unaffected.

The original audit explicitly did not inspect #968's complete commit/run history. Its unqualified sentence that no recorded result is affected is not independently established across that missing history. The narrower finding is that no affected recorded solve was identified within the inspected artifact scope. Deadline-repair tests and an exhaustive archive audit are separate obligations.

## Remaining boundaries

The original limits still apply: the cross-ISA experiment was not run; F4 and KS citation searches across the other repositories were incomplete; #2247's charged S measurements and #2227's quotient-class collision counts are attributed to their source reports and were not independently reproduced here. The other items remain scoped audit records with their stated open work. This PR does not certify every downstream claim. Later corrections must preserve original records and identify their source and validation scope.
