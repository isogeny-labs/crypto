#!/usr/bin/env bash
# Amendment 15 (R2-G, AMENDMENT_R2_G_stageB_units.md): one stage B unit job request, requests_r2i/pfd-bu-<N>.json.
# Usage: make_request_bu.sh IMAGE_JSON N STAMP [PREV|close:<ladder>:<T>]...
#   PREV is the result prefix of an earlier unit job, exactly as that job's ISOLAB_RESULTS_URI
#   (s3://isolab-results-104186811093-us-east-2/runs/pfd-bu-<n>-<stamp>/<uuid>/, with the final "/").
#   A job stopped by the cost guard is never given as PREV (rule 9).
# The lab's request validator (taskq/deploy/eks-karpenter/remote/run-command) refuses a command argument longer than 8192
# characters or containing a newline, so the frozen runner text cannot be one argument. The command is therefore
#   /bin/bash -c "<BOOT>" stageb <PIECE1> <PIECE2> <arguments>
# where PIECE1 PIECE2 are the runner text with every newline written as the letter Q (the runner text contains no Q; checked
# here) and cut in two, and BOOT (one line, below) joins them, turns Q back into newlines, checks the file's SHA-256 against the
# value frozen in Amendment 15 (exit 17 if it differs) and then runs: /bin/bash <file> <arguments>. So the job runs the frozen
# runner byte for byte or does not run it. Refuses to overwrite an existing request.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); repo=$(cd "$here/../.." && pwd)
img=${1:?image.json}; n=${2:?job number}; stamp=${3:?stamp}; shift 3
RUNNER_SHA=__RUNNER_SHA__
runner=$here/stageb_units_runner.sh
[[ $(sha256sum "$runner" | cut -d' ' -f1) == "$RUNNER_SHA" ]] || { echo "stageb_units_runner.sh differs from the frozen runner" >&2; exit 3; }
[[ $n =~ ^[1-9][0-9]*$ && $n -le 12 ]] || { echo "N must be 1..12 (rule 10: at most 12 unit jobs without going back to the owner)" >&2; exit 5; }
out=$here/requests_r2i/pfd-bu-$n.json
[[ ! -e $out ]] || { echo "$out exists" >&2; exit 4; }
for a in "$@"; do
  case "$a" in
    s3://isolab-results-104186811093-us-east-2/runs/pfd-bu-*-*/*/) ;;
    close:*:*) ;;
    *) echo "bad argument: $a" >&2; exit 5 ;;
  esac
done
image=$(jq -er .image_uri "$img"); rev=$(jq -er .source_revision "$img")
psha=$(sha256sum "$repo/research/prime_fall_degree_20261027/PREREGISTRATION.md" | cut -d' ' -f1)
note="Amendment 15 (R2-G) stage B unit job $n: runs G1, then the stage B units not done by the earlier jobs named in the command, in run.sh order, one pilot invocation per (ladder, t); runner sha256 $RUNNER_SHA; earlier jobs and close rules: ${*:-none}. 12 vCPU, 56Gi, no pin; seconds 172800 > 90000, so the Amendment 12 price guard applies. Owner decision 8 (2026-11-10)."
if grep -q Q "$runner"; then echo "the runner text contains the letter Q; the newline encoding needs another letter" >&2; exit 7; fi
enc=$(tr '\n' 'Q' < "$runner"); half=$(( (${#enc} + 1) / 2 )); p1=${enc:0:half}; p2=${enc:half}
[[ ${#p1} -ge 1 && ${#p1} -le 8192 && ${#p2} -ge 1 && ${#p2} -le 8192 ]] || { echo "runner too long for two arguments (${#enc} characters)" >&2; exit 7; }
boot="set -u; r=\$(mktemp) || exit 2; printf '%s' \"\$1\$2\" | tr Q '\\n' > \"\$r\" || exit 2; [ \"\$(sha256sum < \"\$r\" | cut -d' ' -f1)\" = $RUNNER_SHA ] || { echo 'runner text differs from its frozen SHA-256' >&2; exit 17; }; shift 2; exec /bin/bash \"\$r\" \"\$@\""
jq -n --arg name "pfd-bu-$n-$stamp" --arg image "$image" --arg rev "$rev" --arg psha "$psha" --arg note "$note" \
  --arg boot "$boot" --arg p1 "$p1" --arg p2 "$p2" --args \
  '{schema:"isolab.eks.command-request/v1",
    job:{job_name:$name,image:$image,command:(["/bin/bash","-c",$boot,"stageb",$p1,$p2] + $ARGS.positional),cpu:12,memory:"56Gi",seconds:172800,
         isolation_profile:"standard",retries:0,wait_seconds:300},
    provenance:{harness:"claude-code-subagent",source_revision:$rev,study:"prime_fall_degree",protocol_sha256:$psha,note:$note}}' \
  -- "$@" > "$out"
# the request must reproduce the frozen runner: decode the two pieces as BOOT does and compare
[[ $(jq -j '.job.command[4] + .job.command[5]' "$out" | tr Q '\n' | sha256sum | cut -d' ' -f1) == "$RUNNER_SHA" ]] || { echo "request does not reproduce the runner" >&2; rm -f "$out"; exit 6; }
echo "$out"
