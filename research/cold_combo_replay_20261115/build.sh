#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
out=${1:?usage: build.sh OUTPUT_DIRECTORY}
mkdir -p "$out"
repo=$(cd "$here/../.." && pwd)
git -C "$repo" rev-parse HEAD > "$out/source-revision.txt"
(cd "$repo" && sha256sum \
  research/cold_combo_replay_20261115/{driver,solver_public}.rs \
  research/quartic_n4_20261019/{common,ghs,ic,solver}.rs \
  research/quartic_dl1_cd_sqrt_20261113/src/ic2.rs \
  research/ic_gw_fasthash_20261114/src/{ic2,la2}.rs \
  src/cryptanalysis/plane_quartic_pic.rs) > "$out/source.sha256"
rustc --version > "$out/rustc.txt"
rustc --edition 2021 -O "$here/driver.rs" -o "$out/ref"
rustc --edition 2021 -O --cfg combo "$here/driver.rs" -o "$out/combo"
sha256sum "$out/ref" "$out/combo" > "$out/binary.sha256"
