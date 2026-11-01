#!/bin/sh
# Assemble an immutable card only after the native and proof gates pass.
# The resulting hashes must be committed in a host card before run-card.sh.
set -eu
test "$#" -eq 6
source=$(realpath "$1")
build=$(realpath "$2")
binaries=$(realpath "$3")
construction_run=$(realpath "$4")
construction="$construction_run/construction.json"
proof=$(realpath "$5")
output=$6
inputs=/home/ec2-user/n83-m23-20261019/inputs
revision=$(cat "$build/source-commit.txt")
test "$(git -C "$source" rev-parse HEAD)" = "$revision"
test -z "$(git -C "$source" status --porcelain)"
for gate in lib tests build; do test "$(cat "$build/$gate-exit.txt")" = 0; done
test "$(cat "$proof/lean.exit")" = 0
cmp "$proof/SharedRootBits.lean" "$source/research/n83_d18_shared_bits_20261020/lean/SharedRootBits.lean"
sha256sum -c "$build/binaries.sha256"
for name in n83_d18_shared_bits n83_prebuild_cost_m23 n83_small_m4_replay; do
    expected=$(awk -v name="$name" '$2 ~ ("/" name "$") {print $1}' "$build/binaries.sha256")
    actual=$(sha256sum "$binaries/$name" | cut -d ' ' -f 1)
    test "$actual" = "$expected"
done
test "$(cat "$construction_run/exit.txt")" = 0
test "$(awk '{print $1}' "$construction_run/binary.sha256")" = "$(sha256sum "$binaries/n83_d18_shared_bits" | cut -d ' ' -f 1)"
jq -e 'length==1 and .[0].State.ExitCode==0 and .[0].State.OOMKilled==false
    and .[0].HostConfig.Memory==4294967296 and .[0].HostConfig.MemorySwap==4294967296
    and .[0].HostConfig.NanoCpus==1000000000 and .[0].HostConfig.CpusetCpus=="31"
    and .[0].HostConfig.NetworkMode=="none" and .[0].HostConfig.ReadonlyRootfs==true' \
    "$construction_run/container-inspect.json" > /dev/null
module_hash=$(sha256sum "$source/src/cryptanalysis/binary_semaev_chain_sat.rs" | cut -d ' ' -f 1)
example_hash=$(sha256sum "$source/examples/n83_d18_shared_bits.rs" | cut -d ' ' -f 1)
jq -e --arg module "$module_hash" --arg example "$example_hash" '
    .schema=="n83.d18-shared-root-construction/v1" and .status=="PASS"
    and .columns==16627 and (.rows|length)==6 and .search_performed==false
    and .source_module_sha256==$module and .source_example_sha256==$example' "$construction" > /dev/null
test ! -e "$output"
mkdir "$output"
output=$(realpath "$output")
cp "$binaries/n83_prebuild_cost_m23" "$output/worker"
cp "$binaries/n83_small_m4_replay" "$output/replayer"
cp "$binaries/n83_d18_shared_bits" "$output/gate"
cp "$source/research/n83_d18_shared_bits_20261020/run-card.sh" "$output/run-card.sh"
cp "$source/research/n83_d18_shared_bits_20261020/run-docker.sh" "$output/run-docker.sh"
cp "$construction" "$output/construction.json"
cp -R "$construction_run" "$output/construction-run"
cp "$build/source-commit.txt" "$output/source-commit.txt"
cp -R "$build" "$output/build"
cp -R "$proof" "$output/proof"
cp "$proof/lean.exit" "$output/build/proof-exit.txt"
cp "$0" "$output/prepare-card.sh"
cd "$output"
sha256sum worker replayer gate run-card.sh run-docker.sh construction.json \
    source-commit.txt prepare-card.sh build/lib-exit.txt build/tests-exit.txt \
    build/build-exit.txt build/proof-exit.txt proof/lean.log proof/SharedRootBits.lean \
    construction-run/container-inspect.json construction-run/exit.txt \
    construction-run/binary.sha256 > inputs.sha256
printf '%s  %s\n' \
  0c09f62bde61d3c3e987644508c53ab56fd3f545219013b9bde925ba29969275 "$inputs/U/manifest.json" \
  2575cea6b16cfdaa02531c79ca4797d976eb861def3182983c3048ce75dea045 "$inputs/D/manifest.json" \
  2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369 "$inputs/U/objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz" \
  347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7 "$inputs/D/objects/e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682.jsonl.gz" \
  2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5 "$inputs/public/public-targets.json" \
  >> inputs.sha256
sha256sum -c inputs.sha256 > prepare-check.log
sudo -n docker image inspect amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f > prelaunch-image.json
date -u +%FT%TZ > prepared.utc
printf '%s\n' PREPARED_NOT_LAUNCHED > prepare-status.txt
