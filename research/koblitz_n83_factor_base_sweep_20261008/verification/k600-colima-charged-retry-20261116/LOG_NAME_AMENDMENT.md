# Charged-build retry: distinct log-name amendment

**Frozen before another admission.** The [host preflight](HOST_PREFLIGHT_RESULT.md) rejected the first source-correct build launcher before Docker because its log path still named the completed attempt-2 build. No output or container was created.

The corrected [admission](admit-current-2.sh) uses a distinct attempt-6 log, receipt, source file and container. The corrected [build launcher](build-current-2.sh) uses distinct attempt-3 log, receipts, source file and container matching its already new `charged-20261116-attempt-3` output. It reads only the attempt-6 admission PASS from the same clean commit. The source-correct producer SHA, plan, stage caps, cgroup, host floors, disk-stop thresholds and input checks from [the preceding amendment](CURRENT_PRODUCER_AMENDMENT.md) are unchanged. A passing admission is still required before the build.
