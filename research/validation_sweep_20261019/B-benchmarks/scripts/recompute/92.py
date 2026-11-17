#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 92 (20261014-ecbench-rho-optimized).
# Reads committed Callgrind dumps (summary: lines), comparison JSON and the replay audit.
import glob
import json
import os
import re
import subprocess

REPO = "/home/user/crypto"
STUDY = "research/ecbench_rho_optimized_20261014"
BASE = os.path.join(REPO, STUDY)
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/92.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/92.py"
CMP = STUDY + "/sessions/run-20261014T012000Z/comparisons/rho-legacy__rho-strong.json"
AA = STUDY + "/sessions/run-20261014T012000Z/comparisons/rho-strong__rho-strong-aa.json"
AUD = STUDY + "/sessions/run-20261014T012000Z/audit-replay-all.json"


def summary_ir(path):
    with open(path) as fh:
        for line in fh:
            m = re.match(r"summary:\s+(\d+)", line)
            if m:
                return int(m.group(1))
    raise SystemExit("no summary line in " + path)


def parts(dirname, stem):
    files = sorted(glob.glob(os.path.join(BASE, "callgrind", dirname, stem + "*")))
    files = [f for f in files if not f.endswith(".json")]
    out = {}
    for f in files:
        out[os.path.basename(f)] = summary_ir(f)
    return out


opt = parts("final-optimized", "candidate")
leg = parts("final-legacy", "legacy")
base = parts("baseline-7badd44a3", "baseline")
for name, d in [("final-optimized", opt), ("final-legacy", leg), ("baseline", base)]:
    print(name, d, "sum all", sum(d.values()))

# subset that reproduces the headline totals: all parts except the unsuffixed dump and .1
def subset(d):
    return sum(v for k, v in d.items() if k.split(".")[-1] in ("2", "3", "4", "5") and "." in k)

s_opt, s_leg, s_base = subset(opt), subset(leg), subset(base)
print("subset(.2-.5) opt", s_opt, "leg", s_leg, "base", s_base)
print("ratio subset opt/leg", s_opt / s_leg, "opt/base", s_opt / s_base)
print("ratio all-parts opt/leg", sum(opt.values()) / sum(leg.values()), "opt/base", sum(opt.values()) / sum(base.values()))
print("reduction subset", 100 * (1 - s_opt / s_leg))

cmp_ = json.load(open(os.path.join(REPO, CMP)))
cmp_aa = json.load(open(os.path.join(REPO, AA)))
aud = json.load(open(os.path.join(REPO, AUD)))
replays = aud["replays"]
n_cmp = len(replays)
n_eq = sum(1 for r in replays if r.get("reproduced") and r.get("detail") == "identical")
gae = cmp_["ops"]
wall = cmp_["wall"]
aa_wall = cmp_aa["wall"]
print("replays", n_cmp, "identical", n_eq)
print("GAE", gae["ratio_b_over_a"], gae["ci95"], gae["status"], "pairs", gae["pairs"])
print("wall median", wall["median_ratio"], "status", wall["status"])
print("A/A wall median", aa_wall["median_ratio"], "A/A ops", cmp_aa["ops"]["ratio_b_over_a"], cmp_aa["ops"]["ci95"])

REASON = ("the study does not state which Callgrind dump parts form the complete-solve interval; "
          "the subset parts .2-.5 (no unsuffixed dump, no .1) reproduces the headline, but that choice is not stated in the repo, "
          "and all six parts give different values")
def cg_claim(claimed, computed_sub, computed_all, files):
    return {"claimed": claimed,
            "computed": "subset(.2-.5) %s ; all parts %s" % (computed_sub, computed_all),
            "definition": "Callgrind summary: totals per dump file; complete-solve interval rule not stated in repo",
            "files": files, "status": "cannot_define"}

CG_FILES = [STUDY + "/callgrind/final-optimized/candidate.2", STUDY + "/callgrind/final-legacy/legacy.2",
            STUDY + "/callgrind/baseline-7badd44a3/baseline.2"]
claims = [
    cg_claim("0.802155 times complete-solve Callgrind Ir (optimized/legacy)",
             "%.6f" % (s_opt / s_leg), "%.6f" % (sum(opt.values()) / sum(leg.values())), CG_FILES[:2]),
    cg_claim("1,600,543,701 / 1,995,304,985 callgrind.Ir",
             "%d / %d" % (s_opt, s_leg), "%d / %d" % (sum(opt.values()), sum(leg.values())), CG_FILES[:2]),
    cg_claim("19.785% reduction",
             "%.4f%%" % (100 * (1 - s_opt / s_leg)), "%.4f%%" % (100 * (1 - sum(opt.values()) / sum(leg.values()))), CG_FILES[:2]),
    cg_claim("optimized/pre-change 0.804582",
             "%.6f" % (s_opt / s_base), "%.6f" % (sum(opt.values()) / sum(base.values())), CG_FILES),
    {"claimed": "GAE 1.0000 [1.0000, 1.0000]",
     "computed": "ratio_b_over_a %.4f ci95 [%.4f, %.4f] pairs %d status %s" % (gae["ratio_b_over_a"], gae["ci95"][0], gae["ci95"][1], gae["pairs"], gae["status"]),
     "definition": "ops block of comparisons/rho-legacy__rho-strong.json (ecbench.gae, cluster bootstrap)",
     "files": [CMP], "status": "match" if abs(gae["ratio_b_over_a"] - 1.0) < 5e-5 and gae["ci95"] == [1.0, 1.0] else "mismatch"},
    {"claimed": "native 0.9586 wall median (L0 descriptive)",
     "computed": "%.4f" % wall["median_ratio"],
     "definition": "wall.median_ratio of comparisons/rho-legacy__rho-strong.json",
     "files": [CMP], "status": "match" if round(wall["median_ratio"], 4) == 0.9586 else "mismatch"},
]
# identical claim: 720/720 measured replays identical
identical = {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
             "files": [AUD], "status": "match" if n_cmp == 720 and n_eq == 720 else "mismatch"}
for c in claims + [{"claimed": "identical replays", "computed": "%d/%d" % (n_eq, n_cmp), "definition": "", "files": [], "status": identical["status"]}]:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

row = {
    "row_no": 92,
    "slug": "20261014-ecbench-rho-optimized",
    "claims": claims,
    "identical": identical,
    "offrepo_refs": [],
    "aa_lines": [
        "research/ecbench_rho_optimized_20261014/README.md:20: \"no-fallback clause; its 0.9586 optimized/legacy median and 0.9997 optimized A/A\" (A/A median; A/A comparison wall median recomputed: %.4f)" % aa_wall["median_ratio"],
        "research/ecbench_rho_optimized_20261014/PROTOCOL.md:37: \"`rho-strong-aa` repeats the optimized method and supplies the wall-time\"",
    ],
    "isolation_lines": [
        "research/ecbench_rho_optimized_20261014/README.md:17: \"The native wall gate remains **unknown**. This VM exposes no Linux PSI, so the\"",
        "research/ecbench_rho_optimized_20261014/README.md:18: \"preregistered L2 run refused before creating a session. A retained L0\"",
        "research/ecbench_rho_optimized_20261014/README.md:31: table row \"Native wall | unknown; zero L2 pairs | not admitted\"",
    ],
    "protocol_git_date": "ca00c1517 2026-10-17T10:19:32-07:00 (same commit as records.jsonl and callgrind/final-optimized/candidate.3; PROTOCOL.md is not earlier)",
    "first_raw_git_date": "ca00c1517 2026-10-17T10:19:32-07:00 (sessions/run-20261014T012000Z/records.jsonl, same commit)",
    "script": SCRIPT,
    "notes": ("Complete-solve Callgrind rule: " + REASON + ". "
              "Callgrind rows use summary: lines of the committed dump files. "
              "The A/A comparison (rho-strong__rho-strong-aa.json) is ratio 1.0 cluster and its wall median is recomputed above. "
              "Off-repo grep (drive.google, /Volumes, SSD, /Users, s3://, gs://, store:, modal, not committed, scratch) on README.md, PROTOCOL.md, METHODS.txt, TESTS.md: no matches. "
              "The 1.0000 GAE claim is the diagnostic ops ratio; the wall comparison is descriptive (240 of 240 pairs below L2)."),
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("wrote", OUT)
