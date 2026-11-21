"""Validator helpers for research/genus4_prime_order_20261118 (shared by the validation scripts).

Every raw file is read as a git blob at COMMIT, never from the working tree.
"""
import gzip, hashlib, json, os, subprocess

REPO = "/home/user/crypto"
COMMIT = "7c0e84dedacc4acd5d6d587bfc6015fcbd2247ba"
FREEZE = "6da1e3c5b28342e2afd3a8753f844be9398bc5b8"
BIN_COMMIT = "ca722678990abd7c2396310808a84c20b2457398"
D = "research/genus4_prime_order_20261118"
OUT = os.path.join(REPO, D, "validation", "out")


def git(*a):
    return subprocess.run(["git", "-C", REPO, *a], check=True, capture_output=True).stdout


def blob(path, commit=COMMIT):
    return git("show", f"{commit}:{path}")


def sha(b):
    return hashlib.sha256(b).hexdigest()


def tracked(commit=COMMIT, prefix=D):
    return sorted(git("ls-tree", "-r", "--name-only", commit, prefix + "/").decode().split())


def rows_of(path, commit=COMMIT):
    """Rows of one committed .jsonl.gz file, with its sha256."""
    b = blob(path, commit)
    txt = gzip.decompress(b).decode()
    return [json.loads(l) for l in txt.splitlines() if l.strip()], sha(b)


def raw_files(commit=COMMIT):
    return [p for p in tracked(commit) if p.endswith(".jsonl.gz")]


def dump(name, obj):
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, name), "w") as f:
        json.dump(obj, f, indent=1, sort_keys=False)
        f.write("\n")
