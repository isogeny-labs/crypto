# Independent review: private backstop mechanism observation

The retained files support the local mechanism result at its stated scope. The exact image read-back fixture matches the built binary (`7aa53c…`), the copied deadline source matches the controller source (`b7eaf3…`), and the created-container inspection shows a private PID namespace, no network, read-only root filesystem, no capabilities, no mounts/devices, bounded memory/CPU/PIDs, no restart, and the expected root PID1 marker.

The observed container recorded the detached child reaped by signal 9, followed by PID1 exit 125 with `OOMKilled=false`. Cleanup rechecked the exact ID/name/image/owner label, removed that stopped container, and returned zero.

Four evidence notes remain:

1. No retained command receipt cryptographically identifies `run-reviewed-container-v4.sh` and its exact arguments as the invocation, although the outputs are consistent with that script.
2. The build receipt binds time and exit only; source/artifact hashes are retained separately, but the exact cargo command and toolchain are not.
3. Removal success is retained; the later owner-reported live no-labelled-container readback is not saved in this directory.
4. The ordinary iidfile read failed because the successful build wrote a root-owned mode-0600 file. Read-only sudo reconciliation produced the same image ID as the build log and image inspection. This requires no rebuild.

The observation qualifies this one local mechanism only. It does not qualify production packaging, Kubernetes/CRIU behavior, physical deallocation, clocks, timing, or scientific execution.
