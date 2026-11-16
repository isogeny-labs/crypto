#!/bin/sh
# Stage gpu/ecc2k under this directory so ecc2k130/modal_job.py (which uploads
# only the ecc2k130 tree) can build it, as benchmarks/ecc2k-clmad/prepare.sh does:
#   cd ecc2k130 && benchmarks/ecc2k-geometry/prepare.sh && \
#   modal run --detach modal_job.py --job benchmarks/ecc2k-geometry/gpujob.sh --out DIR
# tree/ is a copy and is not committed; the sources of record are gpu/ecc2k.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
rsync -a --delete --exclude 'test_*' --exclude 'bench' --exclude '*.bin' --exclude '*.log' \
  --exclude 'curve_*.h' --exclude 'vec_*.h' --exclude '__pycache__' \
  "$here/../../../gpu/ecc2k/" "$here/tree/"
