#!/usr/bin/env bash
# Infrastructure packaging only: a terminal snapshot, never a completed-study claim.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_archive=${1:?new archive output path}
test ! -e "$joint_archive"
test ! -e "$joint_archive.part"
joint_stage=$(mktemp -d /var/tmp/twist-joint-raw-manifest.XXXXXX)
trap 'rm -rf "$joint_stage"' EXIT
cat > "$joint_stage/terminal-runs.txt" <<'EOF'
algebra-v1
cloud-capability-v1
coordinate-lean-v4
external-audit-v1
external-audit-v2
external-regression-v1
lean-gate-v2
lean-gate-v3
pure-zero-certificate-v1
quartic-factored-lean-v5
quartic-factored-lean-v6
quartic-lean-v4
regression-v1
regression-v2
uniform-lean-v1
uniform-lean-v2
uniform-lean-v3
uniform-zero-v1
variant-regression-v1
verification-v6
verification-v8
zero-triangle-lean-v1
EOF
while IFS= read -r joint_id; do test -d "$joint_study/runs/$joint_id"; done < "$joint_stage/terminal-runs.txt"
python3 - "$joint_stage" <<'PY'
import datetime,json,pathlib,subprocess,sys
p=pathlib.Path(sys.argv[1]); terminal=(p/'terminal-runs.txt').read_text().splitlines()
v={'schema':'joint-image-terminal-raw-checkpoint/v1','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'included_terminal_or_reconciled_interrupted_runs':terminal,'excluded_managed_runs':['external-audit-v3','external-regression-v2','pure-zero-lean-v1','pure-zero-verify-v1','uniform-zero-v2','variant-population-v1'],'scope':'Immutable selected-terminal raw checkpoint. Six other managed lanes are omitted regardless of their state when packaging starts; this is not a completed-study archive or a complete root/cost claim. Later canonical archive updates must preserve this digest as a superseded version.'}
(p/'raw-checkpoint-manifest.json').write_text(json.dumps(v,indent=2)+'\n')
PY
cp "$joint_stage/raw-checkpoint-manifest.json" "$joint_archive.manifest.json"
sed 's|^|runs/|' "$joint_stage/terminal-runs.txt" > "$joint_stage/tar-inputs.txt"
# All included directories have reconciled terminal writers. Active run trees
# are absent from this list and are never read as a complete snapshot.
tar --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner \
  --mode='u+rwX,go+rX,go-w' --format=gnu -C "$joint_stage" -cf - raw-checkpoint-manifest.json \
  -C "$joint_study" -T "$joint_stage/tar-inputs.txt" | xz -6 -T1 > "$joint_archive.part"
mv "$joint_archive.part" "$joint_archive"
sha256sum "$joint_archive" > "$joint_archive.sha256"
stat --printf='%s\n' "$joint_archive" > "$joint_archive.bytes"
