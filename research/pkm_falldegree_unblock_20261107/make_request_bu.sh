#!/usr/bin/env bash
# Amendment 15 (R2-G, AMENDMENT_R2_G_stageB_units.md), runner revision 3: one stage B unit job request,
# requests_r2i/pfd-bu-<N>.json.
# Usage: make_request_bu.sh IMAGE_JSON N STAMP [PREV | close:<ladder>:<T> | allow:<unit>:<K> | skip:<n>:<reason>]...
#   PREV    the result prefix of an earlier unit job, exactly as that job's ISOLAB_RESULTS_URI
#           (s3://isolab-results-104186811093-us-east-2/runs/pfd-bu-<n>-<stamp>/<uuid>/, with the final "/").
#   close:  / allow:  passed to the runner (its rules: close needs a lost unit, allow needs K >= 3; both are carried forward).
#   skip:<n>:<reason>  passed to the runner like the others (it carries and checks them) and counted here: job n (1 <= n < N) is deliberately not a PREV;
#           reason is costguard (stopped by the cost guard, rule 9) or neverran. Every job 1..N-1 must be either a PREV or
#           skipped: (number of PREVs) + (number of skips) == N - 1, all job numbers distinct.
# The lab's request validator (taskq/deploy/eks-karpenter/remote/run-command) refuses a command argument longer than 8192
# characters or containing a newline, so the frozen runner text cannot be one argument. The command is
#   /bin/bash -c "<BOOT>" stageb <PIECE1> ... <PIECEn> <arguments>
# The runner text, every newline written as the letter Q (the runner text must contain no Q; checked here), is cut into n pieces
# of at most 8000 characters. BOOT (one line, no newline, image tools only; n is a literal number in it) joins the first n
# arguments, turns Q back into newlines, writes the file with mktemp, checks its sha256 against the literal frozen hash
# (exit 17 with a message if it differs; a failing mktemp or write exits 18), shifts the pieces away and runs: exec /bin/bash <file> <arguments>.
# The runner itself takes the job number from ISOLAB_JOB_ID (pfd-bu-<N>-<STAMP>) and checks that the PREVs and skips are jobs 1..N-1.
# So the job runs the frozen runner byte for byte or does not run it.
# RUNNER_SHA below is a placeholder that the main session fills in at the freeze. For tests only, RUNNER_SHA_OVERRIDE (the hash)
# and OUT_DIR_OVERRIDE (the output directory) replace the placeholder and requests_r2i/; either one requires an output
# directory outside the repository. Refuses to overwrite an existing request. Prints the request path and the BOOT sha256.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd -P); repo=$(cd "$here/../.." && pwd -P)
[[ $# -ge 3 ]] || { echo "usage: make_request_bu.sh IMAGE_JSON N STAMP [PREV | close:<ladder>:<T> | allow:<unit>:<K> | skip:<n>:<reason>]..." >&2; exit 5; }
img=$1; n=$2; stamp=$3; shift 3
[[ $stamp =~ ^[0-9]+$ ]] || { echo "STAMP must be all digits (the job name is pfd-bu-<N>-<STAMP>)" >&2; exit 5; }
RUNNER_SHA=__RUNNER_SHA__
outdir=$here/requests_r2i
if [[ -n ${RUNNER_SHA_OVERRIDE:-} || -n ${OUT_DIR_OVERRIDE:-} ]]; then
  [[ -n ${OUT_DIR_OVERRIDE:-} ]] || { echo "RUNNER_SHA_OVERRIDE needs OUT_DIR_OVERRIDE (a directory outside the repository)" >&2; exit 8; }
  od=$(readlink -m "$OUT_DIR_OVERRIDE")     # resolved without creating anything
  case "$od/" in "$repo"/*) echo "the overrides are for test directories outside the repository, not $od" >&2; exit 8;; esac
  outdir=$od
  [[ -z ${RUNNER_SHA_OVERRIDE:-} ]] || RUNNER_SHA=$RUNNER_SHA_OVERRIDE
fi
[[ $RUNNER_SHA =~ ^[0-9a-f]{64}$ ]] || { echo "RUNNER_SHA is not a sha256 (the placeholder is filled in at the freeze)" >&2; exit 3; }
runner=$here/stageb_units_runner.sh
[[ $(sha256sum "$runner" | cut -d' ' -f1) == "$RUNNER_SHA" ]] || { echo "stageb_units_runner.sh differs from the frozen runner" >&2; exit 3; }
[[ $n =~ ^[1-9][0-9]*$ && $n -le 12 ]] || { echo "N must be 1..12 (rule 10: at most 12 unit jobs without going back to the owner)" >&2; exit 5; }
out=$outdir/pfd-bu-$n.json
[[ ! -e $out ]] || { echo "$out exists" >&2; exit 4; }

# arguments: runner arguments (PREV, close:, allow:) in order; skip: words are only checked and recorded
rargs=(); skips=(); nprev=0; declare -A seen
for a in "$@"; do
  case "$a" in
    skip:*)
      [[ $a =~ ^skip:([1-9][0-9]*):(costguard|neverran)$ ]] || { echo "bad argument $a: need skip:<n>:costguard or skip:<n>:neverran" >&2; exit 5; }
      k=${BASH_REMATCH[1]}
      [[ $k -lt $n && -z ${seen[$k]:-} ]] || { echo "bad argument $a: job number must be < N and not used twice" >&2; exit 5; }
      seen[$k]=1; skips+=("$a"); rargs+=("$a") ;;
    close:*) [[ $a =~ ^close:[A-Za-z0-9-]+:[0-9]+$ ]] || { echo "bad argument $a" >&2; exit 5; }; rargs+=("$a") ;;
    allow:*) [[ $a =~ ^allow:[A-Za-z0-9-]+:[0-9]+$ ]] || { echo "bad argument $a" >&2; exit 5; }; rargs+=("$a") ;;
    s3://*)
      [[ $a =~ ^s3://isolab-results-104186811093-us-east-2/runs/pfd-bu-([0-9]+)-[0-9]+/[^/]+/$ ]] || { echo "bad argument $a: not a unit job result prefix ending in /" >&2; exit 5; }
      k=$((10#${BASH_REMATCH[1]}))
      [[ $k -ge 1 && $k -lt $n && -z ${seen[$k]:-} ]] || { echo "bad argument $a: job number must be 1 <= number < N and not used twice" >&2; exit 5; }
      seen[$k]=1; nprev=$((nprev + 1)); rargs+=("$a") ;;
    *) echo "bad argument: $a" >&2; exit 5 ;;
  esac
done
[[ $((nprev + ${#skips[@]})) -eq $((n - 1)) ]] || { echo "need (PREVs) + (skips) == N - 1 = $((n - 1)); got $nprev + ${#skips[@]}: every earlier job is a PREV or is skipped with a reason" >&2; exit 5; }

image=$(jq -er .image_uri "$img"); rev=$(jq -er .source_revision "$img")
psha=$(sha256sum "$repo/research/prime_fall_degree_20261027/PREREGISTRATION.md" | cut -d' ' -f1)
note="Amendment 15 (R2-G) stage B unit job $n: runs G1, then the stage B units not done by the earlier jobs named in the command, in run.sh order, one pilot invocation per (ladder, t); runner revision 3 sha256 $RUNNER_SHA, started by a command that checks that hash (exit 17 if it differs); earlier jobs and arguments: ${rargs[*]:-none}; jobs not given as PREV (passed to the runner as skip: arguments): ${skips[*]:-none}. 12 vCPU, 56Gi, no pin; seconds 172800 > 90000, so the Amendment 12 price guard applies. Owner decision 8 (2026-11-10)."
[[ ${#note} -le 2048 ]] || { echo "provenance note is ${#note} characters, over the validator's 2048" >&2; exit 5; }

# the runner text, newlines as Q, in pieces of at most 8000 characters
if grep -q Q "$runner"; then echo "the runner text contains the letter Q; the newline encoding needs another letter" >&2; exit 7; fi
if LC_ALL=C grep -q '[^[:print:][:space:]]' "$runner"; then echo "the runner text is not plain ASCII" >&2; exit 7; fi
enc=$(tr '\n' 'Q' < "$runner")
pieces=(); i=0
while [[ $i -lt ${#enc} ]]; do pieces+=("${enc:i:8000}"); i=$((i + 8000)); done
np=${#pieces[@]}
[[ $np -ge 1 && $np -le 9 ]] || { echo "runner needs $np pieces" >&2; exit 7; }
boot="set -u; r=\$(mktemp) || exit 18; printf '%s' \"\${@:1:$np}\" | tr Q '\\n' > \"\$r\" || exit 18; [ \"\$(sha256sum < \"\$r\" | cut -d' ' -f1)\" = $RUNNER_SHA ] || { echo 'runner text differs from its frozen SHA-256' >&2; exit 17; }; shift $np; exec /bin/bash \"\$r\" \"\$@\""
mkdir -p "$outdir"
jq -n --arg name "pfd-bu-$n-$stamp" --arg image "$image" --arg rev "$rev" --arg psha "$psha" --arg note "$note" \
  --arg boot "$boot" --args \
  '{schema:"isolab.eks.command-request/v1",
    job:{job_name:$name,image:$image,command:(["/bin/bash","-c",$boot,"stageb"] + $ARGS.positional),cpu:12,memory:"56Gi",seconds:172800,
         isolation_profile:"standard",retries:0,wait_seconds:300},
    provenance:{harness:"claude-code-subagent",source_revision:$rev,study:"prime_fall_degree",protocol_sha256:$psha,note:$note}}' \
  -- "${pieces[@]}" ${rargs[@]+"${rargs[@]}"} > "$out"
# the request must reproduce the frozen runner: decode the pieces as BOOT does and compare
got=$(jq -j ".job.command[4:$((4 + np))] | add" "$out" | tr Q '\n' | sha256sum | cut -d' ' -f1)
[[ $got == "$RUNNER_SHA" ]] || { echo "request does not reproduce the runner" >&2; rm -f "$out"; exit 6; }
[[ $(jq -r '.job.command[2]' "$out") == *"shift $np;"* ]] || { echo "BOOT does not name its $np pieces" >&2; rm -f "$out"; exit 6; }
"$here/validate_command_request.sh" "$out" || { echo "request is INVALID" >&2; rm -f "$out"; exit 9; }
echo "$out"
echo "boot_sha256: $(jq -j '.job.command[2]' "$out" | sha256sum | cut -d' ' -f1)"
