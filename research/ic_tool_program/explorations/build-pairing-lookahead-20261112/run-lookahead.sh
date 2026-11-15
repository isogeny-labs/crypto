set -u
cd /home/user/crypto; S=research/ic_tool_program/suite/v1; BIN=/dev/shm/bx/ic-la; ISO=/tmp/claude-0/rulev9/bin/isolated_bench; O=/dev/shm/bx/la; mkdir -p $O
for r in 1 2 3; do for row in k0n61 k0n53; do
 seed=$(jq -r --arg id "$row-M1-T01" '.rows[]|select(.id==$id)|.rho_seed' $S/SUITE.json)
 L="16 32 64 128"; [ $((r%2)) = 0 ] && L="128 64 32 16"
 for la in $L; do out=$O/$row-$la-r$r.json; [ -e $out ] && continue
  until env RAYON_NUM_THREADS=1 KIC_LOOKAHEAD=$la $ISO run --wait --cpus 2 --out $O/iso.jsonl --label la/$row/$la/$r -- $BIN price --params $S/params/S/$row/M1-T01.json --json --out $out.tmp --single-target --rho-seed $seed >/dev/null 2>$O/err; do sleep 10; done
  mv $out.tmp $out; done; done; done
echo DONE > $O/done
