"""Writes the isolab.eks.request/v1 files of XP-20261029-packed-filter from the frozen specs. usage: python3 -I build_requests.py
Each request embeds its spec exactly (the spec is read with json.load and dumped under job.spec).
Twelve requests are the first launches and the contingent reruns of PROTOCOL.md section 4; the names are:
  request-P<b>.json            ic-dgbf-p<b>-20261029            wall, first launch
  request-P<b>-rho.json        ic-dgbf-rho-p<b>-20261029        counted, first launch
  request-P<b>-r2.json         ic-dgbf-p<b>-20261029-r2         wall, host-loss rerun
  request-P<b>-rho-r2.json     ic-dgbf-rho-p<b>-20261029-r2     counted, host-loss rerun
  request-P<b>-nr.json         ic-dgbf-p<b>-20261029-nr         wall, noise rerun
  request-P<b>-nr-r2.json      ic-dgbf-p<b>-20261029-nr-r2      wall, host-loss rerun of the noise rerun
Only the job name differs between a first launch and its reruns."""
import json, sys
SC = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad"
PD = "/dev/shm/src-pf/backlog/XP-20261029-packed-filter"
OUT = SC + "/pf"
FREEZE = "f870408c1762a52dd133e8f1e0bca181cc2a76f5"
BUILD = {
    "image_uri": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b",
    "source_revision": "3271a25d02dcef6da148293fb8f9e2149ce59858",
    "ecbench_sha256": "21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503",
}
RUN_URL = "https://github.com/isogeny-labs/crypto/blob/%s/backlog/XP-20261029-packed-filter/PROTOCOL.md" % FREEZE
# (request tag suffix, spec suffix, job name pattern)
KINDS = [
    ("", "", "ic-dgbf-p%d-20261029"),
    ("-rho", "-rho", "ic-dgbf-rho-p%d-20261029"),
    ("-r2", "", "ic-dgbf-p%d-20261029-r2"),
    ("-rho-r2", "-rho", "ic-dgbf-rho-p%d-20261029-r2"),
    ("-nr", "", "ic-dgbf-p%d-20261029-nr"),
    ("-nr-r2", "", "ic-dgbf-p%d-20261029-nr-r2"),
]
for b in (24, 28, 32):
    for tag, spec_suffix, pattern in KINDS:
        spec = json.load(open("%s/spec-P%d%s.json" % (PD, b, spec_suffix)))
        name = pattern % b
        req = {
            "schema": "isolab.eks.request/v1",
            "job": {"job_name": name, "instance_type": "c8a.2xlarge", "seconds": 3600, "spec": spec, "build": BUILD},
            "provenance": {
                "harness": "claude-code-local",
                "source_revision": BUILD["source_revision"],
                "workflow_revision": BUILD["source_revision"],
                "run_url": RUN_URL,
            },
        }
        out_name = "request-P%d%s.json" % (b, tag)
        with open(OUT + "/" + out_name, "w") as f:
            f.write(json.dumps(req, indent=2) + "\n")
        print("wrote", out_name, name)
