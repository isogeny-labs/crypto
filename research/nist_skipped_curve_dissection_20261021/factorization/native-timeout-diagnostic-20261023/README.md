# Native NFS timeout formatting defect and checked resume

The pinned YAFU sieving timeout message passes a `uint32_t` to `%1.4sec`.
C parses that conversion as `%1.4s`, so the timeout value becomes a string
pointer. The original extracted statement exits **139**. Compiler checks
also reject a second message passing the integer to `%1.4f`.

The [two-line patch](timeout-printf.patch) uses `%u` and an unsigned cast at
both sites. More decisively, a full native control using the exact R05
input and selected polynomial reproduces **original exit 139 versus
patched exit 0**, with the corrected timeout message logged. Both use a
one-second native limit and the same small Q range in independent fresh
directories. The [binary-bound summary](native-control-summary.json)
identifies the tested executables. The original binary hash matches R05.
This reproduces the failure mechanism; the original production instruction
pointer remains unknown because no production backtrace was retained.

The [extracted-statement protocol](PROTOCOL.md) and
[separate full-tool protocol](NATIVE-VALIDATION-PROTOCOL.md) were frozen
before their respective controls. Both bounded diagnostics are finished;
neither launches an ongoing production factorization. The unchanged input
printed under native `C134` is the cofactor itself, not a proper factor.
No new mathematical result follows.

The [checked resume wrapper](resume_with_checked_tool.sh) requires a passing
native control summary bound to `YAFU_PATCHED_BIN` by SHA-256. It obtains
both existing output locks, prepares the checksum-bound checkpoint, retains
the previous executable and patch/control provenance, and installs only
that validated binary. Relations and polynomial files are untouched.
Prepare-only installation and an independent whole-target replay pass.
Wrong-binary binding and active-native-lock controls both reject. The
[later backlog card](../../../../backlog/XP-20261022-nist-p256-nfs-resume/README.md)
now names this wrapper and R05's relation checkpoint; it remains draft and
unqueued until store publication, source pin and selected lab are ready.

For another compatible host, apply the patch to the pinned source with
`patch --binary -p1`, rebuild with matching GMP/libecm and AVX2/BMI2, then
run [run_native_controls.sh](run_native_controls.sh) with
`YAFU_ORIGINAL_BIN`, `YAFU_PATCHED_BIN`, `YAFU_DATA_DIR`, `GNFS_BIN_DIR` and
the matching library path. Supply its actual output as
`YAFU_TIMEOUT_CONTROL_DIR`. This validates that host's actual executable;
the local build hash is not assumed portable across rebuilds.

The first build attempt failed because the Makefile's automatic GMP search
omits Debian's multiarch include directory. The retained second command
sets `GMP_INCDIR=/usr/include/x86_64-linux-gnu` and succeeds. Both attempts
and the earlier historical compiler warnings remain in the raw archive.

The deterministic `runs.tar.xz` and sibling checksum preserve all raw
extracted/native controls, build logs, original and patched test binaries,
commands, state, wrapper preparation and rejection checks. Evidence named
individually stays loose under AGENTS.md section 13a. See the
[report](reports/run-20261023T0113Z/REPORT.md),
[diagram](reports/run-20261023T0113Z/timeout-path.svg), and
[inspected PDF](reports/run-20261023T0113Z/report.pdf).
