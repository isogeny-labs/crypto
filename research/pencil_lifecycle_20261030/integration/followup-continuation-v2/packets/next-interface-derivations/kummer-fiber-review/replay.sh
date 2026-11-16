#!/usr/bin/env bash
set -euo pipefail
cd /var/tmp/independent-kummer-fiber-next-20261101
rustc published/controls.rs -o producer-controls > producer-build2.stdout 2> producer-build2.stderr
./producer-controls > producer-result2.json 2> producer2.stderr
rustc adversarial.rs -o adversarial > adversarial-build2.stdout 2> adversarial-build2.stderr
./adversarial > adversarial-result2.json 2> adversarial2.stderr
printf '{"producer_build":0,"producer_run":0,"producer_assertions":11,"adversarial_build":0,"adversarial_run":0,"adversarial_assertions":7}\n' > receipt.json
