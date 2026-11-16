"""RULING-P28 C6: recompute the C2 identity facts with the validator's own
code, for the launcher-less P28 wall job (and the manifest-completeness fact
for every job). Read-only S3 listing; build fields compared with PROTOCOL
section 4 and the input spec with the frozen spec, since the job has no
launcher request in S3. Reads no field of records.jsonl.
usage: PATH=<awsvenv>/bin:$PATH c2_identity.py OUT.json"""
import json
import os
import subprocess
import sys

import vcommon as v
from j2_instrument import BACKLOG, ECB, REV
from s3check import BUCKET, JOBS, UUID

out = {}
for b, kind, job in JOBS:
    eks = v.session_dir(b, kind)
    p = subprocess.run(["aws", "s3", "ls", "--recursive", f"{BUCKET}/{job}/"], capture_output=True, text=True)
    rel = [l.split(None, 3)[3][len(f"runs/{job}/"):] for l in p.stdout.splitlines() if l.strip()]
    pods = sorted({r.split("/")[0] for r in rel if UUID.match(r.split("/")[0])})
    pod_files = sorted(r.split("/", 1)[1] for r in rel if pods and r.startswith(pods[0] + "/"))
    listed = sorted(l.split(None, 1)[1].strip() for l in open(os.path.join(eks, "SHA256SUMS")) if l.strip())
    je = json.load(open(os.path.join(eks, "job-exit.json")))
    bi = json.load(open(os.path.join(eks, "build-info.json")))
    frozen = json.load(open(os.path.join(BACKLOG, f"spec-P{b}.json" if kind == "wall" else f"spec-P{b}-rho.json")))
    ok_sums, _, _ = v.check_sums(eks)
    f = {
        "POD_UID_COUNT": len(pods),
        "JOB_EXIT_PRESENT": "job-exit.json" in pod_files,
        "JOB_EXIT_IDENTITY": je["job_name"] == job and pods == [je["pod_uid"]],
        "EXIT_CODE": je["exit_code"],
        "SHA256SUMS_OK": ok_sums,
        "MANIFEST_COMPLETE": sorted(x for x in pod_files if x != "SHA256SUMS") == listed,
        "BUILD_INFO": bi["source_revision"] == REV and bi["ecbench_sha256"] == ECB,
        "BINARY_SHA": open(os.path.join(eks, "binary.sha256")).read().split()[0] == ECB,
        "INPUT_SPEC": json.load(open(os.path.join(eks, "input-spec.json"))) == frozen,
        "IMAGE_DIGEST": "unverifiable_without_pod_status",
    }
    f["S3_TERMINAL"] = (f["POD_UID_COUNT"] == 1 and f["JOB_EXIT_PRESENT"] and f["JOB_EXIT_IDENTITY"]
                        and f["SHA256SUMS_OK"] and f["MANIFEST_COMPLETE"] and f["BUILD_INFO"]
                        and f["BINARY_SHA"] and f["INPUT_SPEC"])
    out[f"P{b}-{kind}"] = f
    print(job, f)
v.dump(out, sys.argv[1])
