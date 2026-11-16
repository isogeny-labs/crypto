# Charged-build retry: boot-floor amendment

**Frozen before another admission.** The [attempt-3 host preflight](BOOT_FLOOR_RESULT.md) exited 96 because boot available space was below the prior 20 GiB floor; no Docker container or build output was created. The first admission timeout and its raw record remain unchanged.

The new [admission launcher](admit-rescreen.sh) and matching [build launcher](build-rescreen.sh) set the host boot floor to **15 GiB**. The 40 GiB SSD floor, 10 GiB VM-root floor, 12 GiB/zero-swap/four-CPU/512-PID envelope, clean source and exact plan/archive/image/producer checks remain. The 600-second hard-kill no-work cap and progress markers from the preceding amendment remain. The build retains the 4,800-second hard-kill outer cap and three committed stage caps. It uses the fresh `charged-20261116-attempt-2` output and reads only the distinct attempt-4 admission receipt.

The host boot volume holds small `/private/tmp` logs and receipts for this run; the build output, Colima home and VM image are on the SSD. Lowering the reserve admits the current host state but does not prove sufficient disk for all stages. Check free space before execution, preserve any storage failure with a terminal receipt, and do not launch a worker unless the complete build and separate budget/input gates pass.
