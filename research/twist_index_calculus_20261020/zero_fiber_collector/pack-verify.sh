#!/usr/bin/env bash
# Artifact preservation only. Invoke through the shared busy wrapper.
set -euo pipefail
zero_out=${1:?fresh EFS packaging directory}
zero_repo=$(git rev-parse --show-toplevel)
zero_study="$zero_repo/research/twist_index_calculus_20261020/zero_fiber_collector"
zero_prior="$zero_repo/../crypto-twist-ic/research/twist_index_calculus_20261020/joint_image_elimination"
zero_producer=57cde8bee873b43d50cf2dc4e1e74a6962604f1a
zero_dirs=(native-completion-20261024T1934Z native-gate-v2 native-gate-v3 native-gate-v4 native-gate-v5 native-gate-v6 native-gate-v7 native-gate-journal-20261024T1940Z panel-v1 panel-v2)
test ! -e "$zero_out"
test "$(cat "$zero_study/runs/panel-v2/metadata/terminal-exit")" = 0
test "$(cat "$zero_study/runs/native-gate-journal-20261024T1940Z/metadata/exit")" = 0
mkdir -p "$zero_out/staging/runs" "$zero_out/staging/frozen-source" "$zero_out/staging/reference-slice/cases"
zero_out=$(cd "$zero_out" && pwd)
cp "$0" "$zero_out/executed-script.sh"
git rev-parse HEAD > "$zero_out/packing-source-commit.txt"
date -u +%FT%TZ > "$zero_out/started-utc.txt"
systemctl list-units --state=running --no-legend --plain 'twist-zero*' > "$zero_out/science-owner-check.txt"
test ! -s "$zero_out/science-owner-check.txt"
for zero_dir in "${zero_dirs[@]}"; do
  test -d "$zero_study/runs/$zero_dir"
  cp -al "$zero_study/runs/$zero_dir" "$zero_out/staging/runs/$zero_dir"
  printf '%s\n' "runs/$zero_dir" >> "$zero_out/selected-raw-directories.txt"
done
sha256sum -c "$zero_study/runs/panel-v2/metadata/public-inputs.sha256" > "$zero_out/original-public-inputs-unchanged.txt"
sha256sum -c "$zero_study/runs/panel-v2/metadata/runner.sha256" > "$zero_out/original-runner-unchanged.txt"
"$zero_out/staging/runs/panel-v2/runner" --zero-five-polynomial-summary "$zero_out/staging/runs/panel-v2" "$zero_out/verified-native-summary.json" > "$zero_out/native-summary-verification.stdout" 2> "$zero_out/native-summary-verification.stderr"
cmp "$zero_out/staging/runs/panel-v2/summary.json" "$zero_out/verified-native-summary.json"
cp "$zero_study/runs/panel-v2/metadata/compiled-sources.sha256" "$zero_out/staging/COMPILED_SOURCES.sha256"
mapfile -t zero_sources < <(cut -c67- "$zero_out/staging/COMPILED_SOURCES.sha256")
git archive "$zero_producer" "${zero_sources[@]}" | tar -xf - -C "$zero_out/staging/frozen-source"
(
  cd "$zero_out/staging/frozen-source"
  sha256sum -c ../COMPILED_SOURCES.sha256
) > "$zero_out/frozen-source-verification.txt"
sha256sum -c "$zero_study/runs/panel-v2/metadata/reference-files.sha256" > "$zero_out/original-references-unchanged.txt"
for zero_p in 37 73 97 157 229 241 337; do
  for zero_seed in 0 1 2; do
    zero_id="p$zero_p-seed$zero_seed"
    mkdir "$zero_out/staging/reference-slice/cases/$zero_id"
    for zero_file in summary.json phi-l0-active.json double-l0-active.json; do
      cp -p "$zero_prior/runs/five-zero-v1/cases/$zero_id/$zero_file" "$zero_out/staging/reference-slice/cases/$zero_id/$zero_file"
    done
  done
done
cp -p "$zero_prior/runs/five-point-replay-v1/summary.json" "$zero_out/staging/reference-slice/independent-point-replay-summary.json"
test "$(sha256sum "$zero_out/staging/reference-slice/independent-point-replay-summary.json" | cut -d ' ' -f 1)" = 8b92c608edbb446095ceffb539e995da3f8dbae3e6ee95dd980b87b308a0989d
printf '%s\n' "$zero_producer" > "$zero_out/staging/FROZEN_SOURCE_COMMIT"
test -z "$(find "$zero_out/staging" -type l -print -quit)"
(
  cd "$zero_out/staging"
  find . -type f -printf '%P\0' | LC_ALL=C sort -z | xargs -0 sha256sum
) > "$zero_out/SHA256SUMS"
cp "$zero_out/SHA256SUMS" "$zero_out/staging/SHA256SUMS"
(
  cd "$zero_out/staging"
  find . -type f -not -name SHA256SUMS -printf '%s\t%P\n' | LC_ALL=C sort -k2
) > "$zero_out/staging/FILE_SIZES.tsv"
tar --format=gnu --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner --hard-dereference -C "$zero_out/staging" -cf - . | xz -1 -T1 > "$zero_out/runs.tar.xz"
sha256sum "$zero_out/runs.tar.xz" > "$zero_out/archive.sha256"
stat -c '%s' "$zero_out/runs.tar.xz" > "$zero_out/archive.bytes"
mkdir "$zero_out/fresh"
tar -xJpf "$zero_out/runs.tar.xz" -C "$zero_out/fresh"
(
  cd "$zero_out/fresh"
  sha256sum -c SHA256SUMS
) > "$zero_out/extracted-checksums.txt"
cmp "$zero_out/SHA256SUMS" "$zero_out/fresh/SHA256SUMS"
cmp "$zero_out/staging/FILE_SIZES.tsv" "$zero_out/fresh/FILE_SIZES.tsv"
while IFS= read -r zero_line; do
  zero_path=${zero_line:66}
  cmp "$zero_out/staging/$zero_path" "$zero_out/fresh/$zero_path"
done < "$zero_out/SHA256SUMS"
zero_payload_count=$(wc -l < "$zero_out/SHA256SUMS")
printf '%s\n' "$((zero_payload_count + 2))" > "$zero_out/exact-file-comparisons.txt"
tar --format=gnu --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner --hard-dereference -C "$zero_out/fresh" -cf - . | xz -1 -T1 > "$zero_out/repacked.tar.xz"
cmp "$zero_out/runs.tar.xz" "$zero_out/repacked.tar.xz"
sha256sum "$zero_out/repacked.tar.xz" > "$zero_out/repacked.sha256"
date -u +%FT%TZ > "$zero_out/finished-utc.txt"
printf '0\n' > "$zero_out/exit"
printf 'Native archive checksums, all file comparisons, source reconstruction and byte-identical repack pass.\n'
