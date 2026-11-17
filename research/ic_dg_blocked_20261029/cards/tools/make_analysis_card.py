"""Stages the analysis helper files into the scratchpad tools directory, fills analysis-template.md with the sha256 values and writes the analysis card
to the scratchpad (the copy the executor reads) and next to the tools (the committed copy).
usage: python3 -I make_analysis_card.py   (prints the sha256 of the card)"""
import hashlib, os, re, shutil
SC = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad"
WT = "/dev/shm/wt-blk"
PD = WT + "/backlog/XP-20261029-dg-blocked"
CARDS = WT + "/research/ic_dg_blocked_20261029/cards"
SRC = CARDS + "/tools"
DGB = SC + "/dgb"
ATOOLS = SC + "/dgb-analysis-tools"
FILES = ["prelude.sh", "dgb_facts.py", "project_compare.py", "records_checks.py", "wall_gate.py", "counted_gate.py", "phase_sums.py", "receipt.py", "project_facts.py"]


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


os.makedirs(ATOOLS, exist_ok=True)
vals = {}
for f in FILES:
    shutil.copyfile(SRC + "/" + f, ATOOLS + "/" + f)
    vals["H_" + f.rsplit(".", 1)[0]] = sha(ATOOLS + "/" + f)
for b in (24, 28, 32):
    vals["SPEC%d" % b] = sha("%s/spec-P%d.json" % (PD, b))
    vals["SPECR%d" % b] = sha("%s/spec-P%d-rho.json" % (PD, b))
vals["PROTOCOL"] = sha(PD + "/PROTOCOL.md")
vals["APPROVAL"] = sha(PD + "/APPROVAL.md")
vals["PRELAUNCH"] = sha(CARDS + "/prelaunch-checks.md")
vals["RULING"] = sha(CARDS + "/RULING-P2.md")
vals["DGB"] = DGB
vals["PD"] = PD
vals["WT"] = WT
vals["ATOOLS"] = ATOOLS
tpl = open(SRC + "/analysis-template.md").read()
out = re.sub(r"@@([A-Za-z0-9_]+)@@", lambda m: vals[m.group(1)], tpl)
assert "@@" not in out
open(DGB + "/card-analysis.md", "w").write(out)
open(CARDS + "/card-analysis.md", "w").write(out)
print(hashlib.sha256(out.encode()).hexdigest())
