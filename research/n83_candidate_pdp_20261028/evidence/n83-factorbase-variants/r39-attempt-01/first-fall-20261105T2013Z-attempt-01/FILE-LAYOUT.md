# Spike39 first-fall input-generation capsule

This capsule prepares the 48 spike39 input systems required by frozen protocol `N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2`. It binds generation to the saved r39 factor-base index and the original seed/source. The failed spike39 top-form attempt remains under its own immutable prefix and is not reused.

- `prepare-inputs.sh` stages the existing complete r39 base into a new local staging directory, checks the recovered index and all 384 compressed shard hashes, then invokes the pinned `pdp generate` command once.
- `GENERATION-COMMAND.txt` records the exact generation command. The generator seed is 20261027, embedded in the pinned source.
- `protocol-pdp-factorbase-v2.json` is the frozen first-fall/LFD protocol. `protocol-top-factorbase-v2.json` binds the source-system generation procedure and its frozen digest.
- `r39-base-s3-file-index.json` indexes every producer file and immutable S3 key; `r39-base-index.json` describes the complete three-family base. The 384 large point shards remain in their original S3 snapshot and are hash-checked while staging.
- `source-pdp.rs`, `source-fall.rs`, the Cargo manifests and `instruments/` bind the source and executable digests.
- After generation, `candidate-inputs/inputs.json` lists 48 case descriptors; each `candidate-inputs/<case>/input.json` binds that case’s family, m, target kind and input metadata; `system.sing` is the exact polynomial system passed to `fall`.

The generator preserves its output in the attempt-specific `inputs-v1/` S3 prefix. The later first-fall runner uses separate `data-v1/` and analysis prefixes. No point shards are duplicated in this attempt. A generated input manifest with system/input SHA-256 and byte counts must be committed and read back from S3 before the first fall case starts.
