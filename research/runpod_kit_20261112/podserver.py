#!/usr/bin/env python3
"""Minimal token-guarded file/exec server for a RunPod CPU pod (port 8000, behind the RunPod HTTPS proxy).
  PUT  /<TOKEN>/f/<path>        store body at /work/<path> (parents created; +x if name ends in .sh or has no dot)
  GET  /<TOKEN>/f/<path>        fetch /work/<path>
  POST /<TOKEN>/exec            body = shell command; runs detached in /work; returns {"id": n}; log /work/exec_<n>.log, exit code /work/exec_<n>.rc
  GET  /<TOKEN>/ls              JSON listing of /work (relative paths, sizes)
Token from env POD_TOKEN. Anything else: 404."""
import http.server, json, os, subprocess, threading
TOKEN = os.environ['POD_TOKEN']; ROOT = '/work'; os.makedirs(ROOT, exist_ok=True); lock = threading.Lock(); n = [0]
def safe(p):
    full = os.path.realpath(os.path.join(ROOT, p))
    if not full.startswith(ROOT + '/'): raise ValueError(p)
    return full
class H(http.server.BaseHTTPRequestHandler):
    def _parts(self):
        parts = self.path.lstrip('/').split('/', 2)
        return parts if parts and parts[0] == TOKEN else None
    def _send(self, code, body=b'', ctype='application/octet-stream'):
        self.send_response(code); self.send_header('Content-Type', ctype); self.send_header('Content-Length', str(len(body))); self.end_headers(); self.wfile.write(body)
    def do_PUT(self):
        p = self._parts()
        if not p or len(p) < 3 or p[1] != 'f': return self._send(404)
        f = safe(p[2]); os.makedirs(os.path.dirname(f), exist_ok=True)
        ln = int(self.headers.get('Content-Length', 0)); left = ln
        with open(f, 'wb') as o:
            while left > 0:
                b = self.rfile.read(min(left, 1 << 20)); o.write(b); left -= len(b)
        base = os.path.basename(f)
        if base.endswith('.sh') or '.' not in base: os.chmod(f, 0o755)
        self._send(200, json.dumps({'stored': p[2], 'bytes': ln}).encode(), 'application/json')
    def do_GET(self):
        p = self._parts()
        if not p: return self._send(404)
        if len(p) >= 2 and p[1] == 'ls':
            out = []
            for d, _, fs in os.walk(ROOT):
                for x in fs:
                    fp = os.path.join(d, x); out.append([os.path.relpath(fp, ROOT), os.path.getsize(fp)])
            return self._send(200, json.dumps(sorted(out)).encode(), 'application/json')
        if len(p) < 3 or p[1] != 'f': return self._send(404)
        try:
            with open(safe(p[2]), 'rb') as f: return self._send(200, f.read())
        except (FileNotFoundError, ValueError): return self._send(404)
    def do_POST(self):
        p = self._parts()
        if not p or len(p) < 2 or p[1] != 'exec': return self._send(404)
        cmd = self.rfile.read(int(self.headers.get('Content-Length', 0))).decode()
        with lock: n[0] += 1; i = n[0]
        subprocess.Popen(['bash', '-c', f'({cmd}) > {ROOT}/exec_{i}.log 2>&1; echo $? > {ROOT}/exec_{i}.rc'], cwd=ROOT, start_new_session=True)
        self._send(200, json.dumps({'id': i}).encode(), 'application/json')
    def log_message(self, *a): pass
http.server.ThreadingHTTPServer(('0.0.0.0', 8000), H).serve_forever()
