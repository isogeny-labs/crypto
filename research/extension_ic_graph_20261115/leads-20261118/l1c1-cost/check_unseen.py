"""Freshness gate: a generated public input must not equal, or share a curve with,
any public input this repository has ever committed or dry-run (every git-tracked
*public*.json, every line of every git-tracked *publics*.jsonl -- this includes
the 96 l1-paired targets in ../l1-paired/results/publics.jsonl -- and every
public.json inside a dry-run tarball), or another candidate of this run.
Glue only (python3 -I; equality checks). Derived from ../l1-paired/check_unseen.py,
with the *publics*.jsonl source added.
Usage: python3 -I check_unseen.py REPO CANDIDATE.json...
Prints one JSON line per candidate; exit 0 only if every candidate is unseen."""
import io, json, os, subprocess, sys, tarfile

repo, cands = sys.argv[1], sys.argv[2:]


def key(v):
    return json.dumps([v["p"], v["l"], v["alpha"], v["g"], v["q"]], separators=(",", ":"))


def curve(v):
    return json.dumps([v["p"], v["alpha"]], separators=(",", ":"))


known_keys, known_curves, sources = set(), set(), 0


def add(v):
    global sources
    if isinstance(v, dict) and all(k in v for k in ("p", "l", "alpha", "g", "q")):
        known_keys.add(key(v))
        known_curves.add(curve(v))
        sources += 1


def git_ls(pattern):
    return subprocess.run(["git", "-C", repo, "ls-files", pattern], capture_output=True, text=True, check=True).stdout.split()


for f in git_ls("*public*.json"):
    try:
        add(json.load(open(os.path.join(repo, f))))
    except Exception:
        continue
jsonl_lines = 0
for f in git_ls("*publics*.jsonl"):
    for line in open(os.path.join(repo, f)):
        line = line.strip()
        if line:
            add(json.loads(line))
            jsonl_lines += 1
for t in git_ls("*.tar.gz"):
    if "dry-run" not in t:
        continue
    with tarfile.open(os.path.join(repo, t)) as tf:
        for m in tf.getmembers():
            if m.isfile() and m.name.endswith("public.json"):
                add(json.load(io.TextIOWrapper(tf.extractfile(m))))
if jsonl_lines < 96:
    print(json.dumps({"error": "the l1-paired publics were not found", "jsonl_lines": jsonl_lines}))
    sys.exit(2)
bad = 0
own_keys, own_curves = set(), set()
for c in cands:
    v = json.load(open(c))
    k, cv = key(v), curve(v)
    ok = k not in known_keys and cv not in known_curves and k not in own_keys and cv not in own_curves
    own_keys.add(k)
    own_curves.add(cv)
    bad += 0 if ok else 1
    print(json.dumps({"candidate": c, "unseen": ok, "known_inputs": sources}))
sys.exit(0 if bad == 0 else 1)
