# Read-only accounting v2: identity and manifest-chain checks

The native Rust v2 checker adds required target identity checks, expected-index target and case-label comparisons, and the retained receipt-to-manifest-to-input hash chain. V1 and its evidence remain unchanged.

The unmodified archive passes both `accounting_pass` and `provenance_pass`, with 180 physical rows,60 selected rows and420 indexed dispositions. Setup totals counted once per physical row exactly match the saved analysis. These are historical stored fixtures, not new target runs.

Required observed and index fields are target_index (u64), workload_id, workload_sha256 and algorithm_seed (nonempty strings), compared exactly to the archive row. Each expected case's a/b labels are compared to the archived case. Missing or null required identities now fail. The historical receipt binds the exact pre-execution manifest SHA256; that manifest binds SHA256 and byte length for archive.jsonl and input-index.jsonl. Missing manifest or missing/mismatched binding fails the separate provenance flag and overall exit.

Five bounded copied-file controls all exit1 with explicit provenance errors: missing observed identity, null observed identity, index identity mismatch, altered archive identity and missing manifest. Mutation inputs and raw/output hashes are retained. Controls execute no curve arithmetic or simulation. No source producer is run.

The provenance flag has a deliberately narrow meaning: consistency of the recorded receipt/manifest/two-input chain and selected required identities. It does not authenticate the historical receipt's origin, prove that run.stdout came from the recorded executable, verify every manifest file, or establish full scientific provenance. Accounting and provenance flags are separate. Invalid row/case types or malformed JSON fail the native parser; graceful machine-readable recovery for arbitrary malformed schemas is not claimed.

All180 archive rows remain in the output, including120 unselected rows. Missing costs remain null. Full-cohort obligation remains1260; complete cost ratio and natural yield remain null. Hashing/allocation/comparison/memory and full timing boundaries remain outside the saved accounting fields.

Inputs and source follow the original repository AGENTS.md native implementation/evidence-scope rules. Prior work reused: retained-duplicate-executed/run.py accounting, its pre-execution-manifest and receipt schemas, and root's independent v1 provenance findings. This is file-level reconciliation, not a new mathematical or performance claim.

Reproduce with `cargo run --offline --locked --manifest-path Cargo.toml -- INPUT_DIRECTORY OUTPUT_JSON`; run controls/check.sh only in its recorded private environment. The Rust program invokes no shell or producer process. Source, Cargo.lock and exact input hashes accompany the JSON. Root independent replay/review is pending.
