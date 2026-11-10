#!/usr/bin/env bash
# Round 2: as make_requests_r2e.sh, regenerated after Amendment 12 (R2-E, price guard) was frozen. Differences from make_requests_r2e.sh, ONLY: output into requests_r2f/; no instance pin (PIN_ABOVE raised above every job); one appended provenance sentence.
#   - output into requests_r2e/ (requests_r2c/ and requests_r2d/ stay byte-identical);
#   - one sentence for Amendment 11 appended to provenance.note;
#   - one added request pfd-g3c (job name pfd-g3c-20261109): the stage C G3-only command job of AMENDMENT_R2_D_g3_scope.md section 4.4.
# Usage: make_requests_r2e.sh IMAGE_JSON [STAMP]
# (The header of make_requests_r2d.sh follows.)
# Round 2: as make_requests_r2c.sh (Amendment 9), regenerated after Amendment 10 (R2-A) was frozen. Output into requests_r2d/ so the launched requests_r2c/ files (pfd-a, pfd-c) stay byte-identical. Usage: make_requests_r2d.sh IMAGE_JSON [STAMP]
# A copy of make_requests_r2.sh (which runs make_requests.sh) with ONLY the Amendment 9 changes:
#   - job.cpu 14 -> 12 in every request (the largest whole-core value a 16-vCPU cpu-spot node can schedule);
#   - job.instance_type "r6i.4xlarge" on every request whose seconds > 90000 (owner's US$40 per-job hard cap);
#   - one sentence appended to provenance.note;
#   - output into requests_r2c/ (and, only with R2C_PENDING=1, requests_r2c_pending_amendment/), so requests/ and
#     requests_pending_amendment/ (bound by the R2-1 card) are never rewritten.
# Unchanged: image, command (so the image, entry.sh and entry_r2.sh are unchanged), memory, seconds, job names,
# retries, wait_seconds, isolation_profile, source_revision. protocol_sha256 is sha256sum of PREREGISTRATION.md as it
# stands when this runs (as in make_requests.sh): re-run after every amendment appended to it.
# The thread count follows from cpu: submit-job sets RAYON_NUM_THREADS=$cpu in the pod env, and entry.sh only
# defaults RAYON_NUM_THREADS when it is unset.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); repo=$(cd "$here/../.." && pwd)
img=${1:?image.json}; stamp=${2:-20261108}
image=$(jq -er .image_uri "$img"); rev=$(jq -er .source_revision "$img")
psha=$(sha256sum "$repo/research/prime_fall_degree_20261027/PREREGISTRATION.md" | cut -d' ' -f1)
CPU=12; PIN=r6i.4xlarge; PIN_ABOVE=999999999
r2c=' Amendment 9 (R2-C, AMENDMENT_R2_C_cpu12.md, frozen 2026-11-08): cpu 14->12 because a 14-vCPU pod cannot be scheduled on cpu-spot (14180m > 14000m allocatable); RAYON_NUM_THREADS=12 from the pod env; instance_type r6i.4xlarge where seconds > 90000 (US$40 per job); every wall budget and seconds unchanged. Amendment 10 (R2-A, AMENDMENT_R2_A_stageB_m6t6.md, frozen 2026-11-08): protocol hash after Amendment 10; no field other than provenance changes. Amendment 11 (R2-D, AMENDMENT_R2_D_g3_scope.md, frozen 2026-11-09): G3 applies at m <= 6 (F4 engine capacity); the stage C G3-only job pfd-g3c-20261109 runs before pfd-b and any pfd-x; one or more G3 disagreements stop further PKM submissions; protocol hash after Amendment 11. Amendment 12 (R2-E, AMENDMENT_R2_E_price_guard.md, frozen 2026-11-09): no instance pin; the US$40 per-job ceiling is enforced by the price guard (node type and Spot price read after start).'
out=$here/requests_r2f; mkdir -p "$out"; rm -f "$out"/*.json
req() { # name cpu memory seconds note command...   (make_requests.sh's req, writing to requests_r2c/)
  local name=$1 cpu=$2 mem=$3 secs=$4 note=$5; shift 5
  jq -n --arg name "$name-$stamp" --arg image "$image" --argjson cpu "$cpu" --arg mem "$mem" --argjson secs "$secs" \
    --arg rev "$rev" --arg psha "$psha" --arg note "$note$r2c" --arg pin "$PIN" --argjson above "$PIN_ABOVE" --args \
    '{schema:"isolab.eks.command-request/v1",
      job:({job_name:$name,image:$image,command:$ARGS.positional,cpu:$cpu,memory:$mem,seconds:$secs,
           isolation_profile:"standard",retries:0,wait_seconds:300} + (if $secs > $above then {instance_type:$pin} else {} end)),
      provenance:{harness:"claude-code-subagent",source_revision:$rev,study:"prime_fall_degree",
                  protocol_sha256:$psha,note:$note}}' -- "$@" > "$out/$name.json"
}
E=/usr/local/bin/entry.sh
# Registered cells (run.sh), unchanged; Amendment 2 chains. R2-C: 12 vCPU (6 whole cores) fits a 16-vCPU node; one job at a time.
req pfd-a $CPU 56Gi 172800 "stage A + F4 chain: g1 hky-A g3 d-A cap-A. seconds is a proposal (worst-case sum of per-system budgets ~3.8e5 s; the proposed 172800 s is below it)." $E g1 hky-A g3 d-A cap-A
req pfd-c $CPU 56Gi  86400 "stage C: g1 g4 hky-C g3. seconds is a proposal." $E g1 g4 hky-C g3
req pfd-b $CPU 56Gi 172800 "stage B: g1 hky-B d-B g3 (LAUNCH.md omitted B). m=3 N=21 / m=4 N=20 closures hit the program's own max_entries and read censored. seconds is a proposal." $E g1 hky-B d-B g3
# Amendment 7 A/B: one capped F4 run per job. DENSE = largest multiple of 1e8 with 4*C <= request - 8 GiB.
cap() { # t k c mem dense
  req "pfd-x-t$1-k$2-c$3" $CPU "$4" 90000 "Amendment 7 B bracket step: Kummer m=2 P1 t=$1 draw $2 cap $3, --budget 86400 --max-dense $5. Run only after G3 passed (pfd-a)." $E "cap:$1:$2:$3:$5"
}
for k in 0 1; do
  cap 12 $k 5 56Gi 12800000000;  cap 12 $k 6 56Gi 12800000000
  cap 13 $k 5 112Gi 27900000000; cap 13 $k 6 112Gi 27900000000
  cap 14 $k 6 112Gi 27900000000
done
# Amendment 8 (m=4): max_entries = floor((112Gi-8GiB)/8B) = 13958643712, max_cols 4e6, budget 172800.
m4() { req "pfd-m4-t$1-k$2-c$3" $CPU 112Gi 176400 "Amendment 8 m=4 closure: t=$1 draw $2 hky-cmax $3 --budget 172800. Run after G3. t=5 c=7 and t=6 are NOT generated: projected 80-510 GB exceeds the 128 GiB pool." $E "m4:$1:$2:$3:13958643712:4000000"; }
for k in 0 1; do m4 4 $k 7; m4 5 $k 6; done
# Amendment 11 (R2-D section 4.4, decision 6): the stage C G3-only job, frozen command, own stamp 20261109, no pin (57600 <= 90000).
G3C=$(cat <<'EOF_G3C'
set -u; : "${ISOLAB_RESULTS_URI:?}"; S=/work/g3c; mkdir -p "$S/runs" "$S/gates" || exit 2; SRC='s3://isolab-results-104186811093-us-east-2/runs/pfd-c-20261108/3eeef6a6-9565-415c-918c-b3f92801aafc//runs'; for f in C-t2-m2 C-t2-m3 C-t2-m4 C-t2-m5 C-t2-m6 C-t2-m2-p0 C-t2-m3-p0 C-t2-m4-p0 C-t2-m5-p0 C-t2-m6-p0 C-t3-m2 C-t3-m3 C-t3-m4 C-t3-m5 C-t3-m6 C-t3-m2-p0 C-t3-m3-p0 C-t3-m4-p0 C-t3-m5-p0; do aws s3 cp "$SRC/$f.jsonl" "$S/runs/$f.jsonl" --only-show-errors || exit 3; done; export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-12}; (cd "$S/runs" && sha256sum *.jsonl) > "$S/gates/g3-inputs.sha256"; sha256sum /usr/local/bin/pkm_tower_pilot /usr/local/bin/prime_fall_degree_tools > "$S/binary.sha256"; { echo "nproc: $(nproc)"; echo "rayon: $RAYON_NUM_THREADS"; grep -m1 'model name' /proc/cpuinfo; } > "$S/host.txt"; /usr/local/bin/prime_fall_degree_tools gate3 "$S" /usr/local/bin/pkm_tower_pilot --max-n 20 --jobs 2 --budget 900 > "$S/gates/g3.log" 2>&1; rc=$?; grep -q ', 0 disagreements' "$S/gates/g3.log" || { [ "$rc" = 0 ] && rc=1; }; echo "g3 exit $rc" > "$S/gates/g3.exit"; aws s3 cp "$S/binary.sha256" "$ISOLAB_RESULTS_URI/binary.sha256" --only-show-errors; aws s3 cp "$S/host.txt" "$ISOLAB_RESULTS_URI/host.txt" --only-show-errors; aws s3 sync "$S/gates" "$ISOLAB_RESULTS_URI/gates" --only-show-errors; exit $rc
EOF_G3C
)
stamp_main=$stamp; stamp=20261109
req pfd-g3c $CPU 56Gi 57600 "Amendment 11 (R2-D 4.4): stage C G3 only, over the 19 pfd-c-20261108 run files with m <= 6 (F4 engine capacity), frozen gate3 --max-n 20 --jobs 2 --budget 900. Runs before pfd-b and before any pfd-x. Pass: exit 0 and ', 0 disagreements' in gates/g3.log; one or more disagreements stop further PKM submissions (owner decides)." /bin/bash -c "$G3C"
stamp=$stamp_main
ls "$out" | wc -l
# make_requests_r2.sh step 2: the Amendment R2-B null cells, REQUIRES AMENDMENT FREEZE of R2-B (proposed Amendment 12).
# Not regenerated by default: requests_r2c_pending_amendment/ is left as written (cpu 12, old protocol hash, no pin) until
# R2-B is frozen. Then run with R2C_PENDING=1 and add the pin (176400 > 90000), as for the m4 requests.
[[ "${R2C_PENDING:-0}" == 1 ]] || exit 0
pout=$here/requests_r2c_pending_amendment; mkdir -p "$pout"; rm -f "$pout"/*.json
for k in 0 1; do for tc in 4:7 5:6; do t=${tc%:*}; c=${tc#*:}; name=pfd-m4n-t$t-k$k-c$c
  jq -n --arg name "$name-$stamp" --arg image "$image" --arg rev "$rev" --arg psha "$psha" --argjson cpu "$CPU" \
    --arg note "REQUIRES AMENDMENT FREEZE (AMENDMENT_R2_B_m4_null.md): shape-matched null, Kummer m=4 t=$t draw $k hky-cmax $c, --controls null --null-chain. Run after G3 (pfd-a) and after its Kummer counterpart pfd-m4-t$t-k$k-c$c. Do not launch before the amendment is frozen in PREREGISTRATION.md.$r2c" \
    --args '{schema:"isolab.eks.command-request/v1",
      job:{job_name:$name,image:$image,command:$ARGS.positional,cpu:$cpu,memory:"112Gi",seconds:176400,
           isolation_profile:"standard",retries:0,wait_seconds:300},
      provenance:{harness:"claude-code-subagent",source_revision:$rev,study:"prime_fall_degree",protocol_sha256:$psha,note:$note}}' \
    -- /usr/local/bin/entry_r2.sh "m4n:$t:$k:$c:13958643712:4000000" > "$pout/$name.json"
done; done
ls "$pout" | wc -l
