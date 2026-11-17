# Amendment R2-F, frozen 2026-11-10 as PREREGISTRATION Amendment 14: 56Gi requests for the capped runs at `t = 13, 14` not yet launched

**Status: FROZEN 2026-11-10 as Amendment 14 of research/prime_fall_degree_20261027/PREREGISTRATION.md, before any cap-6 row at
`t = 13` or `t = 14` exists and before any job it covers is submitted.** Written by the main session under decision 6 of
`research/open_leads_execution_20261108/DECISIONS.md`. It changes resources and the `--max-dense` censoring limit, with a fixed
re-run rule. Every number is `[pcc:unverified]`; each is marked measured, cited or derived. An independent adversarial review
(red-team agent, 2026-11-10) read the draft against the source and the preregistration; its edits are applied (section 7).

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
- Admitted evidence that points the same way, without the partial log: the two `t = 13` cap-5 rows (`results/Y1`, `results/Y2`)
  equal the `t = 12` cap-5 rows in `max_cols 16572`, `muladds 179052499158`, `max_dense_entries 53833099` and `basis_len 2922`
  (measured). That is cap 5, not cap 6.

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
  against, it does not steer the computation (source at f13a690df0: `src/cryptanalysis/f4_fp_tower.rs:1578` is the only use, a
  stop before `B'` is built; it is passed in at `:2698`; the test at `:3908` asserts that a run below the limit has the same basis
  as an unlimited run; read by the reviewer and by the main session).
- **If a 56Gi run stops on `--max-dense` (`oversize true`)**: that row is recorded as censored-by-limit and is **not** the cell's
  bracket row. The cell is then run once at the Amendment 7 A size (112Gi, `--max-dense 27,900,000,000`) under a job name with
  the suffix `-m112`, and that run's row is the cell's row. This re-run is fixed here, before any 56Gi row exists, so it is not
  a choice made after seeing an outcome. It is triggered only by `oversize true`, never by `inconsistent`, `timed_out` or any
  other field.
- **A 56Gi run killed by the memory limit** (container reason `OOMKilled`) counts as `oversize true` and is re-run once at 112Gi
  in the same way. Exit 137 is classified by the pod and container reason in the EKS audit log, never by the exit code alone: a
  Spot kill reads "Pod was terminated in response to imminent node shutdown." and falls under owner decision 3.
  Why this case exists (measured at `t = 12`, cap 6): peak memory was 6506 MB with at most 1,357,804,954 dense entries, about
  1.2 times the 4 bytes per entry the formula assumes; near the limit a run can exceed its memory request before the
  `--max-dense` stop fires.
- A 56Gi run that completes (`oversize false`, `timed_out false`) is the cell's row, whatever it reads.
- An `-m112` run that stops on `--max-dense` or is `OOMKilled` is censored (Amendment 7 B); there is no larger size.
- A 56Gi run stopped by its budget (`timed_out true`) is censored as Amendment 7 B says; it is not re-run at 112Gi, because the
  budget is the same at both sizes.

## 5. What does not change

- No hypothesis, §6 rule, cell, seed, cap, command template (the `--max-dense` argument changes as in section 3), image, wall
  budget, job time limit or cost ceiling.
- The two draws of a cell may now differ in memory request (`pfd-x-t13-k0-c6` was submitted at 112Gi and lost; `-k1-c6` runs
  at 56Gi). The request and `host.txt` of every row record it.
- The lost cell `pfd-x-t13-k0-c6` is not relaunched by this amendment. Owner decision 3 allows one relaunch and it was used.
  If the owner allows another attempt, it runs under this amendment (56Gi, with the section 4 rule).
- The `m = 4` requests (112Gi, Amendments 8 and 13) are not touched: their memory limit is `--hky-max-entries`, sized to the
  request by Amendment 8, and the closure is expected to need it.
- The Spot-loss rule (owner decision 3) is unchanged and applies to every job. The `-m112` run is not a Spot relaunch. A
  Spot-lost job is relaunched at its own size. Decision 3's one relaunch counts per cell, across both sizes.
- Cost: worst case, one extra job per cell (at most US$40 by the per-job ceiling). The US$400 study total is checked before each
  `-m112` submission.

## 6. Requests

`make_requests_r2h.sh` regenerates the set into `requests_r2h/` with the new protocol hash. Differences from
`requests_r2g/`, only: `pfd-x-t13-*` and `pfd-x-t14-*` carry `memory "56Gi"` and the stage argument
`cap:T:K:C:12800000000`; one sentence is appended to every `provenance.note`. Jobs already submitted keep the request they
were submitted with.

## 7. Review (2026-11-10)

A red-team agent (Opus) reviewed the draft read-only. Verdict: freeze with edits. What it checked and found:

- `--max-dense` only stops a run; it does not change a finished run's `inconsistent`, `degree_reached` or `basis_len` (source lines
  in section 4). Holds.
- The memory request does not change the computation otherwise: chunk size depends on the thread count only
  (`f4_fp_tower.rs:1694`), which comes from the 12-vCPU request; `/proc` reads feed trace lines and `host.txt` only. Holds, except
  that a run can be killed for exceeding its memory request; the `OOMKilled` rule in section 4 was added for that.
- The re-run trigger is a fixed field, so it leaves no choice after data. Holds.
- The draft's label "resources only" was wrong, because `--max-dense` is Amendment 7 B's censoring limit; the label, the "command"
  wording, the source citation, the decision 3 wording and the cost line were corrected as it asked.
