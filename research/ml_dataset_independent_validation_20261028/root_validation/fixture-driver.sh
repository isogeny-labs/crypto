#!/usr/bin/env bash
# Bounded local software interoperability only; no cloud worker or publication.
set -euo pipefail
phase=${1:?prepare or generate}
evidence=${2:?fresh evidence directory}
producer_source=${3:?source-bound producer binary}
producer_sha=c01e024156067799356f590a0c3ac749da509305bd569e26c9bf084d5402d04a
here=$(cd "$(dirname "$0")" && pwd)
check_binary() {
  test "$(sha256sum "$1" | cut -d' ' -f1)" = "$producer_sha"
}
case "$phase" in
  prepare)
    test ! -e "$evidence"
    mkdir -p "$evidence"
    check_binary "$producer_source"
    cp "$producer_source" "$evidence/crypto-source-bound"
    chmod 0555 "$evidence/crypto-source-bound"
    check_binary "$evidence/crypto-source-bound"
    cp "$here/inputs/protocol.json" "$evidence/protocol.json"
    cp "$here/inputs/policies.json" "$evidence/policies.json"
    protocol_sha=$(sha256sum "$evidence/protocol.json" | cut -d' ' -f1)
    for name in legacy-complete policy-complete legacy-censored; do
      work=5000
      extra=()
      if [ "$name" = policy-complete ]; then extra=(--factor-base-policies "$evidence/policies.json"); fi
      if [ "$name" = legacy-censored ]; then work=16; fi
      timeout --kill-after=5 60 env -u ISOLAB_RESULTS_URI "$evidence/crypto-source-bound" dataset plan \
        --out "$evidence/$name.plan.json" --n 5,6 --m 2 --families koblitz,binary,prime \
        --solvers root --curve-seeds 1 --generators 1 --keys 1 --seed 1 \
        --factor-base-size 2 --degree-bound 3 --work-limit "$work" \
        --protocol-sha256 "$protocol_sha" "${extra[@]}" \
        > "$evidence/$name.plan.stdout.json" 2> "$evidence/$name.plan.stderr.log"
      timeout --kill-after=5 60 env -u ISOLAB_RESULTS_URI "$evidence/crypto-source-bound" dataset preflight \
        --plan "$evidence/$name.plan.json" --protocol "$evidence/protocol.json" \
        > "$evidence/$name.preflight.json" 2> "$evidence/$name.preflight.stderr.log"
      jq -e '.source_commit=="d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83" and .source_dirty==false and .generated_cells==0' \
        "$evidence/$name.preflight.json" > /dev/null
    done
    ;;
  generate)
    # Caller must checkpoint the frozen inputs and handoff before this phase.
    check_binary "$evidence/crypto-source-bound"
    for name in legacy-complete policy-complete legacy-censored; do
      test ! -e "$evidence/$name.run"
    done
    for name in legacy-complete policy-complete legacy-censored; do
      cells=12
      if [ "$name" = policy-complete ]; then cells=24; fi
      timeout --kill-after=5 600 env -u ISOLAB_RESULTS_URI "$evidence/crypto-source-bound" dataset generate \
        --plan "$evidence/$name.plan.json" --protocol "$evidence/protocol.json" \
        --out "$evidence/$name.run" --max-cases "$cells" --seconds-per-case 5 --memory-mib 512 --trace-mib 8 \
        > "$evidence/$name.generate.stdout.json" 2> "$evidence/$name.generate.stderr.log"
    done
    ;;
  *) exit 2 ;;
esac
