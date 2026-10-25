# Shared helpers for the SWB-RECOMPUTE-06 row scripts. Read-only over committed files.
import os, re, subprocess, json

REPO = "/home/user/crypto"
TEXT_EXT = (".md", ".typ", ".sh", ".py", ".txt", ".tex")
OFFREPO_PAT = re.compile(r"drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch")
AA_PAT = re.compile(r"A/A|A-A|[Nn]ull[- ]control|NULL control|[Nn]egative control|null arm|null result|null-vs-null|A/A|identical null|null sweep|null \(")
ISO_PAT = re.compile(r"isolat|taskset|pinned|NUMA|reserved CPU|shared host|contended|unisolated|\bL0\b|\bL1\b|\bL2\b|\bL3\b")


def _walk_text(paths):
    for p in paths:
        full = os.path.join(REPO, p)
        if os.path.isdir(full):
            for root, dirs, files in os.walk(full):
                dirs.sort()
                if any(x in root for x in ("/raw", "/out", "/replay", "/results/")):
                    continue
                for f in sorted(files):
                    if f.endswith(TEXT_EXT):
                        yield os.path.relpath(os.path.join(root, f), REPO)
        elif os.path.isfile(full) and full.endswith(TEXT_EXT):
            yield p


def grep_lines(paths, pat, cap=None):
    out = []
    for rel in _walk_text(paths):
        full = os.path.join(REPO, rel)
        try:
            with open(full, encoding="utf-8", errors="replace") as fh:
                for i, line in enumerate(fh, 1):
                    s = line.rstrip("\n")
                    if pat.search(s):
                        out.append(f"{rel}:{i}: {s}")
        except OSError as e:
            out.append(f"{rel}: UNREADABLE {e}")
    return out[:cap] if cap else out


def git_first_add(path):
    """Commit date of the first commit that added path (read-only git log)."""
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", path],
                       capture_output=True, text=True)
    lines = [l for l in r.stdout.splitlines() if l.strip()]
    if not lines:
        return f"EMPTY (no add commit found for {path})"
    return f"{lines[-1]} [{path}]"


def store_check(names):
    p = os.path.join(REPO, "store/index.json")
    try:
        idx = json.load(open(p))
    except Exception as e:
        return {n: f"index unreadable: {e}" for n in names}
    text = json.dumps(idx)
    return {n: ("yes" if n in text else "no") for n in names}
