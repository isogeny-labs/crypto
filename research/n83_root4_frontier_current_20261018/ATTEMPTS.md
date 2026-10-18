# K3596 current-source pilot attempts

The K3596 full-process pilot checks the exact ordered base and root index on
an already used N83 point. Its timing is excluded from the fresh holdout.

| Attempt | Output location | Recorded outcome | Selection use |
| --- | --- | --- | --- |
| 1 | `/private/tmp/n83-k3596-current-pilot-20261018` | `INFRASTRUCTURE_CENSORED`: the local session was lost, no process remained, and the temporary directory had disappeared before a receipt could be retained | None |
| 2 | `pilot_work` under this experiment directory | `PASS`: 434.890566875 seconds full-process wall time; 8/8 producer checks and 15/15 independent replay checks passed | None |

Attempt 1 has no numerical result. The second attempt uses the same pinned
source, binary and exposed point. Its raw archive is `pilots/k3596.tar.gz`,
SHA-256 `18f48c6ebbf697ffaea8f2b3612f06a8266fdd8b72c5bd72dd34c75508b735c3`.
The replay checks all 596,936 base points, rank 3,596, root-index occupancy,
four-point relation, subgroup scalar, and exclusive online phases. The
archive is freshly extracted and replayed by `prepare_dispatch.py` before
the cloud dispatch can be sealed.
