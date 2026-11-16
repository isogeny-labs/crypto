#!/usr/bin/env bash
# Offline test of scripts/ckpt-s3-sync.sh against the directory-backed fake.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
export FAKE_S3_ROOT=$T/s3 CKPT_AWS=$here/fake-aws.sh
S=$here/../ckpt-s3-sync.sh
mkgen() { mkdir -p "$1/gen-$2"; echo "state $2" > "$1/gen-$2/basis.bin"; echo "{}" > "$1/gen-$2/catalog.json"; [ "${3:-commit}" = commit ] && echo "{\"generation\":$2}" > "$1/gen-$2/checkpoint-manifest.json"; true; }
U=s3://b/dreg/x
mkgen "$T/a" 00000000000000000001; mkgen "$T/a" 00000000000000000002; mkgen "$T/a" 00000000000000000003 partial
"$S" push "$T/a" $U
[ -f $T/s3/b/dreg/x/gen-00000000000000000002/checkpoint-manifest.json ] || { echo "FAIL manifest not uploaded"; exit 1; }
[ ! -e $T/s3/b/dreg/x/gen-00000000000000000003 ] || { echo "FAIL partial generation uploaded"; exit 1; }
# a generation whose manifest never arrived is not committed remotely, and is removed once a newer one is
mkdir -p $T/s3/b/dreg/x/gen-00000000000000000003; echo x > $T/s3/b/dreg/x/gen-00000000000000000003/basis.bin
mkgen "$T/a" 00000000000000000004
"$S" push "$T/a" $U
# keep newest two committed: 4 and 2; 1 pruned
[ ! -e $T/s3/b/dreg/x/gen-00000000000000000001 ] || { echo "FAIL old remote generation kept"; exit 1; }
[ -f $T/s3/b/dreg/x/gen-00000000000000000004/checkpoint-manifest.json ] || { echo "FAIL gen 4 missing"; exit 1; }
[ ! -e $T/s3/b/dreg/x/gen-00000000000000000003 ] || { echo "FAIL orphan remote generation kept"; ls $T/s3/b/dreg/x; exit 1; }
"$S" restore $U "$T/b"
[ -f $T/b/gen-00000000000000000004/checkpoint-manifest.json ] && [ -f $T/b/gen-00000000000000000002/checkpoint-manifest.json ] || { echo "FAIL restore"; ls $T/b; exit 1; }
[ ! -e $T/b/gen-00000000000000000003 ] || { echo "FAIL restored an uncommitted generation"; exit 1; }
cmp $T/a/gen-00000000000000000004/basis.bin $T/b/gen-00000000000000000004/basis.bin
# watch: pushes on its interval and once more on TERM
mkgen "$T/a" 00000000000000000005
"$S" watch "$T/a" $U 1 & w=$!; sleep 3; mkgen "$T/a" 00000000000000000006; kill -TERM $w; wait $w || true
[ -f $T/s3/b/dreg/x/gen-00000000000000000006/checkpoint-manifest.json ] || { echo "FAIL final push on TERM"; exit 1; }
echo "ckpt-s3-sync: all checks passed"
