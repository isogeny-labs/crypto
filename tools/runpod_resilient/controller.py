#!/usr/bin/env python3
"""Keep a RunPod job alive across pod loss, and mirror its output off the pod.

The job's state lives on a network volume (see supervisor.py). This controller runs on the
client side (an agent session, a laptop) and loops:

  1. Ask the RunPod REST API for the pod. If it is gone, exited, or its supervisor has not
     answered for --grace seconds, delete it (if it still exists) and create a replacement on
     the same network volume. The replacement's supervisor restarts the job from
     WORK/autorun.json; the job resumes from its own output files.
  2. Mirror every file under WORK/runs whose size changed into --mirror.
  3. When --done (a regex) matches the file --progress, mirror once more, delete the pod, exit 0.

Every action is appended to --mirror/controller.events.jsonl. The pod's access token is
generated here, kept in --token-file (mode 600) and never printed. The RunPod API key is not
handled here: requests go to api.runpod.io (REST v2) with a placeholder bearer that the agent proxy
replaces (set RUNPOD_API_KEY to send a real one).

  controller.py --volume VOL --name NAME --mirror DIR --token-file F --progress runs/x/progress.txt
                --done 'done m3-t7-dense' [--pod ID] [--cpu-flavors cpu5m,cpu3m] [--vcpu 16]
"""

import argparse
import base64
import json
import os
import re
import secrets
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

API = "https://api.runpod.io/v2"  # REST v2 (rest.runpod.io/v1 is legacy)
HERE = Path(__file__).resolve().parent


def api(method, path, body=None):
    req = urllib.request.Request(API + path, method=method,
                                 data=None if body is None else json.dumps(body).encode(),
                                 headers={"Authorization": "Bearer " + os.environ.get("RUNPOD_API_KEY", "dummy"),
                                          "Content-Type": "application/json",
                                          "User-Agent": "runpod-resilient/1.0"})  # Cloudflare 1010s urllib's
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            raw = r.read()
            return r.status, json.loads(raw) if raw.strip() else None
    except urllib.error.HTTPError as e:
        try:
            return e.code, json.loads(e.read())  # v2 errors are structured: {"status", "title", "detail", ...}
        except Exception:  # noqa: BLE001
            return e.code, None
    except Exception:  # noqa: BLE001  (network error: report as unknown)
        return 0, None


def pod_get(url, token, path, timeout=60):
    req = urllib.request.Request(url + path, headers={"Authorization": "Bearer " + token,
                                                      "User-Agent": "runpod-resilient/1.0"})
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return r.read()


class Controller:
    def __init__(self, a):
        self.a = a
        self.mirror = Path(a.mirror)
        self.mirror.mkdir(parents=True, exist_ok=True)
        self.events = self.mirror / "controller.events.jsonl"
        self.state_file = self.mirror / "controller.state.json"
        st = json.loads(self.state_file.read_text()) if self.state_file.exists() else {}
        self.pod = a.pod or st.get("pod")
        self.sizes = st.get("sizes", {})
        self.recreates = st.get("recreates", 0)
        self.last_ok = time.time()

    def log(self, **kv):
        kv["t"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        with open(self.events, "a") as f:
            f.write(json.dumps(kv) + "\n")
        print(json.dumps(kv), flush=True)

    def save(self):
        self.state_file.write_text(json.dumps({"pod": self.pod, "sizes": self.sizes,
                                               "recreates": self.recreates}))

    def token(self):
        return Path(self.a.token_file).read_text().strip()

    def url(self):
        return f"https://{self.pod}-8888.proxy.runpod.net"

    def create(self):
        tok = secrets.token_hex(24)
        tf = Path(self.a.token_file)
        tf.write_text(tok)
        tf.chmod(0o600)
        sup = base64.b64encode((HERE / "supervisor.py").read_bytes()).decode()
        dc = (api("GET", f"/network-volumes/{self.a.volume}")[1] or {}).get("dataCenter")
        for flavor in self.a.cpu_flavors.split(","):  # v2 takes one flavor per pod: try them in order
            body = {"name": self.a.name, "image": self.a.image, "disk": 20, "ports": ["8888/http"],
                    "entrypoint": ["python3", "-u", "-c"],
                    "cmd": [f"import base64;exec(compile(base64.b64decode('{sup}'),'supervisor','exec'))"],
                    "env": {"MON_TOKEN": tok, "WORK_DIR": self.a.work_dir},
                    "cpu": {"id": flavor, "vcpuCount": self.a.vcpu},
                    "dataCenterIds": [dc] if dc else [],
                    "mounts": {"network": [{"volumeId": self.a.volume, "path": "/workspace"}]}}
            code, r = api("POST", "/pods", body)
            if code in (200, 201) and r and r.get("id"):
                self.pod = r["id"]
                self.last_ok = time.time()
                self.log(event="created", pod=self.pod, flavor=flavor, cost_per_hr=r.get("cost"),
                         dc=r.get("dataCenterId"))
                break
            self.log(event="create_failed", flavor=flavor, http=code, detail=(r or {}).get("detail"))
        else:
            self.pod = None
        self.save()

    def replace(self, why):
        self.log(event="pod_lost", pod=self.pod, why=why)
        if self.pod:
            api("DELETE", f"/pods/{self.pod}")
        self.recreates += 1
        if self.recreates > self.a.max_recreates:
            self.log(event="giving_up", recreates=self.recreates)
            self.save()
            sys.exit(3)
        self.create()

    def sync(self):
        listing = json.loads(pod_get(self.url(), self.token(), "/list"))
        n = 0
        for rel, size in listing:
            if self.sizes.get(rel) == size:
                continue
            data = pod_get(self.url(), self.token(), "/file?path=" + rel, timeout=300)
            dst = self.mirror / Path(rel).relative_to(self.a.strip)
            dst.parent.mkdir(parents=True, exist_ok=True)
            tmp = dst.with_suffix(dst.suffix + ".part")
            tmp.write_bytes(data)
            tmp.replace(dst)
            self.sizes[rel] = len(data)
            n += 1
        self.save()
        return n

    def step(self):
        if not self.pod:
            self.create()
            return False
        code, r = api("GET", f"/pods/{self.pod}")
        if code == 404 or (r and r.get("status") in ("EXITED", "TERMINATED", "ERROR")):
            self.replace(f"api http={code} status={(r or {}).get('status')}")
            return False
        try:
            health = json.loads(pod_get(self.url(), self.token(), "/health"))
            self.last_ok = time.time()
            n = self.sync()
            try:
                prog = pod_get(self.url(), self.token(), "/file?path=" + self.a.progress).decode()
            except urllib.error.HTTPError as e:
                if e.code != 404:  # no progress file yet: the job has not started
                    raise
                prog = ""
        except Exception as e:  # noqa: BLE001
            if time.time() - self.last_ok > self.a.grace:
                self.replace(f"unreachable {int(time.time() - self.last_ok)}s: {type(e).__name__}")
            return False
        if n:
            self.log(event="synced", files=n, running=health.get("running"), boots=len(health.get("boots", [])))
        if re.search(self.a.done, prog):
            self.sync()
            api("DELETE", f"/pods/{self.pod}")
            self.log(event="done_deleted", pod=self.pod)
            self.pod = None
            self.save()
            return True
        return False

    def run(self):
        self.log(event="controller_start", pod=self.pod)
        while not self.step():
            time.sleep(self.a.interval)


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--volume", required=True)
    p.add_argument("--name", required=True)
    p.add_argument("--mirror", required=True)
    p.add_argument("--token-file", required=True)
    p.add_argument("--progress", required=True, help="path under WORK of the progress file")
    p.add_argument("--done", required=True, help="regex; matching it in --progress ends the run")
    p.add_argument("--strip", default="runs", help="leading path removed when mirroring")
    p.add_argument("--pod", help="adopt this existing pod")
    p.add_argument("--cpu-flavors", default="cpu5m,cpu3m")
    p.add_argument("--vcpu", type=int, default=16)
    p.add_argument("--image", default="runpod/base:1.0.2-ubuntu2404")
    p.add_argument("--work-dir", default="/workspace/run")
    p.add_argument("--interval", type=int, default=300)
    p.add_argument("--grace", type=int, default=900)
    p.add_argument("--max-recreates", type=int, default=6)
    Controller(p.parse_args()).run()


if __name__ == "__main__":
    main()
