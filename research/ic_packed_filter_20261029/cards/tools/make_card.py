"""Stages the launch helper files into the scratchpad tools directory, fills card-template.md with the sha256 values and paths, and writes the
launch card to the scratchpad (the copy the executor reads) and next to the tools (the committed copy).
usage: python3 -I make_card.py   (prints the sha256 of the card; run build_requests.py and write prelaunch-checks.md first)"""
import hashlib, os, re, shutil

SC = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad"
WT = "/dev/shm/src-pf"
PD = WT + "/backlog/XP-20261029-packed-filter"
STUDY = WT + "/research/ic_packed_filter_20261029"
CARDS = STUDY + "/cards"
SRC = CARDS + "/tools"
PFD = SC + "/pf"
TOOLS = SC + "/pf-card-tools"
REMOTE = WT + "/taskq/deploy/eks-karpenter/remote"
FILES = ["launch-prelude.sh", "pidwait.py", "rerun_trigger.py", "trigger.jq"]


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def const(name):
    m = re.search(r"^%s=(\S+)$" % name, open(SRC + "/launch-prelude.sh").read(), re.M)
    return m.group(1)


os.makedirs(TOOLS, exist_ok=True)
for f in FILES:
    shutil.copyfile(SRC + "/" + f, TOOLS + "/" + f)

req_tags = []
for b in (24, 28, 32):
    for t in ("", "-rho", "-r2", "-rho-r2", "-nr", "-nr-r2"):
        req_tags.append("P%d%s" % (b, t))
hash_lines = []  # (hash, path)
req_rows = []
for t in req_tags:
    p = "%s/request-%s.json" % (PFD, t)
    h = sha(p)
    hash_lines.append((h, p))
    req_rows.append("- request-%s.json %s" % (t, h))
pd_rows = []
for name in ["PROTOCOL.md", "APPROVAL.md"] + ["spec-P%d%s.json" % (b, k) for b in (24, 28, 32) for k in ("", "-rho")]:
    p = PD + "/" + name
    h = sha(p)
    hash_lines.append((h, p))
    pd_rows.append("- %s %s" % (name, h))
other_rows = []
others = [
    ("research/ic_packed_filter_20261029/cards/prelaunch-checks.md", CARDS + "/prelaunch-checks.md"),
    ("REMOTE/validate-request", REMOTE + "/validate-request"),
    ("REMOTE/launch-remote", REMOTE + "/launch-remote"),
    ("WT/scripts/checkpoint.sh", WT + "/scripts/checkpoint.sh"),
] + [("TOOLS/" + f, TOOLS + "/" + f) for f in FILES]
for label, p in others:
    h = sha(p)
    hash_lines.append((h, p))
    other_rows.append("- %s %s" % (label, h))

vals = {
    "FREEZE": const("FREEZE"),
    "BRANCH": const("BRANCH"),
    "REV": const("REV"),
    "SRC": WT,
    "REMOTE": REMOTE,
    "IMAGE": const("IMAGE"),
    "FBSHA": const("FBSHA"),
    "RUNURL": "https://github.com/isogeny-labs/crypto/blob/%s/backlog/XP-20261029-packed-filter/PROTOCOL.md" % const("FREEZE"),
    "PFD": PFD,
    "TOOLS": TOOLS,
    "PD": PD,
    "WT": WT,
    "STUDY": STUDY,
    "CARDPATH": PFD + "/card-launch.md",
    "REQTABLE": "\n".join(req_rows),
    "PDTABLE": "\n".join(pd_rows),
    "OTHERTABLE": "\n".join(other_rows),
    "HASHLINES": "\n".join("%s  %s" % (h, p) for h, p in hash_lines),
    "NHASH": str(len(hash_lines)),
    "TRIGGERJQ": open(SRC + "/trigger.jq").read().rstrip("\n"),
}
tpl = open(SRC + "/card-template.md").read()
out = re.sub(r"@@([A-Z0-9]+)@@", lambda m: vals[m.group(1)], tpl)
assert "@@" not in out
open(PFD + "/card-launch.md", "w").write(out)
open(CARDS + "/card-launch.md", "w").write(out)
print(hashlib.sha256(out.encode()).hexdigest())
