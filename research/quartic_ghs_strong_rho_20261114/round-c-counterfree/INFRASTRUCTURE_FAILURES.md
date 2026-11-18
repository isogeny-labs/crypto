# Retained infrastructure failures

These failures occurred before the successful source build. They are not
algorithm failures or benchmark results.

- The repository `conductor` command was not available (`command not found`).
- The research-KB wrapper found no executable, so no external novelty or
  literature-absence claim was made.
- Creating the first clone and metadata-light worktree on `/workspace` failed
  because the overlay had no space available at the time.
- A tmpfs shared clone backed by the old partial `/workspace/crypto` object
  store could not lazily retrieve required objects.
- A second tmpfs shared clone backed by the existing results checkout could
  read the relevant sources, but commit construction failed on missing
  unrelated historical blob objects.
- Recovery used a clean, shallow, blob-filtered remote clone in `/dev/shm`.
- The first repository-wide `cargo test --release --lib` attempt in that sparse
  clone failed before compilation because the `certified-relations` path
  dependency was absent. The path dependencies were then materialized and the
  command retried; its final status is recorded separately.

The exact implementation build, standalone controls and IR gate all ran from
the clean recovery clone.
