#!/usr/bin/env bash
# Copy the descent closure files of the frozen library commit into /dev/shm/frozen (read-only git show).
set -euo pipefail
rev=fcda1e22cb0c5424ca3244f783e69be8b99792f4
out=/dev/shm/frozen
mkdir -p "$out/src/cryptanalysis/f4_fp" "$out/src/prime_hyperelliptic"
cd /home/user/crypto/.claude/worktrees/wf_8b7b71c2-376-2
for f in cryptanalysis/f4_fp.rs cryptanalysis/f4_fp/reuse.rs cryptanalysis/f4_fp/certified.rs cryptanalysis/f4_fp/pair_certificate.rs \
         cryptanalysis/zero_dim_la.rs cryptanalysis/jv_bitops.rs cryptanalysis/jv_bitops_uniform.rs cryptanalysis/bitops.rs \
         cryptanalysis/jv_cover.rs cryptanalysis/fx_hash.rs cryptanalysis/gaudry_cubic.rs cryptanalysis/residual_walk.rs \
         cryptanalysis/glv_invariant_base.rs cryptanalysis/groebner_f4.rs prime_hyperelliptic/ops.rs prime_hyperelliptic/mod.rs; do
  if git cat-file -e "$rev:src/$f" 2>/dev/null; then git show "$rev:src/$f" > "$out/src/$f"; echo "ok $f"; else echo "absent $f"; fi
done
sha256sum "$out"/src/cryptanalysis/*.rs "$out"/src/cryptanalysis/f4_fp/*.rs "$out"/src/prime_hyperelliptic/*.rs
for n in jv_cover jv_extension_remainder_dlp jv_sieve_two_large_prime; do :; done
cmp research/extension_ic_graph_20261115/nagao-e2e/library-overlay/jv_cover.rs "$out/src/cryptanalysis/jv_cover.rs" && echo "overlay jv_cover identical to frozen" || echo "overlay jv_cover differs from frozen (expected: overlay adds NagaoSolver)"
