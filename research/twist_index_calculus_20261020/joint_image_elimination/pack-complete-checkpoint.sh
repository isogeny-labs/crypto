#!/usr/bin/env bash
# Deterministic infrastructure checkpoint of all 43 reconciled raw runs.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_archive=${1:?new archive output path on durable workspace volume}
test ! -e "$joint_archive"
test ! -e "$joint_archive.part"
joint_stage=$(mktemp -d /home/ec2-user/scratch/twist-joint-manifest-v2.XXXXXX)
trap 'rm -rf "$joint_stage"' EXIT
for joint_owner in external-regression-v2 external-audit-v3 external-audit-completion-v2 uniform-zero-v2 five-zero-v1 five-point-replay-v1 fp-pruning-audit-v1 zero-identity-audit-v1 publication-gate-v1 historical-s5-match-v3 variant-population-v1 variant-validation-v1; do
  if systemctl is-active --quiet "twist-joint-$joint_owner.service"; then
    printf 'Original owner still active: %s\n' "$joint_owner" >&2
    exit 2
  fi
done
cat > "$joint_stage/terminal-runs.txt" <<'EOF'
algebra-v1
cloud-capability-v1
coordinate-lean-v4
external-audit-completion-v1
external-audit-completion-v2
external-audit-v1
external-audit-v2
external-audit-v3
external-regression-v1
external-regression-v2
five-point-replay-v1
five-zero-v1
fp-pruning-audit-v1
historical-s5-match-v1
historical-s5-match-v2
historical-s5-match-v3
lean-gate-v2
lean-gate-v3
publication-gate-v1
pure-zero-certificate-v1
pure-zero-lean-v1
pure-zero-square-v1
pure-zero-verify-v1
quartic-factored-lean-v5
quartic-factored-lean-v6
quartic-lean-v4
regression-v1
regression-v2
square-bridge-lean-v1
square-ideal-lean-v1
square-zero-verify-v1
uniform-lean-v1
uniform-lean-v2
uniform-lean-v3
uniform-zero-v1
uniform-zero-v2
variant-population-v1
variant-regression-v1
variant-validation-v1
verification-v6
verification-v8
zero-identity-audit-v1
zero-triangle-lean-v1
EOF
python3 - "$joint_stage" "$joint_study" <<'PY'
# Artifact metadata only; no scientific result is generated or aggregated.
import datetime,json,pathlib,subprocess,sys
stage,study=map(pathlib.Path,sys.argv[1:])
selected=(stage/'terminal-runs.txt').read_text().splitlines()
assert len(selected)==43 and len(set(selected))==43
assert sorted(selected)==sorted(p.name for p in (study/'runs').iterdir() if p.is_dir())
v={'schema':'joint-image-terminal-raw-checkpoint/v2','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'included_terminal_or_reconciled_interrupted_runs':selected,'scope':'All 43 explicitly reconciled run directories in the finite joint-image round, including every failed, interrupted and duplicate-retry receipt. This is not completion of the broader IC research goal or a new complete-cost/rho claim.','supersedes_checkpoint_sha256':'f4ddf5ce185d0866ff02e7a4e4085065e59756cf86798aaff85f7a8398eaa39c','preservation':'The prior immutable object and canonical index superseded digest must be retained.'}
(stage/'raw-checkpoint-manifest.json').write_text(json.dumps(v,indent=2)+'\n')
PY
cp "$joint_stage/raw-checkpoint-manifest.json" "$joint_archive.manifest.json"
sed 's|^|runs/|' "$joint_stage/terminal-runs.txt" > "$joint_stage/tar-inputs.txt"
tar --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner \
  --mode='u+rwX,go+rX,go-w' --format=gnu -cf - -C "$joint_stage" raw-checkpoint-manifest.json \
  -C "$joint_study" -T "$joint_stage/tar-inputs.txt" | xz -6 -T1 > "$joint_archive.part"
mv "$joint_archive.part" "$joint_archive"
sha256sum "$joint_archive" > "$joint_archive.sha256"
stat --printf='%s\n' "$joint_archive" > "$joint_archive.bytes"
