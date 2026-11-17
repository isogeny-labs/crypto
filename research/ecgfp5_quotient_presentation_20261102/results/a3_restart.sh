#!/usr/bin/env bash
# A3 queue after the 01:14Z container restart: U5 at the three primes, then U1P_4111.
cd "$(dirname "$0")/.."
date -u +"%FT%TZ A3 queue restarted"
bash run_a3.sh U5_4111 U5_262231 U5_33554371
bash run_a1.sh U1P_4111
date -u +"%FT%TZ queue done"
