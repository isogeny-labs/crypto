#!/usr/bin/env python3
# SWB-RECOMPUTE-12 finalize: fills the static evidence fields (off-repo sentences, A/A lines,
# isolation lines, git add dates) by mechanical grep of the study files named in the card, then
# assembles recompute/SWB-RECOMPUTE-12.jsonl in row order. Reads only committed files.
import glob
import json
import os
import re
import subprocess

REPO = "/home/user/crypto"
B = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks")
CARD = os.path.join(B, "cards/SWB-RECOMPUTE-12.txt")
ROWS_DIR = os.path.join(B, "recompute/rows_swb12")
OUT = os.path.join(B, "recompute/SWB-RECOMPUTE-12.jsonl")
OFF = re.compile(r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|[Mm]odal volume|not committed|scratch")
AA = re.compile(r"A/A|A-A|null[- ]control|[Nn]egative control|[Pp]ositive control|\bcontrol\b|null arm|null result")
ISO = re.compile(r"isolat|\bL0\b|\bL1\b|\bL2\b|shared|uncontrolled|pinned|reserved|no isolation")
TEXT = (".md", ".typ", ".txt", ".tex", ".sh", ".py")
CAP = 8


def parse_card():
    rows = {}
    cur = None
    for line in open(CARD, encoding="utf-8"):
        m = re.match(r"--- ROW (\d+) \| (\S+) \| (\S+)", line)
        if m:
            cur = int(m.group(1))
            rows[cur] = {"slug": m.group(3), "study": [], "raw": []}
            continue
        if cur is None:
            continue
        if line.startswith("STUDY DIRS:"):
            rows[cur]["study"] = [x.strip() for x in line.split(":", 1)[1].split(";") if x.strip()]
        elif line.startswith("RAW CANDIDATES"):
            rows[cur]["raw"] = [x.strip() for x in line.split(":", 1)[1].split(",") if x.strip()]
    return rows


def text_files(paths):
    out = []
    for p in paths:
        full = os.path.join(REPO, p)
        if os.path.isdir(full):
            for root, dirs, files in os.walk(full):
                dirs.sort()
                if "/raw" in root or "/data" in root or "/sessions" in root or "/callgrind" in root:
                    continue
                for f in sorted(files):
                    if f.endswith(TEXT):
                        out.append(os.path.relpath(os.path.join(root, f), REPO))
        elif os.path.isfile(full) and full.endswith(TEXT):
            out.append(p)
    return out


def grep(files, rx, cap=CAP):
    hits = []
    for rel in files:
        for i, line in enumerate(open(os.path.join(REPO, rel), encoding="utf-8", errors="replace"), 1):
            s = line.rstrip("\n")
            if rx.search(s):
                txt = s.strip()
                hits.append("%s:%d: %s" % (rel, i, txt[:300]))
                if len(hits) >= cap:
                    return hits
    return hits


def git_add(path):
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%h %cI", "--", path],
                       capture_output=True, text=True)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else ""


card = parse_card()
out_rows = []
for n in range(89, 97):
    fn = os.path.join(ROWS_DIR, "%d.json" % n)
    row = json.load(open(fn))
    info = card[n]
    files = text_files(info["study"])
    off = grep(files, OFF, cap=20)
    if not row.get("offrepo_refs"):
        row["offrepo_refs"] = off
    if not row.get("aa_lines"):
        row["aa_lines"] = grep(files, AA)
    if not row.get("isolation_lines"):
        row["isolation_lines"] = grep(files, ISO)
    # git dates: protocol = top-level PROTOCOL* or PLAN in study dirs; first raw = earliest add over raw candidates
    proto = [f for f in files if os.path.basename(f).upper().startswith("PROTOCOL")]
    if not row.get("protocol_git_date"):
        dates = [git_add(p) for p in proto if git_add(p)]
        row["protocol_git_date"] = ("; ".join("%s [%s]" % (d, p) for d, p in zip(dates, proto))
                                    if dates else "no PROTOCOL file in study dirs")
    if not row.get("first_raw_git_date"):
        cand = []
        for p in info["raw"]:
            d = git_add(p)
            if d:
                cand.append((d, p))
        if cand:
            cand.sort(key=lambda x: x[0].split(" ")[1])
            row["first_raw_git_date"] = "%s [%s]" % (cand[0][0], cand[0][1])
        else:
            row["first_raw_git_date"] = "EMPTY (no add commit for the raw candidates)"
    if n == 91:
        import glob as g
        tot = eq = 0
        for f in glob.glob(os.path.join(REPO, STUDY := "research/ic_m3_algebraic_vs_mitm_20261014") + "/runs/m*/truth-*.json"):
            for e in json.load(open(f)).get("exhaustive_check") or []:
                tot += 1
                eq += 1 if (e.get("exhaustive") == 1 and e.get("equal") is True) else 0
        row["notes"] = (row.get("notes", "") + " Exhaustive-enumeration check (truth-*.json exhaustive_check entries with exhaustive=1 and equal true): %d of %d entries." % (eq, tot)).strip()
    out_rows.append(row)
    print(n, "off", len(row["offrepo_refs"]), "aa", len(row["aa_lines"]), "iso", len(row["isolation_lines"]),
          "| proto:", row["protocol_git_date"][:90], "| raw:", row["first_raw_git_date"][:90])
    fn_out = fn
    json.dump(row, open(fn_out, "w"), ensure_ascii=False)

with open(OUT, "w") as fh:
    for row in out_rows:
        fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("wrote", OUT, len(out_rows), "rows")
