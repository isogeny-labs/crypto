# The Amendment 8 sequence as it ran on 2026-10-20 (K1, T4a, X1 A0 m = 3, T4b, X1 A11 m = 4, X1 A0 m = 4).
# Copied from the session's scratch directory; paths are the container's.
#!/bin/sh
cd /home/user/crypto
R=research/factor_base_designs_20261015/runs
B=/home/user/crypto/target/release/examples/fb_designs_slope
echo "== K1"
for fam in a0 am3; do
  $B run --family $fam --arms A0,A11 --m 2 --t-min 2 --t-max 9 --budget 300 --vmem-kb 13000000 --file "K1-{arm}-$fam-m{m}" --out-dir $R
  $B run --family $fam --arms A0,A11 --m 3 --t-min 2 --t-max 4 --budget 300 --vmem-kb 13000000 --file "K1-{arm}-$fam-m{m}" --out-dir $R
done
echo "== T4a"
$B admit-tower $R --stop-below 64 --file T4-admission
T4A=$?
echo "T4A-EXIT $T4A"
echo "== X1 A0 m=3 t=5"
$B run --prime p3 --arms A0 --m 3 --t-min 5 --t-max 5 --budget 3600 --grace 300 --vmem-kb 15000000 --file "X1-{arm}-m{m}-p3" --out-dir $R
if [ $T4A -eq 0 ]; then
  echo "== T4b"
  $B run --engine tower --stop-below 64 --arms A0 --m 4 --t-min 4 --t-max 4 --budget 3600 --grace 300 --vmem-kb 15000000 --file "T4-{arm}-m{m}" --out-dir $R
else
  echo "T4b not run: the stop was not admitted"
fi
echo "== X1 A11 m=4 t=4"
$B run --prime p3 --arms A11 --m 4 --t-min 4 --t-max 4 --budget 3600 --grace 300 --vmem-kb 15000000 --file "X1-{arm}-m{m}-p3" --out-dir $R
echo "== X1 A0 m=4 t=4"
$B run --prime p3 --arms A0 --m 4 --t-min 4 --t-max 4 --budget 3600 --grace 300 --vmem-kb 15000000 --file "X1-{arm}-m{m}-p3" --out-dir $R
echo A8-DONE
