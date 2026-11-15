#!/bin/bash
# Second-host replay: every session with the binary that produced it (sha256 in BINARY.sha256).
set -u; R=research/ic_tableb_ksum_20261115; BIN=${1:?path to the ecbench binary}; export RAYON_NUM_THREADS=1
mkdir -p $R/verify-local
ls $R/sessions | grep -E '^P[0-9]+-r[0-9]$' | xargs -P 4 -I{} sh -c "$BIN verify --dir $R/sessions/{} --replay-all --exit-code --out $R/verify-local/{}.json >/dev/null 2>$R/verify-local/{}.err; echo '{} rc='\$?" | sort > $R/verify-local/summary.txt
