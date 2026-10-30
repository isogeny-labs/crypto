#!/bin/bash
# Executes only when explicitly invoked after adapter integration and validation.
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: run.sh BINARY NEW_ATTEMPT_DIRECTORY FROZEN_PROTOCOL' >&2; exit 64; }
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
export PYTHONDONTWRITEBYTECODE=1
export RAYON_NUM_THREADS=1
export OMP_NUM_THREADS=1
fixture_python=${ML_DATASET_FIXTURE_PYTHON:-/home/ec2-user/ml-dataset-work-20261028/protocol-checker-venv/bin/python}
exec "$fixture_python" "$script_dir/driver.py" --binary "$1" --out "$2" --protocol "$3"
