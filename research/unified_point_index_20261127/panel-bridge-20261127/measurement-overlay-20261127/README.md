# Matched native phase overlay development receipt

The exact reviewed `native-metrics.patch` applies byte-for-byte to baseline `57110758a23985975b943a3597ea153518c2279f` and candidate `fde23fd73445463c692f59312dcf319d525de923`. Each patched solver's four common runner/metrics/test files equal the corresponding driver source files. Rust 1.78 locked offline release CLI controls pass 7/7 on each separate pinned snapshot; the current driver also passes 193 library, 5 runner, 7 CLI and 13 panel cases. These are correctness and instrumentation checks, not timings. [pcc:unverified]

Strict all-target Clippy fails only on four inherited lint classes; the same run scoped to those classes passes. Full raw logs, including the strict failure, are retained losslessly in `development.tar.xz`. `native-source.sha256` records the current implementation, overlay and builder inputs. SHA256SUMS pins this receipt. [pcc:unverified]

The overlay exposes five exclusive online clock buckets and basic group/meter counters in successful optimized receipts. The runner's clock excludes process startup and receipt serialization; failed runs lack internal snapshots. `calibrated_total_operations` remains null. No benchmark, checkpoint, image or scientific run occurred. [pcc:unverified]

The prior uploaded head `eb6c31839d190e326eb67114e3aadb558cb759be` passed the unprivileged `native-pair-build` job, but the separate gate job `140654677317` in run `46042850446` failed strict PR-body prose lint on two untagged uses of “speedup.” The exact failed step is in the archive; the complete 5,963,360-byte provider log has SHA-256 `832c816d1a2d3df3e3e63d29452b8267d26ad863c5d65e69ff97cbc5bb990afd` and remains available at the job URL. The PR body was corrected without a waiver; local `pcc.py lint-prose --stdin` now reports zero findings. The new source head still needs hosted CI. [pcc:unverified]

`conductor check` exited 127 because the CLI was absent; its raw local error is retained. No shared reservation or Conductor pass is claimed. [pcc:unverified]

The ordinary unified diff included whitespace-only context lines that the repository patch preflight rejected. The committed overlay uses zero context and `git apply --unidiff-zero`; exact Git revision checks plus post-application byte comparison of all four files prevent a shifted or partial match. Both pinned revisions passed that comparison. No whitespace policy was waived. [pcc:unverified]
