#!/usr/bin/env python3
"""Nonce-sampled re-audit of finished runbook and ecbench sessions (daily job).

    python3 scripts/audit_runs.py [--sample N] [--nonce HEX] [--ecbench BIN --replay K] [--out F]

Finished sessions are every directory whose `session.json` has schema
`ecbench.session/v1` and status `complete`, plus every evidence directory that
holds a `VALIDATION.json` (the validation gate, docs/VALIDATION_GATE.md).  A
nonce is drawn at run time (or given to reproduce a run), each candidate is
ranked by sha256(nonce + path), and the first N are audited, so which sessions
are looked at on a given day cannot be predicted or arranged in advance.

Digest checks (always): session.json's `plan_sha256` and `host_sha256`
match the bytes of plan.json and host.json, and
`records_sha256` matches records.jsonl (a directory with no records.jsonl is not a
session and is not sampled; `spec_sha256` is over the canonical spec and is left to
`ecbench verify`); an evidence directory's VALIDATION.json
is re-checked in full (every pinned sha256, the headline recompute, the band).

Replay (when --ecbench is given): `ecbench verify --dir D --replay K --exit-code`
re-derives K nonce-chosen runs of the session.  Hosted CI is a shared VM, so
this checks counts and answers, never wall time (AGENTS.md, ecbench section).

Exit status 1 when any sampled item fails.  The JSON report names the nonce.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import secrets
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import validation_gate as gate  # noqa: E402

REPO = gate.REPO
SKIP = {".git", "target", "node_modules"}


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def candidates(repo: Path) -> list[tuple[str, str]]:
    """(kind, repo-relative directory) of every auditable finished run."""
    out: list[tuple[str, str]] = []
    for p in sorted(repo.rglob("session.json")):
        rel = p.relative_to(repo)
        if SKIP & set(rel.parts):
            continue
        try:
            s = json.loads(p.read_text())
        except ValueError:
            continue
        if (isinstance(s, dict) and s.get("schema") == "ecbench.session/v1"
                and s.get("status") == "complete" and (p.parent / "records.jsonl").is_file()):
            out.append(("ecbench", str(rel.parent)))
    for p in sorted(repo.rglob(gate.MANIFEST)):
        rel = p.relative_to(repo)
        if not SKIP & set(rel.parts) and "fixtures" not in rel.parts:
            out.append(("validation", str(rel.parent)))
    return out


def sample(items: list[tuple[str, str]], nonce: str, n: int) -> list[tuple[str, str]]:
    key = lambda it: hashlib.sha256(f"{nonce}:{it[0]}:{it[1]}".encode()).hexdigest()
    return sorted(items, key=key)[:n]


def pinned_scratch(repo: Path) -> dict[str, list[str]]:
    """Scratch sessions without spec/host capsules, hash-pinned by docs/ecbench/INCOMPLETE_SESSIONS.tsv."""
    p = repo / "docs/ecbench/INCOMPLETE_SESSIONS.tsv"
    rows = [l.split("\t") for l in p.read_text().splitlines() if l and not l.startswith("#")] if p.is_file() else []
    return {r[0]: r[1:4] for r in rows if len(r) >= 4}


def digest_check(repo: Path, d: str) -> list[str]:
    base, errs = repo / d, []
    pins = pinned_scratch(repo).get(d)
    if pins:
        for name, want in zip(("session.json", "plan.json", "records.jsonl"), pins):
            if not (base / name).is_file() or sha(base / name) != want:
                errs.append(f"{name}: differs from its pin in INCOMPLETE_SESSIONS.tsv")
        return errs
    s = json.loads((base / "session.json").read_text())
    # spec_sha256 is over the canonical spec, not the file's bytes: `ecbench verify` owns it.
    for field, name in (("plan_sha256", "plan.json"), ("host_sha256", "host.json"),
                        ("records_sha256", "records.jsonl")):
        p = base / name
        if field not in s:
            continue
        if not p.is_file():
            errs.append(f"{name} is missing")
        elif sha(p) != s[field]:
            errs.append(f"{name}: sha256 differs from session.json {field}")
    return errs


def audit(repo: Path, kind: str, d: str, ecbench: str | None, replay: int) -> dict:
    res = {"kind": kind, "dir": d, "errors": []}
    if kind == "validation":
        res["errors"] += gate.check_evidence_dir(repo / d, repo)
    else:
        res["errors"] += digest_check(repo, d)
        if ecbench and not res["errors"]:
            r = subprocess.run([ecbench, "verify", "--dir", str(repo / d), "--replay", str(replay),
                                "--exit-code"], capture_output=True, text=True)
            res["replayed"] = replay
            if r.returncode != 0:
                res["errors"].append(f"ecbench verify --replay {replay} exited {r.returncode}: "
                                     + (r.stderr or r.stdout)[-400:])
    res["ok"] = not res["errors"]
    return res


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--sample", type=int, default=8)
    ap.add_argument("--nonce", default=None, help="hex nonce; default: drawn now")
    ap.add_argument("--ecbench", default=None, help="path of a built ecbench binary")
    ap.add_argument("--replay", type=int, default=4, help="runs to replay per session")
    ap.add_argument("--out", type=Path, default=None)
    a = ap.parse_args(argv)
    nonce = a.nonce or secrets.token_hex(8)
    items = candidates(REPO)
    picked = sample(items, nonce, a.sample)
    results = [audit(REPO, k, d, a.ecbench, a.replay) for k, d in picked]
    report = {"schema": "audit-runs/v1", "nonce": nonce, "candidates": len(items),
              "sampled": len(picked), "replay_per_session": a.replay if a.ecbench else 0,
              "failed": sum(not r["ok"] for r in results), "results": results}
    text = json.dumps(report, indent=1) + "\n"
    if a.out:
        a.out.write_text(text)
    print(text if not a.out else f"audit-runs nonce {nonce}: {report['sampled']} of {report['candidates']} "
          f"sampled, {report['failed']} failed")
    for r in results:
        for e in r["errors"]:
            print(f"FAIL {r['dir']}: {e}", file=sys.stderr)
    return 1 if report["failed"] else 0


if __name__ == "__main__":
    sys.exit(main())
