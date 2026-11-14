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
IN, RES = "/opt/in", "/results"
BIN = ["pkm_tower_pilot", "prime_fall_degree_tools", "hky_fp_tower_check", "pfd_grid_zeros", "entry_r2.sh"]
STUDY = ["run.sh", "build-info.json"]
ALLOWED = set(BIN + STUDY)
SHIM = """#!/bin/bash
# aws shim: `aws s3 sync|cp SRC s3://local/PATH` -> copy into /results/PATH (entry_r2.sh stays byte-identical)
[ "$1" = s3 ] || exit 0
src=$3; dst=${4#s3://local}; dst=/results/${dst#/}
case "$2" in
  sync) mkdir -p "$dst" && cp -a "$src/." "$dst/" ;;
  cp) mkdir -p "$(dirname "$dst")" && cp -a "$src" "$dst" ;;
esac
"""
state = {"state": "idle", "exit": None, "started": None, "ended": None, "stages": None}
os.makedirs(IN, exist_ok=True); os.makedirs(RES, exist_ok=True)

def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""): h.update(b)
    return h.hexdigest()

def run(stages, rayon):
    env = dict(os.environ, PATH="/opt/shim:" + os.environ.get("PATH", ""), ISOLAB_RESULTS_URI="s3://local",
               RAYON_NUM_THREADS=str(rayon), HOME="/work")
    env.pop("RUNNER_TOKEN", None)
    with open(f"{RES}/entry.stdout", "wb") as o, open(f"{RES}/entry.stderr", "wb") as e:
        rc = subprocess.call(["/usr/local/bin/entry_r2.sh", *stages], env=env, stdout=o, stderr=e, cwd="/work")
    subprocess.call("cp -a /work/study/runs /work/study/gates /work/study/progress.txt /results/ 2>/dev/null", shell=True)
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
        name = self.path.removeprefix("/upload/")
        if name not in ALLOWED or state["state"] != "idle": return self.reply(400, {"error": "name or state"})
        n = int(self.headers["Content-Length"])
        with open(f"{IN}/{name}", "wb") as f:
            while n > 0:
                b = self.rfile.read(min(n, 1 << 20)); f.write(b); n -= len(b)
        self.reply(200, {"name": name, "sha256": sha(f"{IN}/{name}")})
    def do_POST(self):
        if not self.ok(): return
        if self.path != "/start" or state["state"] != "idle": return self.reply(400, {"error": "path or state"})
        req = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        got = {n: sha(f"{IN}/{n}") for n in ALLOWED if os.path.exists(f"{IN}/{n}")}
        required = set(BIN) | {"run.sh"}   # build-info.json is descriptive only; its hash is recorded, not checked
        bad = {n: [req["sha256"].get(n), got.get(n)] for n in required if not got.get(n) or req["sha256"].get(n) != got.get(n)}
        if bad: return self.reply(409, {"error": "sha256 mismatch", "detail": bad})
        os.makedirs("/opt/study", exist_ok=True); os.makedirs("/opt/shim", exist_ok=True); os.makedirs("/work", exist_ok=True)
        for n in BIN: subprocess.check_call(["install", "-m755", f"{IN}/{n}", f"/usr/local/bin/{n}"])
        for n in STUDY:
            if os.path.exists(f"{IN}/{n}"): subprocess.check_call(["install", "-m644", f"{IN}/{n}", f"/opt/study/{n}"])
        with open("/opt/shim/aws", "w") as f: f.write(SHIM)
        os.chmod("/opt/shim/aws", 0o755)
        with open(f"{RES}/runner_request.json", "w") as f: json.dump({**req, "verified_sha256": got}, f, indent=1)
        state.update(state="running", started=time.strftime("%FT%TZ", time.gmtime()), stages=req["stages"])
        threading.Thread(target=run, args=(req["stages"], int(req.get("rayon", os.cpu_count()))), daemon=True).start()
        self.reply(200, state)
    def do_GET(self):
        if not self.ok(): return
        if self.path == "/status":
            tail = ""
            for p in ("/work/study/progress.txt",):
                if os.path.exists(p): tail = open(p).read()[-4000:]
            mem = open("/proc/meminfo").read().splitlines()[:3]
            return self.reply(200, {**state, "progress": tail, "meminfo": mem, "nproc": os.cpu_count()})
        if self.path == "/results.tar.gz":
            subprocess.call("cp -a /work/study/runs /work/study/gates /work/study/progress.txt /results/ 2>/dev/null", shell=True)
            buf = io.BytesIO()
            with tarfile.open(fileobj=buf, mode="w:gz") as t: t.add(RES, arcname="results")
            return self.reply(200, body=buf.getvalue(), ctype="application/gzip")
        self.reply(404, {"error": "path"})
    def log_message(self, *a): pass

ThreadingHTTPServer(("0.0.0.0", 8000), H).serve_forever()
