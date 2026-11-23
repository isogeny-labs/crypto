#!/bin/bash
# usage: run.sh "t c idx [--planted]" ...
export RAYON_NUM_THREADS=1
cd /home/user/crypto
L=/tmp/claude-0/-home-user/eb02eff3-ee5a-5113-a25c-b6cc0a65fdff/scratchpad/wave3/labE
for cell in "$@"; do
  set -- $cell
  tag="t$1_c$2_i$3${4:+_planted}"
  target/release/examples/hky_depth --p 2013265921 --t $1 --c $2 --idx $3 $4 --out $L/raw.jsonl > $L/out_$tag.json 2> $L/log_$tag.txt
  echo "$tag done: $(python3 -c "import json;d=json.load(open('$L/out_$tag.json'));print(d['r_star'],d['G'],d['final_dim'],d['censored'],d['secs'])")" >> $L/progress.txt
done
