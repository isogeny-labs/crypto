#!/usr/bin/env bash
# Amendment A2 cells (PROTOCOL.md §9, A2).   bash run_a2.sh U4_<p> [...]
# U4: the u-ring system with variable order s1,s2,s3,s4,w,s5 (s5 last = eliminating variable),
# whole solve by the frozen bw pipeline. Outputs go to results/A2/<cell>/.
set -u
H="$(cd "$(dirname "$0")" && pwd)"
W0="$H/../ecgfp5_quotient_solve_20261101"
BW="$H/../ecgfp5_quotient_bw_finish_20261102"
for CELL in "$@"; do
  O="$H/results/A2/$CELL"
  [ -e "$O" ] && { echo "$CELL: present, skipped"; continue; }
  mkdir -p "$O"; date -u +%FT%TZ > "$O/started_at.txt"
  P="${CELL#U4_}"
  if [ "$P" = 4111 ]; then G="$H/results/U1/gen"; else
    G="$O/gen"; "$H/bin/qsolve" generate --m 5 --p "$P" --seed 1 --arms uring --out "$G" > "$O/gen.json" 2> "$O/gen.log"
    echo "generate_exit=$?" >> "$O/exit_codes.txt"; fi
  { echo "s1,s2,s3,s4,w,s5"; tail -n +2 "$G/uring.ms"; } > "$O/u4.ms"
  IFS=, read -r a b c d e w < <(sed -n 2p "$G/uring.planted")
  printf 's1,s2,s3,s4,w,s5\n%s,%s,%s,%s,%s,%s\n' "$a" "$b" "$c" "$d" "$w" "$e" > "$O/u4.planted"
  sha256sum "$O/u4.ms" /usr/bin/msolve "$BW/bin/hnfsolve" "$BW/bin/bwfinish" > "$O/inputs.sha256"
  rm -f "$O/gb.fifo"; mkfifo "$O/gb.fifo"
  "$BW/bin/hnfsolve" --gb "$O/gb.fifo" --dump-P "$O/P.bin" > "$O/dump.json" 2> "$O/dump.log" &
  HP=$!
  OMP_WAIT_POLICY=PASSIVE "$W0/bin/qsolve" run --rss-cap-gib 12 --wall-cap-s 14400 --record "$O/msolve.run.json" -- \
    msolve -v 2 -t 4 -g 2 -f "$O/u4.ms" -o "$O/gb.fifo" > "$O/msolve.log" 2>&1
  echo "msolve_exit=$?" >> "$O/exit_codes.txt"
  wait $HP; echo "hnfsolve_exit=$?" >> "$O/exit_codes.txt"
  rm -f "$O/gb.fifo"
  if [ -s "$O/P.bin" ]; then
    "$BW/bin/bwfinish" --P "$O/P.bin" --b 128 --threads 4 --seed 1 \
      --planted "$O/u4.planted" --elim-out "$O/bw.elim" > "$O/bw.json" 2> "$O/bw.log"
    echo "bwfinish_exit=$?" >> "$O/exit_codes.txt"
    sha256sum "$O/P.bin" > "$O/outputs.sha256"; rm -f "$O/P.bin"
  fi
  rm -f "$O/u4.ms"   # = gen/uring.ms with the variable line changed (hash in inputs.sha256)
  date -u +%FT%TZ > "$O/finished_at.txt"
  echo "$CELL: $(tr '\n' ' ' < "$O/exit_codes.txt")"
done
