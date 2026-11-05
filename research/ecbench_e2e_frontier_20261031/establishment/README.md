# Frontier establishment audit

This directory is the executable evidence gate for the frontier statement in
the parent report. It reruns every locally available deterministic replay and
recomputes the decisive ratios from retained source artifacts. It deliberately
does not relabel a historical binary, a counted unit, or a shared-host result
as a fresh isolated wall measurement.

## Verdict

| Claim | Evidence status on 2026-10-31 |
|:--|:--|
| Generic prime, random-binary, and Koblitz domains are rho-led | Established as the standing admitted repository frontier; the prepared current-source L2 campaign remains unexecuted and `protocol.json` remains `draft` |
| Restricted GHS-weak family is IC-led | Established in the committed counted aggregates: N4 B0/plain-rho is 0.73475 [0.66191, 0.81643] at q=4051; N4-v2 DL/negation-rho is 0.00523 [0.00395, 0.00764] at q=2097091 |
| P20 N=45 improves N=32 but loses to rho | Established by second-host L2 counted evidence: 0.836/0.817/0.746 versus N=32 and 7.95/8.24/9.62 versus rho across the pricing band |
| F5/F6 save solver-local work but not E2E work | Established for the historical experiment binary. Not established for the current binary: only 72/144 deterministic rows all-replay identically |
| Optimized signed-Frobenius rho is 1.246642x faster in instructions | Established from 30 hash-verified Callgrind files and a fresh 720/720 current-binary replay; L2 wall speedup remains unknown |
| Registry coverage is complete | Established: 74 unique top-level method IDs and 11 nested solver backends, with an exact coverage disposition for each |

The GHS raw `units.tar.gz` is named by SHA-256 in its artifact manifest but is
not available from this workspace's unconfigured artifact store. The audit can
therefore verify the frozen protocol and committed aggregates, not recompute
that aggregate from raw unit records here.

The F4/F5/F6 replay failures are retained in `receipts/`. Stored record
integrity and original scalar verification still pass; the current binary
changes phase counts for affected IC arms, including GAE/unpriced accounting
on K13. The historical result remains evidence about its measured binary, but
must not be presented as a current-source measurement.

## Reproduce

From the repository root:

```text
python3 -I research/ecbench_e2e_frontier_20261031/establishment/establish.py
```

The command rewrites `certificate.json` and exits nonzero if any of its 19
artifact-consistency checks fails. A passing certificate means the cited
artifacts agree with the versioned verdicts above; it does not satisfy the four
open obligations recorded in the certificate.
