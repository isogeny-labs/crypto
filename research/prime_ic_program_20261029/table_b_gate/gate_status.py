#!/usr/bin/env python3
"""Read-only gate status for the eight Table B prime rows and the 20-bit fb-size row.

Calls the repository's own checkers (scripts/validation_gate.py, scripts/audit_runs.py)
without a build and without writing anywhere except stdout. Run from any directory:

    python3 research/prime_ic_program_20261029/table_b_gate/gate_status.py

It measures nothing; it reports what the committed evidence says.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / "scripts"))
import validation_gate as gate  # noqa: E402
import audit_runs  # noqa: E402

PRIME_B = [
    "icv1-fp10-t5-192cb216", "icv1-fp12-t19-d4a315df", "icv1-fp14-tm43-158b6914",
    "icv1-fp16-t351-d433f24e", "icv1-fp18-t175-45770ebc", "icv1-fp20-t1293-3c2999ba",
    "icv1-fp22-tm1267-a5acaf6d", "icv1-fp24-t1577-77336155",
]
EVIDENCE_DIRS = [
    "docs/ic/runs",
    "research/ic_validation_gate_20261024/B",
    "benchmarks/20261029-prime-fb-size-20bit",
    "research/prime_base_oracle_tuning_20261029",
]
FB_SESSION = "research/prime_base_oracle_tuning_20261029/sessions/holdout-P20"


def main() -> None:
    print("## 1. validation_gate.check_evidence_dir (empty list = valid)")
    for d in EVIDENCE_DIRS:
        errs = gate.check_evidence_dir(REPO / d, REPO)
        print(f"{d}: {'valid' if not errs else errs}")

    print("\n## 2. Table B prime rows: recomputed standing vs committed docs/ic/leaderboard.json")
    doc = json.loads((REPO / gate.LEADERBOARD_JSON).read_text())
    verdicts = gate.load_verdicts("leaderboard", REPO)
    pre = gate.load_pre_gate(gate.PRE_GATE_LEADERBOARD, REPO)
    rows = {r["slug"]: r for r in doc["board"]}
    for slug in PRIME_B:
        r = rows[slug]
        res = gate.judge_board_row(r, gate.row_digest(r), verdicts, pre, REPO)
        committed = r.get("validation", {}).get("status")
        sweep = res.get("sweep", {})
        print(f"{slug}: recomputed={res['status']!r} committed={committed!r} "
              f"evidence_dir={res['evidence_dir']} reasons={res['reasons']} "
              f"sweep={gate.sweep_text(sweep) if sweep else None} in_pre_gate={slug in pre}")

    print("\n## 3. sweep-A band recompute for the prime rows (x reference at w = 0 / 0.1 / 1)")
    for slug in PRIME_B:
        v = verdicts.get(slug) or verdicts.get("B:" + slug)
        rc = v["recomputed"]
        print(f"{slug}: board x_ref={rc['x_reference']:.4g} band="
              f"{rc['band_ratio_w0']:.4g} / {rc['band_ratio_w0_1']:.4g} / {rc['band_ratio_w1']:.4g} "
              f"pricing_ok={v['pricing_ok']} protocol_frozen_ok={v['protocol_frozen_ok']}")

    print("\n## 4. audit_runs.digest_check on the 20-bit holdout session (no replay; no binary)")
    print(f"{FB_SESSION}: {audit_runs.digest_check(REPO, FB_SESSION) or 'digests ok'}")
    s = json.loads((REPO / FB_SESSION / "session.json").read_text())
    print(f"session_id={s['session_id']} env_class_id={s['env_class_id']} "
          f"binary_sha256={s['binary_sha256']} git_commit={s['git_commit']} "
          f"records_on_disk={(REPO / FB_SESSION / 'records.jsonl').is_file()}")

    print("\n## 5. fb-size replay certificate: independence block and promotion blockers")
    cert = json.loads((REPO / "benchmarks/20261029-prime-fb-size-20bit/replay-certificate.json").read_text())
    ind = cert["independence"]
    print({k: ind[k] for k in ("same_binary_as_measurement", "same_host_class_as_measurement",
                               "independent_implementation", "independent_scalar_check")})
    man = json.loads((REPO / "benchmarks/20261029-prime-fb-size-20bit/VALIDATION.json").read_text())
    print("promotion_blockers:", man.get("promotion_blockers"))


if __name__ == "__main__":
    main()
