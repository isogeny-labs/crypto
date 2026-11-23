#!/usr/bin/env python3
"""Freeze, deviation-D1, binary and raw-data provenance checks.  Writes validation/out/freeze_check.json."""
import datetime, glob, hashlib, json, os, subprocess

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
ROOT = subprocess.check_output(["git", "-C", S, "rev-parse", "--show-toplevel"], text=True).strip()
REL = os.path.relpath(S, ROOT)
FREEZE, D1, SEL, RES = "b5971d86dd", "d0ac2cd27b", "2a3fde9e0f", "469f5bf731"
PO = os.path.join(ROOT, "research/genus4_prime_order_20261118")
SP = "/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad"


def git(*a):
    return subprocess.check_output(["git", "-C", ROOT, *a], text=True)


def gitb(*a):
    return subprocess.check_output(["git", "-C", ROOT, *a])


def sha(b):
    return hashlib.sha256(b).hexdigest()


def ctime(c):
    return git("log", "-1", "--format=%cI", c).strip()


def mtime(f):
    return datetime.datetime.fromtimestamp(os.path.getmtime(f), datetime.timezone.utc).isoformat()


out = {"head": git("rev-parse", "HEAD").strip(), "branch": git("rev-parse", "--abbrev-ref", "HEAD").strip()}
out["tracked_dirty"] = git("status", "--porcelain", "--untracked-files=no", "--", REL).strip()
out["tracked_files"] = git("ls-files", "--", REL).split()
out["commits"] = {c: {"sha": git("rev-parse", c).strip(), "committed": ctime(c),
                      "subject": git("log", "-1", "--format=%s", c).strip()} for c in (FREEZE, D1, SEL, RES)}

# frozen.sha256 against the files now, and against the freeze-commit blobs
fz = {}
for line in open(os.path.join(S, "frozen.sha256")):
    h, name = line.split()
    now = open(os.path.join(S, name), "rb").read()
    blob = gitb("show", f"{FREEZE}:{REL}/{name}")
    ent = {"frozen": h, "now": sha(now), "freeze_blob": sha(blob), "now_eq_frozen": sha(now) == h,
           "freeze_blob_eq_frozen": sha(blob) == h}
    if sha(now) != h:
        rev = now.replace(b"54293", b"60013")
        ent["reversed_54293_to_60013"] = sha(rev)
        ent["reversed_eq_frozen"] = sha(rev) == h
        ent["forward_freeze_blob_eq_now"] = blob.replace(b"60013", b"54293") == now
        ent["count_54293_now"] = now.count(b"54293")
        ent["count_60013_freeze"] = blob.count(b"60013")
        ent["count_54293_freeze"] = blob.count(b"54293")
    fz[name] = ent
out["frozen"] = fz
out["changed_since_freeze"] = git("diff", "--stat", FREEZE, "HEAD", "--", REL).strip().splitlines()
out["d1_commit_files"] = git("show", "--stat", "--format=", D1).strip().splitlines()
out["deviations_first_commit"] = git("log", "--diff-filter=A", "--format=%h %cI", "--", f"{REL}/DEVIATIONS.md").strip()
out["deviations_changed_after_d1"] = git("diff", "--stat", D1, "HEAD", "--", f"{REL}/DEVIATIONS.md").strip()
out["curves_json_commits"] = git("log", "--format=%h %cI", "--", f"{REL}/curves.json").strip().splitlines()
out["tune_json_commits"] = git("log", "--format=%h %cI", "--", f"{REL}/tune.json").strip().splitlines()

# binary
B = f"{SP}/bin/genus_ladder.po"
out["binary"] = {"path": B, "sha256": sha(open(B, "rb").read()),
                 "binary_sha256_file": open(os.path.join(S, "binary.sha256")).read().split()[0],
                 "mtime": mtime(B)}
out["binary"]["match"] = out["binary"]["sha256"] == out["binary"]["binary_sha256_file"]

# raw data: every jsonl/log of the study, its hash, mtime, whether git tracks or ignores it
raw = sorted(glob.glob(os.path.join(S, "*.jsonl")) + glob.glob(os.path.join(S, "*.log"))
             + glob.glob(os.path.join(S, "failed_p60013", "*")))
tracked = set(out["tracked_files"])
man = []
for f in raw:
    r = os.path.relpath(f, ROOT)
    man.append({"path": os.path.relpath(f, S), "sha256": sha(open(f, "rb").read()), "bytes": os.path.getsize(f),
                "mtime": mtime(f), "tracked": r in tracked})
out["raw_manifest_count"] = len(man)
out["raw_tracked"] = sum(m["tracked"] for m in man)
out["raw_bytes"] = sum(m["bytes"] for m in man)
json.dump(man, open(os.path.join(V, "out", "raw_manifest.json"), "w"), indent=0)
out["raw_manifest_sha256"] = sha(open(os.path.join(V, "out", "raw_manifest.json"), "rb").read())


# timeline (file mtimes are producer-controlled; consistent ordering is weak evidence only)
def span(pat):
    fs = glob.glob(os.path.join(S, pat))
    if not fs:
        return None
    ts = sorted(os.path.getmtime(f) for f in fs)
    iso = lambda t: datetime.datetime.fromtimestamp(t, datetime.timezone.utc).isoformat()
    return {"files": len(fs), "first": iso(ts[0]), "last": iso(ts[-1])}


out["timeline"] = {k: span(v) for k, v in {
    "failed_p60013": "failed_p60013/*", "search_g4_p54293": "search_g4_p54293_*", "search_g5_p2357": "search_g5_p2357_*",
    "search_g5_p4099": "search_g5_p4099_*", "tune5": "tune5_*", "b4_p54293": "b4_p54293_*", "b5": "b5_*",
    "rho5": "rho5_*", "fig": "fig_*"}.items()}
ct = lambda c: datetime.datetime.fromisoformat(ctime(c))
ft = lambda k, w: datetime.datetime.fromisoformat(out["timeline"][k][w])
out["order_checks"] = {
    "freeze_before_any_search": all(ct(FREEZE) < ft(k, "first") for k in ("failed_p60013", "search_g5_p2357", "search_g5_p4099", "search_g4_p54293")),
    "d1_commit_before_first_p54293_file": ct(D1) < ft("search_g4_p54293", "first") and ct(D1) < ft("b4_p54293", "first"),
    "d1_seconds_before_first_p54293_file": (ft("search_g4_p54293", "first") - ct(D1)).total_seconds(),
    "searches_done_before_selection_commit": all(ft(k, "last") < ct(SEL) for k in ("search_g4_p54293", "search_g5_p2357", "search_g5_p4099")),
    "selection_commit_before_tune_and_blocks": all(ct(SEL) < ft(k, "first") for k in ("tune5", "b4_p54293", "b5", "rho5")),
    "tune_done_before_b5": ft("tune5", "last") < ft("b5", "first"),
    "results_commit_after_all_data": all(ft(k, "last") < ct(RES) for k in ("tune5", "b4_p54293", "b5", "rho5")),
}

# re-used genus-4 rows of the validated prime-order study: hashes against that validator's pins
pins = {os.path.basename(d["path"]): d["sha256"] for d in json.load(open(os.path.join(
    ROOT, "benchmarks/20261118-genus4-prime-order/validator-report.json")))["raw_data"]}
reuse = {}
for n in ("b_p16411_x.jsonl.gz", "b_p16411_xstar.jsonl.gz", "b_p32771_x.jsonl.gz", "b_p32771_xstar.jsonl.gz"):
    h = sha(open(os.path.join(PO, n), "rb").read())
    reuse[n] = {"sha256": h, "pinned": pins.get(n), "match": h == pins.get(n),
                "tracked": f"research/genus4_prime_order_20261118/{n}" in git("ls-files", "--", f"research/genus4_prime_order_20261118/{n}")}
out["reused_prime_order_rows"] = reuse
json.dump(out, open(os.path.join(V, "out", "freeze_check.json"), "w"), indent=1)
print(json.dumps({k: out[k] for k in ("head", "tracked_dirty", "binary", "order_checks", "raw_manifest_count", "raw_tracked")}, indent=1))
for k, v in fz.items():
    print(k, {x: v[x] for x in v if x not in ("frozen", "now", "freeze_blob")})
print(json.dumps(reuse, indent=1))
