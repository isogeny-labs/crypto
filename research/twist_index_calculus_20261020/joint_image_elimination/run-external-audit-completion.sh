#!/usr/bin/env bash
# Reconcile all terminal external outputs using the already frozen v3 verifier.
# Reuse earlier audit receipts; never repeat msolve or a relation oracle.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_out=${1:?new output directory}
joint_external="$joint_study/runs/external-regression-v2"
joint_prior="$joint_study/runs/external-audit-v3"
test ! -e "$joint_out"
test "$(cat "$joint_prior/metadata/compile-runner.exit")" = 0
sha256sum -c "$joint_prior/metadata/runner.sha256"
mkdir -p "$joint_out/metadata" "$joint_out/audits" "$joint_out/inventory"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
cp "$joint_prior/metadata/runner.sha256" "$joint_out/metadata/reused-runner.sha256"
git rev-parse HEAD > "$joint_out/metadata/orchestration-commit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
# Wait outside the heavy-work reservation for both existing owners.
while systemctl is-active --quiet twist-joint-external-regression-v2.service || systemctl is-active --quiet twist-joint-external-audit-v3.service; do sleep 20; done
systemctl show twist-joint-external-regression-v2.service twist-joint-external-audit-v3.service --property=Id,ActiveState,ExecMainStatus > "$joint_out/metadata/original-owners-terminal.txt"
joint_public="$joint_study/runs/algebra-v1/prepared/public/p37-seed0.json"
joint_public_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
export RAYON_NUM_THREADS=1
ulimit -c 0
for joint_arm in l0-peel phi-l0-active double-l0-active; do
  printf '%s-global-finite-domain\n%s-global-geometric\n' "$joint_arm" "$joint_arm"
  jq -r --arg arm "$joint_arm" '.blocks[].id | $arm+"-"+.' "$joint_study/runs/regression-v1/orbit-inputs/$joint_arm/manifest.json"
  printf '%s-variant-system\n%s-variant-geometric\n' "$joint_arm" "$joint_arm"
done > "$joint_out/metadata/expected-cases.txt"
while IFS= read -r joint_id; do
  joint_case="$joint_external/systems/$joint_id"
  joint_status=missing-terminal-receipt
  joint_rc=null
  if test -f "$joint_case/exit"; then
    joint_rc=$(cat "$joint_case/exit")
    joint_status=external-process-incomplete-or-failed
    if test "$joint_rc" = 0 && test -s "$joint_case/basis.txt"; then
      joint_basis_sha=$(sha256sum "$joint_case/basis.txt" | cut -d ' ' -f 1)
      read -r joint_system_sha joint_system < "$joint_case/inputs.sha256"
      joint_prior_summary="$joint_prior/audits/$joint_id/summary.json"
      if test -f "$joint_prior_summary"; then
        # Bind reused physical-set evidence to the same exact basis and input.
        jq -e --arg basis "$joint_basis_sha" --arg input "$joint_system_sha" '.basis_sha256==$basis and .system_sha256==$input' "$joint_prior_summary" > /dev/null
        cp "$joint_prior_summary" "$joint_out/inventory/$joint_id.native-summary.json"
        joint_status=reused-native-audit
      else
        case "$joint_id" in
          double-l0-active-*) joint_ref=/home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first/p37-seed0/double-l0-active.json ;;
          phi-l0-active-*) joint_ref=/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first/p37-seed0/phi-l0-active.json ;;
          l0-peel-*) joint_ref=/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first/p37-seed0/l0-peel.json ;;
          *) exit 3 ;;
        esac
        joint_command=(prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_prior/runner" --joint-external-audit "$joint_public" "$joint_public_sha" "$joint_system" "$joint_system_sha" "$joint_case/basis.txt" "$joint_basis_sha" "$joint_ref" "$joint_out/audits/$joint_id")
        printf '%q ' "${joint_command[@]}" > "$joint_out/metadata/$joint_id.command.txt"
        set +e
        python3 "$joint_repo/tools/isolated_bench.py" busy -- "${joint_command[@]}" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
        joint_audit_rc=$?
        set -e
        printf '%s\n' "$joint_audit_rc" > "$joint_out/metadata/$joint_id.exit"
        joint_status=new-native-audit-incomplete-or-failed
        if test -f "$joint_out/audits/$joint_id/summary.json"; then
          cp "$joint_out/audits/$joint_id/summary.json" "$joint_out/inventory/$joint_id.native-summary.json"
          joint_status=new-native-audit-terminal
        fi
      fi
    elif test "$joint_rc" = 0; then
      joint_status=external-exit-zero-without-basis
    fi
  fi
  joint_saved_status=absent
  if test -f "$joint_case/status.txt"; then joint_saved_status=$(cat "$joint_case/status.txt"); fi
  jq -n --arg id "$joint_id" --arg status "$joint_status" --arg original_status "$joint_saved_status" --argjson process_exit "$joint_rc" '{id:$id,reconciliation:$status,external_process_exit:$process_exit,original_status:$original_status}' > "$joint_out/inventory/$joint_id.owner.json"
done < "$joint_out/metadata/expected-cases.txt"
# Workflow inventory only. Mathematical yields/ranks stay in native summaries.
jq -s '{schema:"external-audit-terminal-inventory/v1",status:"terminal-inventory",cases:.,scope:"Owner/output reconciliation only; failures and absent bases are not zero physical yield. Read each native summary for mathematical checks. No timing comparison."}' "$joint_out"/inventory/*.owner.json > "$joint_out/summary.json"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'External terminal inventory reconciled. No msolve or group oracle was repeated.\n' > "$joint_out/complete.txt"
