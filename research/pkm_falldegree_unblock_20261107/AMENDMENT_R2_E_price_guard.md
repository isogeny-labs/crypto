# Amendment R2-E, frozen 2026-11-09 as PREREGISTRATION Amendment 12: a price guard replaces the instance pin for long jobs

**Status: FROZEN 2026-11-09 as Amendment 12 of research/prime_fall_degree_20261027/PREREGISTRATION.md, before the `pfd-b` relaunch
is submitted and when no stage B row is admitted.** Resources only. Written by the main session. Every number is
`[pcc:unverified]`; each is marked measured, cited or derived.

## 1. What happened (measured)

- `pfd-b-20261108` (Amendment 9: 12 vCPU, pinned `r6i.4xlarge`) was launched by card B9-1 at 20:34Z on 2026-11-09. Its pod
  started 20:36:33Z and was killed 20:52:48Z: exit 137, pod reason `Terminated`, message "Pod was terminated in response to
  imminent node shutdown." (EKS API-server audit log, the node's kubelet). That is a Spot reclamation, 16 minutes in.
- The node was `r6i.4xlarge`, `us-east-2a`, capacity type `spot` (audit log, node labels).
- The run had finished `g1` (exit 0) and started `B-m3-tower`. **Nothing from it is admitted or analysed.**
- Owner decision 3 (`research/open_leads_execution_20261108/DECISIONS.md`): a lost Spot cell is reported as lost, with at most
  one relaunch under a new name. This is the first Spot loss of `pfd-b`; the relaunch is `pfd-b-20261109-r1`.

## 2. Why the pin is the wrong tool (measured, 2026-11-09 about 21:00Z)

`aws ec2 get-spot-placement-scores --target-capacity 1` (score 1 to 10; higher means the request is more likely to be met):

| request | score |
|:--|--:|
| `r6i.4xlarge` only, region us-east-2 | 3 |
| `r6i.4xlarge` only, each of the three zones | 1 |
| eight 16-vCPU types that fit the job (m6i, m6a, m7i, m7a, r6i, r6a, r7i, r7a `.4xlarge`), region us-east-2 | 9 |
| the same eight types, each of the three zones | 9, 9, 6 |
| fourteen m/r `.4xlarge` types (the eight plus m6id, r6id, m8i, r8i, m8a, r8a), region | 9 |

Amendment 9 pinned every job with `seconds` above 90,000 to `r6i.4xlarge`, to keep the owner's US$40 per job at that type's
price. The pin confines a job of up to 49 h to one Spot pool. An unpinned job lets Karpenter choose among about 30 eligible
types in three zones. With one relaunch allowed, the pin makes losing stage B more likely than it needs to be.

A first reading of 9 was taken with a ten-type list that included two small types (c6i.xlarge, m6i.large), which could
inflate the score. It was discarded and re-measured with eligible 16-vCPU types only; the table shows the re-measurement.

## 3. Rule (the text frozen in Amendment 12)

1. **No pin.** Jobs with `seconds` above 90,000 are submitted without `instance_type`, like the shorter jobs. Every other field is
   unchanged (cpu 12, memory, `seconds`, command, image, wall budgets).
2. **Price guard at start.** Within 30 minutes of the pod's start, the main session reads the node's instance type `T` and zone `Z`
   from the EKS audit log (node labels), and the latest Spot price `P` of `(T, Z)` (`aws ec2 describe-spot-price-history`). The
   bound is Amendment 9's: `B = (seconds/3600 + 0.25) × (P + 0.0088)`. If `B > 40`, the job is deleted
   (`isolab.eks.command-delete/v1`) and resubmitted under a new name. That deletion is a **cost-guard stop, not a Spot loss**: it
   does not use the relaunch of owner decision 3, and nothing from the deleted job is admitted.
3. **Price guard while running.** At every check-in (at most 4 h apart) `P` is read again. With `h` the hours since the pod
   started and `Pmax` the highest price read so far, the projection is
   `h × (Pmax + 0.0088) + (seconds/3600 + 0.25 − h) × (P + 0.0088)`. If it exceeds 40, the job is deleted. It is then reported as
   "stopped by the cost guard", nothing from it is admitted, and whether to run it again goes to the owner.
4. Each reading (`T`, `Z`, `P`, `B` or the projection, and the time) is recorded in the job's receipts directory and in RESULTS.

Thresholds (derived): the start guard deletes above `P = 40/48.25 − 0.0088 = 0.820 $/h` for a 172,800 s job and above
`40/49.25 − 0.0088 = 0.803 $/h` for a 176,400 s job. Amendment 9's price table (2026-11-08) has four eligible types above that
(`r8in`, `m8id`, `r8ib`, `r8idb` `.4xlarge`); the others are between about 0.20 and 0.80 $/h.

## 4. What does not change

No hypothesis, §6 rule, cell, seed, command, image, memory, cpu, wall budget or job time limit changes. The US$40 per job and
US$400 total ceilings are unchanged; the guard enforces the first one by observation instead of by the pin. The instrument note
of Amendment 9 stands: F4 `muladds` are compared only between rows with the same recorded thread count (12 everywhere). The
instance type is recorded per job (Amendment 7 A already requires it).

## 5. Requests

`make_requests_r2f.sh` regenerates the 18 requests of `requests_r2e/` into `requests_r2f/` with the new protocol hash, the pin
removed and one sentence added to the provenance note. `requests_r2f/pfd-b-r1.json` is `requests_r2f/pfd-b.json` with
`job_name` `pfd-b-20261109-r1` and one more note sentence. `requests_r2c/`, `requests_r2d/` and `requests_r2e/` stay
byte-identical. The R2-B draft (null cells) moves to proposed Amendment 13.
