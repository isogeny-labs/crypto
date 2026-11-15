#!/usr/bin/env bash
# S4 measurement (PLAN.md §10): full-base line sieve vs walk at genus 3/4.
#
#   research/hec_ic_speedup_20261113/run_s4.sh OUT_DIR
#
# COUNT / TIME: example dirs of the release builds of s4_sieve at the S4 code
# commit, with and without `--features count-ops`, built from a `git archive`
# of that commit.  The count build runs with relation verification on (gate
# G10, the primary counts); the timing build with it off (descriptive).
# `uptime` is logged before each process.  Gates G10/G11 are checked by
# `summarise_s4.py OUT_DIR --gates-only` at the end.
set -euo pipefail
out=${1:?usage: run_s4.sh OUT_DIR}
COUNT=${COUNT:-/Users/adamburan/pq-work/hec-s4-count/release/examples}
TIME=${TIME:-/Users/adamburan/pq-work/hec-s4-time/release/examples}
mkdir -p "$out"
shasum -a 256 "$COUNT/s4_sieve" "$TIME/s4_sieve" > "$out/binaries.sha256"
uname -a > "$out/host.txt"
log() { echo "$1: $(uptime)" >> "$out/host.txt"; }
log count; HEC_VERIFY=1 "$COUNT/s4_sieve" > "$out/count.txt"
log time;  HEC_VERIFY=0 "$TIME/s4_sieve" > "$out/time.txt"
log end
python3 "$(dirname "$0")/summarise_s4.py" "$out" --gates-only
