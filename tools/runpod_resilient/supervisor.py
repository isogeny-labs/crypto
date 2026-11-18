"""Pod supervisor that survives pod loss: its state lives on a RunPod network volume.

Run by the pod as  python3 -u -c '<this file, base64-wrapped>'  (standard library only).
The work directory is $WORK_DIR (default /workspace/run), on a network volume that outlives
the pod. Every request needs `Authorization: Bearer $MON_TOKEN` (read, then removed from the
environment).

Resilience:
  * POST /run writes its argv to WORK/autorun.json. When a replacement pod boots on the same
    volume and no file named COMPLETE exists anywhere under WORK/runs, the supervisor starts that argv again at once,
    so a job that resumes from its own output files continues without any client action.
  * WORK/boots.log gets one line per boot (time, pod id); WORK/heartbeat.json is rewritten every
    60 s, so the time a lost pod was last alive is known.

Endpoints:
  GET  /health                  running, return code, argv, boots, last heartbeat, meminfo
  PUT  /upload?name=NAME        store the body as WORK/NAME (mode 755); returns its SHA-256
  POST /run                     body {"argv": [...]}: start it in WORK (one at a time), autorun it
  GET  /file?path=REL           a file under WORK
  GET  /list                    [[path, bytes], ...] for files under WORK/runs
"""

import hashlib
import hmac
import json
import os
import subprocess
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

TOKEN = os.environ.pop("MON_TOKEN")
WORK = Path(os.environ.get("WORK_DIR", "/workspace/run"))
WORK.mkdir(parents=True, exist_ok=True)
POD = os.environ.get("RUNPOD_POD_ID", "unknown")
state = {"proc": None, "argv": None}
lock = threading.Lock()


def now():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def start(argv):
    log = open(WORK / "run.log", "ab")
    log.write(f"=== start {now()} pod {POD}: {argv}\n".encode())
    log.flush()
    state["proc"] = subprocess.Popen(argv, cwd=WORK, stdout=log, stderr=log)
    state["argv"] = argv


def heartbeat():
    while True:
        tmp = WORK / "heartbeat.json.tmp"
        tmp.write_text(json.dumps({"t": now(), "pod": POD}))
        tmp.replace(WORK / "heartbeat.json")
        time.sleep(60)


with open(WORK / "boots.log", "a") as f:
    f.write(f"{now()} {POD}\n")
auto = WORK / "autorun.json"
runs = WORK / "runs"
if auto.exists() and not (runs.exists() and any(runs.rglob("COMPLETE"))):
    start(json.loads(auto.read_text())["argv"])
threading.Thread(target=heartbeat, daemon=True).start()


def safe(rel):
    p = (WORK / rel).resolve()
    if not str(p).startswith(str(WORK.resolve()) + os.sep):
        raise ValueError("path outside work dir")
    return p


class H(BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def auth(self):
        if not hmac.compare_digest(self.headers.get("Authorization", ""), "Bearer " + TOKEN):
            self.send_error(401)
            return False
        return True

    def reply(self, code, obj=None, raw=None, ctype="application/json"):
        body = raw if raw is not None else json.dumps(obj).encode()
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        if not self.auth():
            return
        u = urlparse(self.path)
        q = parse_qs(u.query)
        if u.path == "/health":
            with lock:
                p = state["proc"]
                running = p is not None and p.poll() is None
                rc = None if p is None or running else p.returncode
            hb = WORK / "heartbeat.json"
            self.reply(200, {"running": running, "rc": rc, "argv": state["argv"], "pod": POD,
                             "boots": (WORK / "boots.log").read_text().splitlines(),
                             "heartbeat": json.loads(hb.read_text()) if hb.exists() else None,
                             "meminfo": Path("/proc/meminfo").read_text().split("\n")[:3],
                             "nproc": os.cpu_count()})
        elif u.path == "/file":
            try:
                self.reply(200, raw=safe(q["path"][0]).read_bytes(), ctype="application/octet-stream")
            except Exception as e:  # noqa: BLE001
                self.reply(404, {"error": str(e)})
        elif u.path == "/list":
            runs = WORK / "runs"
            self.reply(200, sorted([str(f.relative_to(WORK)), f.stat().st_size]
                                   for f in runs.rglob("*") if f.is_file()) if runs.exists() else [])
        else:
            self.send_error(404)

    def do_PUT(self):
        if not self.auth():
            return
        name = parse_qs(urlparse(self.path).query)["name"][0]
        data = self.rfile.read(int(self.headers["Content-Length"]))
        dst = safe(name)
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_bytes(data)
        dst.chmod(0o755)
        self.reply(200, {"name": name, "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()})

    def do_POST(self):
        if not self.auth():
            return
        if urlparse(self.path).path != "/run":
            self.send_error(404)
            return
        body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        with lock:
            if state["proc"] is not None and state["proc"].poll() is None:
                self.reply(409, {"error": "already running"})
                return
            auto.write_text(json.dumps({"argv": body["argv"], "set": now()}))
            start(body["argv"])
        self.reply(200, {"started": body["argv"]})


ThreadingHTTPServer(("0.0.0.0", 8888), H).serve_forever()
