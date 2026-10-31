#!/usr/bin/env python3
"""E32 (note §9.34): the handover ladder.  Restart-safe runner.

Runs, in ascending p, three curves per size (seeds 1-3; a seed without a
curve is replaced by the next unused seed), each curve: `e32_ladder ic`
(arm B, solver chain, Karatsuba) then `e32_ladder walk` (E27 walk (iii)).
Only the pinned binaries in E32BIN are run.  At most 3 cores: taskset -c 1-3,
RAYON_NUM_THREADS=4 (fixes the pipeline's target stream, §9.19).

Budget (frozen in §9.34): 14 h wall for all runs; a curve's ic run is stopped
after 4 h and recorded as censored; a size is not started if the budget left is
under 3x the previous size's median curve time x p_next/p_prev; p = 4337 only if
>= 3 h remain and >= 8 GB of memory are available after p = 3089.
"""
import json
import os
import statistics
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OUT = os.path.join(ROOT, "experiments/36_handover_ladder")
E32BIN = "/tmp/claude-0/-home-user/1f75bcdd-3a63-5794-929b-19635cbd5274/scratchpad/e32bin"
SIZES = [101, 149, 197, 269, 397, 557, 773, 1093, 1549, 2213, 3089]
OPTIONAL = 4337
E25_SIZES = {101, 149, 197, 269, 397}
BUDGET_S = 14 * 3600
CURVE_CAP_S = 4 * 3600
STATE = os.path.join(OUT, "ladder_state.json")
LOG = os.path.join(OUT, "run.log")


def log(msg):
    line = f"{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())} {msg}"
    print(line, flush=True)
    with open(LOG, "a") as f:
        f.write(line + "\n")


def load_state():
    if os.path.exists(STATE):
        return json.load(open(STATE))
    return {"curves": {}, "spent_s": 0.0, "events": []}


def save_state(st):
    tmp = STATE + ".part"
    json.dump(st, open(tmp, "w"), indent=1)
    os.replace(tmp, STATE)


def mem_available_gb():
    for line in open("/proc/meminfo"):
        if line.startswith("MemAvailable:"):
            return int(line.split()[1]) / 1024 / 1024
    return 0.0


def run(cmd, out_path, timeout):
    env = dict(os.environ, RAYON_NUM_THREADS="4")
    full = ["taskset", "-c", "1-3"] + cmd + ["--json", out_path + ".part"]
    log("run " + " ".join(full))
    t0 = time.time()
    try:
        with open(LOG, "a") as lf:
            r = subprocess.run(full, env=env, cwd=ROOT, stdout=lf, stderr=subprocess.STDOUT,
                               timeout=timeout)
        ok = r.returncode == 0
        status = "ok" if ok else f"exit {r.returncode}"
    except subprocess.TimeoutExpired:
        ok, status = False, "censored (timeout)"
    dt = time.time() - t0
    if ok:
        os.replace(out_path + ".part", out_path)
    return status, dt


def do_curve(st, p, seed):
    key = f"p{p}s{seed}"
    rec = st["curves"].get(key, {})
    ic = os.path.join(OUT, f"ic_{key}.json")
    walk = os.path.join(OUT, f"walk_{key}.json")
    rho_runs = "16" if (p in E25_SIZES and seed <= 2) else "1"
    if rec.get("ic") not in (None, "ok"):
        return rec["ic"]
    if not os.path.exists(ic) and rec.get("ic") is None:
        status, dt = run([f"{E32BIN}/e32_ladder", "ic", "--p", str(p), "--seed", str(seed),
                          "--rho-runs", rho_runs, "--max-residuals", "40000000"], ic, CURVE_CAP_S)
        st["spent_s"] += dt
        rec.update({"ic": status, "ic_s": dt})
        st["curves"][key] = rec
        save_state(st)
        if status != "ok":
            tail = open(LOG).read()[-3000:]
            if "no full-2-torsion" in tail or "E25's curve" in tail:
                status = "no curve: " + status
                rec["ic"] = status
                save_state(st)
            return status
    if not os.path.exists(walk) and rec.get("walk") is None:
        status, dt = run([f"{E32BIN}/e32_ladder", "walk", "--p", str(p), "--seed", str(seed),
                          "--walks", "16", "--pool", "16"], walk, CURVE_CAP_S)
        st["spent_s"] += dt
        rec.update({"walk": status, "walk_s": dt})
        st["curves"][key] = rec
        save_state(st)
        if status != "ok":
            return status
    return "ok"


def curve_seconds(rec):
    return rec.get("ic_s", 0.0) + rec.get("walk_s", 0.0)


def main():
    st = load_state()
    prev = None  # (p, median curve seconds)
    for p in SIZES + [OPTIONAL]:
        left = BUDGET_S - st["spent_s"]
        if p == OPTIONAL:
            if left < 3 * 3600 or mem_available_gb() < 8:
                log(f"stop: p={p} not run (budget left {left/3600:.2f} h, mem {mem_available_gb():.1f} GB)")
                st["events"].append({"stop_before": p, "left_s": left, "mem_gb": mem_available_gb()})
                save_state(st)
                break
        elif prev is not None:
            need = 3 * prev[1] * p / prev[0]
            done = all(st["curves"].get(f"p{p}s{s}", {}).get("walk") == "ok" for s in (1, 2, 3))
            if not done and left < need:
                log(f"stop: p={p} not started (budget left {left:.0f} s < projected {need:.0f} s)")
                st["events"].append({"stop_before": p, "left_s": left, "projected_s": need})
                save_state(st)
                break
        got, seed, times = 0, 1, []
        while got < 3:
            status = do_curve(st, p, seed)
            rec = st["curves"].get(f"p{p}s{seed}", {})
            if status == "ok":
                got += 1
                times.append(curve_seconds(rec))
            elif status.startswith("no curve"):
                log(f"p={p} seed={seed}: no curve; replaced by the next seed")
            else:
                log(f"p={p} seed={seed}: {status}")
                # a failure other than a missing curve: check the log; a censored curve counts
                # as one of the three (it is reported, not replaced).
                if "censored" in status:
                    got += 1
                else:
                    log("unexpected failure; stopping for inspection")
                    sys.exit(2)
            seed += 1
            if seed > 12:
                log(f"p={p}: fewer than 3 curves in seeds 1-12")
                break
        if times:
            prev = (p, statistics.median(times))
        log(f"size p={p} done; spent {st['spent_s']/3600:.2f} h")
    log("finished")


if __name__ == "__main__":
    main()
