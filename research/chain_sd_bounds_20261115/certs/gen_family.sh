#!/bin/sh
# Generate replay certificates for the admissible instance p=12289, n=11 (A=1210, B=0, t=11), k=2..9:
# lower certificate at D = LFD(k)-1 (1 not in V_D, full closure sweep) and upper certificate at D = LFD(k).
set -e
cd "$(dirname "$0")/../native"
B=./target/release/chainsd
lfd() { case $1 in 2|3|4|5) echo 4;; 6|7|8|9) echo 5;; esac; }
for k in 2 3 4 5 6 7 8 9; do
  L=$(lfd $k); Lm=$((L-1))
  for D in $Lm $L; do
    CHAINSD_CERT=../certs/family_p12289_n11_k${k}_D${D}.cert $B closure 12289 $k 1210 0 11 $D verify > ../certs/family_p12289_n11_k${k}_D${D}.json 2>/dev/null
  done
done
echo GEN_DONE
