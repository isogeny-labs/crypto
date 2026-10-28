#!/usr/bin/env python3
"""RunPod launcher for a private repository: the pod clones nothing.

The pod starts an upload receiver on :8000 under a fresh random path. This
session PUTs the files named on the command line. Each must match the sha256
passed in the pod's env (EXPECT), or the pod rejects it. Once all files have
arrived and verified, the pod runs `job.sh` and serves /workspace/out on the
same port. The nonce is not a credential: the hashes guard integrity, and no
API key or token is ever on the pod. Plumbing only; the RunPod call is
`research/dreg_closure_m4l6_20261018/pod/launch.py`'s `call` (key via curl
stdin config).

  upload_launch.py run --name NAME --vcpu 64 --flavors cpu3m --max-seconds N FILE...
  upload_launch.py push ID NONCE FILE...
"""
import argparse, hashlib, json, os, secrets, subprocess, sys, time

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "..", "dreg_closure_m4l6_20261018", "pod"))
import launch  # noqa: E402

RECV = r'''
import hashlib, http.server, json, os, subprocess, threading
NONCE = os.environ["NONCE"]; EXPECT = json.loads(os.environ["EXPECT"])
ROOT = os.environ.get("PODROOT", "/workspace"); IN, OUT = ROOT + "/in", ROOT + "/out"
os.makedirs(IN, exist_ok=True); os.makedirs(OUT, exist_ok=True)
got = {}
lock = threading.Lock()
class H(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *a, **k): super().__init__(*a, directory=OUT, **k)
    def do_PUT(self):
        parts = self.path.strip("/").split("/")
        if len(parts) != 2 or parts[0] != NONCE or parts[1] not in EXPECT or os.path.exists(IN + "/STARTED"):
            self.send_error(403); return
        n = int(self.headers["Content-Length"]); data = self.rfile.read(n)
        if hashlib.sha256(data).hexdigest() != EXPECT[parts[1]]:
            self.send_error(409, "sha256 mismatch"); return
        with open(os.path.join(IN, parts[1]), "wb") as f: f.write(data)
        self.send_response(201); self.end_headers()
        with lock:
            got[parts[1]] = True
            if len(got) == len(EXPECT) and not os.path.exists(IN + "/STARTED"):
                open(IN + "/STARTED", "w").close()
                subprocess.Popen(["setsid", "bash", IN + "/job.sh"], stdout=open(OUT + "/job.log", "a"),
                                 stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
with open(OUT + "/STATUS", "a") as f: f.write("RECEIVER_UP\n")
http.server.ThreadingHTTPServer(("0.0.0.0", 8000), H).serve_forever()
'''

START = r'''( sleep "$POD_MAX_SECONDS"; kill -TERM 1 ) >/dev/null 2>&1 &
command -v python3 >/dev/null || { apt-get update -qq && apt-get install -y -qq python3; } >/dev/null 2>&1
command -v prlimit >/dev/null || apt-get install -y -qq util-linux >/dev/null 2>&1
mkdir -p /workspace/out; printf '%s' "$RECV" > /root/recv.py
setsid python3 /root/recv.py >/workspace/out/recv.log 2>&1 </dev/null &
( apt-get update -qq && apt-get install -y -qq time ) >/dev/null 2>&1 &
[ -x /start.sh ] && exec /start.sh
exec sleep infinity'''


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def run(a):
    expect = {os.path.basename(p): sha(p) for p in a.files}
    nonce = secrets.token_hex(16)
    body = {
        "name": a.name, "cloudType": "SECURE", "imageName": "runpod/base:1.0.2-ubuntu2404",
        "containerDiskInGb": a.disk, "volumeInGb": 0, "ports": ["8000/http"],
        "dockerEntrypoint": ["bash", "-c"], "dockerStartCmd": [START],
        "env": {"NONCE": nonce, "EXPECT": json.dumps(expect), "RECV": RECV,
                "POD_MAX_SECONDS": str(a.max_seconds), **dict(kv.split("=", 1) for kv in a.env)},
        "computeType": "CPU", "cpuFlavorIds": a.flavors.split(","), "vcpuCount": a.vcpu,
    }
    r = launch.call("POST", "/pods", body)
    if not isinstance(r, dict) or "id" not in r:
        sys.exit(f"pod not created: {json.dumps(r)[:600]}")
    print(json.dumps({k: r.get(k) for k in ("id", "name", "vcpuCount", "memoryInGb", "costPerHr", "machineId")}))
    print(json.dumps({"nonce": nonce, "expect": expect}))


def push(a):
    base = f"https://{a.id}-8000.proxy.runpod.net/{a.nonce}/"
    for p in a.files:
        for attempt in range(40):
            out = subprocess.run(["curl", "-sS", "-m", "600", "-o", "/dev/null", "-w", "%{http_code}",
                                  "-X", "PUT", "--data-binary", "@" + p, base + os.path.basename(p)],
                                 capture_output=True, text=True).stdout
            if out == "201":
                print("uploaded", p); break
            time.sleep(30)
        else:
            sys.exit(f"upload failed: {p} (last HTTP {out})")


ap = argparse.ArgumentParser()
sub = ap.add_subparsers(dest="cmd", required=True)
s = sub.add_parser("run")
s.add_argument("--name", required=True); s.add_argument("--vcpu", type=int, default=64)
s.add_argument("--flavors", default="cpu3m"); s.add_argument("--disk", type=int, default=40)
s.add_argument("--max-seconds", type=int, required=True); s.add_argument("--env", action="append", default=[])
s.add_argument("files", nargs="+")
s = sub.add_parser("push"); s.add_argument("id"); s.add_argument("nonce"); s.add_argument("files", nargs="+")
a = ap.parse_args()
{"run": run, "push": push}[a.cmd](a)
