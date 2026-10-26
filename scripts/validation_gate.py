#!/usr/bin/env python3
"""The validation gate (AGENTS.md, "Validation gate", owner rule 2026-10-19).

No row reaches the IC leaderboard (docs/ic/LEADERBOARD.md, docs/ic/leaderboard.json,
docs/ic-leaderboard.html) or benchmarks/README.md unless its evidence directory
carries a `VALIDATION.json` that points to three artifacts:

1. a replay certificate: independent replay of the recorded runs, scalars verified;
2. a validator report: an independent recompute of the headline numbers from raw
   data that is in the repository;
3. a rho reference matched to the protocol: the reference the protocol names, the
   strongest available for the curve (negation, Frobenius or the automorphism
   fold), priced in the same unit and the same binary or process, with the
   AGENTS.md section 5a band at 0, 0.1 and 1 GAE.

Leaderboard rows are never deleted. Benchmark rows removed from the admitted
table require an exact, provenance-bearing archival disposition in
`benchmarks/ARCHIVE.json`; the archive never grants validation. Rows frozen in
`docs/ic/validation/PRE_GATE.json` and `benchmarks/PRE_GATE.json` (a digest per
row) render as "unvalidated (pre-gate)" until a validation sweep verdict or a
VALIDATION.json validates them. A row that is not frozen, or whose digest no
longer matches the frozen one (it was added or changed after the gate), must be
validated or the builder refuses it and the CI check fails.

    python3 scripts/validation_gate.py check [--base REF]    # the CI check
    python3 scripts/validation_gate.py freeze                # (re)write the pre-gate files
    python3 scripts/validation_gate.py evidence DIR          # explain one evidence directory

Stdlib only.  The schema is documented in docs/VALIDATION_GATE.md.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MANIFEST = "VALIDATION.json"
SCHEMA = "validation-gate/v1"

PRE_GATE_LEADERBOARD = "docs/ic/validation/PRE_GATE.json"
PRE_GATE_BENCHMARKS = "benchmarks/PRE_GATE.json"
LEADERBOARD_JSON = "docs/ic/leaderboard.json"
BENCHMARKS_README = "benchmarks/README.md"
BENCHMARKS_ARCHIVE = "benchmarks/ARCHIVE.json"
SWEEP_DIR = "research/validation_sweep_20261019"
SWEEPS = {"leaderboard": f"{SWEEP_DIR}/A-leaderboard/verdicts.jsonl",
          "benchmarks": f"{SWEEP_DIR}/B-benchmarks/verdicts.jsonl"}

# Statuses, strongest first.  Only VALIDATED rows may be added or changed.
VALIDATED = "validated"
PRE_GATE = "unvalidated (pre-gate)"
UNVALIDATED = "unvalidated"
FAILED = "failed validation"

REFERENCE_KINDS = {"negation", "frobenius", "automorphism_fold", "negation_frobenius",
                   "no_symmetry_available"}
GAE_BAND = ("0", "0.1", "1")
CERT_OK = {"verified", "pass", "passed", "pass_independent_replay"}
# What a sweep verdict must carry to validate a row (the three artifacts).
VERDICT_ARTIFACTS = ("replay_certificate", "validator_report", "rho_reference")
VERDICT_PASS = {"validated", "pass", "passed", "ok"}
VERDICT_FAIL = {"failed", "fail", "invalid", "refuted", "rejected"}


# ---------------------------------------------------------------------------
# Digests and row keys.
# ---------------------------------------------------------------------------

def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def row_digest(row: dict) -> str:
    """Digest of a leaderboard board row, `validation` annotation excluded."""
    core = {k: v for k, v in row.items() if k != "validation"}
    return hashlib.sha256(json.dumps(core, sort_keys=True, ensure_ascii=False,
                                     separators=(",", ":")).encode()).hexdigest()


def line_digest(line: str) -> str:
    return hashlib.sha256(line.strip().encode()).hexdigest()


def board_key(row: dict) -> str:
    return f"board:{row['slug']}:{row['source']}"


def board_evidence_dir(row: dict) -> str:
    """A board row's evidence directory is the directory of the file it is read from."""
    return str(Path(row["source"]).parent)


# ---------------------------------------------------------------------------
# One evidence directory.
# ---------------------------------------------------------------------------

def _rel(base: Path, repo: Path, name: str, errors: list[str], what: str) -> Path | None:
    if not isinstance(name, str) or not name:
        errors.append(f"{what}: no path")
        return None
    p = (base / name).resolve()
    if not p.is_relative_to(repo.resolve()):
        errors.append(f"{what}: {name} leaves the repository")
        return None
    if not p.is_file():
        errors.append(f"{what}: {name} is not a file")
        return None
    return p


def _pinned_json(base: Path, repo: Path, spec, what: str, errors: list[str]):
    if not isinstance(spec, dict):
        errors.append(f"{what}: missing")
        return None
    p = _rel(base, repo, spec.get("path"), errors, what)
    if p is None:
        return None
    if spec.get("sha256") != sha256_file(p):
        errors.append(f"{what}: sha256 does not match {spec.get('path')}")
        return None
    try:
        return json.loads(p.read_text())
    except ValueError as e:
        errors.append(f"{what}: {spec.get('path')} is not JSON ({e})")
        return None


def _num(x) -> bool:
    return isinstance(x, (int, float)) and not isinstance(x, bool) and math.isfinite(x)


def check_replay_certificate(cert, errors: list[str]) -> None:
    if not isinstance(cert, dict):
        return
    if str(cert.get("status", "")).lower() not in CERT_OK:
        errors.append(f"replay certificate: status {cert.get('status')!r} is not a verified replay")
    if cert.get("scalars_verified") is not True:
        errors.append("replay certificate: scalars_verified is not true")
    if cert.get("independent") is not True:
        errors.append("replay certificate: independent is not true")
    runs = cert.get("runs")
    if not isinstance(runs, list) or not runs:
        errors.append("replay certificate: no runs")


def check_validator_report(rep, base: Path, repo: Path, errors: list[str]) -> None:
    if not isinstance(rep, dict):
        return
    if str(rep.get("status", "")).lower() not in {"pass", "passed"}:
        errors.append(f"validator report: status {rep.get('status')!r} is not pass")
    if rep.get("independent") is not True:
        errors.append("validator report: independent is not true")
    raw = rep.get("raw_data")
    if not isinstance(raw, list) or not raw:
        errors.append("validator report: raw_data is empty")
    else:
        for item in raw:
            name = item.get("path") if isinstance(item, dict) else item
            p = _rel(base, repo, name, errors, f"validator report raw_data {name}")
            if p and isinstance(item, dict) and item.get("sha256") not in (None, sha256_file(p)):
                errors.append(f"validator report raw_data {name}: sha256 mismatch")
    heads = rep.get("headline")
    if not isinstance(heads, list) or not heads:
        errors.append("validator report: no headline numbers recomputed")
        return
    for h in heads:
        ok = (isinstance(h, dict) and _num(h.get("claimed")) and _num(h.get("recomputed")))
        if not ok:
            errors.append(f"validator report: headline {h!r} lacks numeric claimed/recomputed")
            continue
        tol = h.get("rel_tol", 1e-9)
        scale = max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)
        if abs(h["claimed"] - h["recomputed"]) > tol * scale:
            errors.append(f"validator report: {h.get('name', '?')} claimed {h['claimed']} "
                          f"but recomputed {h['recomputed']}")


def check_rho_reference(ref, errors: list[str]) -> None:
    if not isinstance(ref, dict):
        errors.append("rho_reference: missing")
        return
    if ref.get("kind") not in REFERENCE_KINDS:
        errors.append(f"rho_reference: kind {ref.get('kind')!r} not in {sorted(REFERENCE_KINDS)}")
    for key in ("protocol", "unit", "binary_or_process"):
        if not isinstance(ref.get(key), str) or not ref[key].strip():
            errors.append(f"rho_reference: {key} is empty")
    for key in ("matches_protocol", "strongest_available", "same_unit", "same_binary_or_process"):
        if ref.get(key) is not True:
            errors.append(f"rho_reference: {key} is not true")
    band = ref.get("gae_band")
    if not isinstance(band, dict) or set(band) != set(GAE_BAND):
        errors.append("rho_reference: gae_band must have exactly the keys 0, 0.1 and 1")
    else:
        for k in GAE_BAND:
            v = band[k]
            if not (isinstance(v, dict) and _num(v.get("ic")) and _num(v.get("rho"))
                    and v["ic"] > 0 and v["rho"] > 0):
                errors.append(f"rho_reference: gae_band[{k}] needs positive numeric ic and rho")
    if not isinstance(ref.get("order_holds_across_band"), bool):
        errors.append("rho_reference: order_holds_across_band must be a bool")


def check_evidence_dir(evidence: Path, repo: Path = REPO, slug: str | None = None) -> list[str]:
    """Every reason `evidence` lacks a complete, consistent VALIDATION.json (empty = valid)."""
    m = evidence / MANIFEST
    if not m.is_file():
        return [f"{MANIFEST} is missing"]
    errors: list[str] = []
    try:
        man = json.loads(m.read_text())
    except ValueError as e:
        return [f"{MANIFEST} is not JSON ({e})"]
    if not isinstance(man, dict) or man.get("schema") != SCHEMA:
        return [f"{MANIFEST}: schema is not {SCHEMA}"]
    rows = man.get("rows")
    if rows is not None and slug is not None and slug not in rows:
        errors.append(f"{MANIFEST}: row {slug} is not among its rows")
    check_replay_certificate(
        _pinned_json(evidence, repo, man.get("replay_certificate"), "replay certificate", errors), errors)
    check_validator_report(
        _pinned_json(evidence, repo, man.get("validator_report"), "validator report", errors),
        evidence, repo, errors)
    check_rho_reference(man.get("rho_reference"), errors)
    return errors


# ---------------------------------------------------------------------------
# Frozen pre-gate rows and sweep verdicts.
# ---------------------------------------------------------------------------

def load_pre_gate(rel: str, repo: Path = REPO) -> dict[str, str]:
    p = repo / rel
    if not p.is_file():
        return {}
    d = json.loads(p.read_text())
    return dict(d.get("rows", {}))


def load_verdicts(kind: str, repo: Path = REPO) -> dict[str, dict]:
    """Sweep verdicts by row key.  One JSON object per line; `row` names the row
    (a board key, a slug, a source path or a benchmarks directory); later lines win."""
    p = repo / SWEEPS[kind]
    out: dict[str, dict] = {}
    if not p.is_file():
        return out
    for n, line in enumerate(p.read_text().splitlines(), 1):
        if not line.strip():
            continue
        try:
            v = json.loads(line)
        except ValueError:
            raise SystemExit(f"{SWEEPS[kind]}:{n} is not JSON")
        if isinstance(v, dict) and isinstance(v.get("row"), str):
            out[v["row"]] = v
    return out


def verdict_status(v: dict | None, digest: str) -> tuple[str, str] | None:
    """(status, why) a verdict decides, or None when it decides nothing."""
    if v is None:
        return None
    word = str(v.get("verdict", "")).lower()
    if v.get("row_digest") not in (None, digest):
        return None  # the row changed after the sweep looked at it
    if word in VERDICT_FAIL:
        return FAILED, f"sweep verdict {word}"
    if word in VERDICT_PASS:
        missing = [k for k in VERDICT_ARTIFACTS if not v.get(k)]
        if missing:
            return UNVALIDATED, "sweep verdict lacks " + ", ".join(missing)
        return VALIDATED, "sweep verdict"
    return None


def judge(keys: list[str], digest: str, evidence: Path | None, slug: str | None,
          verdicts: dict[str, dict], pre_gate: dict[str, str], repo: Path = REPO
          ) -> tuple[str, list[str]]:
    """The status of one row and the reasons behind it.

    Order: a failing sweep verdict wins; then a complete VALIDATION.json; then a
    passing sweep verdict; then the frozen pre-gate digest; otherwise unvalidated.
    """
    decided = [verdict_status(verdicts.get(k), digest) for k in keys]
    decided = [d for d in decided if d]
    for st, why in decided:
        if st == FAILED:
            return FAILED, [why]
    errs: list[str] = []
    if evidence is not None:
        errs = check_evidence_dir(evidence, repo, slug)
        if not errs:
            return VALIDATED, []
    for st, why in decided:
        if st == VALIDATED:
            return VALIDATED, [why]
    frozen = [pre_gate[k] for k in keys if k in pre_gate]
    if frozen and frozen[0] == digest:
        return PRE_GATE, errs
    reasons = errs + [d[1] for d in decided]
    if frozen:
        reasons.append("row changed since the pre-gate freeze")
    else:
        reasons.append("row is not in the pre-gate freeze")
    return UNVALIDATED, reasons


# ---------------------------------------------------------------------------
# Leaderboard rows.
# ---------------------------------------------------------------------------

def judge_board_row(row: dict, digest: str, verdicts: dict, pre_gate: dict,
                    repo: Path = REPO) -> dict:
    keys = [board_key(row), row["slug"] + "@" + row["source"], row["slug"]]
    st, why = judge(keys, digest, repo / board_evidence_dir(row), row["slug"],
                    verdicts, pre_gate, repo)
    return {"status": st, "evidence_dir": board_evidence_dir(row), "reasons": why}


def annotate_board(board: list[dict], digest_of, repo: Path = REPO,
                   allow_unvalidated: bool = False) -> None:
    """Add `validation` to every board row; refuse a row that is neither validated nor frozen.

    `digest_of(row)` is the digest of the row as the JSON will hold it (floats rounded)."""
    verdicts = load_verdicts("leaderboard", repo)
    pre_gate = load_pre_gate(PRE_GATE_LEADERBOARD, repo)
    bad = []
    for r in board:
        r["validation"] = judge_board_row(r, digest_of(r), verdicts, pre_gate, repo)
        if r["validation"]["status"] in (UNVALIDATED, FAILED):
            bad.append(r)
    if bad and not allow_unvalidated:
        lines = [f"  {board_key(r)}: {r['validation']['status']}: "
                 + "; ".join(r["validation"]["reasons"]) for r in bad]
        raise SystemExit(
            "the validation gate refuses these leaderboard rows (AGENTS.md, Validation gate): add a "
            f"{MANIFEST} (docs/VALIDATION_GATE.md) to each evidence directory, or pass "
            "--allow-unvalidated for a local build that marks them unvalidated:\n" + "\n".join(lines))


def status_counts(board: list[dict]) -> dict[str, int]:
    out: dict[str, int] = {}
    for r in board:
        s = r["validation"]["status"]
        out[s] = out.get(s, 0) + 1
    return out


# ---------------------------------------------------------------------------
# Benchmarks rows.
# ---------------------------------------------------------------------------

ROW = re.compile(r"^\|\s*\d{4}-\d{2}-\d{2}\s*\|\s*\[`([^`]+)`\]\(([^)]*)\)")


def benchmark_rows(text: str) -> list[tuple[str, str]]:
    """(directory, row text) of every dated row of benchmarks/README.md."""
    return [(m.group(1), line) for line in text.splitlines() if (m := ROW.match(line))]


def benchmark_groups(text: str) -> dict[str, str]:
    """Row text by directory; a directory listed on several rows is judged as one unit."""
    out: dict[str, str] = {}
    for d, line in benchmark_rows(text):
        out[d] = (out[d] + "\n" if d in out else "") + line.strip()
    return out


def judge_benchmark_row(directory: str, line: str, verdicts: dict, pre_gate: dict,
                        repo: Path = REPO) -> dict:
    keys = [f"benchmarks:{directory}", f"benchmarks/{directory}", directory]
    ev = repo / "benchmarks" / directory
    st, why = judge(keys, line_digest(line), ev if ev.is_dir() else None, None,
                    verdicts, pre_gate, repo)
    return {"status": st, "evidence_dir": f"benchmarks/{directory}", "reasons": why}


ARCHIVE_SCHEMA = "validation-gate-archive/v1"
ARCHIVE_DISPOSITIONS = {"censored", "documentary", "incomplete", "superseded"}


def benchmark_archive_errors(repo: Path, admitted: dict[str, str], base: str | None = None) -> list[str]:
    """Check archival records. Archived rows remain historical; they never validate admission."""
    path = repo / BENCHMARKS_ARCHIVE
    if not path.is_file():
        archive = {"schema": ARCHIVE_SCHEMA, "rows": []}
    else:
        try:
            archive = json.loads(path.read_text())
        except ValueError as exc:
            return [f"{BENCHMARKS_ARCHIVE} is not JSON ({exc})"]
    errors: list[str] = []
    if not isinstance(archive, dict) or archive.get("schema") != ARCHIVE_SCHEMA:
        return [f"{BENCHMARKS_ARCHIVE} schema is not {ARCHIVE_SCHEMA}"]
    records = archive.get("rows")
    if not isinstance(records, list):
        return [f"{BENCHMARKS_ARCHIVE} rows must be a list"]
    seen: set[str] = set()
    known: dict[str, str] = {}
    for i, record in enumerate(records):
        label = f"{BENCHMARKS_ARCHIVE} rows[{i}]"
        if not isinstance(record, dict):
            errors.append(f"{label} must be an object")
            continue
        directory, line = record.get("directory"), record.get("row")
        if not isinstance(directory, str) or not directory or not isinstance(line, str):
            errors.append(f"{label} needs directory and exact row text")
            continue
        if directory in seen:
            errors.append(f"{label} duplicates directory {directory}")
        seen.add(directory)
        parsed = benchmark_rows(line)
        if len(parsed) != 1 or parsed[0] != (directory, line):
            errors.append(f"{label} row does not parse to directory {directory}")
        digest = hashlib.sha256(line.strip().encode()).hexdigest()
        if record.get("sha256") != digest:
            errors.append(f"{label} sha256 does not match exact row text")
        if record.get("disposition") not in ARCHIVE_DISPOSITIONS:
            errors.append(f"{label} disposition is not one of {sorted(ARCHIVE_DISPOSITIONS)}")
        if not isinstance(record.get("reason"), str) or not record["reason"].strip():
            errors.append(f"{label} needs a non-empty reason")
        provenance = record.get("provenance")
        if not isinstance(provenance, list) or not provenance or any(
                not isinstance(p, str) or not p.strip() for p in provenance):
            errors.append(f"{label} needs non-empty provenance paths")
        else:
            for source in provenance:
                p = (repo / source).resolve()
                if not p.is_relative_to(repo.resolve()) or not p.is_file():
                    errors.append(f"{label} provenance {source!r} is not a repository file")
        if directory in admitted:
            errors.append(f"{label} is still admitted in {BENCHMARKS_README}")
        known[directory] = line

    if base:
        old_by_dir: dict[str, str] = {}
        old_archive = git_show(base, BENCHMARKS_ARCHIVE)
        if old_archive:
            try:
                old_records = json.loads(old_archive).get("rows", [])
                old_by_dir = {r.get("directory"): r.get("row") for r in old_records if isinstance(r, dict)}
                for directory, line in old_by_dir.items():
                    if known.get(directory) != line:
                        errors.append(f"archival history for {directory} was removed or changed")
            except (ValueError, AttributeError, TypeError):
                errors.append(f"{BENCHMARKS_ARCHIVE} on base is malformed")
        old_readme = git_show(base, BENCHMARKS_README)
        if old_readme is not None:
            old_rows = benchmark_groups(old_readme)
            for directory, old_line in old_rows.items():
                if directory not in admitted and known.get(directory) != old_line:
                    errors.append(f"removed benchmark {directory} lacks an exact archival disposition")
            for directory, line in known.items():
                # A record the base archive already carries was matched when it was added.
                if directory not in old_rows and old_by_dir.get(directory) != line:
                    errors.append(f"archival record for {directory} does not match a row from the base table")
    return errors


# ---------------------------------------------------------------------------
# The CI check.
# ---------------------------------------------------------------------------

def git_show(ref: str, rel: str) -> str | None:
    r = subprocess.run(["git", "show", f"{ref}:{rel}"], cwd=REPO, capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else None


def grown(now: dict[str, str], before: dict[str, str]) -> list[str]:
    """Keys a freeze file gained or re-digested relative to `before`."""
    return sorted(k for k, d in now.items() if before.get(k) != d)


def check(repo: Path = REPO, base: str | None = None, digest_of=None) -> list[str]:
    """Problems with the committed leaderboard.json and benchmarks/README.md (empty = pass)."""
    problems: list[str] = []
    doc = json.loads((repo / LEADERBOARD_JSON).read_text())
    digest_of = digest_of or row_digest
    lb_v, lb_g = load_verdicts("leaderboard", repo), load_pre_gate(PRE_GATE_LEADERBOARD, repo)
    for r in doc["board"]:
        res = judge_board_row(r, digest_of(r), lb_v, lb_g, repo)
        if res["status"] in (UNVALIDATED, FAILED):
            problems.append(f"leaderboard {board_key(r)}: {res['status']}: " + "; ".join(res["reasons"]))
        elif r.get("validation", {}).get("status") != res["status"]:
            problems.append(f"leaderboard {board_key(r)}: committed status "
                            f"{r.get('validation', {}).get('status')!r} is not {res['status']!r}; "
                            "run scripts/build_ic_leaderboard.py")
    bv, bg = load_verdicts("benchmarks", repo), load_pre_gate(PRE_GATE_BENCHMARKS, repo)
    admitted = benchmark_groups((repo / BENCHMARKS_README).read_text())
    for d, line in admitted.items():
        res = judge_benchmark_row(d, line, bv, bg, repo)
        if res["status"] in (UNVALIDATED, FAILED):
            problems.append(f"benchmarks {d}: {res['status']}: " + "; ".join(res["reasons"]))
    problems.extend(benchmark_archive_errors(repo, admitted, base))
    if base:
        # The freeze can only shrink: a PR may not launder a new or changed row into it.
        for rel in (PRE_GATE_LEADERBOARD, PRE_GATE_BENCHMARKS):
            old = git_show(base, rel)
            if old is None:
                continue
            added = grown(load_pre_gate(rel, repo), json.loads(old).get("rows", {}))
            for k in added:
                problems.append(f"{rel} gained or changed {k}; the pre-gate freeze may only shrink")
    return problems


def freeze(repo: Path = REPO, digest_of=None) -> None:
    """Write both pre-gate files from the committed rows (run once, at the gate's start)."""
    digest_of = digest_of or row_digest
    doc = json.loads((repo / LEADERBOARD_JSON).read_text())
    note = ("Rows present when the validation gate began (owner rule, 2026-10-19), each with the "
            "digest it had.  A row listed here renders as 'unvalidated (pre-gate)' until a sweep "
            "verdict or a VALIDATION.json validates it.  This file may only shrink.")
    lb = {board_key(r): digest_of(r) for r in doc["board"]}
    bm = {d: line_digest(line) for d, line in benchmark_groups((repo / BENCHMARKS_README).read_text()).items()}
    for rel, rows in ((PRE_GATE_LEADERBOARD, lb), (PRE_GATE_BENCHMARKS, bm)):
        p = repo / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(json.dumps({"schema": "validation-gate-pre-gate/v1", "frozen": "2026-10-19",
                                 "note": note, "rows": dict(sorted(rows.items()))}, indent=1) + "\n")
        print(f"wrote {rel}: {len(rows)} rows")


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    c = sub.add_parser("check")
    c.add_argument("--base", help="git ref of the PR base; the freeze files may not grow past it")
    sub.add_parser("freeze")
    e = sub.add_parser("evidence")
    e.add_argument("dir", type=Path)
    a = ap.parse_args(argv)
    if a.cmd == "evidence":
        errs = check_evidence_dir(a.dir.resolve())
        print("\n".join(errs) if errs else "valid")
        return 1 if errs else 0
    if a.cmd == "freeze":
        freeze()
        return 0
    problems = check(base=a.base)
    for p in problems:
        print(p, file=sys.stderr)
    if problems:
        print(f"\n{len(problems)} row(s) fail the validation gate (docs/VALIDATION_GATE.md)", file=sys.stderr)
        return 1
    print("validation gate: every row is validated or frozen pre-gate")
    return 0


if __name__ == "__main__":
    sys.exit(main())
