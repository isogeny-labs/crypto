import hashlib, re, sys
SC = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad"
PD = SC + "/wt-wall/backlog/XP-20261029-dg-wall"
DGW = SC + "/dgw"
TOOLS = SC + "/dgw-card-tools"
REMOTE = "/dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote"

def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()

vals = {}
for b in (24, 28, 32):
    vals["REQ%d" % b] = sha("%s/request-P%d.json" % (DGW, b))
    vals["REQR%d" % b] = sha("%s/request-P%d-rho.json" % (DGW, b))
    vals["SPEC%d" % b] = sha("%s/spec-P%d.json" % (PD, b))
    vals["SPECR%d" % b] = sha("%s/spec-P%d-rho.json" % (PD, b))
vals["PROTOCOL"] = sha(PD + "/PROTOCOL.md")
vals["APPROVAL"] = sha(PD + "/APPROVAL.md")
vals["PRELAUNCH"] = sha(PD + "/PRELAUNCH-CHECKS.md")
vals["VALREQ"] = sha(REMOTE + "/validate-request")
vals["LAUNCHREMOTE"] = sha(REMOTE + "/launch-remote")
vals["CHECKPOINT"] = sha(SC + "/wt-pbo/scripts/checkpoint.sh")
vals["PIDWAIT"] = sha(TOOLS + "/pidwait.py")
vals["DGW"] = DGW
vals["PD"] = PD
vals["WT"] = SC + "/wt-wall"
vals["TOOLS"] = TOOLS
vals["REMOTE"] = REMOTE

tpl = open(TOOLS + "/card-template.md").read()
out = re.sub(r"@@([A-Z0-9]+)@@", lambda m: vals[m.group(1)], tpl)
assert "@@" not in out
open(DGW + "/card-launch.md", "w").write(out)
print(hashlib.sha256(out.encode()).hexdigest())
