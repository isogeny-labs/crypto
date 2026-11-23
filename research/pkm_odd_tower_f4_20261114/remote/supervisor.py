"""Pod supervisor for EXP-PKM-ODD-001's high-memory rerun (Amendment 4).

Run by the pod as  python3 -u -c '<this file, base64-wrapped>'.  Standard library only.
Every request needs `Authorization: Bearer $MON_TOKEN` (taken from the environment, then
removed from it). Endpoints:
  GET  /health                  state: running, return code, rows per file
  PUT  /upload?name=NAME        store the body as /work/NAME (mode 755); returns its SHA-256
  POST /run                     body {"argv": [...]}: start it in /work (one at a time)
  GET  /file?path=REL           a file under /work
  GET  /list                    files under /work/runs
"""

import hashlib
import hmac
import json
import os
import subprocess
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

TOKEN = os.environ.pop("MON_TOKEN")
WORK = Path("/work")
WORK.mkdir(parents=True, exist_ok=True)
state = {"proc": None, "argv": None, "rc": None}
lock = threading.Lock()


def safe(rel):
    p = (WORK / rel).resolve()
    if not str(p).startswith(str(WORK.resolve()) + os.sep):
        raise ValueError("path outside /work")
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
            rows = {}
            runs = WORK / "runs"
            if runs.exists():
                for f in sorted(runs.glob("*.jsonl")):
                    rows[f.name] = sum(1 for _ in open(f))
            mem = Path("/proc/meminfo").read_text().split("\n")[:3]
            self.reply(200, {"running": running, "rc": rc, "argv": state["argv"], "rows": rows,
                             "meminfo": mem, "nproc": os.cpu_count()})
        elif u.path == "/file":
            try:
                self.reply(200, raw=safe(q["path"][0]).read_bytes(), ctype="application/octet-stream")
            except Exception as e:  # noqa: BLE001
                self.reply(404, {"error": str(e)})
        elif u.path == "/list":
            runs = WORK / "runs"
            self.reply(200, sorted(str(f.relative_to(WORK)) for f in runs.rglob("*") if f.is_file())
                       if runs.exists() else [])
        else:
            self.send_error(404)

    def do_PUT(self):
        if not self.auth():
            return
        u = urlparse(self.path)
        name = parse_qs(u.query)["name"][0]
        n = int(self.headers["Content-Length"])
        data = self.rfile.read(n)
        dst = safe(name)
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_bytes(data)
        dst.chmod(0o755)
        self.reply(200, {"name": name, "bytes": n, "sha256": hashlib.sha256(data).hexdigest()})

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
            log = open(WORK / "run.log", "ab")
            state["proc"] = subprocess.Popen(body["argv"], cwd=WORK, stdout=log, stderr=log)
            state["argv"] = body["argv"]
        self.reply(200, {"started": body["argv"]})


ThreadingHTTPServer(("0.0.0.0", 8888), H).serve_forever()
