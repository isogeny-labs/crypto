#!/bin/bash
# REDUCED replication (target < 20 min on a shared machine): re-runs the controls and every arm at b = 20 and 24 (+ kpure/kmix/a0 b=28 inst 1) into reduced/raw,
# then checks bit-for-bit equality of every deterministic record against raw/ (only the wall-clock hdr done_ms differs) and recomputes operation counts.
set -u
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk
cd $D
rm -rf reduced; mkdir -p reduced/raw reduced/out
export RAWDIR=reduced/raw OUTDIR=reduced/out
gp -q -f gen_curves.gp       < /dev/null > reduced/raw/curves.txt     2> reduced/raw/curves.err
gp -q -f check_identity.gp   < /dev/null > reduced/raw/identity.txt   2> reduced/raw/identity.err
gp -q -f chainsearch.gp      < /dev/null > reduced/raw/chains.txt     2> reduced/raw/chains.err
gp -q -f check_exhaustive.gp < /dev/null > reduced/raw/exhaustive.txt 2> reduced/raw/exhaustive.err
{
  for B in 20 24; do
    T=$([ $B = 20 ] && echo 80 || echo 80)
    for arm in kpure kmix kpureT a0 a1; do for inst in 1 2; do echo "$arm $B $inst $T"; done; done
  done
  for inst in 1 2; do echo "cross 20 $inst 24"; done
  echo "pc1 24 1 40"; echo "nc1 24 1 40"; echo "kmixidx 24 1 40"
  for arm in kpure kmix a0; do echo "$arm 28 1 60"; done
} | xargs -P 2 -L 1 ./cellrun.sh
rm -rf target
./check_reduced.sh
