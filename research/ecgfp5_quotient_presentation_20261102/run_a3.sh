#!/usr/bin/env bash
# Amendment A3 cells (PROTOCOL.md §9, A3).   bash run_a3.sh U5_<p> [...]
# U5: the u-ring system (U1's presentation, unchanged), whole solve by msolve -g 2 ->
# hnfsolve_u -> bwfinish_u -> check_points. Outputs go to results/A3/<cell>/.
set -u
H="$(cd "$(dirname "$0")" && pwd)"
W0="$H/../ecgfp5_quotient_solve_20261101"
for CELL in "$@"; do
  O="$H/results/A3/$CELL"
  [ -e "$O" ] && { echo "$CELL: present, skipped"; continue; }
  mkdir -p "$O"; date -u +%FT%TZ > "$O/started_at.txt"
  P="${CELL#U5_}"
  if [ "$P" = 4111 ]; then G="$H/results/U1/gen"; else
    G="$O/gen"; "$H/bin/qsolve" generate --m 5 --p "$P" --seed 1 --arms uring --out "$G" > "$O/gen.json" 2> "$O/gen.log"
    echo "generate_exit=$?" >> "$O/exit_codes.txt"; fi
  sha256sum "$G/uring.ms" /usr/bin/msolve "$H/bin/hnfsolve_u" "$H/bin/bwfinish_u" "$H/src/hnfsolve_u.cpp" "$H/src/bwfinish_u.cpp" > "$O/inputs.sha256"
  rm -f "$O/gb.fifo"; mkfifo "$O/gb.fifo"
  "$H/bin/hnfsolve_u" --gb "$O/gb.fifo" --dump-P "$O/P2.bin" > "$O/dump.json" 2> "$O/dump.log" &
  HP=$!
  OMP_WAIT_POLICY=PASSIVE "$W0/bin/qsolve" run --rss-cap-gib 12 --wall-cap-s 14400 --record "$O/msolve.run.json" -- \
    msolve -v 2 -t 4 -g 2 -f "$G/uring.ms" -o "$O/gb.fifo" > "$O/msolve.log" 2>&1
  echo "msolve_exit=$?" >> "$O/exit_codes.txt"
  wait $HP; echo "hnfsolve_exit=$?" >> "$O/exit_codes.txt"
  rm -f "$O/gb.fifo"
  if [ -s "$O/P2.bin" ]; then
    "$H/bin/bwfinish_u" --P "$O/P2.bin" --b 128 --threads 4 --seed 1 \
      --planted "$G/uring.planted" --elim-out "$O/bw.elim" > "$O/bw.json" 2> "$O/bw.log"
    echo "bwfinish_exit=$?" >> "$O/exit_codes.txt"
    sha256sum "$O/P2.bin" > "$O/outputs.sha256"; rm -f "$O/P2.bin"
    python3 "$H/src/check_points.py" "$G/uring.ms" "$O/bw.json" > "$O/check_points.json"
    echo "check_points_exit=$?" >> "$O/exit_codes.txt"
  fi
  date -u +%FT%TZ > "$O/finished_at.txt"
  echo "$CELL: $(tr '\n' ' ' < "$O/exit_codes.txt")"
done
