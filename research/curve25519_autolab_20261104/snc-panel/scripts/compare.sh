#!/usr/bin/env bash
# Pre-registered pairs (PROTOCOL.md section 2), paired bootstrap per weight.
set -e
D=research/curve25519_autolab_20261104/snc-panel
cmp() { for w in 0 0.1 1; do ./target/release/ecbench compare --dir $D/sessions/k$1 --a $2 --b $3 --resamples 4000 --seed 20261110 --weight $w --save | head -3; done; }
cmp 20 inc tw2cj-lazy
cmp 20 tw2cj-lazy tw2cj
for k in 24 28 32; do
  cmp $k inc inc-lazy
  cmp $k inc-lazy tw2cj-lazy
  cmp $k tw2cj-lazy tw2cj
done
for k in 20 24 28 32; do for a in inc tw2cj-lazy; do cmp $k rho-neg $a; done; done
