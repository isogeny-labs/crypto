#!/usr/bin/env bash
# Run out/jobs.txt (one command per line, longest first) with 4 workers: each line becomes its own script
# under the scratch dir; exit status and wall seconds go to out/replay/jobs.log.
set -u
O=/home/user/crypto/research/genus4_prime_order_20261118/validation/out
V=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/val4po
J=$V/jobs; rm -rf $J; mkdir -p $J $O/replay
i=0
while IFS= read -r line; do
  i=$((i + 1)); f=$(printf '%s/j%03d.sh' $J $i)
  printf '#!/usr/bin/env bash\nt0=$(date +%%s)\n%s\nrc=$?\necho "j%03d rc=$rc s=$(( $(date +%%s) - t0 ))" >> %s/replay/jobs.log\n' "$line" $i $O > $f
done < $O/jobs.txt
echo "start $(date -u +%FT%TZ) jobs=$i genus_ladder.val $(sha256sum $V/bin/genus_ladder.val | cut -c1-64) val_po $(sha256sum $V/bin/val_po | cut -c1-64)" > $O/replay/jobs.log
ls $J/j*.sh | xargs -P 4 -n 1 bash
echo "end $(date -u +%FT%TZ)" >> $O/replay/jobs.log
