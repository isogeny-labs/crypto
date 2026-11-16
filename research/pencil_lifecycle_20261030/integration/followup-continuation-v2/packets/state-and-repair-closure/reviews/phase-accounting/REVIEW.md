# Independent saved phase-counter review

Fresh compilation and replay of the source published in PR3056 at commit 60346e7f2b831aa88f89d7967b9e74e21220422c reproduced result2.json's arm summaries, issues and input hashes exactly. Five factor-base arms and two projective-law arms each have 1,260 standardized rows, 1,260 candidate phase-conservation checks and 1,260 rho checks. The separate bit-stratum tally contains 49 arm/bit entries of 180 rows. These are checks of saved evidence, not new experiment results.

## Source and acceptance

The source indexes native rows by (p,target_index), rejects duplicate native keys, joins standardized rows, and compares bits and workload_id. It rejects duplicate standardized (arm,bits,p,target_index) identities. Each candidate/rho record must have nonempty aggregate counters and phase lists, unique nonempty phase names, identical aggregate/phase counter keys, and unsigned u64 values. Phase sums use u128 and must equal every declared aggregate exactly. Fifty original input files were read and hash-checked unchanged.

The executable is a structured reporter, not a standalone acceptance gate. Identity/counter failures appear in issues or unresolved counters, and missing rows appear in summary counters, while the process can exit zero. It does not require seven arms or 1,260 rows. The independent accept.jq gate therefore requires the exact seven arm names, each dictionary exactly {standard_rows:1260,candidate_phase_conservation_pass:1260,rho_phase_conservation_pass:1260}, no issues and an unchanged-input flag. Empty output is rejected separately. The saved baseline satisfies this gate.

## Independent malformed-input controls

Eight controls use copies of saved data: duplicate standardized identity; duplicate native key; altered bits; altered workload_id; negative phase counter; empty phase list; omitted phase; and a header-only empty cohort. Both duplicate cases exit101 with no accepted report. The other six exit0, but all eight are rejected by the explicit review gate. Original saved evidence remains unchanged. The exact mutated-file hashes, outputs, stderr and receipts are retained under controls/. The producer's built-in one positive and five negative conservation controls also execute on each invocation.

## Claim boundary

The result establishes exact conservation of the recorded counters across their recorded phases. It does not establish that those counters include every physical operation, that setup was allocated once per physical row or workload, or that cold/warm costs and retries were charged. It does not reverify the historical scalar answer or provide new timing, pricing or cost ratios. Counter equality can hold even if both the aggregate and phase instruments omit the same activity.

## Attestation

The reviewer inspected the exact published source before replay, used the producer result to locate saved inputs and expected arms, and read the published phase-accounting review plan. This is source-first independent execution, not blind derivation. The reviewer did not author this checker. The full input corpus, source, Cargo lock, compiler version and resulting binary are bound in the packet. No solver/scientific generation, native curve operation, cloud action or repository mutation occurred. REVIEW, status, control receipts and the inspected PDF/SVG form the scoped independent result.
