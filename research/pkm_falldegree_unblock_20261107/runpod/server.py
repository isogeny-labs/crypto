"""Study runner for one RunPod reserved CPU pod (Amendment 15 draft, PKM fall-degree study).

Runs the frozen entry_r2.sh unchanged. Endpoints (all need header X-Token == $RUNNER_TOKEN):
  PUT  /upload/<name>      store a file under /opt/in/<name> (names from ALLOWED only)
  POST /start              body: JSON {"stages": [...], "sha256": {name: hex}, "rayon": N}
                           verifies every uploaded file's SHA-256, installs them at the frozen image's
                           paths, then runs entry_r2.sh <stages> in the background with an `aws` shim
                           that writes to /results instead of S3
  GET  /status             JSON: state, exit code, progress.txt tail, memory
  GET  /results.tar.gz     tar of /results (runs/, gates/, progress.txt, binary.sha256, host.txt, ...)
Stdlib only. No result is interpreted here.
"""
import hashlib, hmac, io, json, os, subprocess, tarfile, threading, time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

TOKEN = os.environ["RUNNER_TOKEN"].encode()
IN, RES = "/opt/in", os.environ.get("RESULTS_DIR", "/results")  # rp.sh sets /workspace/results: the pod volume (E3.5)
BIN = ["pkm_tower_pilot", "prime_fall_degree_tools", "hky_fp_tower_check", "pfd_grid_zeros", "entry_r2.sh"]
STUDY = ["run.sh", "build-info.json"]
# Amendment R4 (draft): the isogeny-at-P1 job set. The R4 pilot binary, plus `repo.tar`, a `git archive` of the committed
# paths entry_iso.sh needs (unpacked at /opt/repo, so the scripts run from their committed bytes).
ISO_BIN = ["pkm_tower_pilot_r4", "prime_fall_degree_tools", "hky_fp_tower_check"]
ISO_ENTRY = "/opt/repo/research/pkm_falldegree_unblock_20261107/iso/entry_iso.sh"
ISO_OPTIONAL = ["pkm_tower_pilot", "rows.tar"]   # Amendment 20: frozen Kummer pilot and stage B rows for g3-b
# Amendment 31: m = 2 closure cells (entry "m2c"): the gated pilot and grid-zero binaries plus the committed entry_m2c.sh.
M2C_BIN = ["pkm_tower_pilot", "pfd_grid_zeros", "entry_m2c.sh"]
ALLOWED = set(BIN + STUDY + ISO_BIN + ["repo.tar"] + ISO_OPTIONAL + M2C_BIN)
SHIM = """#!/bin/bash
# aws shim: `aws s3 sync|cp SRC s3://local/PATH` -> copy into /results/PATH (entry_r2.sh stays byte-identical)
[ "$1" = s3 ] || exit 0
src=$3; dst=${4#s3://local}; dst=${RESULTS_DIR:-/results}/${dst#/}
case "$2" in
  sync) mkdir -p "$dst" && cp -a "$src/." "$dst/" ;;
  cp) mkdir -p "$(dirname "$dst")" && cp -a "$src" "$dst" ;;
esac
"""
state = {"state": "idle", "exit": None, "started": None, "ended": None, "stages": None}
RUNNER_SHA256 = os.environ.get("RUNNER_SHA256", "")  # set by rp.sh create to sha256(server.py); recorded, E3.8


def cgroup_memory_bytes():
    """The container's memory limit: cgroup v2 memory.max, else v1, else MemTotal (E3.3)."""
    for p in ("/sys/fs/cgroup/memory.max", "/sys/fs/cgroup/memory/memory.limit_in_bytes"):
        try:
            v = open(p).read().strip()
            if v != "max" and int(v) < 1 << 60:
                return int(v)
        except (OSError, ValueError):
            pass
    return int(open("/proc/meminfo").read().split()[1]) * 1024


def cgroup_cpus():
    """CPUs this container may use: cgroup v2 cpu.max quota, else the affinity set (E3.4)."""
    n = len(os.sched_getaffinity(0))
    try:
        q, per = open("/sys/fs/cgroup/cpu.max").read().split()                      # cgroup v2
        if q != "max":
            n = min(n, int(q) // int(per))
    except (OSError, ValueError):
        try:                                                                       # cgroup v1
            q = int(open("/sys/fs/cgroup/cpu/cpu.cfs_quota_us").read())
            per = int(open("/sys/fs/cgroup/cpu/cpu.cfs_period_us").read())
            if q > 0:
                n = min(n, q // per)
        except (OSError, ValueError):
            pass
    return n
os.makedirs(IN, exist_ok=True); os.makedirs(RES, exist_ok=True)

def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""): h.update(b)
    return h.hexdigest()

def oom_kills():
    """oom_kill count from the cgroup (v2), or None if unreadable (E3.3)."""
    for p in ("/sys/fs/cgroup/memory.events", "/sys/fs/cgroup/memory/memory.oom_control"):  # cgroup v2, then v1
        try:
            for line in open(p):
                k, v = line.split()
                if k == "oom_kill":
                    return int(v)
        except (OSError, ValueError):
            continue
    return None


def run(entry, stages, rayon, seconds):
    env = dict(os.environ, PATH="/opt/shim:" + os.environ.get("PATH", ""), ISOLAB_RESULTS_URI="s3://local",
               RAYON_NUM_THREADS=str(rayon), HOME="/work")
    env.pop("RUNNER_TOKEN", None)
    oom0 = oom_kills()
    with open(f"{RES}/entry.stdout", "wb") as o, open(f"{RES}/entry.stderr", "wb") as e:
        proc = subprocess.Popen([entry, *stages], env=env, stdout=o, stderr=e, cwd="/work", start_new_session=True)
        try:
            rc = proc.wait(timeout=seconds)            # E3.2: the job's registered time limit, enforced here
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid, 9)
            proc.wait()
            rc = "deadline"
    oom1 = oom_kills()
    state["oom_kill_delta"] = None if oom0 is None or oom1 is None else oom1 - oom0
    subprocess.call(f"cp -a /work/study/runs /work/study/gates /work/study/progress.txt {RES}/ 2>/dev/null", shell=True)
    state.update(state="done", exit=rc, ended=time.strftime("%FT%TZ", time.gmtime()))
    with open(f"{RES}/runner_state.json", "w") as f: json.dump(state, f)

class H(BaseHTTPRequestHandler):
    def ok(self):
        if hmac.compare_digest(self.headers.get("X-Token", "").encode(), TOKEN): return True
        self.send_response(403); self.end_headers(); return False
    def reply(self, code, obj=None, body=None, ctype="application/json"):
        data = body if body is not None else json.dumps(obj).encode()
        self.send_response(code); self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(data))); self.end_headers(); self.wfile.write(data)
    def do_PUT(self):
        if not self.ok(): return
        # /upload/<name> writes the file; /append/<name> appends one chunk (Amendment 16: the RunPod proxy rejects
        # request bodies of a few MB, so the client sends 2 MB chunks). The /start SHA-256 check is unchanged.
        mode = "ab" if self.path.startswith("/append/") else "wb"
        name = self.path.removeprefix("/append/" if mode == "ab" else "/upload/")
        if name not in ALLOWED or state["state"] != "idle": return self.reply(400, {"error": "name or state"})
        n = int(self.headers["Content-Length"])
        with open(f"{IN}/{name}", mode) as f:
            while n > 0:
                b = self.rfile.read(min(n, 1 << 20)); f.write(b); n -= len(b)
        self.reply(200, {"name": name, "sha256": sha(f"{IN}/{name}")})
    def do_POST(self):
        if not self.ok(): return
        if self.path != "/start" or state["state"] != "idle": return self.reply(400, {"error": "path or state"})
        req = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        got = {n: sha(f"{IN}/{n}") for n in ALLOWED if os.path.exists(f"{IN}/{n}")}
        iso = req.get("entry") == "iso"
        m2c = req.get("entry") == "m2c"
        # E3.2/E3.3/E3.4: refuse before anything runs.
        seconds = int(req.get("seconds", 0))
        cost_per_hr, max_usd = float(req.get("cost_per_hr", 0)), float(req.get("max_usd", 40))
        if seconds <= 0 or cost_per_hr <= 0:
            return self.reply(409, {"error": "seconds and cost_per_hr are required"})
        idle = int(req.get("idle_seconds", 14400))  # F-R3-2: billed until the next check-in deletes the pod (up to ~4 h)
        if cost_per_hr * (seconds + idle) / 3600 > max_usd:
            return self.reply(409, {"error": "cost at the time limit exceeds max_usd",
                                    "projected_usd": cost_per_hr * (seconds + idle) / 3600, "max_usd": max_usd})
        if any(os.scandir(RES)):                    # F-R3-4: a relaunch uses a new pod; never mix into old results
            return self.reply(409, {"error": "RESULTS_DIR not empty"})
        mem = cgroup_memory_bytes()
        floor = int(req.get("memory_floor_gib", 56)) * 2**30
        if mem < floor:
            return self.reply(409, {"error": "memory below floor", "memory_bytes": mem, "floor_bytes": floor})
        if oom_kills() is None:                                  # E3.3: OOM must be classifiable
            return self.reply(409, {"error": "cgroup memory.events (oom_kill) unreadable"})
        cpus = cgroup_cpus()
        if cpus < 12:
            return self.reply(409, {"error": "fewer than 12 vCPU", "cpus": cpus})
        required = set(ISO_BIN) | {"repo.tar"} if iso else set(M2C_BIN) if m2c else set(BIN) | {"run.sh"}  # build-info.json: recorded only
        bad = {n: [req["sha256"].get(n), got.get(n)] for n in required if not got.get(n) or req["sha256"].get(n) != got.get(n)}
        if iso:   # optional iso uploads must match the hash the request names for them
            bad.update({n: [req["sha256"].get(n), got.get(n)] for n in ISO_OPTIONAL
                        if n in req["sha256"] and req["sha256"][n] != got.get(n)})
        if bad: return self.reply(409, {"error": "sha256 mismatch", "detail": bad})
        os.makedirs("/opt/study", exist_ok=True); os.makedirs("/opt/shim", exist_ok=True); os.makedirs("/work", exist_ok=True)
        if iso:
            for n in ISO_BIN: subprocess.check_call(["install", "-m755", f"{IN}/{n}", f"/usr/local/bin/{n}"])
            if "pkm_tower_pilot" in req["sha256"]:
                subprocess.check_call(["install", "-m755", f"{IN}/pkm_tower_pilot", "/usr/local/bin/pkm_tower_pilot"])
            if "rows.tar" in req["sha256"]:
                os.makedirs("/work/rows_in", exist_ok=True)
                subprocess.check_call(["tar", "-xf", f"{IN}/rows.tar", "-C", "/work/rows_in"])
            os.makedirs("/opt/repo", exist_ok=True)
            subprocess.check_call(["tar", "-xf", f"{IN}/repo.tar", "-C", "/opt/repo"])
            # R2-E5: every bundled file must match the committed pins sent with the request (iso/BUNDLE.sha256 at `commit`).
            pins = req.get("bundle_sha256") or {}
            if not pins or not req.get("commit"):
                return self.reply(409, {"error": "iso mode needs commit and bundle_sha256"})
            badb = {n: [h, sha(f"/opt/repo/{n}") if os.path.exists(f"/opt/repo/{n}") else None]
                    for n, h in pins.items() if not os.path.exists(f"/opt/repo/{n}") or sha(f"/opt/repo/{n}") != h}
            if badb:
                return self.reply(409, {"error": "bundle sha256 mismatch", "detail": badb})
        elif m2c:
            for n in M2C_BIN: subprocess.check_call(["install", "-m755", f"{IN}/{n}", f"/usr/local/bin/{n}"])
        else:
            for n in BIN: subprocess.check_call(["install", "-m755", f"{IN}/{n}", f"/usr/local/bin/{n}"])
            for n in STUDY:
                if os.path.exists(f"{IN}/{n}"): subprocess.check_call(["install", "-m644", f"{IN}/{n}", f"/opt/study/{n}"])
        with open("/opt/shim/aws", "w") as f: f.write(SHIM)
        os.chmod("/opt/shim/aws", 0o755)
        with open(f"{RES}/runner_request.json", "w") as f: json.dump({**req, "verified_sha256": got}, f, indent=1)
        state.update(state="running", started=time.strftime("%FT%TZ", time.gmtime()), stages=req["stages"])
        entry = ISO_ENTRY if iso else "/usr/local/bin/entry_m2c.sh" if m2c else "/usr/local/bin/entry_r2.sh"
        state.update(seconds=seconds, cost_per_hr=cost_per_hr, max_usd=max_usd, memory_bytes=mem, cpus=cpus,
                     runner_sha256=RUNNER_SHA256, commit=req.get("commit"))
        threading.Thread(target=run, args=(entry, req["stages"], 12, seconds), daemon=True).start()  # rayon pinned at 12
        self.reply(200, state)
    def do_GET(self):
        if not self.ok(): return
        if self.path == "/status":
            tail = ""
            for p in ("/work/study/progress.txt",):
                if os.path.exists(p): tail = open(p).read()[-4000:]
            mem = open("/proc/meminfo").read().splitlines()[:3]
            return self.reply(200, {**state, "progress": tail, "meminfo": mem, "nproc": os.cpu_count()})
        if self.path in ("/results.tar.gz", "/manifest"):
            subprocess.call("cp -a /work/study/runs /work/study/gates /work/study/progress.txt /work/study/binary.sha256 "
                            f"/work/study/host.txt {RES}/ 2>/dev/null", shell=True)
            man = {}
            for root, _, files in os.walk(RES):
                for n in files:
                    p = os.path.join(root, n)
                    man[os.path.relpath(p, RES)] = sha(p)
            if self.path == "/manifest":                    # E3.5: the pod's own hashes, checked against the fetch
                return self.reply(200, {"files": man, "state": state})
            tmp = "/tmp/results.tar.gz"                      # on disk, not in memory (F10)
            with tarfile.open(tmp, mode="w:gz") as t: t.add(RES, arcname="results")
            data = open(tmp, "rb").read()
            return self.reply(200, body=data, ctype="application/gzip")
        self.reply(404, {"error": "path"})
    def log_message(self, *a): pass

ThreadingHTTPServer(("0.0.0.0", 8000), H).serve_forever()
