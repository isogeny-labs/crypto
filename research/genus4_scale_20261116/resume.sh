#!/usr/bin/env bash
# Resume after a container restart (~21:19 UTC, 2026-11-16) killed p16411a mid-curve.
# Same frozen binary (binary.sha256). Curve 172 is re-run whole into p16411a_resume.jsonl;
# rows present in both files must be identical in every field but wall_ns (determinism check).
# Then the protocol's plain-rho stage (search.sh tail), unchanged.
set -euo pipefail
d=research/genus4_scale_20261116
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.rhorefs
common="--genera 4 --targets 3 --seeds 2 --arms wR --cc 2.0 --max-h 2 --calibrate"
$B run --primes 16411 --curves 1 $common --threads 2 --rho-cap 5e8 --curve-seeds 16411=172 --out $d/p16411a_resume.jsonl > $d/p16411a_resume.log 2>&1
seeds() { python3 -c "import json,sys; print('/'.join(sorted({str(r['curve_seed']) for f in sys.argv[2:] for r in map(json.loads, open(f)) if r.get('kind')=='instance' and r['p']==int(sys.argv[1])}, key=int)))" "$@"; }
s8=$(seeds 8209 $d/p8209.jsonl); s16=$(seeds 16411 $d/p16411a.jsonl $d/p16411b.jsonl)
$B run --primes 8209 --curves 3 --genera 4 --targets 1 --seeds 2 --arms P --cc 2.0 --max-h 2 --threads 2 --rho-cap 1.2e8 --curve-seeds 8209=$s8 --out $d/p8209_plain.jsonl > $d/p8209_plain.log 2>&1 &
$B run --primes 16411 --curves 3 --genera 4 --targets 1 --seeds 2 --arms P --cc 2.0 --max-h 2 --threads 2 --rho-cap 5e8 --curve-seeds 16411=$s16 --out $d/p16411_plain.jsonl > $d/p16411_plain.log 2>&1 &
wait
echo resume-done
