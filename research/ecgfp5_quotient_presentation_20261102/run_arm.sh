#!/usr/bin/env bash
# One frozen arm (PROTOCOL.md §2-§3).   sh run_arm.sh {A0|V1|V2|U1}
set -u
ARM="$1"
H="$(cd "$(dirname "$0")" && pwd)"
W0="$H/../ecgfp5_quotient_solve_20261101"
BW="$H/../ecgfp5_quotient_bw_finish_20261102"
O="$H/results/$ARM"
mkdir -p "$O"
date -u +%FT%TZ > "$O/started_at.txt"
SRC="$W0/results/m5_p4111_planted/quot.ms"
case "$ARM" in
  A0) cp "$SRC" "$O/sys.ms" ;;
  V1) { echo "s5,s4,s3,s2,s1,w"; tail -n +2 "$SRC"; } > "$O/sys.ms" ;;
  V2) { echo "w,s1,s2,s3,s4,s5"; tail -n +2 "$SRC"; } > "$O/sys.ms" ;;
  U1) "$H/bin/qsolve" generate --m 5 --p 4111 --seed 1 --arms uring --out "$O/gen" > "$O/gen.json" 2> "$O/gen.log"
      echo "generate_exit=$?" > "$O/exit_codes.txt"
      cp "$O/gen/uring.ms" "$O/sys.ms" ;;
  *) echo "unknown arm $ARM"; exit 2 ;;
esac
sha256sum "$O/sys.ms" /usr/bin/msolve "$H/bin/qsolve" > "$O/inputs.sha256"

if [ "$ARM" != U1 ]; then
  OMP_WAIT_POLICY=PASSIVE "$W0/bin/qsolve" run --rss-cap-gib 12 --wall-cap-s 14400 \
    --record "$O/msolve.run.json" -- \
    msolve -v 2 -t 4 -g 1 -f "$O/sys.ms" -o "$O/lm.out" > "$O/msolve.log" 2>&1
  echo "msolve_exit=$?" >> "$O/exit_codes.txt"
  rm -f "$O/sys.ms"   # A0/V1/V2 are W0's quot.ms (hash recorded) with a changed variable line
else
  rm -f "$O/gb.fifo"; mkfifo "$O/gb.fifo"
  "$BW/bin/hnfsolve" --gb "$O/gb.fifo" --dump-P "$O/P.bin" > "$O/dump.json" 2> "$O/dump.log" &
  HP=$!
  OMP_WAIT_POLICY=PASSIVE "$W0/bin/qsolve" run --rss-cap-gib 12 --wall-cap-s 14400 \
    --record "$O/msolve.run.json" -- \
    msolve -v 2 -t 4 -g 2 -f "$O/sys.ms" -o "$O/gb.fifo" > "$O/msolve.log" 2>&1
  echo "msolve_exit=$?" >> "$O/exit_codes.txt"
  wait $HP; echo "hnfsolve_exit=$?" >> "$O/exit_codes.txt"
  rm -f "$O/gb.fifo"
  if [ -s "$O/P.bin" ]; then
    "$BW/bin/bwfinish" --P "$O/P.bin" --b 128 --threads 4 --seed 1 \
      --planted "$O/gen/uring.planted" --elim-out "$O/bw.elim" > "$O/bw.json" 2> "$O/bw.log"
    echo "bwfinish_exit=$?" >> "$O/exit_codes.txt"
    sha256sum "$O/P.bin" > "$O/outputs.sha256"
    rm -f "$O/P.bin"
  fi
fi
date -u +%FT%TZ > "$O/finished_at.txt"
cat "$O/exit_codes.txt"
