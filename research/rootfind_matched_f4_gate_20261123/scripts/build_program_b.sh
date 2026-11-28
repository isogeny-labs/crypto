#!/usr/bin/env bash
# Rebuild program B's exact chart solver (primal_parabola_20261109_v5/native.cpp)
# from byte-identical repo sources. Its include "../dual_lattice_20261109_v2/frozen_prefix.hpp"
# is absent from the repo; dual_ladder_20261109_v3/EXTRACTION.json records that its
# frozen_prefix.hpp is that parent's prefix (prefix_sha256 607da94a...), so it is placed there.
set -euo pipefail
REPO=$(cd "$(dirname "$0")/../../.." && pwd)
SRC=$REPO/research/rootfinding_checkpoint_20261109/code/research
OUT=${1:?outdir}
mkdir -p "$OUT/r/primal_parabola_20261109_v5" "$OUT/r/dual_lattice_20261109_v2" "$OUT/r/nist_sum_gmp_20261109_v1"
cp "$SRC/primal_parabola_20261109_v5/native.cpp" "$SRC/primal_parabola_20261109_v5/frozen_lll_prefix.hpp" "$OUT/r/primal_parabola_20261109_v5/"
cp "$SRC/dual_ladder_20261109_v3/frozen_prefix.hpp" "$OUT/r/dual_lattice_20261109_v2/"
cp "$SRC/nist_sum_gmp_20261109_v1/gmp_field.hpp" "$OUT/r/nist_sum_gmp_20261109_v1/"
sha256sum "$OUT"/r/*/* 
g++ -O2 -std=c++17 -o "$OUT/program_b" "$OUT/r/primal_parabola_20261109_v5/native.cpp" -lgmpxx -lgmp
echo built "$OUT/program_b"
