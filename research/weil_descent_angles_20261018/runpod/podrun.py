"""Run one fixed command on a RunPod CPU pod against pinned, pushed source; read results back.

Plumbing only: it computes nothing. Used because this session has a RunPod key
but no AWS secret, so the repo's `ephemeral` runner (AGENTS.md §9a) and the S3
channel of `ecc2k130/runpod_job.py` are unavailable.

    python3 podrun.py launch --name h1 --sha COMMIT --file examples/n2_routes.rs \
        --vcpu 32 --cmd 'rustc --edition 2021 -O examples/n2_routes.rs -o n2 && N2_THREADS=32 ./n2 "$OUT" hsweep'
    python3 podrun.py status POD
    python3 podrun.py fetch POD OUTDIR     # results.tgz once the command has finished
    python3 podrun.py kill POD             # terminate (always do this)

The pod downloads each --file at the pinned commit from raw.githubusercontent.com
(the repository is public), checks it against the SHA-256 computed here, runs the
command fixed at launch, and then serves only two read-only paths, the status
and results.tgz, behind https://POD-8000.proxy.runpod.net/TOKEN/. Nothing is
uploaded into the pod. The token stays in ~/.cache/podrun/POD.json and is
never committed.
"""
import argparse
import hashlib
import json
import os
import pathlib
import secrets
import shlex
import subprocess
import sys
import time
import urllib.request

API = "https://rest.runpod.io/v1"
CACHE = pathlib.Path.home() / ".cache" / "podrun"
REPO = "aburan28/crypto"

SERVER = r'''
import http.server, os
TOKEN = os.environ["PODRUN_TOKEN"]
class H(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        parts = self.path.split("/")
        if len(parts) != 3 or parts[1] != TOKEN or parts[2] not in ("status", "results.tgz"):
            self.send_response(403); self.end_headers(); return
        try:
            if parts[2] == "status":
                body = open("/work/phase").read().encode()
                try:
                    body += b"\n" + open("/work/out/job.log", "rb").read()[-4000:]
                except OSError:
                    pass
            else:
                body = open("/work/results.tgz", "rb").read()
        except OSError:
            self.send_response(404); self.end_headers(); return
        self.send_response(200); self.send_header("Content-Length", str(len(body))); self.end_headers()
        self.wfile.write(body)
    def log_message(self, *a):
        pass
http.server.ThreadingHTTPServer(("0.0.0.0", 8000), H).serve_forever()
'''


def api(method, path, body=None):
    req = urllib.request.Request(API + path, method=method,
                                 data=None if body is None else json.dumps(body).encode(),
                                 headers={"Authorization": "Bearer " + os.environ["RUNPOD_API_KEY"],
                                          "Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        t = r.read()
        return json.loads(t) if t else {}


def proxy(pod, token, name):
    url = "https://%s-8000.proxy.runpod.net/%s/%s" % (pod, token, name)
    with urllib.request.urlopen(url, timeout=300) as r:
        return r.read()


def launch(a):
    root = pathlib.Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True,
                                       text=True, check=True).stdout.strip())
    # the commit must be pushed and the files must match it exactly
    fetch_lines = []
    for f in a.file:
        blob = subprocess.run(["git", "show", "%s:%s" % (a.sha, f)], cwd=root, capture_output=True, check=True).stdout
        digest = hashlib.sha256(blob).hexdigest()
        url = "https://raw.githubusercontent.com/%s/%s/%s" % (REPO, a.sha, f)
        fetch_lines.append("mkdir -p %s && curl -sSfL %s -o %s && echo '%s  %s' | sha256sum -c -"
                           % (shlex.quote(os.path.dirname(f) or "."), shlex.quote(url), shlex.quote(f), digest, shlex.quote(f)))
    token = secrets.token_hex(16)
    job = " && ".join(fetch_lines) + " && " + a.cmd
    boot = "\n".join([
        "mkdir -p /work/src /work/out && cd /work/src && echo starting > /work/phase",
        "(command -v python3 >/dev/null && command -v /usr/bin/time >/dev/null) || "
        "(apt-get update -qq && apt-get install -y -qq python3 time curl >/dev/null)",
        "cat > /srv.py <<'PYEOF'\n" + SERVER + "\nPYEOF",
        "python3 /srv.py &",
        "(lscpu; free -g; nproc; uname -a; rustc --version) > /work/out/host.txt 2>&1",
        "echo running > /work/phase",
        "export OUT=/work/out",
        "/usr/bin/time -v -o /work/out/rusage.txt bash -c %s > /work/out/job.log 2>&1; rc=$?" % shlex.quote(job),
        "echo $rc > /work/out/exit-code; tar -czf /work/results.tgz -C /work/out .; echo \"done $rc\" > /work/phase",
        "sleep %d" % (a.hold_hours * 3600),
    ])
    body = {"name": a.name, "imageName": a.image, "computeType": "CPU", "cpuFlavorIds": a.flavor.split(","),
            "vcpuCount": a.vcpu, "containerDiskInGb": a.disk, "ports": ["8000/http"],
            "env": {"PODRUN_TOKEN": token}, "dockerStartCmd": ["bash", "-c", boot]}
    pod = api("POST", "/pods", body)
    pid = pod["id"]
    CACHE.mkdir(parents=True, exist_ok=True)
    meta = {"id": pid, "token": token, "name": a.name, "sha": a.sha, "files": a.file, "cmd": a.cmd,
            "vcpu": a.vcpu, "flavor": a.flavor, "image": a.image, "launched_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "cost_per_hr": pod.get("costPerHr"), "machine": pod.get("machine")}
    (CACHE / (pid + ".json")).write_text(json.dumps(meta, indent=1))
    print(pid)


def load(pid):
    return json.loads((CACHE / (pid + ".json")).read_text())


def status(a):
    m = load(a.pod)
    info = api("GET", "/pods/" + a.pod)
    print(info.get("desiredStatus"), info.get("costPerHr"))
    try:
        print(proxy(a.pod, m["token"], "status").decode())
    except Exception as e:
        print("proxy not reachable yet:", e)


def fetch(a):
    m = load(a.pod)
    st = proxy(a.pod, m["token"], "status").decode()
    if not st.startswith("done"):
        sys.exit("not done: " + st.splitlines()[0])
    out = pathlib.Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    (out / "results.tgz").write_bytes(proxy(a.pod, m["token"], "results.tgz"))
    (out / "pod.json").write_text(json.dumps({k: v for k, v in m.items() if k != "token"}, indent=1))
    print("fetched", out / "results.tgz")


def kill(a):
    api("DELETE", "/pods/" + a.pod)
    print("terminated", a.pod)


p = argparse.ArgumentParser()
sp = p.add_subparsers(dest="cmd_", required=True)
l = sp.add_parser("launch")
l.add_argument("--name", required=True)
l.add_argument("--sha", required=True, help="a pushed commit of aburan28/crypto")
l.add_argument("--file", action="append", required=True, help="repo-relative path fetched at --sha")
l.add_argument("--cmd", required=True, help="command run in the fetched tree; $OUT is the output dir")
l.add_argument("--vcpu", type=int, default=16)
l.add_argument("--flavor", default="cpu5c,cpu3c")
l.add_argument("--image", default="rust:1-bookworm")
l.add_argument("--disk", type=int, default=20)
l.add_argument("--hold-hours", type=int, default=6, help="keep serving results this long, then exit")
l.set_defaults(f=launch)
for name, f in (("status", status), ("kill", kill)):
    q = sp.add_parser(name)
    q.add_argument("pod")
    q.set_defaults(f=f)
q = sp.add_parser("fetch")
q.add_argument("pod")
q.add_argument("out")
q.set_defaults(f=fetch)
a = p.parse_args()
a.f(a)
