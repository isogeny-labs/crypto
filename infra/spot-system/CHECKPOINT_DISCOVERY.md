# Checkpoint discovery

`checkpoint-discovery` is a native Rust tool that inventories live experiments,
finds checkpoint candidates, and selects committed generations whose files pass
size and SHA-256 checks. It never starts or signals an experiment, executes an
inferred resume command, registers old output as recoverable state, or changes
the Spot capacity/cutover settings.

Build a portable AL2023 x86_64 executable and inspect the host:

```bash
RUSTFLAGS='-C target-cpu=x86-64' cargo build --release --locked \
  --manifest-path infra/spot-system/checkpoint-discovery/Cargo.toml
infra/spot-system/checkpoint-discovery/target/release/checkpoint-discovery \
  --root /srv/experiments --search-root /home/ec2-user/crypto
```

Use `--output FILE` for an atomically replaced mode-0600 JSON inventory.
`--search-root` is repeatable. The tool finds candidate `.ckpt`, `.checkpoint`,
`.ckpt.json`, `checkpoint.json` and `checkpoint-manifest.json` files. Candidate
filenames alone never establish resumability. Unknown/native application formats
require an application-specific restore test and migration into the contract.

Live discovery checks same-user processes with executables under the configured
roots, plus interpreted runs whose cwd and stdout file are in those roots. It
reports executable/cwd/output paths, PID and checkpoint hints, not command text,
environment values, or file contents. It recognizes the existing factor-base
worker's absent checkpoint interface, Joux–Vitse's `--json FILE` → `FILE.ckpt`,
and `rr_degree_ladder --out FILE` resume candidates. These hints remain unverified.
Environment-only checkpoint paths such as `ISO1_CHECKPOINT` require explicit
search-root coverage; discovery does not read `/proc/PID/environ`.

Hidden directories, build targets, node databases, SSH-key directories and
symlinks are excluded from the general search. Checkpoint trees are inspected
separately, including hidden partial files, and symlinks there block admission.
Defaults limit traversal to 100,000 entries/24 levels and hashing to 1 GiB;
`--max-entries` and `--max-hash-bytes` override them. Missing/unreadable manifests,
failed probes and exhausted limits appear as coverage issues. An incomplete
inventory is not proof that no experiments/checkpoints exist.
Durable state and live process working directories are scanned before general
archives so a large historical run tree cannot consume their search budget.

## Managed generation contract

New applications can opt into `checkpoint_contract: "manifest-sha256-v1"` in
`jobs.json`. They must implement this protocol; changing the declaration cannot
retrofit checkpoint support into a running process.

1. Use `$SPOT_CHECKPOINT_DIR` for immutable generation directories.
2. Write/fsync every state file required to resume. A generation must describe
   the complete state, not a collection of unrelated incremental outputs.
3. Write `checkpoint-manifest.json` **last**, using a temporary file, file fsync,
   atomic rename and directory fsync. Keep at least one prior valid generation.
4. Bind the manifest to the current job id and the `sha256` in the supervisor's
   `state/JOB/spec.json`. The tool also checks that its stored `spec` exactly
   equals the current `jobs.json` entry.
5. On `$SPOT_RESUMED=1`, restore the generation named by
   `$SPOT_CHECKPOINT_MANIFEST`. Do not pick a different generation or silently
   start from zero if restore fails. Recheck application invariants on restore.
6. Checkpoint periodically and on SIGTERM; exit within the supervisor's
   75-second grace period. Keep the workload foreground and outputs durable.

Example manifest structure (replace the uppercase placeholders and byte count
with the actual values; only the listed fields are accepted):

```json
{
  "schema": "spot.checkpoint/v1",
  "job_id": "search-001",
  "spec_sha256": "SHA256_FROM_STATE_SPEC_JSON",
  "generation": 12,
  "committed": true,
  "files": [
    {"path": "progress.bin", "sha256": "SHA256_OF_PROGRESS_BIN", "bytes": 1234}
  ]
}
```

Paths are relative to the generation directory, with no absolute paths, parent
traversal, duplicate entries or symlink components. Files must be regular and
remain unchanged while hashed. Discovery selects the highest valid generation
number, allowing fallback when a newer generation is corrupt/incomplete. Two
valid generations with the same highest number are ambiguous and rejected.
It retains rejected files and explains each rejection.

The supervisor invokes `checkpoint-discovery --root ROOT --job ID --require-valid`
before starting a manifest-contract job. A fresh job can start once with no
state. A durable `attempt.json` marks admission; a later attempt without any
valid generation is rejected instead of silently restarting from zero. The
selected manifest is passed to the application. For local validation,
`SPOT_CHECKPOINT_DISCOVERY_BIN` can select the executable; workers use
`/opt/spot/bin/checkpoint-discovery` by default.

Inventory statuses are `fresh`, `validated`, `complete`, `unverified`, or
`invalid`. `validated` means identity/file integrity, not proof of application
checkpoint correctness, CPU portability, exactly-once effects or EFS durability.
`--require-valid` requires a unique job, complete scan coverage and an admissible
status; it exits 2 on refusal. Legacy `atomic-files-and-sigterm-v1` jobs keep
their existing application-controlled resume behavior; discovery reports their
uncommitted checkpoint files as unverified.

## Automatic inventory

The runtime archive includes the executable. `install_services.py` installs a
`spot-checkpoint-discovery.timer` that refreshes
`/srv/experiments/discovery/current-host.json` every 900 seconds after a scan
finishes, with an explicit 64-GiB hashing budget for native matrices.
`--discovery-only` installs only this service/timer and requires an
existing durable mount. For the original host, add
`--search-root /home/ec2-user/crypto`; new workers search the durable root.
Node state and active experiments are not restarted by this installation.

The directory's name does not establish durability: verify that
`/srv/experiments` is the expected IAM/TLS EFS mount before relying on it.
Files discovered outside it still need coherent migration. Keep the host
protected until each actual application passes a kill/restore test.

## Validation

```bash
cargo test --locked --manifest-path infra/spot-system/checkpoint-discovery/Cargo.toml
cargo clippy --locked --manifest-path infra/spot-system/checkpoint-discovery/Cargo.toml \
  --all-targets -- -D warnings
```

Tests include generation ordering/fallback, digest corruption, incomplete state,
changed job identity, missing files, symlink/traversal/FIFO rejection, resource
limits, private report writes and process inventory redaction. A native fixture
runs under the production supervisor, is killed, has newer generations corrupted,
and resumes from the older discovered generation to the expected final output.
This synthetic recovery test does not establish recovery of the five existing
non-checkpointed factor-base jobs.
