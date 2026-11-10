# DRAFT Amendment R2-F (proposed number 14): 56Gi requests for the capped runs at `t = 13, 14` not yet launched

**Status: draft, NOT applied, NOT frozen.** Written 2026-11-10 by the main session under decision 6 of
`research/open_leads_execution_20261108/DECISIONS.md`. Target when frozen:
`research/prime_fall_degree_20261027/PREREGISTRATION.md`, appended as "Amendment 14". Resources only. Every number is
`[pcc:unverified]`; each is marked measured, cited or derived.

## 1. What happened (measured)

- Cell `pfd-x-t13-k0-c6` (Kummer `m = 2`, `P1`, `t = 13`, draw 0, cap 6) was launched twice with its Amendment 7 A request
  (112Gi) and was killed both times by an EC2 Spot interruption, after about 10 and 20 minutes. No result row exists.
  `SPOT_LOSSES.md` has the times, the audit-log events and the node types.
- Since 2026-11-09 20:00Z, every 128 GiB (r-family) node on the pool that ran longer than a minute was interrupted within about
  50 minutes (4 of 4). The two 30-minute cap-6 runs at `t = 12`, which asked for 56Gi and ran on `m7i.4xlarge` (64 GiB), were
  not interrupted. One `m7a.4xlarge` was interrupted after 31 minutes. Seven nodes: suggestive, not established.
- A 112Gi request can only be placed on a node with 128 GiB. A 56Gi request can be placed on 64 GiB nodes as well.

## 2. What I know that the amendment must not lean on (disclosure)

- I read the last line of the lost run's partial log (`SPOT_LOSSES.md`, loss 1): through step 26 its counters equal the
  `t = 12` cap-6 log's. That is not admitted data. It makes me expect that the `t = 13` cap-6 run needs about as much
  memory as the `t = 12` runs did (`max_dense_entries 1357804954`, about 5.4 GB at 4 bytes per entry; measured at `t = 12`).
- The amendment below does not depend on that expectation being right: section 4 says what happens when it is wrong.
- No `t = 13` or `t = 14` cap-6 row exists. The `t = 13` cap-5 rows exist (both draws: cap 5 does not refute).

## 3. The change (plan)

For the Amendment 7 B capped jobs at `t = 13` and `t = 14` **that have never been submitted** (today: `pfd-x-t13-k1-c6`,
`pfd-x-t14-k0-c6`, `pfd-x-t14-k1-c6`, and any later step of those brackets):

- memory request **56Gi** instead of 112Gi;
- `--max-dense` by Amendment 7 A's own formula, the largest multiple of 10^8 with `4·C <= request − 8 GiB`: **12,800,000,000**
  instead of 27,900,000,000 (derived; the value the `t = 12` requests carry);
- nothing else: same image, binaries, command stage (`cap:T:K:C:DENSE`), `--budget 86400`, 12 vCPU, job time limit 90,000 s,
  no instance pin, seeds, draws, caps and bracket order.

## 4. What a smaller limit can change, and the rule for it (fixed before any such row)

- `--max-dense` is a stop limit. Amendment 7 B already says: "A capped run stopped by its budget or `--max-dense` is censored:
  it bounds `d_F` from neither side."
- A run that completes under 12.8e9 dense entries computes the same thing it would under 27.9e9: the limit is only compared
  against, it does not steer the computation (cited: Amendment 7 A; the `t = 12` rows completed under the same limit).
- **If a 56Gi run stops on `--max-dense` (`oversize true`)**: that row is recorded as censored-by-limit and is **not** the cell's
  bracket row. The cell is then run once at the Amendment 7 A size (112Gi, `--max-dense 27,900,000,000`) under a job name with
  the suffix `-m112`, and that run's row is the cell's row. This re-run is fixed here, before any 56Gi row exists, so it is not
  a choice made after seeing an outcome. It is triggered only by `oversize true`, never by `inconsistent`, `timed_out` or any
  other field.
- A 56Gi run that completes (`oversize false`, `timed_out false`) is the cell's row, whatever it reads.
- A 56Gi run stopped by its budget (`timed_out true`) is censored as Amendment 7 B says; it is not re-run at 112Gi, because the
  budget is the same at both sizes.

## 5. What does not change

- No hypothesis, §6 rule, cell, seed, cap, command, image, wall budget, job time limit or cost ceiling.
- The two draws of a cell may now differ in memory request (`pfd-x-t13-k0-c6` was submitted at 112Gi and lost; `-k1-c6` runs
  at 56Gi). The request and `host.txt` of every row record it.
- The lost cell `pfd-x-t13-k0-c6` is not relaunched by this amendment. Owner decision 3 allows one relaunch and it was used.
  If the owner allows another attempt, it runs under this amendment (56Gi, with the section 4 rule).
- The `m = 4` requests (112Gi, Amendments 8 and 13) are not touched: their memory limit is `--hky-max-entries`, sized to the
  request by Amendment 8, and the closure is expected to need it.
- The Spot-loss rule (owner decision 3) is unchanged and applies to every job.

## 6. Requests

`make_requests_r2h.sh` regenerates the set into `requests_r2h/` with the new protocol hash. Differences from
`requests_r2g/`, only: `pfd-x-t13-*` and `pfd-x-t14-*` carry `memory "56Gi"` and the stage argument
`cap:T:K:C:12800000000`; one sentence is appended to every `provenance.note`. Jobs already submitted keep the request they
were submitted with.
