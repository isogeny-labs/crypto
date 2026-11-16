#!/usr/bin/env bash
# One frozen cell (PROTOCOL.md §4): msolve -g 2 (reduced grevlex basis) streamed through a FIFO
# into hnfsolve --dump-P, then bwfinish (block Wiedemann, b = 128) on the dumped matrix.
#
#   sh run_cell.sh <p>          p in {4111, 262231, 33554371}
#
# Inputs are the merged W0 systems; the reference eliminant is W0's msolve parametrisation.
set -u
P="$1"
H="$(cd "$(dirname "$0")" && pwd)"
W0="$H/../ecgfp5_quotient_solve_20261101"
IN="$W0/results/m5_p${P}_planted"
O="$H/results/m5_p${P}"
BIN="${BIN:-$H/bin}"           # hnfsolve, bwfinish built from src/ (build.sh)
mkdir -p "$O"
rm -f "$O/gb.fifo"; mkfifo "$O/gb.fifo"
date -u +%FT%TZ > "$O/started_at.txt"
sha256sum "$IN/quot.ms" /usr/bin/msolve "$BIN/hnfsolve" "$BIN/bwfinish" > "$O/inputs.sha256"

"$BIN/hnfsolve" --gb "$O/gb.fifo" --dump-P "$O/P.bin" > "$O/dump.json" 2> "$O/dump.log" &
HP=$!
OMP_WAIT_POLICY=PASSIVE "$W0/bin/qsolve" run --rss-cap-gib 12 --wall-cap-s 14400 \
  --record "$O/msolve.run.json" -- \
  msolve -v 2 -t 4 -g 2 -f "$IN/quot.ms" -o "$O/gb.fifo" > "$O/msolve.log" 2>&1
echo "msolve_exit=$?" > "$O/exit_codes.txt"
wait $HP; echo "hnfsolve_exit=$?" >> "$O/exit_codes.txt"
rm -f "$O/gb.fifo"

"$BIN/bwfinish" --P "$O/P.bin" --b 128 --threads 4 --seed 1 \
  --planted "$IN/quot.planted" --elim-out "$O/bw.elim" > "$O/bw.json" 2> "$O/bw.log"
echo "bwfinish_exit=$?" >> "$O/exit_codes.txt"

python3 -I "$H/src/elim_from_msolve.py" "$IN/quot.out" > "$O/w0.elim"
if cmp -s "$O/bw.elim" "$O/w0.elim"; then echo "eliminant_equals_w0=true"; else echo "eliminant_equals_w0=false"; fi >> "$O/exit_codes.txt"
sha256sum "$O/P.bin" "$O/bw.elim" "$O/w0.elim" > "$O/outputs.sha256"
rm -f "$O/P.bin"                # 1.3 GB; its hash is recorded, and it is a deterministic function of quot.ms
date -u +%FT%TZ > "$O/finished_at.txt"
cat "$O/exit_codes.txt"
