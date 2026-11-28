#!/bin/bash
# 36-bit a=-3 P-256-class rung: B probe + four paired seeds, then the tuned
# rho arm from the `cryptanalysis` repo on the identical public targets.
#
# Timing note: this host is shared with other agent sessions, so wall time is
# not comparable across runs.  Every ladder invocation is therefore wrapped in
# /usr/bin/time -p and the CPU time (user+sys) is recorded next to it; the
# README reports CPU time as the intrinsic cost and wall time only as observed.
#
# The ladder's own rho arm is skipped (--no-rho): it is a single unoptimised
# walk, and the paired arm is the tuned `ca solve --alg rho`.
set -u
cd "$(dirname "$0")/../.." || exit 1
D=research/prime_36bit_20261122
LADDER=./target/release/examples/a3_s4_ladder
CA="$HOME/cryptanalysis/build/ca"
TIME=/usr/bin/time

P=68719476731
A=68719476728
BB=33
N=68719022539
G=19,72892532

run_one() { # $1=bits $2=seed $3=fb  $4=out-json  $5=out-time
  echo "load=$(uptime | sed 's/.*load averages: //')" > "$5"
  $TIME -p $LADDER "$1" "$2" p256 mitm "$3" sparse --no-rho \
    > "$4" 2>> "$5"
}

: > "$D/a3_36bit_sweep.log"
for B in 1600 2048 2560 3072; do
  run_one 36 20261121 "$B" "$D/.row_$B.json" "$D/.time_$B.txt"
  echo "== B=$B $(head -1 "$D/.time_$B.txt")" >> "$D/a3_36bit_sweep.log"
  grep -E '^(real|user|sys)' "$D/.time_$B.txt" | sed 's/^/   /' >> "$D/a3_36bit_sweep.log"
  grep -m1 "^{" "$D/.row_$B.json" >> "$D/a3_36bit_sweep.log"
done

: > "$D/a3_36bit_u64_seeds.jsonl"
: > "$D/a3_36bit_seeds_timing.txt"
for S in 20261121 20261122 20261123 20261124; do
  run_one 36 "$S" 2048 "$D/.row_$S.json" "$D/.time_$S.txt"
  echo "== seed=$S $(head -1 "$D/.time_$S.txt")" >> "$D/a3_36bit_seeds_timing.txt"
  grep -E '^(real|user|sys)' "$D/.time_$S.txt" | sed 's/^/   /' >> "$D/a3_36bit_seeds_timing.txt"
  grep -m1 "^{" "$D/.row_$S.json" >> "$D/a3_36bit_u64_seeds.jsonl"
done

# Paired tuned-rho arm, 36 bits, on the identical public target of each seed.
: > "$D/a3_36bit_rho_paired.jsonl"
python3 - "$D" "$P" "$A" "$BB" "$N" "$G" "$CA" <<'PY' >> "$D/a3_36bit_rho_paired.jsonl"
import json, subprocess, sys
d, p, a, b, n, g, ca = sys.argv[1:8]
for line in open(f"{d}/a3_36bit_u64_seeds.jsonl"):
    line = line.strip()
    if not line.startswith("{"):
        continue
    row = json.loads(line)
    q = ",".join(str(v) for v in row["public_target_q"])
    out = subprocess.run(
        [ca, "solve", "--alg", "rho", "--group", "ec", "--p", p, "--a", a,
         "--b", b, "--order", n, "--g", g, "--h", q],
        capture_output=True, text=True).stdout.strip().splitlines()
    rho = json.loads(out[-1]) if out else {"status": "no-output"}
    rho["target_seed"] = row["target_seed"]
    rho["ic_ms"] = row["ic_stages"]["total_ms"]
    rho["public_target_q"] = row["public_target_q"]
    rho["ic_recovered"] = row["ic_recovered"]
    rho["rho_agrees"] = rho.get("x") is not None and str(rho["x"]) == row["ic_recovered"]
    print(json.dumps(rho, sort_keys=True))
PY

# Same paired arm at 32 bits, on the four archived targets, so the 32-bit row
# can be restated against a tuned rho instead of the ladder's built-in walk.
: > "$D/a3_32bit_rho_paired.jsonl"
python3 - "$D" "$CA" <<'PY' >> "$D/a3_32bit_rho_paired.jsonl"
import json, subprocess, sys
d, ca = sys.argv[1:3]
p, a, b, n, g = "4294967291", "4294967288", "267", "4295072597", "1,2811385579"
for line in open("research/prime_32bit_20261122/a3_32bit_u64_seeds.jsonl"):
    line = line.strip()
    if not line.startswith("{"):
        continue
    row = json.loads(line)
    q = ",".join(str(v) for v in row["public_target_q"])
    out = subprocess.run(
        [ca, "solve", "--alg", "rho", "--group", "ec", "--p", p, "--a", a,
         "--b", b, "--order", n, "--g", g, "--h", q],
        capture_output=True, text=True).stdout.strip().splitlines()
    rho = json.loads(out[-1]) if out else {"status": "no-output"}
    rho["target_seed"] = row["target_seed"]
    rho["ic_ms"] = row["ic_stages"]["total_ms"]
    rho["ic_recovered"] = row["ic_recovered"]
    rho["rho_agrees"] = rho.get("x") is not None and str(rho["x"]) == row["ic_recovered"]
    print(json.dumps(rho, sort_keys=True))
PY

rm -f "$D"/.row_*.json "$D"/.time_*.txt

# 32-bit CPU-time control on the same four seeds, so the 32 -> 36 step is a
# measured CPU ratio rather than a wall-clock guess.
: > "$D/a3_32bit_cpu.txt"
for S in 20261121 20261122 20261123 20261124; do
  echo "== seed=$S" >> "$D/a3_32bit_cpu.txt"
  $TIME -p $LADDER 32 "$S" p256 mitm 1600 sparse --no-rho \
    > /dev/null 2>> "$D/a3_32bit_cpu.txt"
done

echo "rung complete"
