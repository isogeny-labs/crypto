#!/bin/bash
set -euo pipefail
n83_work=/home/ec2-user/crypto-n83-resume
n83_scratch=/home/ec2-user/scratch/n83-native-phase-20261019
n83_old=/home/ec2-user/scratch/n83-docker-sweep-20261018
n83_study=research/koblitz_n83_factor_base_sweep_20261008
n83_bundle=$n83_scratch/raw-bundle
mkdir -p "$n83_bundle/$n83_study/verification" "$n83_bundle/transport/v1" "$n83_bundle/transport/v2" "$n83_bundle/transport/v3" "$n83_bundle/validation"
for v in native-phase-20261019 native-phase-20261019-v2; do
 mkdir -p "$n83_bundle/$n83_study/verification/$v"
 cp -a "$n83_work/$n83_study/verification/$v/exec" "$n83_bundle/$n83_study/verification/$v/"
done
for v in native-phase-20261019 native-phase-20261019-v2 native-phase-20261019-v3; do
 mkdir -p "$n83_bundle/$n83_study/verification/$v"
 cp "$n83_work/$n83_study/verification/$v/PROTOCOL.md" "$n83_bundle/$n83_study/verification/$v/"
done
cp -a "$n83_scratch/cards" "$n83_scratch/specs" "$n83_bundle/transport/v1/"
cp "$n83_scratch"/controls*.json "$n83_scratch/controls-relay.txt" "$n83_bundle/transport/v1/"
for v in v2 v3; do
 cp -a "$n83_scratch/$v/cards" "$n83_scratch/$v/specs" "$n83_bundle/transport/$v/"
 cp "$n83_scratch/$v/"*.json "$n83_scratch/$v/"*-relay.txt "$n83_bundle/transport/$v/"
 for forged in "$n83_scratch/$v/"forged-*; do
  if test -d "$forged"; then cp -a "$forged" "$n83_bundle/transport/$v/"; fi
 done
done
for pattern in native-runner* native-phase-build* native-phase-full-tests* selector-control*; do
 for f in "$n83_old/validation/"$pattern; do test ! -f "$f" || cp "$f" "$n83_bundle/validation/"; done
done
cp "$n83_scratch/validation.json" "$n83_scratch/historical-seal-check.log" "$n83_scratch/historical-seal.sha256" "$n83_bundle/validation/"
cp "$n83_old/bin/docker" "$n83_bundle/binaries/docker-wrapper"
cp "$n83_scratch/collect-evidence.sh" "$n83_bundle/validation/"
(cd "$n83_bundle" && find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS)
(cd "$n83_bundle" && sha256sum --check SHA256SUMS > "$n83_scratch/raw-bundle-check.log")
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -C "$n83_scratch" -cf - raw-bundle | xz -6 -T1 > "$n83_scratch/runs.tar.xz"
sha256sum "$n83_scratch/runs.tar.xz" > "$n83_scratch/runs.tar.xz.sha256"
