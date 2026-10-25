#!/usr/bin/env python3
"""Run the gate's own checks on a staged manifest as if it sat in EVIDENCE_DIR (nothing is written there).
usage: check_staged.py STAGED_MANIFEST EVIDENCE_DIR [SLUG ...]"""
import json, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "scripts"))
import validation_gate as g

man_p, ev = Path(sys.argv[1]), Path(sys.argv[2]).resolve()
man = json.loads(man_p.read_text()); errors = []
if man.get("schema") != g.SCHEMA: errors.append("schema")
g.check_replay_certificate(g._pinned_json(ev, g.REPO, man.get("replay_certificate"), "replay certificate", errors), errors)
g.check_validator_report(g._pinned_json(ev, g.REPO, man.get("validator_report"), "validator report", errors), ev, g.REPO, errors)
g.check_rho_reference(man.get("rho_reference"), errors)
for s in sys.argv[3:]:
    if man.get("rows") is not None and s not in man["rows"]: errors.append(f"row {s} not in rows")
print("valid" if not errors else "INVALID: " + "; ".join(errors))
