#!/usr/bin/env bash
# Optional native Singular replay of the fixed original SS5 incidence fiber.
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-ss5-replay.XXXXXX")}
mkdir -p "$output"
awk '/^poly unitprod=1;/{exit} {print}' "$expected/cm22-incidence.sing" > "$output/ss5.sing"
cat "$round/SS5.sing" >> "$output/ss5.sing"
cmp "$expected/cm22-ss5-fiber-02.sing" "$output/ss5.sing"
(ulimit -v 4194304; timeout 1200 "${SINGULAR:-Singular}" -q "$output/ss5.sing") > "$output/ss5.log" 2>&1
! rg '\?|error|failed' "$output/ss5.log"
rg -q '^SS5_FIBER_ALL_CHECKS_PASS$' "$output/ss5.log"
cmp "$expected/cm22-ss5-fiber-02.log" "$output/ss5.log"
printf 'PASS: original SS5 fiber, ideal lifts, all units, Jacobian and Frobenius replay byte for byte.\n'
