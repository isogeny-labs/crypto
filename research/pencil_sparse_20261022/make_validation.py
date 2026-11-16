"""Package native-verified artifacts and their hashes for the repository gate."""

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"


def pin(path):
    return {
        "path": path.relative_to(ROOT).as_posix(),
        "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
    }


rho = json.loads((RESULTS / "rho-band.json").read_text())
rho["symmetry_check"] = pin(RESULTS / "rho-symmetry-check.json")
gate = {
    "schema": "validation-gate/v1",
    "scope": "degree-peeling point-driven pencil, nine 60-target frozen pilot curves",
    "replay_certificate": pin(RESULTS / "replay-certificate.json"),
    "validator_report": pin(RESULTS / "validation.json"),
    "rho_reference": rho,
}
(ROOT / "VALIDATION.json").write_text(json.dumps(gate, indent=2, sort_keys=True) + "\n")
