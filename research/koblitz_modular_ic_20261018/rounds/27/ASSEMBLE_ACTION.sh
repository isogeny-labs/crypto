#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
sha256sum -c action-inputs.sha256
mkdir -p source-data
cat ACTION-header.sing runs/rr27-export01/charts-complete.sing runs/rr27-export01/basis-complete.sing runs/rr27-export01/jets-complete.sing runs/move27-01/move-certificate.sing LAURENT27.sing ACTION27.sing > source-data/action-input-template.sing
sha256sum source-data/action-input-template.sing > source-data/action-input.sha256
tar -cJf checkpoint-live.tar.xz runs source-data
