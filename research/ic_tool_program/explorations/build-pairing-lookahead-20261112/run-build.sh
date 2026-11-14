set -u
cd /home/user/crypto; S=research/ic_tool_program/suite/v1; BIN=/dev/shm/tgt-probe/release/ic; ISO=/tmp/claude-0/rulev9/bin/isolated_bench; O=/dev/shm/bx
for r in 1 2 3 4; do for row in k0n53 k0n61; do
 seed=$(jq -r --arg id "$row-M1-T01" '.rows[]|select(.id==$id)|.rho_seed' $S/SUITE.json)
 arms="off on"; [ $((r%2)) = 0 ] && arms="on off"
 for arm in $arms; do out=$O/$row-$arm-r$r.json; [ -e $out ] && continue
  env=(RAYON_NUM_THREADS=1); [ $arm = off ] && env+=(KIC_BUILD_PAIRED=0)
  until env "${env[@]}" $ISO run --wait --cpus 2 --out $O/iso.jsonl --label bx/$row/$arm/$r -- $BIN price --params $S/params/S/$row/M1-T01.json --json --out $out.tmp --single-target --rho-seed $seed >/dev/null 2>$O/err; do sleep 10; done
  mv $out.tmp $out; done; done; done
echo DONE > $O/done
