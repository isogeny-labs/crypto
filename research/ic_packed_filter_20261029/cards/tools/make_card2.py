"""Stages the analysis helper files into the scratchpad tools directory, fills card2-template.md with the sha256 values and paths, and writes the card for the
work after the runs to the scratchpad (the copy the executor reads) and next to the tools as ../card-analysis.md (the committed copy).
usage: python3 -I make_card2.py   (prints the sha256 of the card; write prelaunch-checks.md first)"""
import hashlib, os, re, shutil

SC = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad"
WT = "/dev/shm/src-pf"
PD = WT + "/backlog/XP-20261029-packed-filter"
STUDY = WT + "/research/ic_packed_filter_20261029"
CARDS = STUDY + "/cards"
SRC = CARDS + "/tools"
PFD = SC + "/pf"
ATOOLS = SC + "/pf-analysis-tools"
FB = "/dev/shm/pf-bundle/ecbench"
FILES = ["analysis-prelude.sh", "session_facts.py", "project_compare.py", "records_checks.py", "work_checks.py", "sprime.py", "positions.py", "wall_gate.py", "counted_gate.py", "phase_sums.py", "receipt.py", "project_facts.py"]


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def const(name):
    return re.search(r"^%s=(\S+)$" % name, open(SRC + "/launch-prelude.sh").read(), re.M).group(1)


os.makedirs(ATOOLS, exist_ok=True)
for f in FILES:
    shutil.copyfile(SRC + "/" + f, ATOOLS + "/" + f)
pins = []
hash_lines = []
for name in ["PROTOCOL.md", "APPROVAL.md"] + ["spec-P%d%s.json" % (b, k) for b in (24, 28, 32) for k in ("", "-rho")]:
    p = PD + "/" + name
    pins.append("  - %s %s" % (name, sha(p)))
    hash_lines.append("%s  %s" % (sha(p), p))
p = CARDS + "/prelaunch-checks.md"
pins.append("  - cards/prelaunch-checks.md %s" % sha(p))
hash_lines.append("%s  %s" % (sha(p), p))
for f in FILES:
    p = ATOOLS + "/" + f
    pins.append("  - %s %s" % (f, sha(p)))
    hash_lines.append("%s  %s" % (sha(p), p))
pins.append("  - ecbench %s" % sha(FB))
hash_lines.append("%s  %s" % (sha(FB), FB))
assert sha(FB) == const("FBSHA")
vals = {
    "FREEZE": const("FREEZE"), "REV": const("REV"), "IMAGE": const("IMAGE"), "FBSHA": const("FBSHA"), "FB": FB,
    "PD": PD, "STUDY": STUDY, "ATOOLS": ATOOLS, "CARDPATH": PFD + "/card-analysis.md",
    "PINS": "\n".join(pins), "HASHLINES": "\n".join(hash_lines), "NHASH": str(len(hash_lines)),
}
tpl = open(SRC + "/card2-template.md").read()
out = re.sub(r"@@([A-Z0-9]+)@@", lambda m: vals[m.group(1)], tpl)
assert "@@" not in out
open(PFD + "/card-analysis.md", "w").write(out)
open(CARDS + "/card-analysis.md", "w").write(out)
print(hashlib.sha256(out.encode()).hexdigest())
