cd /home/user/crypto; R=research/jv_cover_p_scaling_20261024/results; B=target/release/examples/jv_cover
for s in 1 2; do a=$(date +%s)
 timeout 1500 $B --exp dlp --sizes 1009 --first-seed $s --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64 --json $R/dlp_p1009_s$s.json > $R/dlp_p1009_s$s.log 2>&1
 e=$?; echo "dlp_p1009_s$s exit $e wall_s $(( $(date +%s)-a ))" >> $R/walls.txt; [ $e != 0 ] && break; done
echo "1009 DONE" >> $R/walls.txt
