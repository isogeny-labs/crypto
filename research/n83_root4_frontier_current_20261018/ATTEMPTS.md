# K3596 current-source pilot attempts

The K3596 full-process pilot checks the exact ordered base and root index on
an already used N83 point. Its timing is excluded from the fresh holdout.

| Attempt | Output location | Recorded outcome | Selection use |
| --- | --- | --- | --- |
| 1 | `/private/tmp/n83-k3596-current-pilot-20261018` | `INFRASTRUCTURE_CENSORED`: the local session was lost, no process remained, and the temporary directory had disappeared before a receipt could be retained | None |
| 2 | `pilot_work` under this experiment directory | Pending independently replayed receipt | None |

Attempt 1 has no numerical result. The second attempt uses the same pinned
source, binary and exposed point. Its archived raw output and verifier
receipt must pass before the sealed cloud dispatch is created.
