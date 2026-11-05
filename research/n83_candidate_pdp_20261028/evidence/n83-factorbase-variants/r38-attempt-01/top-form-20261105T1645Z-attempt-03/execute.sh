#!/usr/bin/env bash
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
bash "$HERE/build-exact.sh"
bash "$HERE/run-spike38-topform.sh"
