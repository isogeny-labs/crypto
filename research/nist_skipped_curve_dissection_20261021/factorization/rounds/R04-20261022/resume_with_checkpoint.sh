#!/usr/bin/env bash
# One bounded native run, followed by exact recovery of any saved candidates.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
id=$1 out=$2 seconds=${3:-1800} option=${4:---resume}
[[ $id == skipped_256_876-twist2 ]]
export NFS_POLY_BATCH=${NFS_POLY_BATCH:-120}
bash "$here/../R02-20261022/resume_job.sh" "$id" "$out" "$seconds" "$option"
[[ $option != --prepare-only ]] || exit 0
if jq -e '.status=="complete"' "$out/result.json" >/dev/null; then exit 0; fi
shopt -s nullglob
polys=("$out/state/nfs.dat.p" "$out/state/nfs.dat."*.p)
for file in "${polys[@]}"; do
  if [[ -s $file ]]; then
    bash "$here/prepare_polynomial_checkpoint.sh" "$out" \
      33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
    break
  fi
done
