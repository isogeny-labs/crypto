#!/usr/bin/env python3
"""Launch / collect / delete the RunPod timed run (see PROTOCOL.md). The RunPod key is injected by the session's
network proxy for rest.runpod.io; this script sends a placeholder bearer. The pod token is kept outside the repository
(STATE file in the scratchpad), never committed.

  launch.py payload STD_BIN FAST_BIN       build payload.tar.gz (deterministic) and print its sha256
  launch.py create                         create the pod, wait for its proxy, upload the payload
  launch.py status                         pod job status
  launch.py collect                        download results into results/holdout/ (once the job has exited)
  launch.py delete                         delete the pod"""
import base64, hashlib, io, json, os, secrets, sys, tarfile, time, urllib.request, urllib.error
HERE = os.path.dirname(os.path.abspath(__file__))
STATE = os.environ.get("MTR_STATE", "/tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/mtr_state.json")
PAYLOAD = os.environ.get("MTR_PAYLOAD", "/tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/mtr_payload.tar.gz")
REST = "https://rest.runpod.io/v1"
INST = os.path.join(HERE, "../mt_fieldops_v5_20261112/instances")
RECEIPTS = os.path.join(HERE, "launch")


def api(method, path, body=None):
    req = urllib.request.Request(REST + path, method=method, data=None if body is None else json.dumps(body).encode(),
                                 headers={"Authorization": "Bearer proxy-injected", "Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        t = r.read()
        return json.loads(t) if t else {}


def pod_url(st, route):
    return f"https://{st['pod_id']}-8000.proxy.runpod.net/{st['token']}/{route}"


def http(method, url, data=None, timeout=120):
    req = urllib.request.Request(url, method=method, data=data)
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return r.read()


def receipt(name, obj):
    os.makedirs(RECEIPTS, exist_ok=True)
    json.dump(obj, open(os.path.join(RECEIPTS, name), "w"), indent=1, sort_keys=True)


def payload(std_bin, fast_bin):
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w:gz", format=tarfile.GNU_FORMAT) as t:
        def add(src, arc, mode):
            b = open(src, "rb").read()
            ti = tarfile.TarInfo(arc); ti.size = len(b); ti.mode = mode; ti.mtime = 0; ti.uid = ti.gid = 0
            t.addfile(ti, io.BytesIO(b))
        add(os.path.join(HERE, "run.py"), "run.py", 0o755)
        add(std_bin, "bin/mt_fieldops_std", 0o755)
        add(fast_bin, "bin/mt_fieldops_fast", 0o755)
        for pb in (24, 28, 32):
            for i in range(4):
                add(os.path.join(INST, f"W{pb}-{i}.json"), f"instances/W{pb}-{i}.json", 0o644)
    data = buf.getvalue()
    open(PAYLOAD, "wb").write(data)
    files = {"run.py": hashlib.sha256(open(os.path.join(HERE, "run.py"), "rb").read()).hexdigest(),
             "bin/mt_fieldops_std": hashlib.sha256(open(std_bin, "rb").read()).hexdigest(),
             "bin/mt_fieldops_fast": hashlib.sha256(open(fast_bin, "rb").read()).hexdigest()}
    rec = {"payload_sha256": hashlib.sha256(data).hexdigest(), "payload_bytes": len(data), "files_sha256": files}
    receipt("payload.json", rec)
    print(json.dumps(rec, indent=1))


def create():
    if os.path.exists(STATE):
        sys.exit("state exists: a pod was already created for this run; collect/delete it first")
    pin = json.load(open(os.path.join(RECEIPTS, "payload.json")))["payload_sha256"]
    data = open(PAYLOAD, "rb").read()
    assert hashlib.sha256(data).hexdigest() == pin, "payload changed since it was pinned"
    token = secrets.token_hex(24)
    podjob = open(os.path.join(HERE, "pod/podjob.py"), "rb").read()
    body = {"name": "mt-timed-runpod-20261112", "computeType": "CPU", "cpuFlavorIds": ["cpu5c"],
            "cpuFlavorPriority": "custom", "vcpuCount": 4, "cloudType": "SECURE", "interruptible": False,
            "imageName": "runpod/base:1.0.2-ubuntu2404", "containerDiskInGb": 20, "volumeInGb": 0,
            "ports": ["8000/http"], "dockerEntrypoint": ["bash", "-c"],
            "dockerStartCmd": ["echo $PODJOB_B64 | base64 -d > /podjob.py && python3 /podjob.py"],
            "env": {"TOKEN": token, "PAYLOAD_SHA256": pin, "JOB_ARGV": json.dumps(["python3", "run.py", "holdout"]),
                    "PIN": "2", "REPEATS": "3", "PODJOB_B64": base64.b64encode(podjob).decode()}}
    t0 = time.time()
    pod = api("POST", "/pods", body)
    st = {"pod_id": pod["id"], "token": token, "created": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    json.dump(st, open(STATE, "w"))
    pub = {k: v for k, v in pod.items() if k not in ("env",)}
    receipt("create.json", {"request": {k: v for k, v in body.items() if k != "env"},
                            "request_env_keys": sorted(body["env"]), "payload_sha256": pin,
                            "podjob_sha256": hashlib.sha256(podjob).hexdigest(), "response": pub})
    print("pod", pod["id"], pod.get("costPerHr"), pod.get("cpuFlavorId"))
    while time.time() - t0 < 1800:
        try:
            s = json.loads(http("GET", pod_url(st, "status"), timeout=30))
            print("pod job server up:", s); break
        except Exception as e:
            time.sleep(15)
    else:
        sys.exit("pod did not come up within 30 min; inspect and delete it")
    r = json.loads(http("PUT", pod_url(st, "payload"), data=data, timeout=600))
    print("upload:", r)
    receipt("upload.json", {"pod_id": st["pod_id"], "response": r,
                            "uploaded": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())})


def status():
    st = json.load(open(STATE))
    print(json.loads(http("GET", pod_url(st, "status"), timeout=30)))


def collect():
    st = json.load(open(STATE))
    s = json.loads(http("GET", pod_url(st, "status"), timeout=30))
    if s.get("running"):
        sys.exit("job still running")
    out = os.path.join(HERE, "results/holdout"); os.makedirs(out, exist_ok=True)
    got = {}
    for f in ("holdout/runs.jsonl", "holdout/opcost.jsonl", "holdout/host.json", "holdout/DONE", "job.log"):
        try:
            b = http("GET", pod_url(st, "result/" + f), timeout=600)
        except urllib.error.HTTPError as e:
            got[f] = f"HTTP {e.code}"; continue
        dst = os.path.join(out, os.path.basename(f)); open(dst, "wb").write(b)
        got[f] = hashlib.sha256(b).hexdigest()
    receipt("collect.json", {"pod_id": st["pod_id"], "status": s, "files_sha256": got,
                             "collected": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())})
    print(json.dumps(got, indent=1))


def delete():
    st = json.load(open(STATE))
    r = api("DELETE", f"/pods/{st['pod_id']}")
    receipt("delete.json", {"pod_id": st["pod_id"], "response": r,
                            "deleted": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())})
    os.rename(STATE, STATE + ".done")
    print("deleted", st["pod_id"])


if __name__ == "__main__":
    cmd = sys.argv[1]
    {"payload": lambda: payload(sys.argv[2], sys.argv[3]), "create": create, "status": status,
     "collect": collect, "delete": delete}[cmd]()
