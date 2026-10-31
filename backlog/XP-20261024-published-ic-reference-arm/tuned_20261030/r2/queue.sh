#!/bin/bash
cd "$(dirname "$0")/.."
for c in k1n9 k1n11 k0n13 k0n15L; do echo "START $c $(date +%T)"; r2/runcell.sh $c > r2/runs/$c.log 2>&1; echo "END $c $(date +%T) rc=$?"; done
