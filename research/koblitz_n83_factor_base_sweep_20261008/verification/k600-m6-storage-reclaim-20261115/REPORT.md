# N83 generated-cache storage reclamation

At 2026-11-15 21:32 UTC, the inactive `/Volumes/SSD990-2/tmp/n83_native_replay_build` directory contained Cargo's `CACHEDIR.TAG`, a `debug/` target last modified on 2026-10-27, and no process command line referencing that target. `du -sh` reported 4.6 GiB. Its generated `n83-exclusion-audit` Mach-O arm64 executable had SHA-256 `286c8b26cf3d94f6320bb5f9dec468780979ded202e94e4600ad1e579064f507`; this identifies the removed cache artifact, not a preserved executable.

`cargo clean --target-dir /Volumes/SSD990-2/tmp/n83_native_replay_build` exited 0 and reported 4,250 files and 5.8 GiB removed. `df -h /Volumes/SSD990-2` showed 3.2 GiB available immediately before and 7.8 GiB immediately after. The `df` change is observational because concurrent host activity and APFS accounting may affect it. No source, frozen input, raw research record, or S3 object was removed. This is capacity preparation, not a build, relation or IC runtime result.

The next admission check must confirm that both the SSD and boot volumes remain stable before a new isolated library test or cold Linux build. The prior test attempts remain censored; this cleanup does not clear validation or qualify Docker, which was unresponsive on the latest check. No performance graph changes.
