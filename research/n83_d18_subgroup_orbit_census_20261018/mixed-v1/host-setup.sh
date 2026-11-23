#!/bin/bash
set -euo pipefail
cd /home/ec2-user/n83-mixed
mkdir -p source snapshot inputs host-setup
(date -u +%FT%TZ; uname -a; lscpu) > host-setup/host-identity.txt
tar -xzf source-cad5fb2-minimal.tar.gz -C source
tar -xzf snapshot-cad5fb2.tar.gz -C snapshot
tar -xf inputs-clean.tar -C inputs
sha256sum inputs/U/objects/*.jsonl.gz inputs/D/objects/*.jsonl.gz > host-setup/input-sha256.txt
python3 - <<'PY'
from pathlib import Path
import hashlib,json
root=Path('/home/ec2-user/n83-mixed')
pkg=json.loads((root/'source-package.json').read_text())
att=json.loads((root/'snapshot/attestation.json').read_text())
assert att['source_commit']==pkg['source_commit'] and att['status_clean'] is True
for name,sha in pkg['compiled_source_sha256'].items():
 a=(root/'source'/name).read_bytes();b=(root/'snapshot'/name).read_bytes()
 assert hashlib.sha256(a).hexdigest()==sha==hashlib.sha256(b).hexdigest(),name
print('PASS seven compiled-source bytes and snapshot')
PY
sudo dnf install -y docker rust cargo gcc pkgconf-pkg-config openssl-devel > host-setup/install.log 2>&1
sudo systemctl enable --now docker > host-setup/docker-service.log 2>&1
sudo docker pull amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f > host-setup/docker-pull.log 2>&1
sudo docker image inspect amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f > host-setup/image-inspect.json
cd source
CARGO_TARGET_DIR=/home/ec2-user/n83-mixed/target cargo build --release --example koblitz_n83_factor_base_export --locked > ../host-setup/build.log 2>&1
sha256sum /home/ec2-user/n83-mixed/target/release/examples/koblitz_n83_factor_base_export > ../host-setup/binary-sha256.txt
date -u +%FT%TZ > ../host-setup/ready.utc
cat ../host-setup/binary-sha256.txt
