#!/usr/bin/env bash
# E43 G2.259, the components' existing controls reused: e39_checks (F's
# controls, main run, salt E39C0001), e40_checks (V's, the v2 run) and
# e42_checks (W's, E42b's run, salt E42C0002), built at the E43 commit, must
# reproduce the committed control records byte for byte. Restart-safe.
#   usage: replay_controls.sh <E43 bin dir> <cpus>
set -u
cd "$(dirname "$0")/../.."
out=experiments/50_compose_fw/replay_controls
mkdir -p "$out"
nb=$1; cpus=$2
export RAYON_NUM_THREADS=1
go() { # name binary args...
  name=$1; b=$2; shift 2
  [ -s "$out/$name.json" ] || { taskset -c "$cpus" "$nb/$b" "$@" --json "$out/$name.json.part" >"$out/$name.log" 2>&1 \
    && mv "$out/$name.json.part" "$out/$name.json"; }
}
go e39_checks e39_checks --salt E39C0001 --calls 32
go e40_checks e40_checks --calls 32
go e42_checks_b e42_checks --calls 32 --salt E42C0002
same() { cmp -s "$1" "$2" && echo true || echo false; }
cat >experiments/50_compose_fw/replay_controls.json <<JSON
{"what": "E43 G2.259: the components' control runs rebuilt at the E43 commit, byte for byte against the committed records",
 "runs": [
  {"control": "E39 F controls (e39_checks --salt E39C0001 --calls 32)", "reference": "experiments/46_pivot_rules_confirm3/checks.json", "identical": $(same "$out/e39_checks.json" experiments/46_pivot_rules_confirm3/checks.json)},
  {"control": "E40 V controls (e40_checks --calls 32)", "reference": "experiments/47_macaulay_entries/checks.json", "identical": $(same "$out/e40_checks.json" experiments/47_macaulay_entries/checks.json)},
  {"control": "E42b W controls (e42_checks --calls 32 --salt E42C0002)", "reference": "experiments/49_macaulay_tails/checks_b.json", "identical": $(same "$out/e42_checks_b.json" experiments/49_macaulay_tails/checks_b.json)}
 ]}
JSON
cat experiments/50_compose_fw/replay_controls.json
