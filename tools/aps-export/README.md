# Native APS export and audit reader

`aps-measurement-export` packages an ecbench plan, literal sealed record lines,
and explicit execution-history sidecar into `experiment.export/v1`. It does
not launch a solver, execute an analysis script, upload evidence, or qualify a
clock. Development observations carry producer standing [pcc:unverified].

```sh
cargo build --release --locked --manifest-path tools/aps-export/Cargo.toml
tools/aps-export/target/release/aps-measurement-export build EVIDENCE_ROOT request.json --out NEW_EXPORT.json
tools/aps-export/target/release/aps-measurement-export check EVIDENCE_ROOT NEW_EXPORT.json
tools/aps-export/target/release/aps-measurement-export audit-record EVIDENCE_ROOT one-record.json
```

The build target can be changed with normal Cargo settings. Every evidence
path is relative to `EVIDENCE_ROOT`; absolute paths, parent components and
symlinks escaping it are rejected. Inputs are capped at 64 MiB per file. Output
creation is exclusive, so an existing export cannot be silently overwritten.

The request has this shape:

```json
{
  "schema": "aps.export-request/v1",
  "export_id": "aps-run-001",
  "created": "2026-11-04T00:00:00Z",
  "protocol": "protocol.json",
  "plan": "session/plan.json",
  "records": "session/records.jsonl",
  "history": "execution-history.json",
  "source": {"repo": "crypto", "commit": "40-lowercase-hex", "dirty": false},
  "producer": {"runtime": "native-rust", "model": null, "session": null, "role": "executor"}
}
```

Use the actual source commit, creation time and producer. The frozen protocol
must pin the exact plan bytes in `extensions.aps_export.plan_sha256`; its
analysis-script pin is checked when present. Both canonical schemas and the
protocol hash are checked. This is not a full protocol-design validator:
`backend_record.protocol_design_validated` remains false. The two schemas are
vendored from crypto-autoresearcher commit
774611edb0a58861c2991372b81e49be0a0644b4; their Git blob identities are
`707ebf6f814d96bb44b36a9ccb3e25128e1aaf5e` (export) and
`9d8543e6b448264c333649e1a4dfd83a2a297bd5` (protocol).

The history sidecar describes process executions, not APS's internal retries:

```json
{
  "schema": "aps.execution-history/v1",
  "events": [
    {"seq": 0, "attempts": 1, "resumed": 0, "checkpoint": null,
     "status": "succeeded", "started_at": null, "finished_at": null}
  ]
}
```

Each observed execution needs known `attempts >= 1` and `resumed >= 0`.
Restores need their checkpoint policy and generation. A recordless execution
may report `failed`, `timeout`, `cancelled`, `infra_error` or `interrupted`,
with its explicit attempt history. It cannot report success. Unknown history
is rejected rather than inferred as zero. The adapter checks consistency; it
does not attest that a history was actually observed.

Every row is bound to its planned method, workload, seed, round and warmup.
Missing slots are retained by identity in `backend_record.missing_execution_slots`;
they are not invented attempts, successful rows or exclusions. The canonical
`flow` convention counts represented rows. The full plan population is retained
as `planned_execution_slots`. `build` can preserve a partial export, while
`check` rejects an export with unobserved planned slots.

`check` reopens the pinned inputs and reconstructs the entire export. It checks
the literal record seal, native counters and evidence hashes/sizes; arithmetic
reads the explicit curve and target without the producer's Montgomery code.
See [CONTRACT.md](CONTRACT.md) for the exact scope. All exported clocks retain
`wall_valid: false`. Unknown native work is never promoted to complete `S`.

Development commands:

```sh
cargo fmt --check --manifest-path tools/aps-export/Cargo.toml
cargo clippy --locked --manifest-path tools/aps-export/Cargo.toml --all-targets -- -D warnings
cargo test --release --locked --manifest-path tools/aps-export/Cargo.toml
cargo test --release --locked --test aps_export_bridge
```

The `tests/fixtures/cli/` clocks, producer/source pins and rows are synthetic
contract fixtures, not experiment results or a real protocol freeze. The
minimal `tests/fixtures/synthetic-protocol.json` template is authored from
the canonical schema. It contains no historical execution configuration,
host metadata or benchmark claims. Tests add only synthetic source/binary
placeholders and the fixture plan binding; they do not admit a design.
