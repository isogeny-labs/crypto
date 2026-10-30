#!/bin/sh
# Amendment 15 gate: a checkpointed system that is killed and resumed gives the
# row of the uninterrupted run, on this host, with this binary.
#
#   selftest_checkpoint.sh BINARY OUT_DIR
#
# The system is A0, m = 3, t = 4 (about 16 s of F4 on the hosts of this
# study). It runs once plain, then twice more with `--checkpoint-every 1` (the
# second time also with `--checkpoint-in-step-every 1`, Amendment 16), each
# killed with SIGKILL twice (the parent and the child, as a Spot reclaim would)
# and resumed to the end. Every counted field of each resumed row must match the
# plain row; each must report at least two segments and a carried time. Exit 0
# only on a match.
set -eu

bin=$1
out=$2
d=$out/selftest
mkdir -p "$d"
mem_kb=$(awk '/^MemTotal:/ {print $2}' /proc/meminfo)
v_kb=$((mem_kb - 524288))
args="--arms A0 --m 3 --t-min 4 --t-max 4 --targets 1 --budget 900 --grace 120 --vmem-kb $v_kb"

"$bin" run $args --file plain --out-dir "$d" > "$d/plain.stdout" 2>&1
plain_s=$(python3 -I -c 'import json,sys; print(int(json.loads(open(sys.argv[1]).readline())["time"]))' "$d/plain.jsonl")
kill_after=$((plain_s / 3))
[ "$kill_after" -ge 2 ] || kill_after=2
echo "selftest: plain run took ${plain_s} s; killing the checkpointed run after ${kill_after} s, twice"

# chain NAME FLAGS...: run the system with a checkpoint every second (and FLAGS), kill
# it twice, resume to the end; the rows go to $d/NAME.jsonl
chain() {
    name=$1
    shift
    for i in 1 2; do
        "$bin" run $args --checkpoint-every 1 "$@" --file "$name" --out-dir "$d" > "$d/$name.$i.stdout" 2>&1 &
        pid=$!
        sleep "$kill_after"
        # by exact process name: `pkill -f "$bin"` would also match this script's own command line
        pkill -9 -x "$(basename "$bin" | cut -c1-15)" || true
        wait "$pid" 2>/dev/null || true
        echo "selftest: $name kill $i done; checkpoint files: $(ls "$d" | grep -c "$name.*ckpt" || true)"
    done
    "$bin" run $args --checkpoint-every 1 "$@" --file "$name" --out-dir "$d" > "$d/$name.3.stdout" 2>&1
}
chain resumed
# Amendment 16: the same, with a snapshot of the dense reduction every second inside a step
chain resumed_step --checkpoint-in-step-every 1

python3 -I - "$d/plain.jsonl" "$d/resumed.jsonl" "$d/resumed_step.jsonl" <<'PY'
import json, sys
plain = [json.loads(l) for l in open(sys.argv[1])]
assert len(plain) == 1, f"plain rows: {len(plain)}"
p = plain[0]
fields = ["D", "solving_degree_max", "last_productive_degree", "degree_reached",
          "max_cols_to_solution", "steps_to_solution", "steps", "max_rows", "max_cols",
          "field_ops", "basis_len", "pairs_above_bound", "planted_ok", "status"]
print("selftest: plain  ", {f: p.get(f) for f in fields})
if p.get("status") != "ok":
    sys.exit(f"selftest FAILED: plain status {p.get('status')}")
for name, path in (("resumed", sys.argv[2]), ("resumed_step", sys.argv[3])):
    res = [json.loads(l) for l in open(path)]
    assert len(res) >= 1, f"{name}: no resumed row"
    r = res[-1]
    bad = [f for f in fields if p.get(f) != r.get(f)]
    print(f"selftest: {name}", {f: r.get(f) for f in fields}, "segments", r.get("segments"),
          "ckpt_carried_s", r.get("ckpt_carried_s"))
    if bad:
        sys.exit(f"selftest FAILED ({name}): fields differ: {bad}")
    if not (r.get("segments", 0) >= 2 and r.get("ckpt_carried_s", 0) > 0):
        sys.exit(f"selftest FAILED ({name}): the row did not resume (segments {r.get('segments')}, carried {r.get('ckpt_carried_s')})")
print("selftest: PASSED")
PY
