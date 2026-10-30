#!/bin/bash
# Executes only when explicitly invoked after adapter integration and validation.
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: run.sh BINARY NEW_ATTEMPT_DIRECTORY FROZEN_PROTOCOL' >&2; exit 64; }
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
export PYTHONDONTWRITEBYTECODE=1
export RAYON_NUM_THREADS=8
exec python3 "$script_dir/driver.py" --binary "$1" --out "$2" --protocol "$3"
