= Terminal restrictions and constant-residual S3
2026-11-15 · PR3744 · TZT-20261115-terminal-domains · local unsigned evidence

#image("outcomes.svg", width: 100%)

*Observed:* all 686 query executions complete with zero invalid decisions. Per 98 rows, constants decides 50, identity-first 64, marginal tail 78, correlated tail 74, sliced 98 and fresh pair-table 98. Sliced A/A has zero outcome or conflict discrepancies. Each repetition uses the same 49 instances; it is not an independent population sample.

*Construction:* for P+Q+R=T, enumerate the actual final point R and solve P+Q=T−R with a fixed-target two-point S3 system. Every residual shares the same total conflict, model and deadline budget. Incomplete residual work stays UNKNOWN. Actual supplied signs and identity branches remain; every accepted sum is replayed in the original group.

Marginal tail restricts the finite intermediate to x(T−B); correlated tail additionally retains its coupling with the last x. Both use established native domain machinery. Identity-first searches the same complete masks in a different order. The classical reference rebuilds its exact pair table for every query; its setup is charged, with no free cached truth.

*Nontrivial controls:* retain all 33 earlier cases and add 16 holdouts at n83/131,d6/8. Planted targets come from three distinct x values and are nonidentity and outside the base. Wide bases have 26/134 points at n83 and 26/112 at n131. Sliced finds every distinct-x planted holdout, including n131,d8. These are controlled targets on the actual curve, not the public challenge Q.

*Accounting:* per 98 executions, recorded additions+doublings are 3342 sliced and 132124 fresh pair-table; maximum sliced SAT allocation is 981 variables. Those units exclude field/ANF construction and SAT work, and setup could be amortized in another workload. No runtime ratio or full-cost gain is claimed. The extra coupled constraint leaves more UNKNOWN here than the marginal constraint; both outcomes are retained.

*Flow and controls:* planned 686, attempted 686, completed 686, excluded 0, interrupted 0, retries 0. A separate 35-call engineering pilot and native analysis smoke pass their fixed gates. One CPU, 2GiB, 600-second combined pilot/freeze/run cap; each query 2000 conflicts and 750ms safety fuse. Shared host, no L2 clock. The user's local EKS/S3 exception applies; no paid compute launched.

*Validation:* native audit rechecks all 49 truths and replays all 398 saved original point tuples; no mismatch. Canonical export/source/file check passes. Two focused exact small-group/cap checks and final production regressions are retained in the scale study. No independent signed review exists. Completed earlier panels remain historical; the new strategy changes their scoped non-singleton UNKNOWN outcome.

*Reproduction and identity:* README.md links the exact manifest, raw rows, exits, pilot, summary, export and native sources. Source 3827643341debd85404333b090bd2fbfec6f3472; protocol 7883b155e9c7015e213d879c21a3c4fd0aa79145b408cc00f1adf54e913a2c94; binary 55c9827dda5a903b5386087e6118170c959809aa9246b695524ec55b277dc664. This reuses the existing S3/group/meet-in-the-middle algebra; no mathematical originality or challenge-recovery claim.
