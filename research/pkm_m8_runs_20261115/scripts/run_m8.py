#!/usr/bin/env python3
"""m8-runs runner (PRESET sections 3-4). Usage: run_m8.py OUTDIR

Caps: RLIMIT_AS 6e9; T = 1 if nvars <= 16 else 2; wall 300 s if nvars <= 16 else 1200 s.
Order: (nvars, m, arm, stage, k, idx). Ladder stop rule: r0 censored -> skip r1 and deeper rungs.
Global budget 3.0 h of msolve wall. A run with a res.json is not repeated.
"""
import json
import os
import resource
import signal
import subprocess
import sys
import time

sys.path.insert(0, "/home/user/crypto/research/prime_field_ic_closure_20261112/finish/pkm-msolve/scripts")
import msparse as MP  # noqa: E402

AS_CAP = 6_000_000_000
BUDGET_S = 3.0 * 3600


def setlimits():
    resource.setrlimit(resource.RLIMIT_AS, (AS_CAP, AS_CAP))
    os.setpgrp()


def now():
    return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())


def run_one(out, meta):
    name = meta["name"]
    rd = os.path.join(out, "runs")
    os.makedirs(rd, exist_ok=True)
    resf = os.path.join(rd, name + ".res.json")
    if os.path.exists(resf):
        return json.load(open(resf))
    nv = meta["nvars"]
    T = 1 if nv <= 16 else 2
    wall = 300 if nv <= 16 else 1200
    cmd = ["msolve", "-t", str(T), "-v", "2", "-g", "2", "-f", os.path.join(out, "systems", name + ".ms"),
           "-o", os.path.join(rd, name + ".out")]
    with open(os.path.join(out, "progress.txt"), "a") as pf:
        pf.write(f"{now()} start {name} nvars={nv} T={T} wall={wall} as={AS_CAP}\n")
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
    proc.returncode = 0
    dt = time.time() - t0
    logf.close()
    code = os.waitstatus_to_exitcode(status)
    rounds = MP.parse_trace(os.path.join(rd, name + ".log"))
    if stop is None and code != 0:
        stop = "crash_or_mem"
    res = dict(name=name, cmd=" ".join(cmd), threads=T, wall_cap=wall, as_cap=AS_CAP, exit=code, stop=stop,
               wall_s=round(dt, 3), ru_maxrss_kB=ru.ru_maxrss, ru_utime=round(ru.ru_utime, 2),
               n_rounds=len(rounds), D=max((r["deg"] for r in rounds), default=None),
               W=max((r["cols"] for r in rounds), default=None),
               rows_at_W=(max(rounds, key=lambda r: r["cols"])["rows"] if rounds else None),
               last_deg=(rounds[-1]["deg"] if rounds else None))
    outf = os.path.join(rd, name + ".out")
    if code == 0 and os.path.exists(outf):
        body = "\n".join(ln for ln in open(outf).read().split("\n") if not ln.startswith("#")).strip()
        res["unit"] = body.replace(" ", "").startswith("[1]")
        res["gb_len"] = (body.count(",\n") + 1) if body.startswith("[") else None
    json.dump(res, open(resf, "w"))
    with open(os.path.join(out, "progress.txt"), "a") as pf:
        pf.write(f"{now()} end {name} exit={code} stop={stop} D={res['D']} W={res['W']} wall={dt:.1f}s "
                 f"rss={ru.ru_maxrss}kB unit={res.get('unit')}\n")
    return res


def key(m):
    return (m["nvars"], m["m"], m["arm"], m.get("stage", ""), m.get("k", 0), m["kind"], m["idx"])


def main():
    out = sys.argv[1]
    metas = sorted((json.loads(l) for l in open(os.path.join(out, "index.jsonl"))), key=key)
    stopped = set()        # ladders (m) stopped by the stop rule
    skipped = []
    used = 0.0
    for meta in metas:
        name = meta["name"]
        lad = meta.get("ladder")
        if lad is not None and lad in stopped:
            skipped.append((name, "ladder_stopped"))
            continue
        if used >= BUDGET_S:
            skipped.append((name, "budget"))
            continue
        r = run_one(out, meta)
        used += r["wall_s"]
        print(now(), name, "D", r["D"], "W", r["W"], "unit", r.get("unit"), "stop", r["stop"], "wall", r["wall_s"],
              "used", round(used), flush=True)
        if lad is not None and r["stop"] is not None and meta["idx"] == 0:
            stopped.add(lad)
    json.dump(dict(skipped=skipped, used_s=used, stopped_ladders=sorted(stopped)),
              open(os.path.join(out, "skipped.json"), "w"), indent=1)
    print("DONE used", round(used), "skipped", len(skipped), flush=True)


if __name__ == "__main__":
    main()
