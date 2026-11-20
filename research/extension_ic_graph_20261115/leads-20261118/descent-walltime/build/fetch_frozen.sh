#!/usr/bin/env bash
# Copy the frozen descent closure (commit fcda1e22, the descent-finder lane's
# boundary; zero_dim_la.rs and jv_bitops.rs there are byte-identical to Run50's
# 11c9a031 and to current main) into $SCRATCH/frozen with read-only `git show`.
set -euo pipefail
rev=fcda1e22cb0c5424ca3244f783e69be8b99792f4
out=${SCRATCH:?}/frozen
rm -rf "$out"; mkdir -p "$out/src/cryptanalysis/f4_fp" "$out/src/prime_hyperelliptic"
cd /home/user/crypto
for f in cryptanalysis/f4_fp.rs cryptanalysis/f4_fp/reuse.rs cryptanalysis/f4_fp/certified.rs cryptanalysis/f4_fp/pair_certificate.rs \
         cryptanalysis/zero_dim_la.rs cryptanalysis/jv_bitops.rs cryptanalysis/jv_bitops_uniform.rs cryptanalysis/bitops.rs \
         cryptanalysis/jv_cover.rs cryptanalysis/fx_hash.rs cryptanalysis/gaudry_cubic.rs cryptanalysis/residual_walk.rs \
         cryptanalysis/glv_invariant_base.rs cryptanalysis/groebner_f4.rs prime_hyperelliptic/ops.rs prime_hyperelliptic/mod.rs; do
  if git cat-file -e "$rev:src/$f" 2>/dev/null; then git show "$rev:src/$f" > "$out/src/$f"; else echo "absent $f"; fi
done
