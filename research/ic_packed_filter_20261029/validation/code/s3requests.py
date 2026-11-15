"""Read-only: each launcher's request.json (S3 launcher prefix) must embed the
frozen spec (JSON-equal) and name the image digest, revision and ecbench
sha256 of PROTOCOL section 4. The P28-r2 local receipt copy is compared too.
usage: PATH=<awsvenv>/bin:$PATH s3requests.py OUT.json"""
import json
import os
import subprocess
import sys

import vcommon as v
from j2_instrument import BACKLOG, ECB, REV
from s3check import BUCKET, JOBS

IMAGE = "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b"


def check(req, frozen):
    j = req["job"]
    return {"spec_json_equal_frozen": j["spec"] == frozen, "image_uri": j["build"]["image_uri"] == IMAGE,
            "source_revision": j["build"]["source_revision"] == REV, "ecbench_sha256": j["build"]["ecbench_sha256"] == ECB,
            "instance_type": j.get("instance_type"), "seconds": j.get("seconds"), "harness": req["provenance"]["harness"],
            "job_name": j["job_name"]}


out = {}
s3 = json.load(open(os.path.join(os.path.dirname(__file__), "out", "s3check.json")))
for b, kind, job in JOBS:
    frozen = json.load(open(os.path.join(BACKLOG, f"spec-P{b}.json" if kind == "wall" else f"spec-P{b}-rho.json")))
    rec = {}
    for lid in s3["jobs"][f"P{b}-{kind}"]["launchers"]:
        p = subprocess.run(["aws", "s3", "cp", f"{BUCKET}/{job}/launcher/{lid}/request.json", "-"], capture_output=True)
        rec[lid] = check(json.loads(p.stdout), frozen) if p.returncode == 0 else {"error": "absent"}
    if b == 28 and kind == "counted":
        local = json.load(open(os.path.join(v.ROOT, "P28", "receipts-P28-rho-r2", "request.json")))
        rec["local:P28/receipts-P28-rho-r2/request.json"] = check(local, frozen)
    out[f"P{b}-{kind}"] = rec
    print(job, json.dumps(rec))
v.dump(out, sys.argv[1])
