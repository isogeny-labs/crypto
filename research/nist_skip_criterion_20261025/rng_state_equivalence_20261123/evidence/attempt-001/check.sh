#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p evidence
test ! -e evidence/build.status
test ! -e evidence/check.status
compiler=${CXX:-g++}
"$compiler" --version >evidence/compiler.txt
set +e
"$compiler" -std=c++17 -O2 -Wall -Wextra -Werror state_check.cpp -o /tmp/nist-rng-state-check-20261123 >evidence/build.stdout 2>evidence/build.stderr
build_status=$?
printf '%s\n' "$build_status" >evidence/build.status
set -e
test "$build_status" -eq 0
sha256sum /tmp/nist-rng-state-check-20261123 >evidence/binary.sha256
set +e
timeout 10s /tmp/nist-rng-state-check-20261123 >evidence/states.tsv 2>evidence/check.stderr
check_status=$?
printf '%s\n' "$check_status" >evidence/check.status
set -e
test "$check_status" -eq 0
cat evidence/check.stderr
