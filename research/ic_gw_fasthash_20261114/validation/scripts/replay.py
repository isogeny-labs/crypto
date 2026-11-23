# Validator replay runner: one process at a time; per unit: loadavg before, wall, child rusage (wait4).
# usage: python3 -I replay.py OUTDIR PLAN.tsv   (PLAN lines: idx tag arm bin q seed)
import hashlib, json, os, subprocess, sys, time, datetime
out, plan = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
log = open(os.path.join(out, "replay-log.jsonl"), "a")
shas = {}
for line in open(plan):
    if not line.strip() or line.startswith("#"): continue
    idx, tag, arm, b, q, s = line.split()
    fn = f"{idx}_{tag}_{arm}_{q}_{s}.jsonl"
    path = os.path.join(out, fn)
    if os.path.exists(path): continue
    if b not in shas: shas[b] = hashlib.sha256(open(b, "rb").read()).hexdigest()
    la = open("/proc/loadavg").read().strip()
    others = subprocess.run(["ps", "-eo", "pid,pcpu,comm", "--sort=-pcpu", "--no-headers"], capture_output=True, text=True).stdout.splitlines()[:3]
    t0 = time.time(); start = datetime.datetime.utcnow().isoformat(timespec="seconds") + "Z"
    with open(path + ".tmp", "w") as fo, open(path + ".err", "w") as fe:
        p = subprocess.Popen([b, "ic", q, s, "D", "3.0", "1.7"], stdout=fo, stderr=fe)
        _, status, ru = os.wait4(p.pid, 0)
    wall = time.time() - t0
    rc = os.waitstatus_to_exitcode(status)
    meta = {"rec": "val_unit", "idx": int(idx), "tag": tag, "arm": arm, "bin": b, "bin_sha256": shas[b], "q": int(q), "seed": int(s),
            "cmd": f"{b} ic {q} {s} D 3.0 1.7", "rc": rc, "start_utc": start, "wall_s": round(wall, 3),
            "utime_s": ru.ru_utime, "stime_s": ru.ru_stime, "maxrss_kb": ru.ru_maxrss, "loadavg_before": la, "top3_before": others}
    os.rename(path + ".tmp", path)
    log.write(json.dumps(meta) + "\n"); log.flush()
    print(json.dumps({k: meta[k] for k in ("idx", "tag", "arm", "q", "seed", "rc", "wall_s", "utime_s", "loadavg_before")}), flush=True)
