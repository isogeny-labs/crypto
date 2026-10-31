#!/usr/bin/env bash
# Preserve and verify the failed frozen qualification; never admit its timing.
set -euo pipefail
cd "$(dirname "$0")"
jq -e '.phase=="failed" and .failure=="fresh benchmark epoch failed; measurement is not retried" and .attempt==1 and .restores==1 and (.generations|length)==2 and all(.generations[];.encrypted and .snapshot_ready and .readback_verified) and .request.deadline_unix_seconds==1792650847' final-state.json >/dev/null
jq -e '.native_replay and (.timing_valid==false) and .supervisor.generation==2 and .supervisor.attempt==1' computation-result.json >/dev/null
sha256sum -c computation.sha256 > computation-hash-check.txt
for stage in computation/control/attempt-1 fresh; do
  jq -e '.ok and .records==3606 and .verified_records==3606 and (.replays|length)==3600 and all(.replays[];.reproduced) and .problems==[]' "$stage/audit.json" >/dev/null
done
cmp -n "$(stat -c%s g2-prefix-records.jsonl)" g2-prefix-records.jsonl computation/compute/session/records.jsonl
wc -cl g2-prefix-records.jsonl computation/compute/session/records.jsonl > prefix-preserved.txt
(cd fresh && sha256sum -c SHA256SUMS) > fresh-sha256.txt
test "$(wc -l < fresh-sha256.txt)" -eq 18
jq -e '.accepted==false and .required=="L2" and .executions==3606 and .invalid_outcomes==[] and .grades==[{"level":"L1","count":3},{"level":"L2","count":3603}] and [.below_l2[].seq]==[676,1406,1826]' fresh/l2-gate.json >/dev/null
jq -e '.exit_code==42 and .pod_uid=="fb1260f3-d86c-44c7-ae62-c5506963d3cb"' fresh/job-exit.json >/dev/null
test ! -e fresh/control-comparison.json
jq -e '.stable.logical_cpus==8 and .stable.physical_cores==8 and all(.stable.topology.cpus[];(.siblings|length)==1)' fresh/session/host.json >/dev/null
jq -e --slurpfile p protocol.json '.source_revision==$p[0].request.build.source_revision and .ecbench_sha256==$p[0].request.build.ecbench_sha256' fresh/build-info.json >/dev/null
for generation in 1 2; do
  jq -r --argjson n "$generation" '.generations[]|select(.number==$n)|"\(.archive_sha256)  g\(.number)-private-checkpoint.tar"' final-state.json | sha256sum -c -
done > checkpoint-readback-hashes.txt
jq -s '{schema:"isolab.periodic.qualification-outcome/v1",run:.[0].request.run_name,outcome:"failed",failure:.[0].failure,source:.[0].request.build.source_revision,runtime_image:.[0].request.runtime_image_uri,restores:.[0].restores,generations:.[0].generations,computation:{native_replay:.[1].native_replay,timing_valid:.[1].timing_valid,records:.[2].records,replays:(.[2].replays|length),prefix_records:2486,prefix_bytes:11113922},fresh:{accepted:.[3].accepted,records:.[3].executions,grades:.[3].grades,rejected_sequences:[.[3].below_l2[].seq],native_replay:.[4].ok,replays:(.[4].replays|length),aa:"not evaluated: L2 gate exited first",payload_hashes_verified:18,exit_code:42,retries:0,job:.[0].benchmark_job,job_uid:.[0].benchmark_uid,pod_uid:"fb1260f3-d86c-44c7-ae62-c5506963d3cb",results_uri:"s3://isolab-results-104186811093-us-east-2/runs/ecbench-periodic-20261022-b-epoch-1/fb1260f3-d86c-44c7-ae62-c5506963d3cb/"}}' final-state.json computation-result.json computation/control/attempt-1/audit.json fresh/l2-gate.json fresh/audit.json > verification.json
printf 'Verified preserved recovery evidence and terminal L2 rejection; qualification remains failed.\n'
