#!/usr/bin/env bash
set -euo pipefail
zero_repo=/home/ec2-user/crypto-twist-zero-fiber
zero_meta="$zero_repo/research/twist_index_calculus_20261020/zero_fiber_collector/results/publication/final-metadata-v1"
cd "$zero_repo"
git rev-parse HEAD > "$zero_meta/checked-head.txt"
date -u +%FT%TZ > "$zero_meta/started-utc.txt"
git show HEAD:scripts/check-large-files.sh > "$zero_meta/check-large-files.sh"
cmp "$zero_meta/check-large-files.sh" /home/ec2-user/crypto-twist-ic/scripts/check-large-files.sh
bash "$zero_meta/check-large-files.sh" 5abfb020dd7e993eb315d34f93beacc2200e1585 HEAD > "$zero_meta/large-files.stdout" 2> "$zero_meta/large-files.stderr"
printf '0\n' > "$zero_meta/large-files.exit"
git diff --exit-code 1055dcbcdb4998e78e22c900ab737ddc9913de70 HEAD -- Cargo.toml Cargo.lock src examples > "$zero_meta/root-library-parity.stdout" 2> "$zero_meta/root-library-parity.stderr"
printf '0\n' > "$zero_meta/root-library-parity.exit"
git diff --exit-code 5abfb020dd7e993eb315d34f93beacc2200e1585 HEAD -- docs/ic/leaderboard.json docs/ic/LEADERBOARD.md docs/ic-leaderboard.html docs/ic/validation/PRE_GATE.json benchmarks/PRE_GATE.json benchmarks/README.md benchmarks/ARCHIVE.json scripts/validation_gate.py scripts/check-large-files.sh > "$zero_meta/validation-metadata-parity.stdout" 2> "$zero_meta/validation-metadata-parity.stderr"
printf '0\n' > "$zero_meta/validation-metadata-parity.exit"
sha256sum -c research/twist_index_calculus_20261020/zero_fiber_collector/runs/panel-v2/metadata/compiled-sources.sha256 > "$zero_meta/frozen-native-sources.stdout" 2> "$zero_meta/frozen-native-sources.stderr"
printf '0\n' > "$zero_meta/frozen-native-sources.exit"
(
  cd "$zero_repo/research/twist_index_calculus_20261020/zero_fiber_collector"
  sha256sum -c runs.tar.xz.sha256
) > "$zero_meta/linked-archive.stdout" 2> "$zero_meta/linked-archive.stderr"
printf '0\n' > "$zero_meta/linked-archive.exit"
(
  cd /home/ec2-user/crypto-twist-ic
  sha256sum -c /home/ec2-user/scratch/twist-image-unrelated-dirty-before-publication.sha256
) > "$zero_meta/protected-unrelated.stdout" 2> "$zero_meta/protected-unrelated.stderr"
printf '0\n' > "$zero_meta/protected-unrelated.exit"
git diff --check > "$zero_meta/diff-check.stdout" 2> "$zero_meta/diff-check.stderr"
printf '0\n' > "$zero_meta/diff-check.exit"
date -u +%FT%TZ > "$zero_meta/finished-utc.txt"
printf '0\n' > "$zero_meta/exit"
