# Resume: Chained ECDLP transfers / PR #3877

## Goal
Continue the user's autonomous research across crypto, cryptanalysis and crypto-autoresearcher. Prioritize explicit DLP-preserving transfers, chained Jacobians, prime-field descent and fully accounted solving cost.

## Lead / hypothesis
CT01: replace the legacy split-only Python maps with native Mumford maps on the odd-degree chart of C:y²=x⁶+1 over F101. CT02: compare arbitrary classes with the existing BigUint implementation. CT03/CT04: structural kernel/descent proof, then genuine Jacobian hop. See research/chained_jacobian_transfers_20261110/QUEUE.md.

## Spec
Historical PROTOCOL.md is frozen history. New development uses Rust and public toy fixture p=101, root=10, E:y²=x³+1, r=17, P=(4,41). Sources and exact chart obligations: SOURCES_20261111.md. No experiment/benchmark is admitted yet.

## State
Repositories restored. Branch research/chained-jacobian-transfer-20261110 at 4ff4da410d6b10834a59e843455a79a12b1baa30. KB unavailable; autoresearcher locked; no AWS/checkpoint binding. Native replacement not yet implemented. Historical audit failures saved outside Git in /workspace/scratch/pr3877-ci-failed.txt and reference unrelated archived sessions.

## Next steps
In /workspace/crypto: implement native crate, cargo test --release --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml. Add differential root example, then run cargo test --release --lib and touched examples. Register claims with actual standing, retain output and reports, commit/push to #3877. Follow QUEUE.md for next task.

## Runs in flight
None. Rust stable 1.99.1 installed under ~/.cargo/bin; use that path. No remote allocation or paid job.

## Stop and success conditions
Local development checks only until checkpoint gate can push and live IsoLab budgets/capacity are known. Do not bypass S3 gate. Continue unblocked implementation/proof work; preserve state on session resource limit. No speedup promotion without complete measured evidence and independent signed verdict.

## Open questions
Restore KEY through environment secret settings for autoresearcher; restore checkpoint token/AWS via configured bindings. Need primary Richelot source review and a concrete correspondence before claiming a new chain. Need exact branch protection and final-head CI before merge.
