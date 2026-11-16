#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-1m-20261020
study=research/koblitz_n83_factor_base_sweep_20261008
proof="$study/verification/native-natural-1m-20261020"
pub="$work/publication"
mkdir "$pub"
cp -a "$work/checkpoint-v20-20261021/raw-bundle" "$pub/runs"
raw="$pub/runs"
mv "$raw/README.md" "$raw/PRE-WORKER-CHECKPOINT.md"
rm "$raw/SHA256SUMS"
for id in k2048-v20 k4096-v20; do
 cp -a "$proof/exec/$id" "$raw/runs/"
 for ext in stdout stderr receipt.json relay.json; do cp "$proof/exec/$id.$ext" "$raw/runs/"; done
 cp "$work/cards/$id"* "$raw/plans/cards/"
 cp "$work/$id.fresh-audit.json" "$work/$id.memory-summary.json" "$raw/validation/"
done
cp -a "$work/specs-v20" "$raw/plans/"
cp -a "$work/analysis" "$raw/"
cp "$work/live-capture/"*v20* "$raw/validation/live-capture/"
cp -a "$proof/completed-v20" "$proof/checkpoint-v20-20261021" "$raw/validation/"
cp -a "$study/reports/run-20261020T1546Z" "$raw/report"
cp "$proof/RESULT.md" "$raw/RESULT.md"
cp "$work/seal-final.sh" "$work/publish-final.sh" "$raw/"
cat > "$raw/README.md" <<'EOF'
Full immutable1M campaign raw evidence, completed2026-10-21UTC.
Seven actual attempts in runs/: K2048-v15 preflight refusal; K4096-v16
historical1M/zero terminal with later REJECTED swap profile; K2048-v16
host-interrupted, missing terminal/receipt/verified cleanup; K2048-v18
lock refusal; K2048-v19 stopped at nonterminal420K/swap drift; final
K2048-v20 and K4096-v20 complete1M/zero with native audit/profile PASS.
All older failures remain unadmitted. No records fabricated for missing data.
All available originals, receipts/cards/actual relays/assessments, public inputs,
worker/controllers, source snapshots, full raw traces/memory profiles,
validation/tests/smokes/diagnosis/recovery and the inspected report are included.
PRE-WORKER-CHECKPOINT.md is historical; its no-v20-run statement applies only
at that checkpoint. Current report describes final arms, n=1 perbase, no ranking.
No credentials/env dumps/private targets. All diagnostic controls were owned.
Original800K raw and prior checkpoints have separately verified immutable S3
addresses in validation/recovery and checkpoint custody; their large archive
blobs are not duplicated. Previous source freezes/ELFs and incomplete profiles
are retained unchanged. Exact historical native replay requires retained original
absolute worker/public-input paths; extracted bytes are separately compared.
This archive cannot contain its own upload receipt. Final PUT/HEAD/GET/payload/
native replay/repack proof and sidecar are stored outside it in PUBLICATION.md.
Verify SHA256SUMS. Canonical tar root runs, UTC2026-10-21,sorted,uid/gid0,
normalized public modes,gnuformat,xz-6-T1. See seal-final.sh/publish-final.sh.
EOF
(cd "$raw"; find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > ../SHA256SUMS.tmp; mv ../SHA256SUMS.tmp SHA256SUMS)
(cd "$pub"; tar --sort=name --mtime='2026-10-21 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > runs.tar.xz)
sha256sum "$pub/runs.tar.xz" "$raw/SHA256SUMS" > "$pub/hashes.txt"
stat -c %s "$pub/runs.tar.xz" > "$pub/bytes.txt"
chown ec2-user:ec2-user "$pub" "$pub/runs.tar.xz" "$pub/hashes.txt" "$pub/bytes.txt"
