"""Fills card-template.md with the sha256 values and paths and writes the launch card to the scratchpad (the copy the executor reads) and next to the tools (the committed copy).
usage: python3 -I make_card.py   (prints the sha256 of the card; run build_requests.py first)"""
import hashlib, re
SC = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad"
WT = "/dev/shm/wt-blk"
PD = WT + "/backlog/XP-20261029-dg-blocked"
CARDS = WT + "/research/ic_dg_blocked_20261029/cards"
DGB = SC + "/dgb"
TOOLS = SC + "/dgb-card-tools"
REMOTE = "/dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote"


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


vals = {}
for b in (24, 28, 32):
    vals["REQ%d" % b] = sha("%s/request-P%d.json" % (DGB, b))
    vals["REQR%d" % b] = sha("%s/request-P%d-rho.json" % (DGB, b))
    vals["SPEC%d" % b] = sha("%s/spec-P%d.json" % (PD, b))
    vals["SPECR%d" % b] = sha("%s/spec-P%d-rho.json" % (PD, b))
vals["PROTOCOL"] = sha(PD + "/PROTOCOL.md")
vals["APPROVAL"] = sha(PD + "/APPROVAL.md")
vals["PRELAUNCH"] = sha(CARDS + "/prelaunch-checks.md")
vals["RULING"] = sha(CARDS + "/RULING-P2.md")
vals["VALREQ"] = sha(REMOTE + "/validate-request")
vals["LAUNCHREMOTE"] = sha(REMOTE + "/launch-remote")
vals["CHECKPOINT"] = sha(WT + "/scripts/checkpoint.sh")
vals["PIDWAIT"] = sha(TOOLS + "/pidwait.py")
vals["DGB"] = DGB
vals["PD"] = PD
vals["WT"] = WT
vals["TOOLS"] = TOOLS
vals["REMOTE"] = REMOTE
tpl = open(CARDS + "/tools/card-template.md").read()
out = re.sub(r"@@([A-Z0-9]+)@@", lambda m: vals[m.group(1)], tpl)
assert "@@" not in out
open(DGB + "/card-launch.md", "w").write(out)
open(CARDS + "/card-launch.md", "w").write(out)
print(hashlib.sha256(out.encode()).hexdigest())
