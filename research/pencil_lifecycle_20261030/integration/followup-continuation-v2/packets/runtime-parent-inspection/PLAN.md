# Read-only base image inspection

Inspect only sha256:347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476 on the existing local Docker socket. Save its layers as data to a fresh private directory, with a 90-second command deadline and 768 MiB output-file limit. No pull, create, start, build, tag or deletion; no payload execution. Source metadata, not a scientific measurement.

Check the exported config digest against the exact image ID and each uncompressed layer digest against rootfs.diff_ids before interpreting selected headers. Inspect root, /opt, /opt/isolab, /opt/isolab/recovery and /lib, /fixtures and /.mountpoint, /qualification and /exec-gate. Retain relevant whiteout and ancestor type changes; do not resolve uncertain link/whiteout cases silently. Root mount defaults are not inferred from absent headers.

This is read-only inspection under the existing research and reconciliation authorization, not renewal of historical assembly or qualification attempts. Execution admission remains separate. Docker documents image save as exporting parent layers: https://docs.docker.com/reference/cli/docker/image/save/. No managed environment-status tool is available; use the recorded explicit local socket with an empty Docker configuration, no credentials or network action.
