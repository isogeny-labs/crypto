#!/usr/bin/env python3
"""RunPod launcher for this study's hosts. Plumbing only (like taskq/deploy/ephemeral).

  launch.py run  --job closure|pkm --name NAME --branch BR [--vcpu 32] [--flavors cpu5m,cpu3m]
                 [--max-seconds N] [--gpu "NVIDIA H200" --gpu-count 2 --min-ram-per-gpu 250]
  launch.py ls | status ID | stop ID | url ID
"""
import argparse, json, os, subprocess, sys

API = "https://rest.runpod.io/v1"
KEY = os.environ["RUNPOD_API_KEY"]

def call(method, path, body=None):
    # curl, not urllib: this container's egress proxy answers urllib POSTs with 403.
    cmd = ["curl", "-sS", "-m", "90", "-X", method, API + path, "-H", f"Authorization: Bearer {KEY}", "-H", "Content-Type: application/json"]
    if body is not None:
        cmd += ["-d", json.dumps(body)]
    out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
    return json.loads(out or "null")

START = r'''( sleep "$POD_MAX_SECONDS"; echo "lifetime cap reached" >> /root/boot.log; kill -TERM 1 ) >/tmp/pod-lifetime.log 2>&1 &
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null 2>&1; apt-get install -y -qq curl ca-certificates >/dev/null 2>&1
curl -sSL "https://raw.githubusercontent.com/aburan28/crypto/$BRANCH/research/dreg_closure_m4l6_20261018/pod/boot.sh" -o /root/boot.sh
setsid bash /root/boot.sh "$JOB" >>/root/boot.log 2>&1 </dev/null &
[ -x /start.sh ] && exec /start.sh
exec sleep infinity'''

def run(a):
    body = {
        "name": a.name, "cloudType": "SECURE", "imageName": "runpod/base:1.0.2-ubuntu2404",
        "containerDiskInGb": a.disk, "volumeInGb": 0, "ports": ["8000/http"],
        "dockerEntrypoint": ["bash", "-c"], "dockerStartCmd": [START],
        "env": {"JOB": a.job, "BRANCH": a.branch, "POD_MAX_SECONDS": str(a.max_seconds), **dict(kv.split("=", 1) for kv in a.env)},  # no secrets on the pod
    }
    if a.gpu:
        body.update({"computeType": "GPU", "gpuTypeIds": [a.gpu], "gpuCount": a.gpu_count,
                     "minRAMPerGPU": a.min_ram_per_gpu, "minVCPUPerGPU": a.min_vcpu_per_gpu})
    else:
        body.update({"computeType": "CPU", "cpuFlavorIds": a.flavors.split(","), "vcpuCount": a.vcpu})
    r = call("POST", "/pods", body)
    if not isinstance(r, dict) or "id" not in r:
        sys.exit(f"pod not created: {json.dumps(r)[:600]}")
    print(json.dumps({k: r.get(k) for k in ("id", "name", "desiredStatus", "cpuFlavorId", "vcpuCount", "memoryInGb", "costPerHr", "machineId")}))
    print("results:", f"https://{r['id']}-8000.proxy.runpod.net/")

def main():
    p = argparse.ArgumentParser(); sub = p.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run"); r.add_argument("--job", required=True); r.add_argument("--name", required=True)
    r.add_argument("--branch", required=True); r.add_argument("--vcpu", type=int, default=32)
    r.add_argument("--flavors", default="cpu5m,cpu3m"); r.add_argument("--disk", type=int, default=150)
    r.add_argument("--max-seconds", type=int, default=172800); r.add_argument("--gpu"); r.add_argument("--gpu-count", type=int, default=1)
    r.add_argument("--min-ram-per-gpu", type=int, default=8); r.add_argument("--min-vcpu-per-gpu", type=int, default=2)
    r.add_argument("--env", action="append", default=[], help="extra K=V for the pod (no secrets)")
    for c in ("status", "stop", "terminate", "url"): sub.add_parser(c).add_argument("id")
    sub.add_parser("ls")
    a = p.parse_args()
    if a.cmd == "run": run(a)
    elif a.cmd == "ls":
        for q in call("GET", "/pods"): print(q["id"], q.get("name"), q.get("desiredStatus"), q.get("vcpuCount"), q.get("memoryInGb"), q.get("costPerHr"))
    elif a.cmd == "status":
        q = call("GET", f"/pods/{a.id}"); print(json.dumps({k: q.get(k) for k in ("id", "name", "desiredStatus", "vcpuCount", "memoryInGb", "costPerHr", "lastStatusChange")}))
    elif a.cmd == "stop": print(call("POST", f"/pods/{a.id}/stop"))
    elif a.cmd == "terminate": print(call("DELETE", f"/pods/{a.id}"))
    elif a.cmd == "url": print(f"https://{a.id}-8000.proxy.runpod.net/")

if __name__ == "__main__": main()
