#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p evidence
test ! -e evidence/build.status
test ! -e evidence/check.status
set +e
g++ -std=c++17 -O2 -Wall -Wextra -Werror check.cpp -o /tmp/nist-visitation-check-20261124 >evidence/build.stdout 2>evidence/build.stderr
status=$?
printf '%s\n' "$status" >evidence/build.status
set -e
test "$status" -eq 0
set +e
timeout 10s /tmp/nist-visitation-check-20261124 historical.tsv point-attempts.tsv >evidence/constraints.tsv 2>evidence/check.stderr
status=$?
printf '%s\n' "$status" >evidence/check.status
set -e
test "$status" -eq 0
cat evidence/check.stderr
