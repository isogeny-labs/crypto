# Diem generation recovery and durable checkpoints — 2026-10-19

The original generator retained the entire 52-instance grid in memory and only
wrote `results/instances.jsonl` after all workers finished. It logged 50 completed
instances before the Spot host stopped at 16:19:35 UTC on October 19. After the
16:22:45 reboot, the persistent results directory was empty. Those progress
messages do not establish saved instances. This attempt remains interrupted.

The recovered `AMENDMENTS-20261018.md` is preserved verbatim, including its claim
that `instances.jsonl` had already been generated and checked. That sentence is
not supported by the surviving files and is superseded by the interruption audit
above. No cost cells are being launched from that assertion.

## Change

The original dirty scientific module and study driver were recovered from
`f82299858ee96687f302cdd48253042dac591124` plus the host's uncommitted files.
The module's existing `skip_la` support is preserved. Checkpointing changes only
the generator's storage and resumption behavior; grid, seeds, three generation
threads and mathematical controls are unchanged.

- Each completed `(k, idx)` record is synced to disk, atomically renamed and its
  containing directory synced before completion is announced.
- A kernel-held exclusive lock prevents concurrent generators. Killing the writer
  or rebooting releases the lock; the lock file itself is never unlinked.
- The manifest binds the exact task grid and driver, checkpoint, scientific module,
  frozen protocol and amendments source bytes. Each record has a payload checksum.
  Resume validates these identities and keys, then computes only missing records.
- Partial temporary files are retained and ignored. Corrupt records, changed
  source/grid, unbound old outputs and conflicting completed outputs stop the run.
- Only all 52 durable records permit publication of `instances.jsonl` and
  `FROZEN.sha256`. Publication is atomic; generated records still require their
  mathematical controls and the protocol's review/commit gate before cost cells.

Durability is qualified on macOS and Linux filesystem semantics. The native code
uses `std::fs::File::try_lock` (Rust 1.89 or newer). This is not a performance study.

## Validation and live execution

The exact-source native crate passed 18 library tests (six Diem controls and twelve
SHA-256 controls) and all nine checkpoint tests. The checkpoint tests include
SIGKILL recovery, exclusive writer ownership, exact record reuse, refusal to
publish incomplete grids, payload corruption, source/grid mismatches and protection
of frozen outputs. `native-validation.tar.gz` preserves the source and Cargo lock.

The existing host was checked for an active generator before deployment. None was
present. Original dirty files were backed up and hashes checked before replacing
the driver and adding the checkpoint module. Deployment and launch receipts are
retained here. Driver PID 549179 was launched at 18:32:03 UTC; it runs bounded
Linux library tests, checkpoint tests, build and preflight before generation.
Linux validation and generation completion are pending in this initial record.
A later receipt must establish their result; local tests alone do not establish it.

The driver uses the existing host and cores 26–28, two build jobs and a 24-hour
outer bound for generation. No new cloud instance was provisioned. Its persistent
state is `resume-20261019/latest-state.json` and all phase logs remain in a
unique run directory. Source hashes are checked before and after validation.
The generator independently holds the output-directory lock.

## Resume and next gate

From the exact checkout and source, after successful validation:

```sh
cargo build --release --locked --offline --example diem_quartic_study
# Only when no generator is active; validates all previously saved records.
target/release/examples/diem_quartic_study gen-status
# Reuses every validated durable record; the output lock rejects duplicates.
taskset -c 26,27,28 nice -n 10 target/release/examples/diem_quartic_study gen
```

On the current host, use the existing driver rather than launching another copy.
After another host interruption, inspect process identity and the saved state
before restarting. The kernel locks prevent duplicates, but no auto-boot service
is installed. Once generation finishes, review controls and commit the frozen
instance list before running cost cells. W2b still depends on W1 and W2a landing.

The full repository library suite is a separate publication gate. The focused
Linux check and the isolated tests do not replace that suite.

## Linux validation and confirmed resumption

At 18:43:43 UTC the driver was generating, with exactly one generator process
(PID 607388, affinity 26–28). All six Diem library controls and all nine checkpoint
tests passed on Linux, including killed-writer recovery. Build and preflight also
passed. The first 37 of 52 durable records were copied locally and their payload
checksums, source identities and keys verified. This is a progress snapshot, not
a completed grid or a final frozen list. See `linux/` for receipts and logs.

The full repository test binary is separately running with two test threads on
cores 24–25, a 30-minute outer bound and a durable receipt. Publication remains
pending that integration result. The existing generation process is left running.
