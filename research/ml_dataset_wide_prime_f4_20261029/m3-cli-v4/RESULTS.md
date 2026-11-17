# Wide-prime m3 CLI software validation

Runtime source `210e8ea66e16a375c988cf76a983d9ddff306b47`; CLI SHA-256 `8d6947c533342c65b475507a6a416547bdfc5b5cc99966be4a8ac82a91b15fd9`. Frozen protocol committed in `1e93b53378aa` before the six draws. Protocol runner completed all32 commands without retries or resampling.

Validation:285 focused passes, zero failures, one ignored; full library5402 passes, zero failures,123 ignored; release CLI build and strict Clippy for lib/bin crypto/tests passed. Tests exercise canonical chain/group checks, first-certificate persistence, partial/deadline/admission consistency, rehashed central mutations and graph-export compatibility. These counts belong to the named runtime, not later code.

The fixed CLI panel covers n47,53,61, m3, both twists, seed42, requested factor-base size2, degree bound4 and work limit50000. Each of six cells actually invoked native F4, reported unit_derived, and completed independently checked original signed-domain extraction without a PDP witness. The retained outer status is partial. Native basis derivation remains unverified; the negative extraction evidence is not an independently certified Gröbner derivation. No ECDLP scalar was recovered, no first/last fall degree was measured or proved, and no full execution trace or exact instruction/RSS claim is made.

Every generated cell, intermediate artifact, telemetry file, export, report and command log is retained in the content-addressed S3 fixture archive named by evidence/wide-m3-cli-fixture-v4-210e8ea-s3.json. The exact CLI binary/build gates and full/focused validation are separately archived. All five archive receipts bind fresh downloaded bytes and contained hashes.

After downloading both binary and fixture, all six dataset verify calls passed; their stdout exactly matched the originals. All six artifact/export checks passed again. evidence/restore-replay.json records this source-bound native replay. This is not a separate standalone consumer implementation: independent-consumer m3 negative/extraction verification and native basis derivation remain false.

## Handoff 2026-10-30

### What was done

Implemented bounded four-variable native F4 support and the supervised m3 adapter, retained original affine-chain inputs and unrestricted auxiliary coordinate, checked signed group extraction with first-certificate persistence, and completed the fixed CLI/S3 restore fixture described above.

### Decisions made

Keep source-bound native, extraction, PDP, trace, instruction and scalar labels distinct. Preserve all requested broader families/solvers/arities/policies as unfinished requirements. This is a finite software validation panel; verified scientific cloud dataset cells remain0.

### Open threads

Current-head hosted CI and PR integration; wider arity/family adapters; independently verified basis derivation; nonlinear extraction; all requested factor-base/solver variants, instructions and intermediate trajectories; budget-authorized scientific corpus production. Original cloud budget/deadline is unchanged.

### Errors

Earlier source281 full attempt lacked tracked fixture input; retained failure and exact-source retry were archived. Source281 CLI lint failed helper visibility; correction210 passed its own full/focused/build/lint gates. Metadata parser now selects the terminal parent summary so nested child tests are not double-counted. All historical evidence remains separately source-bound.
