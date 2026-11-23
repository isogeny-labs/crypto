#!/usr/bin/env bash
# Second RunPod runner (PROTOCOL.md, 2026-11-12 13:15 entry). ecbench holds a global bench lock, so jobs run one at a time.
# After each job it re-uploads the results tarball to the same presigned PUT URL, so progress is visible in S3.
set -u
mkdir -p /w && cd /w
python3 - <<'PY'
import os, urllib.request
for k, v in os.environ.items():
    if k.startswith("GET_"):
        urllib.request.urlretrieve(v, k[4:].lower().replace("__", "-").replace("_", "."))
PY
chmod +x ecbench
echo "a0be1b2a3a02f5385c9b9654622b01923f3f54a52904bd157861e0cbe8796d29  ecbench" | sha256sum -c - > binary-check.txt 2>&1 || echo BINARY_HASH_MISMATCH > FAILED
( nproc; lscpu; head -3 /proc/meminfo; uname -a ) > host.txt 2>&1
upload() {
  sha256sum spec-*.json ecbench > inputs.sha256 2>/dev/null
  tar czf /results.tgz --exclude=./ecbench . 2>/dev/null; mv /results.tgz results.tgz
  sha256sum results.tgz > results.sha256
  python3 - <<'PY'
import os, urllib.request
for fn, key in (("results.tgz", "RESULTS_PUT_URL"), ("results.sha256", "SHA_PUT_URL")):
    urllib.request.urlopen(urllib.request.Request(os.environ[key], data=open(fn, "rb").read(), method="PUT")).read()
PY
  rm -f results.tgz
}
if [ ! -f FAILED ]; then
  for id in ${JOBS}; do
    ./ecbench run --spec spec-$id.json --out s-$id --cpus none --allow-busy > s-$id.run.log 2>&1
    ./ecbench verify --dir s-$id --replay-all --exit-code --out s-$id.audit.json > s-$id.verify.log 2>&1
    echo "$id exit=$?" >> exits.txt
    upload
  done
fi
echo ALL_DONE >> exits.txt
upload
