#!/usr/bin/env bash
set -euo pipefail
set -o noclobber
cd /home/ec2-user/crypto-n83-resume
k=$1
case "$k" in
  2048) arm=c3; worker_seconds=8957; command_seconds=10157 ;;
  4096) arm=c1; worker_seconds=9000; command_seconds=10200 ;;
  *) exit 2 ;;
esac
work=/home/ec2-user/scratch/n83-arena-reclamation-20261022
old=/home/ec2-user/scratch/n83-natural-1m-20261020
proof=research/koblitz_n83_factor_base_sweep_20261008/verification/native-arena-reclamation-20261023
runner="$work/bin/runner-arena-continuation-v5"
worker="$work/bin/native-arena-admission-v3"
recipe="$work/compile-launch-corrected-continuation-card.sh"
prior_budget="$PWD/$proof/admission-correction/budget.json"
budget="$PWD/$proof/launch-correction/budget.json"
pressure_gate() {
  for resource in cpu io; do
    awk -v resource="$resource" '
      $1=="some" {
        split($2, value, "=");
        printf "%s PSI some avg10=%s (required<=5.00)\n", resource, value[2];
        if (value[2]+0 > 5) exit 1;
        found=1
      }
      END { if (!found) exit 1 }
    ' "/proc/pressure/$resource" || { printf 'specification_error: %s PSI readiness gate failed\n' "$resource" >&2; return 1; }
  done
}
pressure_gate
test -z "$(git status --porcelain)"
rg -qx PASS_LAUNCH_CORRECTION_NATIVE_TESTS_AND_BUILDS "$work/launch-correction-validation/status.txt"
sha256sum -c "$work/launch-correction-validation/binaries.sha256"
jq -e '.status=="PASS_STARTUP_BARRIER_SCOPE_SYNC_REAPPLY_AND_CLEANUP" and .resource_reapply_keeps_swap_zero==true and .worker_held_before_release==true' "$work/launch-correction-barrier-smoke/result.json" > /dev/null
jq -e '.status=="PASS_CHARGE_REFUSAL_WITHIN_ORIGINAL_ARM" and .charged_worker_seconds==13 and .corrective_worker_ceiling_seconds==8987 and .charged_plus_corrective_seconds==9000 and .new_allowance_seconds==0' "$prior_budget" > /dev/null
jq -e '.status=="PASS_CHARGE_LAUNCH_REFUSAL_WITHIN_ORIGINAL_ARM" and .prior_charged_worker_seconds==13 and .additional_charged_worker_seconds==30 and .cumulative_charged_worker_seconds==43 and .corrective_worker_ceiling_seconds==8957 and .charged_plus_corrective_seconds==9000 and .new_allowance_seconds==0' "$budget" > /dev/null
jq -e '.outcome=="stopped" and .relay_verbatim==true and .execution_conforms_to_frozen_argv==true and .recommendation=="respecify"' "$work/director-k2048-c1-assessment-20261023/assessment.json" > /dev/null
jq -e '.outcome=="impeded" and .relay_verbatim==true and .execution_conforms_to_frozen_argv==true and .whole_command_seconds==29.838953191 and .admitted_as_search_result==false' "$work/director-k2048-c2-assessment-20261023/assessment.json" > /dev/null
for base in 2048 4096; do
  jq -e '.status=="PASS_OWNERSHIP_COUNTERS_AND_RETAINED_SNAPSHOT_SIGNATURES" and .matched_conflict_snapshots==1001 and .reclamation_required==true' "$work/assessment/k$base-r1/reclamation.json" > /dev/null
  jq -e '.status=="PASS_CONTAINER_MEMORY_PROFILE_CUSTODY_AND_COUNTERS"' "$work/assessment/k$base-r1/memory.json" > /dev/null
  jq -e '.outcome=="executed_as_specified" and .relay_verbatim==true and .specification_errors==[]' "$work/director-k$base-r1-assessment-20261023/assessment.json" > /dev/null
done
rg -qx PASS_K2048_PUT_HEAD_FRESH_GET_PAYLOAD_NATIVE_AUDIT_MEMORY_RECLAMATION_AND_REPACK "$work/k2048-r1-publication-v3/status.txt"
rg -qx PASS_K4096_PUT_HEAD_FRESH_GET_PAYLOAD_NATIVE_AUDIT_MEMORY_RECLAMATION_AND_REPACK "$work/k4096-r1-publication/status.txt"
head=$(git rev-parse HEAD)
protocol_sha=$(sha256sum "$proof/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
oldspec="$old/specs-v20/k$k.json"
panel=$(jq -r .panel "$oldspec"); fixtures=$(jq -r .fixtures "$oldspec"); image=$(jq -r .image "$oldspec")
mkdir -p "$work/specs" "$work/cards" "$proof/exec"
spec="$work/specs/k$k-$arm.json"
freeze="$work/specs/k$k-$arm.freeze.json"
card="$work/cards/k$k-$arm.json"
prompt="$work/cards/k$k-$arm.prompt.txt"
for path in "$spec" "$freeze" "$card" "$prompt"; do test ! -e "$path"; done
"$runner" freeze "$PWD" "$panel" "$fixtures" "$worker" "$image" "$k" 1500000 "$worker_seconds" "$protocol_sha" "$freeze"
jq '.isolate=true | .cpu_reservation=[14,30] | .memory_limit_bytes=8589934592 | .systemd_swap_sync=true | .lock_wait_seconds=600' "$freeze" > "$spec"
binds='[]'
oldworker=$(jq -r .worker "$oldspec")
while IFS= read -r path; do
  path=${path/"$old/bin/runner-v20"/"$runner"}
  path=${path/"$old/specs-v20/k$k.json"/"$spec"}
  path=${path/"$oldworker"/"$worker"}
  digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$old/cards/k$k-v20.json")
extra=(
  "$PWD/$proof/PROTOCOL.md" "$PWD/$proof/CONTINUATION.md" "$PWD/$proof/RESPEC-C2.md" "$PWD/$proof/RESPEC-C3.md" "$prior_budget" "$budget" "$recipe"
  "$work/launch-correction-validation/status.txt" "$work/launch-correction-validation/binaries.sha256"
  "$work/launch-correction-barrier-smoke/result.json"
  "$work/director-k2048-c1-assessment-20261023/assessment.json"
  "$work/director-k2048-c2-assessment-20261023/assessment.json"
  "$PWD/tools/n83-sweep-runner/src/storage_summary.rs"
  "$PWD/research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-1m-20261020/exec/k$k-v20/stderr.log"
)
for base in 2048 4096; do
  extra+=("$work/assessment/k$base-r1/reclamation.json" "$work/assessment/k$base-r1/memory.json" "$work/director-k$base-r1-assessment-20261023/assessment.json")
done
for path in "${extra[@]}"; do
  digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
jq --arg h "$head" --argjson b "$binds" --arg runner "$runner" --arg spec "$spec" --arg arm "$arm" --argjson seconds "$command_seconds" '
  walk(if type=="string" then gsub("native-natural-1m-20261020";"native-arena-reclamation-20261023") | gsub("native-natural-1m";"native-arena-reclamation-continuation") | gsub("-v20";"-"+$arm) else . end)
  | .commit=$h | .binds=$b | .commands[0].argv[0]=$runner | .commands[0].argv[2]=$spec
  | .commands[0].timeout_seconds=$seconds
  | .commands[0].outputs += [(.commands[0].argv[-1]+"/lock-wait.json")]
  | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$old/cards/k$k-v20.json" > "$card"
native_path=$(jq -r .native_path "$card")
git check-ignore "$proof/exec/k$k-$arm.relay.json" "$proof/exec/k$k-$arm.receipt.json"
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$prompt"
test -z "$(git status --porcelain)"
pressure_gate
sha256sum "$card" "$prompt"
