# Validation and exact execution boundaries

Frozen executed source: `aaf9e246a3af4be371d7089dc86fc32c61c2f5ac`.
The protocol, plan, inputs, native source, canonical task-card tool and both
executables were hash-bound before the single research attempt.

- ONE complete `cargo test --locked --release --lib -- --test-threads=1`
  command passed: **4,400 passed,0 failed,118 existing ignores,0 filtered**.
  Networking including loopback was enabled. Rust1.98.0,
  CARGO_BUILD_JOBS2, RAYON_NUM_THREADS1. This reused the unchanged library
  binary; it was not freshly compiled. Library log and exit0 are retained.
- All1,634 native input hashes stayed exact before the full-library command,
  after that run and during publication. The731 tracked compiled library
  dependencies match. Unit binary SHA256:
  `8ce288f3bfcc1529dc63f91d71bdffa852226582fd3bef110b26e789e8261e1f`.
  Its exact embedded traits literal SHA256 is
  `6b3d5bc69a6d4f321455762bce671ee1f5d949582f38ac325c3a88a193f78922`,
  at offset4484392. The earlier `native-inputs-after.json` was captured
  during the run; the separate `native-inputs-final.json` and final binding
  were captured after it. Neither is silently substituted for the other.
- Twelve CLI unit tests, seven real subprocess integration tests, strict
  release CLI Clippy, production CLI build and locked all-target compile
  pass. The integration suite exercises actual point witnesses for all
  three paths, corruption refusal and write-once output behavior, alongside
  the original sweep/resume/resource controls.
- Five independent native audit tests, strict release all-target audit
  Clippy and production audit build pass. Nine CI-scope tests pass. Curve
  naming reports0 problems. Relevant CI Bash syntax passes.
- All472 native trial rows and143 original point witnesses are independently
  audited using separate polynomial-field/group arithmetic on this host.
  The audit checks355 algebraic roots,691 case-wise unordered point pairs,
  16 quotient preimages, exact memberships/subgroups and projected rank.
  It does not rerun F4 or establish independent-host certification.
- The native verifier passes all2,493 sealed experiment member hashes and
  all143 returned point witnesses. Missing/limited calls remain results;
  they were not retried. `analysis.json` is byte-exact native audit output.
- The director's24 bindings, three single-attempt commands, six streams and
  five declared output-digest occurrences verify. Canonical `check-relay`
  itself remains exit1 because serialized output keys reorder its expected
  lines. The narrower recovered director verification confirms byte-exact
  actual runner-order relay and execution scope. Its helper, original
  failure, canonical classification and limits remain in `cards/`.

## Executables and installation

Research and installed CLI SHA256:
`a7217ed919d40f46ac0d6644122f8218543039b2b4625365f0fb8d51a2670e97`.
Independent audit executable SHA256:
`7fd76a360811908f8281bc3558e8dcbab71be4bd6ccee420ad2263712e8c911d`.
These identify the actual executables; a future rebuild is not assumed
byte-identical. Exact source and system records support native replay.

The updated executable is installed at
`/home/agent/.local/bin/factor-base-sweep`. Outside-checkout `--help` and
sealed-run `verify` pass with PATH restricted to the local bin and system
bin directories, without a Cargo launcher. The prior PATH executable was
preserved as `factor-base-sweep-pre-oracle-20261019`, SHA256
`5272caabfe93a42b76a307d3ce5effbeb1cc8fe7aa7e5b1209f4afd4c67141e4`.
The original older research executable remains unchanged for its byte-bound
resumes. Local installation is not a published release or public download.

## Packaging and publication

The deterministic raw archive and its checksum sidecar preserve sealed
science, independent audit, command/exit logs, input/binary bindings,
host/toolchain metadata, setup failures and relay records. Subsequent
extraction, deterministic repacking and the exact new CI replay are saved
separately in `checks.tar.xz`; the original experiment directory is never
appended to. `PUBLICATION.json` records the actual sizes, hashes and checks.

The five-page PDF and SVG/150dpi PNG plots are regenerated from recorded
native fields and inspected. Python only handles metadata, canonical card
orchestration and rendering; native Rust performs all new research math
and verification. Benchmark browsing copies come from these same files.

Conductor returned127, the previously pinned workflow fetch404, Lean and
Lake127; these are unavailable, not passes. On October25 GitHub reads of
both PRs/current AGENTS returned404 and Git transport403. Current remote
heads, checks and merge policy are therefore unverified. The prior fully
read AGENTS blob is applied locally, with this limitation stated. No
publication or merge is claimed and no credentials were recovered or tried.

All existing registry, alias, store, prior research, historical evidence
and draft XP-card bytes are preserved. No root Cargo dependency, F4 source,
old test assertion, limit, ignore or coverage rule changed. Complete native
pricing, fastest external comparison and regularity certification remain
open; no full-cost or end-to-end performance result is admitted.
