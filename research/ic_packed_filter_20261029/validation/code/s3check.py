"""S3 copy check (TASK-20261029-654f73). Read-only: lists each job's S3 prefix,
requires exactly one pod-uid prefix equal to job-exit.json's pod_uid, streams
every file named in the repository SHA256SUMS (and SHA256SUMS itself) from that
prefix and compares its sha256 with the repository file. Reads each launcher
result.json (exit code, state, build block) when present. Writes nothing to S3.

usage: PATH=<awsvenv>/bin:$PATH s3check.py OUT.json
"""
import hashlib
import json
import os
import re
import subprocess
import sys

import vcommon as v

BUCKET = "s3://isolab-results-104186811093-us-east-2/runs"
JOBS = [
    (24, "wall", "ic-dgbf-p24-20261029"),
    (24, "counted", "ic-dgbf-rho-p24-20261029"),
    (32, "wall", "ic-dgbf-p32-20261029"),
    (32, "counted", "ic-dgbf-rho-p32-20261029"),
    (28, "counted", "ic-dgbf-rho-p28-20261029-r2"),
    (28, "wall", "ic-dgbf-p28-20261029"),
]
UUID = re.compile(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")


def aws(*args, binary=False):
    p = subprocess.run(["aws", *args], capture_output=True, timeout=600)
    if p.returncode == 1 and args[:2] == ("s3", "ls") and not p.stdout and not p.stderr.strip():
        return ""  # `aws s3 ls` exits 1 on an empty prefix
    if p.returncode != 0:
        raise RuntimeError(p.stderr.decode()[:400])
    return p.stdout if binary else p.stdout.decode()


def s3_sha(uri):
    p = subprocess.Popen(["aws", "s3", "cp", uri, "-"], stdout=subprocess.PIPE)
    h = hashlib.sha256()
    for chunk in iter(lambda: p.stdout.read(1 << 20), b""):
        h.update(chunk)
    if p.wait() != 0:
        return None
    return h.hexdigest()


def main():
    out = {"bucket": BUCKET, "jobs": {}, "original_p28_counted_prefix_objects": None}
    lst = aws("s3", "ls", "--recursive", f"{BUCKET}/ic-dgbf-rho-p28-20261029/")
    out["original_p28_counted_prefix_objects"] = len([l for l in lst.splitlines() if l.strip()])
    for b, kind, job in JOBS:
        eks = v.session_dir(b, kind)
        je = json.load(open(os.path.join(eks, "job-exit.json")))
        lst = aws("s3", "ls", "--recursive", f"{BUCKET}/{job}/")
        keys = [l.split(None, 3)[3] for l in lst.splitlines() if l.strip()]
        rel = [k[len(f"runs/{job}/"):] for k in keys]
        pods = sorted({r.split("/")[0] for r in rel if UUID.match(r.split("/")[0])})
        launchers = sorted({r.split("/")[1] for r in rel if r.startswith("launcher/")})
        rec = {"job": job, "objects": len(keys), "pod_prefixes": pods,
               "job_exit_pod_uid": je.get("pod_uid"), "job_exit_code": je.get("exit_code"),
               "job_exit_job_name": je.get("job_name"),
               "one_pod_prefix_equal_to_job_exit": pods == [je.get("pod_uid")],
               "launchers": launchers, "files": {}, "launcher_results": []}
        if len(pods) == 1:
            names = [l.split(None, 1)[1].strip() for l in open(os.path.join(eks, "SHA256SUMS")) if l.strip()]
            for name in names + ["SHA256SUMS"]:
                local = v.sha256_file(os.path.join(eks, name))
                remote = s3_sha(f"{BUCKET}/{job}/{pods[0]}/{name}")
                rec["files"][name] = {"local": local, "s3": remote, "equal": local == remote}
            rec["all_files_equal"] = all(x["equal"] for x in rec["files"].values())
        for lid in launchers:
            if f"launcher/{lid}/result.json" in rel:
                r = json.loads(aws("s3", "cp", f"{BUCKET}/{job}/launcher/{lid}/result.json", "-"))
                rec["launcher_results"].append({"launcher": lid, "exit_code": r.get("exit_code"),
                                                "state": r.get("state"), "build": r.get("build"),
                                                "pod_uid": r.get("pod_uid")})
        out["jobs"][f"P{b}-{kind}"] = rec
        print(job, rec["objects"], pods, rec["one_pod_prefix_equal_to_job_exit"],
              rec.get("all_files_equal"), [(x["exit_code"], x["state"]) for x in rec["launcher_results"]])
    print("original P28 counted prefix objects:", out["original_p28_counted_prefix_objects"])
    v.dump(out, sys.argv[1])


if __name__ == "__main__":
    main()
