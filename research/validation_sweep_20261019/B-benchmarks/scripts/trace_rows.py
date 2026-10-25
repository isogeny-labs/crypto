#!/usr/bin/env python3
"""Deterministic tracer for the benchmarks/README.md audit (slice B).

Reads benchmarks/README.md, and for each requested row resolves the evidence
directories the row links, lists what raw data is committed, and records the
literal keyword hits (A/A control, isolation words, 'identical', Drive/SSD
references, matched rho). It judges nothing: every field is a count, a path
or a quoted line. Usage: trace_rows.py <first_row> <last_row> <out.jsonl>
(row numbers are 1-based positions among the table rows, README order).
"""
import json, os, re, sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
BENCH = REPO / "benchmarks"
RAW_EXT = {".jsonl", ".json", ".csv", ".tsv", ".log", ".txt", ".tar", ".gz", ".zst", ".zip", ".bin", ".parquet", ".yaml", ".yml", ".out", ".xz", ".tgz", ".sha256", ".tsv", ".ndjson"}
STRICT_EXT = {".jsonl", ".csv", ".tsv", ".ndjson", ".tar", ".gz", ".xz", ".zst", ".zip", ".tgz", ".out", ".log"}
DOC_EXT = {".md", ".txt", ".yaml", ".yml", ".json", ".tex"}
KEYS = {
    "aa": r"\bA/A\b|A-A control|a/a arm|null[- ]control",
    "isolated_bench": r"isolated_bench|isolab",
    "tier_L0": r"\bL0\b", "tier_L1": r"\bL1\b(?! cache)", "tier_L2": r"\bL2\b(?! cache)", "tier_L3": r"\bL3\b(?! cache)",
    "contended": r"\bcontended\b", "uncontended": r"uncontended",
    "no_isolation": r"no isolation|unqualified|shared VM|not isolated|no PMU",
    "identical": r"bit-identical|byte-identical|identical (output|result|count|digest|fingerprint)|0 mismatch|outputs? (are )?identical",
    "frozen": r"frozen|pre-?regist|PROTOCOL|decision rule",
    "offrepo": r"drive\.google|store:[a-z]|Google Drive|gdrive|drive\.google|\bSSD\b|/Volumes/|/mnt/|s3://|gs://|not committed|not in the repo|off-repo|local disk|LFS|provider ZIP|\bstore\b.*sha",
    "rho": r"\brho\b|Pollard",
    "matched_rho": r"matched rho|same[- ]protocol rho|rho reference|rho control",
    "class_word": r"\b(advance|engineering|relabelling|relabeling|accounting)\b",
}
RATIO = re.compile(r"\d+(?:\.\d+)?\s*[×x](?![a-z0-9_])")

def table_rows():
    if len(sys.argv) > 4:
        return json.load(open(sys.argv[4]))
    out = []
    for i, l in enumerate((BENCH / "README.md").read_text().splitlines(), 1):
        if l.startswith("| 20"):
            c = [x.strip() for x in l.strip().strip("|").split(" | ")]
            out.append(dict(readme_line=i, date=c[0], entry=c[1], measured=c[2], headline=c[3], host=c[4], study=c[5]))
    return out

def links(cell):
    return [m for m in re.findall(r"\]\(([^)]+)\)", cell)] + re.findall(r"`((?:research|docs|results|benchmarks)/[^`]+)`", cell)

def resolve(p):
    p = p.split("#")[0]
    if p.startswith("http"): return None
    q = (BENCH / p).resolve() if not p.startswith(("research/", "docs/", "results/", "benchmarks/")) else (REPO / p).resolve()
    try: q.relative_to(REPO)
    except ValueError: return None
    return q

def scan_dir(d, cap=4000):
    files = []
    for root, _, fs in os.walk(d):
        for f in fs:
            files.append(Path(root) / f)
            if len(files) >= cap: break
    return files

def keyhits(text):
    return {k: len(re.findall(v, text, re.I)) for k, v in KEYS.items()}

def main():
    a, b, out = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3]
    rows = table_rows()
    with open(out, "w") as fo:
        for n in range(a, b + 1):
            r = rows[n - 1]
            slug = re.search(r"`(\d{8}-[^`]+)`", r["entry"]).group(1)
            dirs = {}
            for l in links(r["study"]) + links(r["entry"]):
                q = resolve(l)
                if q is not None: dirs[str(q.relative_to(REPO))] = q
            bdir = BENCH / slug
            dirs[str(bdir.relative_to(REPO))] = bdir
            ev = {}
            texts = r["measured"] + "\n" + r["headline"] + "\n" + r["host"] + "\n"
            texts_by = {"row": texts}
            for name, q in dirs.items():
                if q.is_file():
                    par = q.parent
                    study_parent = par.relative_to(REPO).parts[:1] in (("research",), ("ecc2k130",), ("gpu",), ("paper",), ("papers",), ("isogeny_algos",)) and len(par.relative_to(REPO).parts) >= 2
                    files = scan_dir(par) if study_parent else [q]; root = par
                else:
                    files = scan_dir(q) if q.exists() else []; root = q
                raw = [f for f in files if f.suffix in RAW_EXT and f.stat().st_size > 0]
                big = sum(f.stat().st_size for f in files)
                doc_hits = {}
                for f in files:
                    if f.suffix in DOC_EXT and f.stat().st_size < 2_000_000:
                        try: t = f.read_text(errors="replace")
                        except Exception: continue
                        for k, v in keyhits(t).items(): doc_hits[k] = doc_hits.get(k, 0) + v
                        texts_by[str(f.relative_to(REPO))] = t
                ev[name] = dict(exists=q.exists(), n_files=len(files), bytes=big,
                                n_raw=len(raw), n_raw_strict=sum(f.suffix in STRICT_EXT or f.name in ("measurements.json","results.json","summary.json","raw.json") for f in files), raw_examples=[str(f.relative_to(REPO)) for f in sorted(raw, key=lambda f: -f.stat().st_size)[:6]],
                                pdfs=[str(f.relative_to(REPO)) for f in files if f.suffix == ".pdf"][:4],
                                svg=sum(f.suffix == ".svg" for f in files), png=sum(f.suffix == ".png" for f in files),
                                keyword_hits_in_docs=doc_hits)
            quotes = {}
            for k in ("aa", "offrepo", "identical", "tier_L0", "tier_L1", "tier_L2", "tier_L3", "isolated_bench", "no_isolation", "class_word"):
                q = []
                for where, t in texts_by.items():
                    for line in t.splitlines():
                        if re.search(KEYS[k], line, re.I):
                            q.append(f"{where}: {line.strip()[:220]}")
                            if len(q) >= 4: break
                    if len(q) >= 4: break
                quotes[k] = q
            rec = dict(row_no=n, readme_line=r["readme_line"], date=r["date"], slug=slug, host=r["host"],
                       headline_ratios=RATIO.findall(r["headline"])[:12], row_hits=keyhits(texts),
                       evidence=ev, quotes=quotes)
            fo.write(json.dumps(rec) + "\n")
main()
