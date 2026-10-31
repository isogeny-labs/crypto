#!/usr/bin/env python3
"""RunPod launcher for this study's hosts. Plumbing only (like taskq/deploy/ephemeral).

  launch.py run  --job closure|pkm --name NAME --branch BR [--vcpu 32] [--flavors cpu5m,cpu3m]
                 [--max-seconds N] [--gpu "NVIDIA H200" --gpu-count 2 --min-ram-per-gpu 250]
  launch.py ls | status ID | stop ID | url ID
"""
import argparse, json, os, subprocess, sys

API = "https://rest.runpod.io/v1"
KEY = os.environ["RUNPOD_API_KEY"]

def call(method, path, body=None, url=None):
    # curl, not urllib: this container's egress proxy answers urllib POSTs with 403.
    # The key goes to curl as a config file on stdin, never on its command
    # line (argv is readable by every process in the container).
    cmd = ["curl", "-sS", "-m", "90", "-K", "-", "-X", method, url or API + path, "-H", "Content-Type: application/json"]
    if body is not None:
        cmd += ["--data-binary", json.dumps(body)]
    cfg = 'header = "Authorization: Bearer %s"\n' % KEY
    out = subprocess.run(cmd, input=cfg, capture_output=True, text=True, check=True).stdout
    return json.loads(out or "null")

START = r'''( sleep "$POD_MAX_SECONDS"; echo "lifetime cap reached" >> /root/boot.log; kill -TERM 1 ) >/tmp/pod-lifetime.log 2>&1 &
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null 2>&1; apt-get install -y -qq curl ca-certificates >/dev/null 2>&1
curl -sSL "https://raw.githubusercontent.com/aburan28/crypto/$BRANCH/${BOOT:-research/dreg_closure_m4l6_20261018/pod/boot.sh}" -o /root/boot.sh
setsid bash /root/boot.sh "$JOB" >>/root/boot.log 2>&1 </dev/null &
[ -x /start.sh ] && exec /start.sh
exec sleep infinity'''

def run(a):
    body = {
        "name": a.name, "cloudType": "SECURE", "imageName": "runpod/base:1.0.2-ubuntu2404",
        "containerDiskInGb": a.disk, "volumeInGb": 0, "ports": ["8000/http"],
        "dockerEntrypoint": ["bash", "-c"], "dockerStartCmd": [START],
        "env": {"JOB": a.job, "BRANCH": a.branch, "BOOT": a.boot, "POD_MAX_SECONDS": str(a.max_seconds), **dict(kv.split("=", 1) for kv in a.env)},  # no secrets on the pod
    }
    if a.gpu:
        body.update({"computeType": "GPU", "gpuTypeIds": a.gpu.split(","), "gpuCount": a.gpu_count,
                     "minRAMPerGPU": a.min_ram_per_gpu, "minVCPUPerGPU": a.min_vcpu_per_gpu})
    else:
        body.update({"computeType": "CPU", "cpuFlavorIds": a.flavors.split(","), "vcpuCount": a.vcpu})
    if a.dry_run:
        print(json.dumps(body, indent=1)); return
    r = call("POST", "/pods", body)
    if not isinstance(r, dict) or "id" not in r:
        sys.exit(f"pod not created: {json.dumps(r)[:600]}")
    print(json.dumps({k: r.get(k) for k in ("id", "name", "desiredStatus", "cpuFlavorId", "vcpuCount", "memoryInGb", "costPerHr", "machineId")}))
    print("results:", f"https://{r['id']}-8000.proxy.runpod.net/")

GQL = "https://api.runpod.io/graphql"

def fetch(base, path, dest):
    """Recursive copy of a python http.server listing (no key needed: the proxy URL is public)."""
    import html, re, urllib.parse
    listing = subprocess.run(["curl", "-sSfL", "-m", "120", base + path], capture_output=True, text=True).stdout
    for href in re.findall(r'<a href="([^"]+)">', listing):
        name = urllib.parse.unquote(html.unescape(href))
        if name.startswith(("/", "..", "?")):
            continue
        if name.endswith("/"):
            fetch(base, path + href, os.path.join(dest, name))
        else:
            os.makedirs(dest, exist_ok=True)
            subprocess.run(["curl", "-sSfL", "-m", "600", base + path + href, "-o", os.path.join(dest, name)], check=False)
            print(os.path.join(dest, name))

def prices(a):
    """One JSON line per CPU instance id (flavor-vcpu-ramGB) and per GPU type with its RAM per GPU."""
    stamp = subprocess.run(["date", "-u", "+%FT%TZ"], capture_output=True, text=True).stdout.strip()
    for inst in a.instances.split(","):
        fl = inst.split("-")[0]
        q = '{ cpuFlavors { id specifics(input:{instanceId:"%s"}) { stockStatus securePrice } } }' % inst
        r = call("POST", "", {"query": q}, url=GQL)
        for f in r["data"]["cpuFlavors"]:
            if f["id"] == fl:
                print(json.dumps({"queried": stamp, "instance": inst, **f["specifics"]}))
    q = '{ gpuTypes { id memoryInGb securePrice lowestPrice(input:{gpuCount:1, secureCloud:true}) { uninterruptablePrice minMemory minVcpu stockStatus } } }'
    for g in call("POST", "", {"query": q}, url=GQL)["data"]["gpuTypes"]:
        lp = g.get("lowestPrice") or {}
        if (lp.get("minMemory") or 0) >= 100:
            print(json.dumps({"queried": stamp, "gpu": g["id"], "securePrice": g["securePrice"], **lp}))

def main():
    p = argparse.ArgumentParser(); sub = p.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run"); r.add_argument("--job", required=True); r.add_argument("--name", required=True)
    r.add_argument("--branch", required=True); r.add_argument("--vcpu", type=int, default=32)
    r.add_argument("--flavors", default="cpu5m,cpu3m"); r.add_argument("--disk", type=int, default=150)
    r.add_argument("--max-seconds", type=int, default=172800); r.add_argument("--gpu"); r.add_argument("--gpu-count", type=int, default=1)
    r.add_argument("--min-ram-per-gpu", type=int, default=8); r.add_argument("--min-vcpu-per-gpu", type=int, default=2)
    r.add_argument("--env", action="append", default=[], help="extra K=V for the pod (no secrets)")
    r.add_argument("--boot", default="research/dreg_closure_m4l6_20261018/pod/boot.sh", help="repo path of the boot script the pod fetches from BRANCH")
    r.add_argument("--dry-run", action="store_true", help="print the request body (no key in it) and create nothing")
    pr = sub.add_parser("prices", help="secure-cloud $/h of CPU instances and high-RAM GPU types (GraphQL)")
    pr.add_argument("--instances", default="cpu3c-2-4,cpu5c-2-4,cpu3c-32-64,cpu3g-8-32,cpu3g-32-128,cpu3m-16-128,cpu3m-32-256,cpu5c-32-64,cpu5g-32-128,cpu5m-32-256")
    for c in ("status", "stop", "terminate", "url"): sub.add_parser(c).add_argument("id")
    f = sub.add_parser("fetch", help="mirror the pod's :8000 listing (a sub-path of it) into DEST")
    f.add_argument("id"); f.add_argument("dest"); f.add_argument("--path", default="")
    sub.add_parser("ls")
    a = p.parse_args()
    if a.cmd == "run": run(a)
    elif a.cmd == "prices": prices(a)
    elif a.cmd == "fetch": fetch(f"https://{a.id}-8000.proxy.runpod.net/", a.path, a.dest)
    elif a.cmd == "ls":
        for q in call("GET", "/pods"): print(q["id"], q.get("name"), q.get("desiredStatus"), q.get("vcpuCount"), q.get("memoryInGb"), q.get("costPerHr"))
    elif a.cmd == "status":
        q = call("GET", f"/pods/{a.id}"); print(json.dumps({k: q.get(k) for k in ("id", "name", "desiredStatus", "vcpuCount", "memoryInGb", "costPerHr", "lastStatusChange")}))
    elif a.cmd == "stop": print(call("POST", f"/pods/{a.id}/stop"))
    elif a.cmd == "terminate": print(call("DELETE", f"/pods/{a.id}"))
    elif a.cmd == "url": print(f"https://{a.id}-8000.proxy.runpod.net/")

if __name__ == "__main__": main()
