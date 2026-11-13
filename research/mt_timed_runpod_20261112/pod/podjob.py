#!/usr/bin/env python3
"""One fixed, hash-pinned job for one ephemeral RunPod CPU pod (port 8000, reached through the RunPod HTTPS proxy).
Standard library only. There is no command endpoint: what runs is fixed when the pod is created.

Environment (set at pod creation):
  TOKEN           32+ hex characters; every route starts with /<TOKEN>/
  PAYLOAD_SHA256  sha256 of the only payload accepted (a .tar.gz built from a committed study directory)
  JOB_ARGV        JSON list: the single command run in /work/payload after extraction (fixed at creation)

Routes:
  PUT /<TOKEN>/payload        accepted once, only if its sha256 equals PAYLOAD_SHA256; then extracted to
                              /work/payload and JOB_ARGV started there (stdout+stderr to /work/payload/job.log)
  GET /<TOKEN>/status         {"payload": bool, "running": bool, "exit": code or null}
  GET /<TOKEN>/result/<path>  read-only file under /work/payload/results/ or job.log"""
import hashlib, http.server, json, os, subprocess, tarfile, threading

TOKEN = os.environ["TOKEN"]
PIN = os.environ["PAYLOAD_SHA256"]
ARGV = json.loads(os.environ["JOB_ARGV"])
ROOT = "/work/payload"
assert len(TOKEN) >= 32 and len(PIN) == 64 and isinstance(ARGV, list) and ARGV
STATE = {"payload": False, "proc": None}
LOCK = threading.Lock()


def under(base, rel):
    p = os.path.realpath(os.path.join(base, rel))
    return p if p == base or p.startswith(base + "/") else None


class H(http.server.BaseHTTPRequestHandler):
    def _route(self):
        parts = self.path.split("?", 1)[0].split("/", 3)
        if len(parts) < 3 or parts[1] != TOKEN:
            self.send_error(404)
            return None
        return parts[2], (parts[3] if len(parts) > 3 else "")

    def _json(self, o, code=200):
        b = json.dumps(o).encode()
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(b)))
        self.end_headers()
        self.wfile.write(b)

    def do_PUT(self):
        r = self._route()
        if not r:
            return
        if r[0] != "payload":
            return self.send_error(404)
        with LOCK:
            if STATE["payload"]:
                return self._json({"error": "payload already accepted"}, 409)
            n = int(self.headers.get("Content-Length", "0"))
            data = b""
            while len(data) < n:
                chunk = self.rfile.read(min(n - len(data), 1 << 20))
                if not chunk:
                    break
                data += chunk
            if hashlib.sha256(data).hexdigest() != PIN:
                return self._json({"error": "sha256 mismatch"}, 403)
            os.makedirs(ROOT, exist_ok=True)
            tmp = "/work/payload.tar.gz"
            open(tmp, "wb").write(data)
            with tarfile.open(tmp) as t:
                for m in t.getmembers():
                    if not under(ROOT, m.name) or m.issym() or m.islnk():
                        return self._json({"error": "unsafe member " + m.name}, 400)
                t.extractall(ROOT)
            STATE["payload"] = True
            log = open(os.path.join(ROOT, "job.log"), "ab")
            STATE["proc"] = subprocess.Popen(ARGV, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
        self._json({"accepted": True, "argv": ARGV})

    def do_GET(self):
        r = self._route()
        if not r:
            return
        kind, rel = r
        if kind == "status":
            p = STATE["proc"]
            return self._json({"payload": STATE["payload"], "running": bool(p and p.poll() is None),
                               "exit": (p.poll() if p else None)})
        if kind == "result":
            base = ROOT if rel == "job.log" else os.path.join(ROOT, "results")
            p = under(base, rel if rel != "job.log" else "job.log")
            if not p or not os.path.isfile(p):
                return self.send_error(404)
            b = open(p, "rb").read()
            self.send_response(200)
            self.send_header("Content-Length", str(len(b)))
            self.end_headers()
            self.wfile.write(b)
            return
        self.send_error(404)

    def log_message(self, *a):
        pass


http.server.ThreadingHTTPServer(("0.0.0.0", 8000), H).serve_forever()
