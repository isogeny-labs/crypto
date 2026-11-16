# IC line-layout tournament v6: V6-AMENDMENT-1 (host epoch H2)

**Status: FROZEN when the main session merges it to `main` (the merge commit is its freeze).**
Filed 2026-11-06, before any wall field of session s1 or s2 was read
(`filed_before_affected_data: true`). It implements, word for word,
[`DECISION-V6-REDESIGN-20261106.md`](DECISION-V6-REDESIGN-20261106.md) section 15
(Revision 4, coordinator ruling). It changes no spec, seed, threshold, arm, image,
binary, deadline or analysis script. The frozen protocol is
[`protocol-v6.json`](protocol-v6.json), `IC-LINE-LAYOUT-E2E-FP28-V6-20261106`.

**Prior freeze (recorded; reproducible).** The original freeze was
`protocol_sha256` `12415faa3330276b6ccbed718a03bc598d9717576c332577024ebc4e9752543c`,
commit `7c173e5a1d418dce4c734c43a25718a71107a36c`, `frozen_at` `2026-11-06T04:25:05Z`.
It is reproduced by `experiment_protocol.py hash` on the amended document with
`amendments` set to `[]` and the key `extensions.v6_amendment_1` removed. After this amendment the `freeze` block
of `protocol-v6.json` holds the freeze of the document as it now stands (the new
canonical hash, with the commit and `frozen_at` of the re-freeze), and the prior
freeze is recorded in `protocol-v6.json` under `extensions.v6_amendment_1.prior_freeze`.

Where this file and a field of `protocol-v6.json` named in section 6 disagree, this
file governs; everything not named there is unchanged.

## 1. Reason

Both wall jobs finished at 04:55:54Z with launcher state `succeeded` and `job-exit`
`exit_code` 0, and passed the integrity checks the main session reads without opening
a wall field. They ran one after the other on one node. Recorded identities (the
`job_name`, `pod_uid` and `node_name` of each `job-exit.json`, and the job UIDs of the
launcher logs):

| Session | Job UID | Pod UID | `node_name` |
|---|---|---|---|
| s1 `ic-line-v6-s1-20261106` | `10c77943-3a48-409e-bac7-74bb2a921926` | `c4934aa4-5562-44e2-8789-4a6a26a43e4e` | `ip-10-84-67-216.us-east-2.compute.internal` |
| s2 `ic-line-v6-s2-20261106` | `6726db8e-db94-4eaa-9d06-989d6b12fcea` | `9c36a122-d184-4105-9e42-94df3241d815` | `ip-10-84-67-216.us-east-2.compute.internal` |

The frozen factor `host_epoch` requires each wall job on its own fresh node with a
distinct node identity. The pod anti-affinity keeps two pods off one node only while
both exist; the node was reused. The Job and Pod identities are distinct and the node
identity is not, so the frozen host-epoch condition is not met by s1 and s2. This is an
infrastructure fact, not evidence about the candidate. No frozen rule can supply a
second host epoch: host-loss reruns cover only jobs with no `job-exit.json`, and there
is no noise-rerun clause.

## 2. Changes (numbered as in decision section 15.3)

1. **s1 and s2 are kept whole and both stay decision-bearing.** Together they are host
   epoch **H1**, node `ip-10-84-67-216.us-east-2.compute.internal`. Neither is
   discarded, demoted or replaced.
2. **One additional wall job, `ic-line-v6-s3-20261106`, is host epoch H2.**
   - **Request:** `launch-v6/request-session-3.json`. It is byte-identical to
     `launch-v6/request-session-1.json` apart from two lines: `job.job_name`
     (`ic-line-v6-s3-20261106`) and the provenance `harness` label (`session 3`
     instead of `session 1`). Same spec (`spec-fp28-v6.json`, sha256
     `0116954ff2646a81fa4383ef87125d445f2f3d3d4315a4ed49db9d0845515a81`), seeds, image,
     binary and 3,600 s deadline.
   - **Distinct-node rule, before launch.** The card confirms that no node named
     `ip-10-84-67-216.us-east-2.compute.internal` exists, and records the confirmation
     on the card. Observable form of the check (section 4).
   - **Distinct-node rule, after the run.** The `node_name` in s3's `job-exit.json`
     differs from `ip-10-84-67-216.us-east-2.compute.internal`. The launcher records no
     `providerID`, instance id or node creation time in the job output (the only node
     field `run-ecbench` writes is `node_name`, from `spec.nodeName`), so the node name
     is the identity that decides. Any further identity the card collects is recorded
     and does not decide.
   - **If s3 lands on H1's node anyway** (its `node_name` equals H1's): v6 cannot
     satisfy the host-epoch factor. C-wall is then at most `inconclusive`, the
     per-session results are reported descriptively, and no further session is added.
   - **Reruns.** s3 follows the frozen host-loss rule unchanged: a job with no
     `job-exit.json` is rerun once, whole, as the same request with the name suffixed
     `-r2` (`ic-line-v6-s3-20261106-r2`); a job with a non-zero `job-exit` is never
     rerun. A rerun's own `node_name` is the one tested against H1's. No `-r2`
     request file is frozen. The rerun's request is generated at rerun time
     (decision section 15.5, point 6), and the card records two checks before
     launch:
     - `jq -S -c 'del(.job.job_name)'` of the generated request is byte-equal to
       that of `launch-v6/request-session-3.json`;
     - its `job.job_name` equals `ic-line-v6-s3-20261106-r2`.

     The same applies to the `-r2` of any other v6 job.
   - **If s3 yields no result.** This happens if s3 and its `-r2` both write no
     `job-exit.json`, or if s3 exits non-zero. s3 then enters the combination as
     `invalid_measurement`. That is an infrastructure outcome, not evidence about
     the candidate.
3. **Decision rule (stricter than before, no selection).** C-wall is supported only if
   **each of s1, s2 and s3** separately passes every frozen gate of
   `PROTOCOL-V6.md` section 6 and `protocol-v6.json` `decision_rule`, and the host-epoch
   condition holds between H1 and H2 (section 3). Otherwise the frozen per-session
   verdicts are combined by the frozen overall precedence over all three sessions:
   all three `supported_within_scope` gives `supported_within_scope`; otherwise any
   `invalid_measurement` gives `invalid_measurement`; otherwise any `inconclusive` gives
   `inconclusive`; otherwise `within_control_distribution`. No session may be dropped or
   chosen after data. A session whose job wrote a non-zero `job-exit.json` is
   `invalid_measurement` under the frozen stopping rule and enters the combination as
   such.
4. **Analysis, with `analyze-v6.sh` unchanged** (sha256
   `23e614ad0b7c74ba53a4fcd51097a44194987617661343b2243b6dec6d0f422e`, also
   `protocol-v6.json` `analysis_plan.script.sha256`). The script takes exactly two
   sessions and decides each independently, with the fixed seeds 2026110661, 2026110662
   and 2026110663. It is run twice, with the ecbench binary extracted from the pinned
   image and checked against `73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`:
   - (i) `analyze-v6.sh ECBENCH reports/v6/analysis-a --counted <counted> <s1> <s2>`;
   - (ii) `analyze-v6.sh ECBENCH reports/v6/analysis-b --counted <counted> <s3> <s1>`.

   Here `<s1>`, `<s2>`, `<s3>` and `<counted>` are the `session/` directories of the
   jobs. Neither file's `overall` field is the verdict. The final verdict is the
   section 3 combination over three per-session verdicts: s1 and s2 from
   `analysis-a/verdict.json`, s3 from `analysis-b/verdict.json`.
   **s1 determinism check.** For the object of `.sessions[]` whose `dir` is the s1
   directory, the fields `verify_exit`, `verdict` and `gates` in `analysis-a` and in
   `analysis-b` must be equal (exact command in section 3). A mismatch is
   `invalid_measurement` for the whole of v6. `S1DIR` is the same absolute path in
   both runs, and the card records it. If run (ii) cannot produce `verdict.json`
   (for example, `analyze-v6.sh` exits 2 because s3 has no `records.jsonl`), s3 is
   `invalid_measurement`. The determinism check is then not applicable, because the
   combined verdict is already `invalid_measurement`. Whatever `analyze-v6.sh`
   reports, a session whose `job-exit.json` is non-zero is `invalid_measurement`;
   the card reads `job-exit.json`, and the script does not.
5. **Sealing.** No wall field, `ecbench compare` or `analyze-v6.sh` output of s1 or s2
   is opened until both of these are true: (a) this amendment is frozen and merged to
   `main`; (b) s3 (or its `-r2`) has written `job-exit.json`. Until then only
   `job-exit.json` and the node and Job/Pod identity fields are read. s1 and s2 records
   may not be copied into any report before then. Reading `job-exit.json` of s3 is
   permitted by (b) itself.
6. **Budget.** The rolling hard-bound gate of decision section 14.1 applies unchanged:
   before each launch L (s3, the counted job, any `-r2`), the card records
   `actual_spend + HB(L) + HB(every first-attempt job not yet launched)` and requires it
   to be at most USD 5.00, with `HB(job) = (job seconds / 3600) x 0.43108 + 100 x 0.005`
   USD (USD 0.93108 for a 3,600 s job). s3 is a first-attempt job and is reserved in the
   gate of every launch that precedes it. If a gate fails the director stops and returns
   to the coordinator before that launch.
7. **Scope.** The claim's hardware scope becomes "two c8a.2xlarge host epochs (three
   sessions)". The `PROTOCOL-V6.md` section 2 boundary is otherwise unchanged.

## 3. Exact checks

Host-epoch condition between H1 and H2 (read from `job-exit.json` of s1, s2 and s3):

```
jq -e --slurpfile a S1/job-exit.json --slurpfile b S2/job-exit.json 'select(.exit_code == 0 and .node_name != $a[0].node_name and .node_name != $b[0].node_name and .pod_uid != $a[0].pod_uid and .pod_uid != $b[0].pod_uid)' S3/job-exit.json
```

It prints the `job-exit.json` object and exits 0 when the condition holds, and prints
nothing and exits 4 when it does not. (H1 itself is `.node_name` equal in s1 and s2.)

s1 determinism check (`S1DIR` is the s1 `session/` path exactly as passed to the script):

```
jq -S -c --arg d "S1DIR" '.sessions[] | select(.dir == $d) | {verify_exit, verdict, gates}' reports/v6/analysis-a/verdict.json
jq -S -c --arg d "S1DIR" '.sessions[] | select(.dir == $d) | {verify_exit, verdict, gates}' reports/v6/analysis-b/verdict.json
```

Each prints one line; the two lines must be byte-equal.

Final verdict over the three per-session verdicts V1, V2, V3:

```
jq -n -r --argjson s '["V1","V2","V3"]' 'if all($s[]; . == "supported_within_scope") then "supported_within_scope" elif any($s[]; . == "invalid_measurement") then "invalid_measurement" elif any($s[]; . == "inconclusive") then "inconclusive" else "within_control_distribution" end'
```

If the host-epoch condition does not hold (section 2 item 2), a combined result of
`supported_within_scope` is replaced by `inconclusive`; any other combined result stands.

## 4. Pre-launch node check, observable form

The decision names the check `kubectl get node ip-10-84-67-216.us-east-2.compute.internal`
reporting NotFound. The agent has no kubectl binary, no kubeconfig and no route to the
cluster API, and the launcher's `isolab.eks.command-status/v1` request reads one Job's
state, not Nodes. The same fact is therefore read from the EC2 API with the executor's
AWS identity: no instance that is not `terminated` has private DNS name
`ip-10-84-67-216.us-east-2.compute.internal` or private address `10.84.67.216`.

```
aws ec2 describe-instances --filters Name=private-dns-name,Values=ip-10-84-67-216.us-east-2.compute.internal Name=instance-state-name,Values=pending,running,shutting-down,stopping,stopped --query 'length(Reservations[].Instances[])' --output text
aws ec2 describe-instances --filters Name=private-ip-address,Values=10.84.67.216 Name=instance-state-name,Values=pending,running,shutting-down,stopping,stopped --query 'length(Reservations[].Instances[])' --output text
```

Each must print `0`. The check is made at launch time; it does not prevent a later node
from receiving the same private address and so the same name, which the after-the-run
rule of section 2 item 2 catches.

## 5. Budget at filing

Gate arithmetic and the actual-spend basis live on the launch cards, not here, because
they depend on the moment of each launch. Section 2 item 6 fixes only the rule and the
`HB` formula.

## 6. Fields of `protocol-v6.json` superseded

Their text is not edited in the JSON (the freeze keeps them as filed); this file
supersedes them as follows.

| Field | Replaced by |
|---|---|
| `variable_ledger.factors[name=host_epoch].how` | "Two host epochs. H1 is the pair s1 and s2, which ran one after the other on node ip-10-84-67-216.us-east-2.compute.internal. H2 is the single job s3, on a node whose `node_name` in job-exit.json differs from H1's. All three sessions must pass every gate, and the Job, Pod and node identities of H2 must differ from those of H1." |
| `stopping_rules[1].rule` (scientific stage) | "Scientific stage, conditional only on the qualification: submit three wall jobs from launch-v6/request-session-1.json, request-session-2.json and request-session-3.json (spec-fp28-v6.json, byte-identical in all three apart from the job name and provenance label), run each once under the host-loss rule, and stop. s3 is submitted only after V6-AMENDMENT-1 is merged to main and the pre-launch node check of PROTOCOL-V6-AMENDMENT-1.md section 4 prints 0 twice." |
| `decision_rule.text` (combination) | Section 2 item 3 and section 3 of this file: each of s1, s2 and s3 must pass every per-session gate; the overall precedence runs over three sessions; the host-epoch condition decides support. The per-session gates are unchanged. |
| `claims[id=C-wall].text` | "...in each of three L2 sessions on two c8a.2xlarge host epochs." |
| `analysis_plan` invocations, `analysis_plan.inputs`, `analysis_plan.outputs` | Section 2 item 4: two runs of the unchanged `analyze-v6.sh`, `analysis-a` on (s1, s2) and `analysis-b` on (s3, s1), each with the counted session; three wall directories and one counted directory. |
| `execution.placement.concurrency_note` | "The wall requests s1 and s2 were submitted back to back and ran one after the other on one node. s3 is submitted alone, after this amendment is merged to main and after the node of H1 is absent from the cluster. The counted job may be submitted after s1 and s2 wrote job-exit.json and before s3." |
| `execution.placement.jobs.wall_sessions` | 3 |
| `execution.placement.route.requests`, `.job_names`, `.job_deadline_seconds` | each gains `session_3`: `research/ic_line_layout_tournament_20261031/launch-v6/request-session-3.json`, `ic-line-v6-s3-20261106`, 3600 |
| `sample_size.power.note` | "...three independent sessions (two host epochs) must each separately put the 95% full-wall interval below 1." |
| `hypotheses[H-layout].predictions[P-wall].minimum_effect` and `.provenance.decisive_scope` | "...in each of three L2 sessions on two c8a.2xlarge host epochs (s1 and s2 on H1, s3 on H2)" (decision section 15.5, point 8) |
| `hypotheses[H-layout].predictions[P-parity].provenance.decisive_scope` | all candidate/incumbent pairs in all three wall sessions |
| `controls.positive.expected_detection` | "...in each of the three wall sessions" |
| `execution.placement.route.budget_note` and `stopping_rules[kind=budget].rule` | the rolling hard-bound gate of decision section 14.1, as section 2 item 6 states; the fixed 6.5 node-hour worst case no longer applies |

The frozen host-loss rule, the invalidation rule, the budget ceiling of USD 5 and the
qualification stage are unchanged.
