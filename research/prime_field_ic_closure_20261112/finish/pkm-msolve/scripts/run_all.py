#!/usr/bin/env python3
"""Run msolve on the PRESET systems, in the PRESET order, one at a time.

Usage: run_all.py OUTDIR [--only PREFIX ...]
Per run: msolve -t T -v 2 -g 2 -f SYS.ms -o runs/NAME.out > runs/NAME.log
  T = 1 if nvars <= 16 else 4; RLIMIT_AS = 14e9; wall 600 s if N <= 12 else 2400 s.
Writes runs/NAME.res.json (exit, wall, ru_maxrss, trace summary) and progress.txt.
A run that has a res.json is not repeated.
"""
import json
import os
import resource
import signal
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import msparse as MP  # noqa: E402

AS_CAP = 14_000_000_000


def order_key(meta):
    arm, m, t, kind = meta["arm"], meta["m"], meta["t"], meta.get("kind")
    if arm == "C0":
        return (0, t, m, meta["name"])
    if arm == "A" and t <= 2:
        return (1, t, m, kind, meta["name"])
    if arm == "A" and t == 3:
        return (2, m, kind, meta["name"])
    if arm == "B":
        return (3, t, m, meta["name"])
    if arm == "C":
        return (4, meta["p"], m, meta["name"])
    if arm == "A" and t == 4:
        return (5, meta["name"])
    return (9, meta["name"])


def setlimits():
    resource.setrlimit(resource.RLIMIT_AS, (AS_CAP, AS_CAP))
    os.setpgrp()


def run_one(out, meta, force=False):
    name = meta["name"]
    sysf = os.path.join(out, "systems", name + ".ms")
    rd = os.path.join(out, "runs")
    os.makedirs(rd, exist_ok=True)
    resf = os.path.join(rd, name + ".res.json")
    if os.path.exists(resf) and not force:
        return json.load(open(resf))
    nvars = meta.get("nvars") or (meta["N"] + meta["m"] - 2)
    T = 1 if nvars <= 16 else 4
    wall = 600 if meta["N"] <= 12 else 2400
    cmd = ["msolve", "-t", str(T), "-v", "2", "-g", "2", "-f", sysf, "-o", os.path.join(rd, name + ".out")]
    with open(os.path.join(out, "progress.txt"), "a") as pf:
        pf.write(f"{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())} start {name} T={T} wall={wall} as={AS_CAP}\n")
    logf = open(os.path.join(rd, name + ".log"), "w")
    t0 = time.time()
    proc = subprocess.Popen(cmd, stdout=logf, stderr=subprocess.STDOUT, preexec_fn=setlimits)
    stop = None
    while True:
        pid, status, ru = os.wait4(proc.pid, os.WNOHANG)
        if pid != 0:
            break
        if time.time() - t0 > wall:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pid, status, ru = os.wait4(proc.pid, 0)
            stop = "wall"
            break
        time.sleep(0.25)
    proc.returncode = 0  # reaped by hand
    dt = time.time() - t0
    logf.close()
    code = os.waitstatus_to_exitcode(status)
    rounds = MP.parse_trace(os.path.join(rd, name + ".log"))
    if stop is None and code != 0:
        stop = "crash"
    res = dict(name=name, cmd=" ".join(cmd), threads=T, wall_cap=wall, exit=code, stop=stop,
               wall_s=round(dt, 3), ru_maxrss_kB=ru.ru_maxrss, ru_utime=round(ru.ru_utime, 2),
               n_rounds=len(rounds), D=max((r["deg"] for r in rounds), default=None),
               W=max((r["cols"] for r in rounds), default=None),
               rows_at_W=(max(rounds, key=lambda r: r["cols"])["rows"] if rounds else None),
               max_rows=max((r["rows"] for r in rounds), default=None),
               last_deg=(rounds[-1]["deg"] if rounds else None))
    outf = os.path.join(rd, name + ".out")
    if code == 0 and os.path.exists(outf):
        txt = open(outf).read()
        body = "\n".join(ln for ln in txt.split("\n") if not ln.startswith("#")).strip()
        res["unit"] = body.replace(" ", "").startswith("[1]")
        res["gb_len"] = (body.count(",\n") + 1) if body.startswith("[") else None
    with open(resf, "w") as fh:
        json.dump(res, fh)
    with open(os.path.join(out, "progress.txt"), "a") as pf:
        pf.write(f"{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())} end {name} exit={code} stop={stop} "
                 f"D={res['D']} W={res['W']} wall={dt:.1f}s rss={ru.ru_maxrss}kB unit={res.get('unit')}\n")
    return res


def main():
    out = sys.argv[1]
    only = sys.argv[3:] if len(sys.argv) > 2 and sys.argv[2] == "--only" else None
    metas = [json.loads(l) for l in open(os.path.join(out, "index.jsonl"))]
    metas.sort(key=order_key)
    for meta in metas:
        if only and not any(meta["name"].startswith(o) for o in only):
            continue
        r = run_one(out, meta)
        print(r["name"], "D", r["D"], "W", r["W"], "unit", r.get("unit"), "stop", r["stop"], "wall", r["wall_s"],
              flush=True)


if __name__ == "__main__":
    main()
