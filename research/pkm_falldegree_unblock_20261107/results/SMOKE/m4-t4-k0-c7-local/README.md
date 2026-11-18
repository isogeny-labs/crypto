# Local m = 4 smoke test (owner decision 17): NOT a registered row

The probe is `m = 4`, `t = 4`, `c = 7`, draw 0, on the session machine (4 vCPU, 15 GB), using the frozen `pkm_tower_pilot`.
It differs from the registered command in two settings: entry cap 1.2e9 instead of 13.96e9, and budget 7,200 s instead of
172,800 s. It enters no reading, gate or median.

## Outcome (measured)

- **Attempt 1** (05:34:46Z): stopped by the session's 30-minute background limit. No row (`attempt1.txt`).
- **Attempt 2** (06:05:02–07:49:15Z, budget 6,000 s, 4 threads): **killed by SIGKILL (exit −9) after 6,252 s.**
  - Peak RSS 13,952,544 kB, about 13.95 GB, on a 15.7 GB machine.
  - RSS rose almost linearly, about 2.2 MB/s (`rss_samples.final.txt`, one sample per minute).
  - The kill is consistent with the kernel's OOM killer. The kernel log was not readable here, so this is an inference.
  - The pilot writes a row only when a system finishes, so there is **no row**: no closure degree reached, nothing known
    about `d_F`.
- **What it shows.** Under this machine's limits, this cell does not finish in about 1.7 h or about 14 GB.
  - The entry cap (1.2e9 entries, about 9.6 GB of matrix data) did not keep the process's total memory under the cap.
  - The registered 112Gi request (13.96e9 entries) is about 8 times this cap, and Amendment 8's projection for
    `t = 4, c = 7` is 8–33 GB. Both are consistent with a probe this small not finishing.
  - It does not tell whether the registered job finishes inside its 48 h budget.
