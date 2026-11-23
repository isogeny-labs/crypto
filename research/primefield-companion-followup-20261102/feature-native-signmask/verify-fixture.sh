#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/../../.."
component=research/primefield-companion-followup-20261102/feature-native-signmask
sha256sum --check "$component/check-lock.sha256"
cmp "$component/program.tsv" <(jq -r '.terms[] | [.coefficient, (.offsets | map(tostring) | join(","))] | @tsv' research/primefield-companion-followup-20261102/feature-decoder-binding/program.json)
