#!/usr/bin/env bash
# Frozen order: S1 P1 S2 P2, one process at a time on the whole host.
set -euo pipefail
bin=$1; out=$2; here=$(dirname "$0")
S="all-summands+t2.2+meet5+rebuild+indexed+parallel"
P="all-summands+t2.2+meet5+rebuild+indexed+parallel+parbuild"
"$here/run_rep.sh" "$bin" "$out/S1" S1 "$S"
"$here/run_rep.sh" "$bin" "$out/P1" P1 "$P"
"$here/run_rep.sh" "$bin" "$out/S2" S2 "$S"
"$here/run_rep.sh" "$bin" "$out/P2" P2 "$P"
