#!/usr/bin/env python3
"""Run validation/out/jobs.txt sequentially (single-threaded), one process at a time.

Binaries are hashed before every job; a hash that differs from the one recorded at the first job
stops the queue.  Outputs: validation/out/replay/<tag>.jsonl (+ .log, + _core.jsonl for exports);
validation/out/replay/jobs.log (one JSON line per job: start, end, exit code, binary sha256).
A job whose output already ends in a 'done' row (or, for valpo, has its run rows) is skipped.
"""
import hashlib, json, os, subprocess, sys, time

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(V, "out", "replay")
SP = "/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad"
BIN = {"po": f"{SP}/bin/genus_ladder.po", "blpt": f"{SP}/bin/genus_ladder.blpt",
       "valpo": f"{SP}/val4po/bin/val_po"}
EXPECT = {"po": "493ee2942473b8708a0b52f40ab10095c4da83ab0f0755ed3e5dbc02771b6921",
          "valpo": "81bbb7452728a377bbe809c2616fa72cf4e61fd3843be0b4ac9c38dff4b19c1d",
          "blpt": "e210549a18937be8bce65521f40093e1dbac765174ed481901f7d848460342fd"}


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def done(tag, kind):
    f = os.path.join(OUT, tag + ".jsonl")
    if not os.path.exists(f) or os.path.getsize(f) == 0:
        return False
    rows = [json.loads(l) for l in open(f) if l.strip()]
    if kind == "valpo":
        return sum(r.get("kind") == "run" for r in rows) == 2
    return any(r.get("kind") == "done" for r in rows)


only = sys.argv[1:]  # optional tag filter
log = open(os.path.join(OUT, "jobs.log"), "a")
for line in open(os.path.join(V, "out", os.environ.get("JOBS_FILE", "jobs.txt"))):
    j = json.loads(line)
    if only and not any(o in j["tag"] for o in only):
        continue
    if done(j["tag"], j["kind"]):
        continue
    b = BIN[j["kind"]]
    h = sha(b)
    if h != EXPECT[j["kind"]]:
        log.write(json.dumps({"tag": j["tag"], "error": "binary hash changed", "sha256": h}) + "\n")
        sys.exit(f"binary hash changed: {b}")
    out = os.path.join(OUT, j["tag"] + ".jsonl")
    errf = open(os.path.join(OUT, j["tag"] + ".log"), "w")
    env = dict(os.environ)
    t0 = time.time()
    if j["kind"] == "valpo":
        p = subprocess.run([b], input=j["stdin"] + "\n", stdout=open(out, "w"), stderr=errf, text=True, env=env)
    else:
        if j["kind"] == "blpt":
            core = os.path.join(OUT, j["tag"] + "_core.jsonl")
            if os.path.exists(core):
                os.remove(core)
            env["GENUS_LADDER_EXPORT_CORE"] = core
        p = subprocess.run([b] + j["args"] + ["--out", out], stdout=errf, stderr=errf, env=env)
    log.write(json.dumps({"tag": j["tag"], "kind": j["kind"], "binary": b, "sha256": h,
                          "start_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(t0)),
                          "seconds": round(time.time() - t0, 1), "exit": p.returncode}) + "\n")
    log.flush()
