#!/usr/bin/env python3
"""Record one proof-carrying claim per validated leaderboard row.

For every row of `docs/ic/leaderboard.json` whose evidence directory carries a
complete `VALIDATION.json` (scripts/validation_gate.py), write a `pcc.claim/v1`
of kind `measurement` under `pcc/claims/` whose proofs are the row's replay
certificate and validator report, pinned by SHA-256, and append it to the
ledger. The claim's statement quotes the row's numbers as the board shows them;
its `row` block carries the board key and the gate's row digest, so a later
change to the row is visible against the claim.

Idempotent: a row that already has a claim with the same row digest is skipped.
A row that changed since its claim gets a new claim; the old one stays in the
ledger as history (claims are never edited).

    python3 scripts/pcc_leaderboard_claims.py [--dry-run]

Writing a claim establishes nothing. The claim's standing comes only from a
signed verdict in the ledger (`python3 tools/pcc/pcc.py standing <id>`;
docs/SABOTAGE_RESISTANCE.md).
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO / "scripts"))
sys.path.insert(0, str(REPO / "tools" / "pcc"))
import pcc  # noqa: E402
import validation_gate as gate  # noqa: E402


def existing_claims() -> dict[str, dict]:
    """board key -> the latest claim recorded for it."""
    out: dict[str, dict] = {}
    claims_dir = REPO / pcc.CLAIMS_DIR
    for p in sorted(claims_dir.glob("*.json")) if claims_dir.is_dir() else []:
        c = json.loads(p.read_text(encoding="utf-8"))
        row = c.get("row") if isinstance(c, dict) else None
        if isinstance(row, dict) and isinstance(row.get("board_key"), str):
            out[row["board_key"]] = c
    return out


def _fmt(x) -> str:
    return f"{x:.6g}" if isinstance(x, (int, float)) and not isinstance(x, bool) else str(x)


def statement(row: dict) -> str:
    best = row["best"]
    family = row.get("family")
    head = (f"Index-calculus leaderboard Table {family}, row {row['slug']} ({row.get('legacy')}, "
            f"{row.get('regime')} regime, log2 r = {_fmt(row.get('log2_r'))}): ")
    if family == "A":
        body = (f"recipe {best['variant']} costs S = {_fmt(best['s'])} per sqrt(r) "
                f"(setup {_fmt(best.get('s_setup'))}, online {_fmt(best.get('s_online'))}); "
                f"the matched rho reference costs S = {_fmt(row['reference_s'])}; "
                f"cold IC/rho = {_fmt(best['ratio_rho'])}")
        ci = best.get("ratio_rho_ci")
        if ci:
            body += f" (bootstrap 95% CI {_fmt(ci[0])} to {_fmt(ci[1])})"
        if best.get("online_speedup") is not None:
            body += f"; online rho/IC = {_fmt(best['online_speedup'])}"
    else:
        body = (f"best recipe {best['variant']} costs S_IC = {_fmt(best['s'])} GAE per sqrt(r); "
                f"the matched rho reference ({row.get('reference')}) costs S_ref = {_fmt(row['reference_s'])}; "
                f"IC/rho = {_fmt(best['ratio_rho'])}, IC/floor = {_fmt(best.get('ratio_floor'))}")
    return head + body + ". The gap to matched rho is the distance remaining, not a speedup."


def proofs_for(evidence: Path, manifest: dict) -> list[dict]:
    """The replay and recompute proofs, with repository-relative paths."""
    out = []
    for key, kind in (("replay_certificate", "replay"), ("validator_report", "recompute")):
        spec = manifest[key]
        path = (evidence / spec["path"]).resolve()
        rel = path.relative_to(REPO).as_posix()
        actual = pcc.sha256_file(path)
        if actual != spec["sha256"]:
            raise SystemExit(f"{rel}: VALIDATION.json pins {spec['sha256'][:12]}, file hashes {actual[:12]}")
        proof = {"kind": kind, "path": rel, "sha256": actual}
        if kind == "recompute":
            proof["evidence_dir"] = evidence.relative_to(REPO).as_posix()
        out.append(proof)
    return out


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--dry-run", action="store_true")
    a = ap.parse_args(argv)
    board = json.loads((REPO / "docs/ic/leaderboard.json").read_text(encoding="utf-8"))["board"]
    have = existing_claims()
    commit = pcc.head_commit(REPO)
    written = skipped = unvalidated = 0
    for row in board:
        key = gate.board_key(row)
        evidence = REPO / gate.board_evidence_dir(row)
        errors = gate.check_evidence_dir(evidence, REPO, row["slug"])
        if errors:
            unvalidated += 1
            continue
        digest = gate.row_digest(row)
        prior = have.get(key)
        if prior and prior.get("row", {}).get("row_digest") == digest:
            skipped += 1
            continue
        manifest = json.loads((evidence / gate.MANIFEST).read_text(encoding="utf-8"))
        claim = {
            "schema": pcc.CLAIM_SCHEMA,
            "id": pcc.new_claim_id(),
            "kind": "measurement",
            "statement": statement(row),
            "scope": (f"one leaderboard row as built by scripts/build_ic_leaderboard.py from {row['source']}; "
                      f"validated under docs/VALIDATION_GATE.md by {evidence.relative_to(REPO).as_posix()}/VALIDATION.json "
                      f"(rho reference kind {manifest.get('rho_reference', {}).get('kind')}, "
                      f"order holds across the 0/0.1/1 GAE band: {manifest.get('rho_reference', {}).get('order_holds_across_band')})"),
            "proofs": proofs_for(evidence, manifest),
            "row": {"board_key": key, "table": row.get("family"), "slug": row["slug"], "source": row["source"],
                    "row_digest": digest, "supersedes": prior["id"] if prior else None},
            "producer": {"commit": commit, "session": "scripts/pcc_leaderboard_claims.py", "model": ""},
            "created_at": pcc.utcnow(),
        }
        reasons: list[str] = []
        if not pcc.validate_claim(claim, reasons):
            raise SystemExit(f"{key}: {'; '.join(reasons)}")
        print(f"{'would write' if a.dry_run else 'write'} {claim['id']}  {key}")
        if not a.dry_run:
            path = REPO / pcc.CLAIMS_DIR / f"{claim['id']}.json"
            path.write_text(json.dumps(claim, indent=1, sort_keys=True) + "\n", encoding="utf-8")
            pcc.append_entry(REPO / pcc.LEDGER_PATH, "claim",
                             {"claim_id": claim["id"], "claim_sha256": pcc.claim_digest(claim),
                              "path": path.relative_to(REPO).as_posix(), "row": key})
        written += 1
    print(f"claims: {written} written, {skipped} already current, {unvalidated} rows without a complete VALIDATION.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
