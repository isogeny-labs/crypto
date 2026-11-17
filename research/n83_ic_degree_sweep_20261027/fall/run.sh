#!/bin/bash
set -euo pipefail
cd /work
mkdir results inputs
upload() { aws s3 sync results "$ISOLAB_RESULTS_URI" --only-show-errors; }
trap upload EXIT
uname -a > results/host.txt
lscpu >> results/host.txt
cp /opt/fall-protocol.json results/protocol.json
cp /opt/fall-source-revision.txt results/source-revision.txt
sha256sum /opt/fall > results/binary.sha256
/opt/fall selfcheck > results/controls.json
aws s3 sync s3://isolab-results-104186811093-us-east-2/runs/n83-degree-m2-m9-20261027-v2/25e3489a-612a-4d40-aaa4-93a1b963ff30/inputs/ inputs/ --exclude '*' --include 'inputs.json' --include '*/system.sing' --only-show-errors
sha256sum inputs/inputs.json > results/input-list.sha256
printf '%s  %s\n' '67c84280824036652fc47702a5a15ceedf3acfe3511361c7f236198e79957bb3' 'inputs/inputs.json' | sha256sum --check > results/input-list-verified.txt
timeout --signal=TERM --kill-after=10 750 /opt/fall run /work/inputs /work/results/certificates > results/producer.jsonl
for certificate in results/certificates/*-natural.json results/certificates/*-planted.json; do
  /opt/fall replay /work/inputs "$certificate" >> results/replay.jsonl
done
(cd results && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
upload
