#!/usr/bin/env bash
# Read-only `git show` of the two pinned source trees into $SCRATCH (tooling only).
#  - fcda1e22: its 33 src files match Run41's Root41 manifest
#    (../../../validation/extension-pipeline-source.sha256) byte for byte; Run50's
#    driver linked that Root41 library.
#  - 11c9a031: Run50's recorded source revision; the deferred-pair modules the
#    Run50 driver compiled in by #[path] (manifest validation/deferred50-source.sha256).
set -euo pipefail
: "${SCRATCH:?}"
root=fcda1e22cb0c5424ca3244f783e69be8b99792f4
run50=11c9a0312f0b0a8ae85be325e72cbf9602d9d777
cd /home/user/crypto
rm -rf "$SCRATCH/frozen" "$SCRATCH/run50src"
mkdir -p "$SCRATCH/frozen/src/cryptanalysis/f4_fp" "$SCRATCH/frozen/src/prime_hyperelliptic" "$SCRATCH/run50src"
for f in f4_fp.rs f4_fp/reuse.rs f4_fp/pair_certificate.rs zero_dim_la.rs jv_bitops.rs jv_bitops_uniform.rs bitops.rs \
         jv_cover.rs fx_hash.rs gaudry_cubic.rs residual_walk.rs glv_invariant_base.rs groebner_f4.rs \
         jv_sieve.rs jv_sieve_graph.rs jv_sieve_two_large_prime.rs jv_sieve_two_large_prime_dlp.rs jv_counted_linear.rs \
         jv_homogeneous_quotient.rs jv_extension_remainder_dlp.rs jv_principal_relation.rs \
         large_prime_filter.rs large_prime_filter_mst.rs jv_large_prime_rank_bound.rs; do
  git show "$root:src/cryptanalysis/$f" > "$SCRATCH/frozen/src/cryptanalysis/$f"
done
git show "$root:src/prime_hyperelliptic/ops.rs" > "$SCRATCH/frozen/src/prime_hyperelliptic/ops.rs"
for f in jv_deferred_pair_graph jv_extension_deferred_pair_dlp jv_extension_quadratic_pair_dlp jv_extension_sparse_dlp \
         jv_homogeneous_quotient jv_principal_relation jv_quadratic_pair_graph jv_quadratic_pair_relation \
         jv_quartic_discriminant jv_sparse_collection_rank jv_sparse_kernel; do
  git show "$run50:src/cryptanalysis/$f.rs" > "$SCRATCH/run50src/$f.rs"
done
# Root41 manifest check of every fetched fcda1e22 file that the manifest lists.
grep ' src/' research/extension_ic_graph_20261115/validation/extension-pipeline-source.sha256 | while read -r h p; do
  f="$SCRATCH/frozen/$p"; [ -f "$f" ] || continue
  [ "$(sha256sum < "$f" | cut -c1-64)" = "$h" ] || { echo "Root41 manifest mismatch $p"; exit 4; }
done
# Run50 manifest check of the 11c9a031 modules.
grep ' src/cryptanalysis/jv_' research/extension_ic_graph_20261115/validation/deferred50-source.sha256 | while read -r h p; do
  f="$SCRATCH/run50src/$(basename "$p")"; [ -f "$f" ] || continue
  [ "$(sha256sum < "$f" | cut -c1-64)" = "$h" ] || { echo "Run50 manifest mismatch $p"; exit 4; }
done
echo fetched
