#!/usr/bin/env python3
"""Lessons ledger (schema crypto.lesson/v1): how agents and skills learn.

Every agent and skill in this repository reads the lessons for its scope
before it acts and records what happened after it acts. A lesson is a
falsifiable rule of practice ("when X, do Y") with evidence. It is promoted
only by independent confirmations, demoted by refutations, and adopted
lessons are folded into the skill or agent text by a PR that cites them.

One JSON file per entry under docs/lessons/entries/. An entry's id is the
SHA-256 of its canonical content, so entries never merge-conflict, and an
edited entry no longer matches its id. Entries are never edited or deleted;
a correction is a new entry (test, retire).

    lessons.py add    --scope S[,S] --trigger "..." --action "..." --evidence E [--evidence E] [--why "..."]
    lessons.py test   --lesson ID --outcome confirm|refute --evidence E [--context "..."]
    lessons.py outcome --scope S --task "..." --result success|partial|fail|blocked --evidence E
                       [--applied ID,...] [--note "..."]
    lessons.py retire --lesson ID --reason "..." --evidence E
    lessons.py brief  [--scope S] [--limit N]     what to read before acting
    lessons.py stats  [--scope S] [--json]        outcome rates with Wilson 95% intervals
    lessons.py show   ID
    lessons.py verify [--base REF]                ids match content; references resolve;
                                                  no entry from REF was edited or removed

Status of a lesson, derived from its tests (never stored):
  candidate  fewer than 2 confirmations from distinct evidence
  adopted    >= 2 distinct confirmations and confirmations > 2 * refutations
  contested  refuted at least once and not adopted
  retired    a retire entry, or refutations >= confirmations + 2
Every entry is [pcc:unverified] until a pcc claim cites it.
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import math
import subprocess
import sys
from pathlib import Path

SCHEMA = "crypto.lesson/v1"
ROOT = Path(__file__).resolve().parents[1]
DEFAULT_DIR = ROOT / "docs" / "lessons" / "entries"
KINDS = ("lesson", "test", "outcome", "retire")
RESULTS = ("success", "partial", "fail", "blocked")


def canonical(body: dict) -> bytes:
    return json.dumps(body, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def entry_id(body: dict) -> str:
    prefix = {"lesson": "L", "test": "T", "outcome": "O", "retire": "R"}[body["kind"]]
    return f"{prefix}-{hashlib.sha256(canonical(body)).hexdigest()[:12]}"


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load(directory: Path) -> dict[str, dict]:
    entries: dict[str, dict] = {}
    if not directory.is_dir():
        return entries
    for path in sorted(directory.glob("*.json")):
        try:
            entries[path.stem] = json.loads(path.read_text())
        except (OSError, json.JSONDecodeError) as exc:
            entries[path.stem] = {"_error": str(exc)}
    return entries


def write(directory: Path, body: dict) -> str:
    body = {"schema": SCHEMA, **body}
    eid = entry_id(body)
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / f"{eid}.json"
    if path.exists():
        print(f"{eid} already recorded", file=sys.stderr)
        return eid
    path.write_text(json.dumps(body, indent=2, sort_keys=True, ensure_ascii=False) + "\n")
    return eid


def split_list(values: list[str] | None) -> list[str]:
    out: list[str] = []
    for v in values or []:
        out.extend(x.strip() for x in v.split(",") if x.strip())
    return out


def need_evidence(evidence: list[str]) -> list[str]:
    ev = [e.strip() for e in evidence if e.strip()]
    if not ev:
        sys.exit("error: at least one --evidence (path, commit, PR, job or receipt) is required")
    for e in ev:
        local = e.split("#", 1)[0]
        if "/" in local and not local.startswith(("http", "s3:")) and ":" not in local:
            if not (ROOT / local).exists():
                print(f"warning: evidence path {local} does not exist in this checkout", file=sys.stderr)
    return ev


# ---------------------------------------------------------------------------
# Derivation
# ---------------------------------------------------------------------------

def lesson_status(lid: str, entries: dict[str, dict]) -> dict:
    confirms: set[str] = set()
    refutes: set[str] = set()
    retired = False
    for e in entries.values():
        if e.get("kind") == "test" and e.get("lesson") == lid:
            key = "|".join(sorted(e.get("evidence", [])))
            (confirms if e.get("outcome") == "confirm" else refutes).add(key)
        if e.get("kind") == "retire" and e.get("lesson") == lid:
            retired = True
    c, r = len(confirms), len(refutes)
    if retired or r >= c + 2:
        status = "retired"
    elif c >= 2 and c > 2 * r:
        status = "adopted"
    elif r >= 1:
        status = "contested"
    else:
        status = "candidate"
    return {"status": status, "confirms": c, "refutes": r}


def matches(scopes: list[str], scope: str | None) -> bool:
    return scope is None or "*" in scopes or scope in scopes


def wilson(k: int, n: int, z: float = 1.96) -> tuple[float, float]:
    if n == 0:
        return (0.0, 1.0)
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, c - h), min(1.0, c + h))


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------

def cmd_add(a, d: Path) -> int:
    body = {"kind": "lesson", "created": now(), "scope": split_list(a.scope) or ["*"],
            "trigger": a.trigger.strip(), "action": a.action.strip(),
            "why": (a.why or "").strip(), "evidence": need_evidence(a.evidence)}
    print(write(d, body))
    return 0


def require_lesson(lid: str, entries: dict[str, dict]) -> None:
    if entries.get(lid, {}).get("kind") != "lesson":
        sys.exit(f"error: {lid} is not a recorded lesson")


def cmd_test(a, d: Path) -> int:
    entries = load(d)
    require_lesson(a.lesson, entries)
    body = {"kind": "test", "created": now(), "lesson": a.lesson, "outcome": a.outcome,
            "context": (a.context or "").strip(), "evidence": need_evidence(a.evidence)}
    print(write(d, body))
    st = lesson_status(a.lesson, load(d))
    print(f"{a.lesson}: {st['status']} ({st['confirms']} confirm, {st['refutes']} refute)")
    return 0


def cmd_outcome(a, d: Path) -> int:
    entries = load(d)
    applied = split_list(a.applied)
    for lid in applied:
        require_lesson(lid, entries)
    body = {"kind": "outcome", "created": now(), "scope": a.scope.strip(), "task": a.task.strip(),
            "result": a.result, "applied": applied, "note": (a.note or "").strip(),
            "evidence": need_evidence(a.evidence)}
    print(write(d, body))
    return 0


def cmd_retire(a, d: Path) -> int:
    entries = load(d)
    require_lesson(a.lesson, entries)
    body = {"kind": "retire", "created": now(), "lesson": a.lesson, "reason": a.reason.strip(),
            "evidence": need_evidence(a.evidence)}
    print(write(d, body))
    return 0


def lesson_rows(entries: dict[str, dict], scope: str | None) -> list[tuple[str, dict, dict]]:
    rows = []
    for lid, e in entries.items():
        if e.get("kind") == "lesson" and matches(e.get("scope", []), scope):
            rows.append((lid, e, lesson_status(lid, entries)))
    order = {"adopted": 0, "contested": 1, "candidate": 2, "retired": 3}
    rows.sort(key=lambda r: (order[r[2]["status"]], -r[2]["confirms"], r[1].get("created", "")))
    return rows


def cmd_brief(a, d: Path) -> int:
    entries = load(d)
    rows = [r for r in lesson_rows(entries, a.scope) if r[2]["status"] != "retired"]
    label = a.scope or "all scopes"
    if not rows:
        print(f"lessons ({label}): none recorded yet. Record one with tools/lessons.py add.")
        return 0
    print(f"lessons ({label}): {len(rows)} live; adopted first. Status is derived from tests.")
    for lid, e, st in rows[: a.limit]:
        print(f"- [{st['status']} +{st['confirms']}/-{st['refutes']}] {lid} ({','.join(e['scope'])}): "
              f"when {e['trigger']} -> {e['action']}")
    if len(rows) > a.limit:
        print(f"  ... {len(rows) - a.limit} more: tools/lessons.py brief --limit {len(rows)}")
    s = outcome_stats(entries, a.scope)
    for scope, v in s.items():
        print(f"  outcomes {scope}: {v['success']}/{v['n']} success "
              f"(95% CI {v['ci'][0]:.2f}-{v['ci'][1]:.2f})")
    return 0


def outcome_stats(entries: dict[str, dict], scope: str | None) -> dict[str, dict]:
    out: dict[str, dict] = {}
    for e in entries.values():
        if e.get("kind") != "outcome" or (scope and e.get("scope") != scope):
            continue
        v = out.setdefault(e["scope"], {r: 0 for r in RESULTS} | {"n": 0})
        v[e["result"]] += 1
        v["n"] += 1
    for v in out.values():
        v["ci"] = wilson(v["success"], v["n"])
    return dict(sorted(out.items()))


def cmd_stats(a, d: Path) -> int:
    entries = load(d)
    s = outcome_stats(entries, a.scope)
    lessons = {lid: st for lid, _, st in lesson_rows(entries, a.scope)}
    if a.json:
        print(json.dumps({"outcomes": s, "lessons": lessons}, indent=2))
        return 0
    print("| scope | n | success | partial | fail | blocked | success 95% CI |")
    print("|:--|--:|--:|--:|--:|--:|:--|")
    for scope, v in s.items():
        print(f"| {scope} | {v['n']} | {v['success']} | {v['partial']} | {v['fail']} | {v['blocked']} "
              f"| {v['ci'][0]:.2f}-{v['ci'][1]:.2f} |")
    counts: dict[str, int] = {}
    for st in lessons.values():
        counts[st["status"]] = counts.get(st["status"], 0) + 1
    print(f"\nlessons: {counts or 'none'}")
    return 0


def cmd_show(a, d: Path) -> int:
    entries = load(d)
    e = entries.get(a.id)
    if e is None:
        sys.exit(f"error: no entry {a.id}")
    print(json.dumps(e, indent=2, sort_keys=True))
    if e.get("kind") == "lesson":
        print(json.dumps(lesson_status(a.id, entries)))
        for tid, t in sorted(entries.items()):
            if t.get("lesson") == a.id:
                print(f"  {tid} {t['kind']} {t.get('outcome', t.get('reason', ''))}: {', '.join(t['evidence'])}")
    return 0


def verify(d: Path, base: str | None) -> list[str]:
    errors: list[str] = []
    entries = load(d)
    for eid, e in entries.items():
        if "_error" in e:
            errors.append(f"{eid}: unreadable ({e['_error']})")
            continue
        if e.get("schema") != SCHEMA or e.get("kind") not in KINDS:
            errors.append(f"{eid}: wrong schema or kind")
            continue
        if entry_id(e) != eid:
            errors.append(f"{eid}: content does not match id (edited entry)")
        if not e.get("evidence"):
            errors.append(f"{eid}: no evidence")
        if e["kind"] in ("test", "retire") and entries.get(e.get("lesson"), {}).get("kind") != "lesson":
            errors.append(f"{eid}: references unknown lesson {e.get('lesson')}")
        if e["kind"] == "outcome":
            if e.get("result") not in RESULTS:
                errors.append(f"{eid}: bad result")
            for lid in e.get("applied", []):
                if entries.get(lid, {}).get("kind") != "lesson":
                    errors.append(f"{eid}: applied unknown lesson {lid}")
        if e["kind"] == "lesson" and not (e.get("trigger") and e.get("action")):
            errors.append(f"{eid}: lesson needs trigger and action")
    if base:
        rel = d.resolve().relative_to(ROOT)
        try:
            listing = subprocess.run(["git", "-C", str(ROOT), "ls-tree", "--name-only", f"{base}:{rel}"],
                                     capture_output=True, text=True, check=True).stdout.split()
        except subprocess.CalledProcessError:
            listing = []
        for name in listing:
            old = subprocess.run(["git", "-C", str(ROOT), "show", f"{base}:{rel}/{name}"],
                                 capture_output=True, check=False).stdout
            path = d / name
            if not path.exists():
                errors.append(f"{name}: removed (entries are append-only)")
            elif path.read_bytes() != old:
                errors.append(f"{name}: changed since {base} (entries are append-only)")
    return errors


def cmd_verify(a, d: Path) -> int:
    errors = verify(d, a.base)
    for e in errors:
        print(f"FAIL {e}")
    print(f"lessons verify: {len(load(d))} entries, {len(errors)} error(s)")
    return 1 if errors else 0


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--dir", type=Path, default=DEFAULT_DIR, help="entries directory")
    sub = p.add_subparsers(dest="cmd", required=True)

    s = sub.add_parser("add"); s.add_argument("--scope", action="append")
    s.add_argument("--trigger", required=True); s.add_argument("--action", required=True)
    s.add_argument("--why"); s.add_argument("--evidence", action="append", default=[])

    s = sub.add_parser("test"); s.add_argument("--lesson", required=True)
    s.add_argument("--outcome", required=True, choices=("confirm", "refute"))
    s.add_argument("--context"); s.add_argument("--evidence", action="append", default=[])

    s = sub.add_parser("outcome"); s.add_argument("--scope", required=True)
    s.add_argument("--task", required=True); s.add_argument("--result", required=True, choices=RESULTS)
    s.add_argument("--applied", action="append"); s.add_argument("--note")
    s.add_argument("--evidence", action="append", default=[])

    s = sub.add_parser("retire"); s.add_argument("--lesson", required=True)
    s.add_argument("--reason", required=True); s.add_argument("--evidence", action="append", default=[])

    s = sub.add_parser("brief"); s.add_argument("--scope"); s.add_argument("--limit", type=int, default=15)
    s = sub.add_parser("stats"); s.add_argument("--scope"); s.add_argument("--json", action="store_true")
    s = sub.add_parser("show"); s.add_argument("id")
    s = sub.add_parser("verify"); s.add_argument("--base")

    a = p.parse_args(argv)
    return {"add": cmd_add, "test": cmd_test, "outcome": cmd_outcome, "retire": cmd_retire,
            "brief": cmd_brief, "stats": cmd_stats, "show": cmd_show, "verify": cmd_verify}[a.cmd](a, a.dir)


if __name__ == "__main__":
    sys.exit(main())
