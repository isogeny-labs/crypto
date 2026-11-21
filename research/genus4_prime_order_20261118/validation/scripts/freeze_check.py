#!/usr/bin/env python3
"""C1: freeze and order.  Frozen files hash to frozen.sha256 at the freeze commit and at COMMIT;
the freeze commit holds no data; every data file and every deviation enters in a later commit;
each deviation precedes the data it concerns.  Writes out/freeze_check.json."""
import re, sys
sys.path.insert(0, __import__("os").path.dirname(__file__))
from common import *

res = {"commit": COMMIT, "freeze": FREEZE, "checks": {}, "fail": []}
fz = blob(f"{D}/frozen.sha256", FREEZE).decode().split("\n")
want = {l.split()[1]: l.split()[0] for l in fz if l.strip()}
res["frozen_list"] = want
for name, h in want.items():
    a = sha(blob(f"{D}/{name}", FREEZE)); b = sha(blob(f"{D}/{name}", COMMIT))
    ok = a == h == b
    res["checks"][f"hash:{name}"] = {"recorded": h, "at_freeze": a, "at_commit": b, "ok": ok}
    if not ok:
        res["fail"].append(f"hash {name}")
for meta in ("frozen.sha256", "binary.sha256"):
    log = git("log", "--format=%H", f"{FREEZE}^..{COMMIT}", "--", f"{D}/{meta}").decode().split()
    ok = log == [FREEZE]
    res["checks"][f"unchanged:{meta}"] = {"commits_touching": log, "ok": ok}
    if not ok:
        res["fail"].append(f"{meta} changed after freeze")
res["binary_sha256_line"] = blob(f"{D}/binary.sha256", FREEZE).decode().strip()

# the freeze commit's files, and its parent's study files
fc = git("show", "--name-only", "--format=", FREEZE).decode().split()
parent_has = git("ls-tree", "-r", "--name-only", f"{FREEZE}^", D + "/").decode().split()
res["freeze_commit_files"] = fc
data_in_freeze = [f for f in fc if f.endswith((".jsonl", ".jsonl.gz", ".json")) and "/" + D.split("/")[-1] in "/" + f]
res["checks"]["freeze_has_no_data"] = {"data_files": data_in_freeze, "ok": not data_in_freeze}
res["checks"]["parent_has_no_study_files"] = {"files": parent_has, "ok": not parent_has}
src_after = git("diff", "--name-only", BIN_COMMIT, COMMIT, "--", "src", "Cargo.toml", "Cargo.lock", "crates").decode().split()
res["checks"]["no_source_change_bin_commit_to_commit"] = {"files": src_after, "ok": not src_after}
res["checks"]["bin_commit_is_freeze_parent"] = {"ok": git("rev-parse", f"{FREEZE}^").decode().strip() == BIN_COMMIT}

# first commit of each file under D
order = git("rev-list", "--reverse", "--topo-order", f"{FREEZE}^..{COMMIT}").decode().split()
idx = {c: i for i, c in enumerate(order)}
first = {}
for c in order:
    for f in git("show", "--name-only", "--format=", "--diff-filter=A", c).decode().split():
        if f.startswith(D + "/") and f not in first:
            first[f] = c
res["first_commit"] = {f[len(D) + 1:]: c[:10] for f, c in sorted(first.items())}
data = [f for f in first if f.endswith(".jsonl.gz")]
late = [f for f in data if idx[first[f]] <= idx[FREEZE]]
res["checks"]["all_data_after_freeze"] = {"n_data_files": len(data), "not_after": late, "ok": not late}

# deviations: the commit that added each Dn paragraph
dev_commits = git("log", "--reverse", "--format=%H", f"{FREEZE}..{COMMIT}", "--", f"{D}/DEVIATIONS.md").decode().split()
devs = {}
for c in dev_commits:
    txt = blob(f"{D}/DEVIATIONS.md", c).decode()
    for m in re.finditer(r"\*\*(D\d+) \(([^)]*)\)", txt):
        devs.setdefault(m.group(1), {"commit": c[:10], "stamp": m.group(2)})
res["deviations"] = devs
chunks_first = first.get(f"{D}/search_chunks.sh")
fixes = git("log", "--reverse", "--format=%H", f"{FREEZE}..{COMMIT}", "--", f"{D}/search_chunks.sh").decode().split()
res["search_chunks_commits"] = [c[:10] for c in fixes]
def first_of(prefix):
    cs = [first[f] for f in data if f[len(D) + 1:].startswith(prefix)]
    return min(cs, key=lambda c: idx[c]) if cs else None
groups = {"D1 concerns residue-3 re-scan (search_p32771_r1-1800-3_*)": ("D1", "search_p32771_r1-1800-3_"),
          "D2 concerns block 1801-2200 (search_p32771_r1801-2200-all_*)": ("D2", "search_p32771_r1801-2200-all_"),
          "D2 concerns block 2201-2600 (search_p32771_r2201-2600-all_*)": ("D2", "search_p32771_r2201-2600-all_")}
for label, (dn, pre) in groups.items():
    dc = next(c for c in dev_commits if c.startswith(devs[dn]["commit"]))
    fc_ = first_of(pre)
    res["checks"][f"order:{label}"] = {"deviation_commit": dc[:10], "data_first_commit": fc_[:10],
                                       "deviation_strictly_before": idx[dc] < idx[fc_],
                                       "same_commit": dc == fc_}
# the re-scan script (as fixed) precedes the re-scan data
last_fix = fixes[-1]
res["checks"]["search_chunks_fixed_before_rescan_data"] = {
    "last_change": last_fix[:10], "rescan_first": first_of("search_p32771_r1-1800-3_")[:10],
    "ok": idx[last_fix] < idx[first_of("search_p32771_r1-1800-3_")]}
# commit messages of the D2-relevant commits (the rule-driven extension announced before its data?)
res["d202_message"] = git("log", "-1", "--format=%s", "d202be88ca").decode().strip()
res["b4e4_message"] = git("log", "-1", "--format=%s", "b4e483061d").decode().strip()
dump("freeze_check.json", res)
print(json.dumps({k: v.get("ok", v) for k, v in res["checks"].items()}, indent=1))
print("deviations", devs, "fail", res["fail"])
