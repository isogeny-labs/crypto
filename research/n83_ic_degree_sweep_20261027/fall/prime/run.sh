#!/bin/bash
set -euo pipefail
cd /work
mkdir results inputs
upload() { aws s3 sync results "$ISOLAB_RESULTS_URI" --only-show-errors; }
trap upload EXIT
uname -a > results/host.txt
lscpu >> results/host.txt
cp /opt/prime-protocol.json results/protocol.json
cp /opt/prime-source-revision.txt results/source-revision.txt
cp /opt/r3-results.json results/r3-results.json
sha256sum /opt/prime_fall /opt/fall > results/binary.sha256
printf '%s  %s\n' 95bc8431d46e11cffa11051b2ae9ff82c2b5c8cfc0dfef1b373629c50c4c94a2 /opt/r3-results.json | sha256sum --check > results/r3-results-verified.txt
/opt/prime_fall selfcheck > results/controls.json
aws s3 sync s3://isolab-results-104186811093-us-east-2/runs/n83-degree-m2-m9-20261027-v2/25e3489a-612a-4d40-aaa4-93a1b963ff30/inputs/ inputs/ --exclude '*' --include 'prime-*/system.sing' --only-show-errors
export R3_VERIFIER=/opt/fall
timeout --signal=TERM --kill-after=10 300 /opt/prime_fall run /opt/r3-results.json /work/inputs /work/results/certificates > results/producer.log
upload
for certificate in results/certificates/*-natural.json results/certificates/*-planted.json; do
  /opt/prime_fall replay "$certificate" >> results/replay.jsonl
done
(cd results && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
upload
