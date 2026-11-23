#!/bin/zsh
# Item 3: extend the supersingular D-equality cells (PR #1947 Q2) beyond
# the measured N <= 20 at m = 2: ordinary vs ss-j1728 arms at t = 11, 12.
# Same p (14,680,063 = 7*2^21 - 1, p = 3 mod 4) and default seeds as the study.
set -u
cd /Users/adamburan/ic-work/crypto
export CARGO_TARGET_DIR=/Volumes/SSD990-2/llm/build/crypto-target
export PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH"
OUT=research/pkm_ss_d_equality_ext_20261101
mkdir -p "$OUT"
for arm in ordinary ss-j1728; do
  cargo run --release --example pkm_ss_tower -- f4 --p 14680063 --arm $arm \
    --m 2 --t-min 11 --t-max 12 --targets 2 --budget 259200 \
    --out "$OUT/${arm}_m2_t11_12.jsonl" >> "$OUT/run.log" 2>&1
done
echo "done $(date)" >> "$OUT/run.log"
