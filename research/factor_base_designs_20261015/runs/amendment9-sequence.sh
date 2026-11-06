# The Amendment 9 sequence as it ran on 2026-10-20 (P4, Z1, M5), then Amendment 10's Z1 re-run at m = 2 from t = 3.
# Copied from the session's scratch directory; paths are the container's.
#!/bin/sh
cd /home/user/crypto
R=research/factor_base_designs_20261015/runs
B=/home/user/crypto/target/release/examples/fb_designs_slope
echo "== P4"
$B run --prime p4 --arms A0,A11 --m 2 --t-min 2 --t-max 8 --budget 300 --vmem-kb 13000000 --file "P4-{arm}-m{m}" --out-dir $R
$B run --prime p4 --arms A0,A11 --m 3 --t-min 2 --t-max 4 --budget 300 --vmem-kb 13000000 --file "P4-{arm}-m{m}" --out-dir $R
echo "== Z1"
$B run --target-structure xr0 --arms A0,A11 --m 2 --t-min 2 --t-max 9 --budget 300 --vmem-kb 13000000 --file "Z1-{arm}-m{m}" --out-dir $R
$B run --target-structure xr0 --arms A0,A11 --m 3 --t-min 2 --t-max 4 --budget 300 --vmem-kb 13000000 --file "Z1-{arm}-m{m}" --out-dir $R
echo "== M5"
$B run --arms A0 --m 5 --t-min 3 --t-max 3 --budget 300 --vmem-kb 13000000 --file "M5-{arm}-m{m}" --out-dir $R
echo A9-DONE
# Amendment 10 (run after the above, from the commit that registered it):
# $B run --target-structure xr0 --arms A0,A11 --m 2 --t-min 3 --t-max 9 --budget 300 --vmem-kb 13000000 --file 'Z1-{arm}-m{m}' --out-dir $R
