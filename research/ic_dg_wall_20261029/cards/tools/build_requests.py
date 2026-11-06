import json, sys
SC = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad"
PD = SC + "/wt-wall/backlog/XP-20261029-dg-wall"
OUT = SC + "/dgw"
FREEZE = "6e4804417baa031f6a1aee9b9b54c67c90c4dafe"
BUILD = {
    "image_uri": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c",
    "source_revision": "be2969cebf30a0cfb79088fd10094324c3efb3bb",
    "ecbench_sha256": "88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c",
}
RUN_URL = "https://github.com/isogeny-labs/crypto/blob/%s/backlog/XP-20261029-dg-wall/PROTOCOL.md" % FREEZE
for b in (24, 28, 32):
    for kind in ("wall", "rho"):
        spec_name = "spec-P%d%s.json" % (b, "" if kind == "wall" else "-rho")
        name = "ic-dgw-p%d-20261029" % b if kind == "wall" else "ic-dgw-rho-p%d-20261029" % b
        out_name = "request-P%d%s.json" % (b, "" if kind == "wall" else "-rho")
        spec = json.load(open(PD + "/" + spec_name))
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
        with open(OUT + "/" + out_name, "w") as f:
            f.write(json.dumps(req, indent=2) + "\n")
        print("wrote", out_name, name)
