#!/bin/sh
# The frozen order (PROTOCOL.md §3): one arm at a time.
cd "$(dirname "$0")"
for a in A0 V1 V2 U1; do sh run_arm.sh $a; done
