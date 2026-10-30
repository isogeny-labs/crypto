#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
sha256sum -c quadrics-inputs.sha256
mkdir -p source-data
printf 'ring quadratic27=2,(w),dp;\n' > source-data/quadrics-input-template.sing
cat runs/action27-01/basis-jets389.sing runs/action27-01/invariant-jets129.sing runs/action27-01/original-coordinate-coefficients.sing QUADRICS27.sing >> source-data/quadrics-input-template.sing
sha256sum source-data/quadrics-input-template.sing > source-data/quadrics-input.sha256
tar -cJf checkpoint-live.tar.xz runs source-data
