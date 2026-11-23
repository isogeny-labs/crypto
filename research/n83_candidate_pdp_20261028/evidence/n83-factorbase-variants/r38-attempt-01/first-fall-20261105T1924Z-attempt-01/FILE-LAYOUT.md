# Spike38 first-fall continuation capsule

This capsule binds a bounded first-fall run to the frozen protocol and the 48 spike38 systems. The exact capsule-file hashes are in `CONTINUATION-BINDING.json`.

## File map

- `run-first-fall.py`: validates protocol, manifest, binary and every input hash; runs one system, replays its certificate in a fresh process, records failures, and builds output custody indexes.
- `launch.sh`: invokes the runner once with CPU 0, an 8 GiB virtual-memory limit, one attempt, and a 12,850-second outer timeout.
- `protocol-pdp-factorbase-v2.json`: frozen experiment protocol source.
- `input-manifest.json`: 48 system/input hashes and case descriptors. Each system is `system.sing`; each case descriptor is `input.json`. Source files remain in the spike38 attempt-09 S3 result bundle.
- `source-fall.rs`, `Cargo.toml`, `Cargo.lock`: source and build provenance for the pinned executable.
- `instruments/fall`: executable retained in the S3 capsule; SHA-256 is recorded in `CONTINUATION-BINDING.json`.

The exact result layout and its file semantics are documented in the generated `FILE-LAYOUT.json` after execution. The data run has a separate `data-v1/` prefix so capsule publication cannot contaminate the empty-output preflight. LFD closure is not part of this binary or run; its fields remain null.
