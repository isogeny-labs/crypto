#!/usr/bin/env bash
# Validator: replay two recorded chain cells (exponent study, k20 m3 D = 9 and m2 D = 57) with the frozen reference
# binary dc2c30a4 under the parent's limits, and compare every per-target counted field with the recorded cells.
# Usage: l3_frozen_replay.sh <outdir>
set -u
OUT=${1:?outdir}; mkdir -p "$OUT"; R=/home/user/crypto/research/curve25519_exponent_study_20261108/cells
OLD=/home/user/bin-archive/curve25519_family-dc2c30a4
[ "$(sha256sum $OLD | cut -d' ' -f1)" = dc2c30a4242d80281622b2cb42cbaf9cb457d041ffbc9a4d35794bb893bf3faf ] || { echo hash; exit 1; }
for md in "3 9" "2 57"; do set -- $md; m=$1; d=$2; seed=$((20261108 + 1000 * m + d))
  bash -c "ulimit -v 3500000; RAYON_NUM_THREADS=1 $OLD f4sub 20 sub $d 0 mont $m sym 6 $seed 1200" | tail -n 1 > "$OUT/frozen_m${m}_d${d}.json"
  python3 -I -c "
import json,sys
a=json.load(open('$OUT/frozen_m${m}_d${d}.json')); b=json.load(open('$R/k20_m${m}_sym_d${d}_g0.json'))
K=('target_x','field_ops','solving_degree_max','max_cols','max_rows','steps','inconsistent','basis_len')
print('m=$m D=$d frozen replay equals record:', [tuple(t[k] for k in K) for t in a['per_target']]==[tuple(t[k] for k in K) for t in b['per_target']], a['mean_field_ops'])"
done
