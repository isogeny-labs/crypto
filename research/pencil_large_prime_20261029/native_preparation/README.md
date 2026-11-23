# Native preparation and plan parity

This standalone Rust crate replaces the active execution paths of the historical
`prepare_native_parity.py` and `prepare_execution_specs.py` scripts. The Python
files and their receipts remain unchanged as provenance. They are not used by
this crate, its tests, or the commands below.

The repository sources checked before implementing this port were
`AGENTS.md` lines 568–589, both historical scripts, `validation_inputs/manifest.json`,
`prepared_specs/manifest.json`, `native_parity/result.json`,
`prepared_native_plans/result.json`, and the receipts under
`static_build_validation/`. These are the primary contract for this mechanical
port. No mathematical or performance method changes here, so external
literature does not define the required behavior.

The commands are native and fail closed:

```text
cargo run --release -- prepare-archive ARCHIVE OUT
cargo run --release -- prepare-specs INPUTS OUT
cargo run --release -- check-archive BINARY INPUTS OUT [--expected-plans DIR]
cargo run --release -- check-prepared BINARY INPUTS SPECS OUT [--expected-plans DIR]
cargo run --release -- prepare-disjoint-enrollment REQUEST OUT
cargo run --release -- check-disjoint-enrollment REQUEST OUT
```

`prepare-archive` verifies the frozen archive SHA-256, reads the zstd/tar stream
without extracting paths to the filesystem, copies the seven archived spec
members byte for byte, and projects exactly the 1,260 public workload identities,
targets, seeds and size labels. It excludes planted answers.

`prepare-specs` verifies every projected-input digest and emits the seven cohort
specs plus the two ordered gates. Its Rust regression test requires all nine
specs and their manifest to be byte identical to the retained prepared files.
The active contract is denominator 8, matrix coefficient cap 25,000,000,
8 GiB memory admission, 3,600 seconds per execution, one warmup, five measured
streams, L2 isolation, and paired shuffled arms `pencil`, `rho`, and `rho_AA`.

The two check commands invoke only `ecbench plan --doc`. They never run a solver
or target. Archive checking compares all 1,260 workload IDs, target coordinates,
target seeds, target indices and archived first algorithm seeds. Prepared checking
also verifies the three arm roles, identical resolved rho/reference and A/A
methods, paired per-workload seeds, the first measured seed, all resource and
measurement settings, both gate memberships, 22,680 cohort plan slots, 18,900
measured slots and 3,780 warmups. Supplying `--expected-plans` additionally
requires byte equality with retained plan JSON.

All outputs remain preparation or run-free plan evidence. They do not authorize
an experiment, attest a source build, measure a cost, recover an unknown scalar,
or establish a cost ratio against rho.

`prepare-disjoint-enrollment` is an isolated continuation route documented in
`DISJOINT_ENROLLMENT.md`. It leaves `prepare-specs` and `check-prepared` and
their original nine-file inventory unchanged. A strict, hash-bound request
names the approved seven cohort specifications and plans, the retained p739
gate specification and plan, the pinned instrument and build information, and
explicit protocol-lineage files. The command derives only the cohort-16
remainder specification, invokes only `ecbench plan --doc` for that remainder,
and proves a complete 22,680-slot bijection against the original seven plans.
Its certificate and mappings preserve original and enrolled sequence numbers
and workload indices. They make no admission, timing, launch, or empirical
claim.

`check-disjoint-enrollment` is solver-free. It rereads every hash-bound input,
rederives the permitted specification change, reparses all seven original and
eight enrolled plans, recomputes both mappings, and compares their exact bytes
and certificate hashes. Consumers need not trust the certificate's `passed`
or count fields.
