# Native exporter build gate

The frozen [protocol](PROTOCOL.md) was committed as `246a7ed01b` before
the first K=16,627 construction. The checked exporter source is the unchanged
file from parent commit `a44fc59`; its full content hash is recorded by the
object manifest at construction. The arm64 Linux release binary SHA-256 is
`c40614fab6b73e59e5f29fc57d7ef53680be3de962a902a26eb25a6c12c9257b`.

The offline build used the pinned image in the protocol, a read-only source,
read-only cached Cargo registry and Git dependencies, a separate writable
target directory, four CPUs, 8 GiB memory, zero swap and a 600-second outer
cap. The command inside the container was:

```sh
cargo build --release --offline --locked --no-default-features \
  --example koblitz_n83_factor_base_export
```

The [build log](build.stderr.log) reports completion in 4m29s. The
[container state](build.container-state.json) records exit 0 and no OOM. The
same bounded environment ran:

```sh
cargo test --release --offline --locked --no-default-features \
  --example koblitz_n83_factor_base_export
```

The [test log](example-tests.stdout.log) records 19 passed, 0 failed, and
the [test container state](example-tests.container-state.json) records exit 0
and no OOM. The [container Git check](container-git-status-before-commit.txt)
verified that the source and Git worktree are visible inside the offline
container; it listed only the then-uncommitted build/test records. The next
commit freezes those records so the exporter's clean-source gate can run.

No factor-base object was built by these build and test commands.
