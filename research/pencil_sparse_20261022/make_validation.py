"""Pin the independently replayed evidence and copy native-priced rho bands."""

import csv
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"
CURVES = (739, 1151, 1187, 3371, 3527, 3931, 11731, 11887, 12143)


def pin(path):
    return {
        "path": path.relative_to(ROOT).as_posix(),
        "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
    }


certificate = RESULTS / "replay-certificate.json"
validator = RESULTS / "validation.json"
symmetry = RESULTS / "rho-symmetry-check.json"
assert json.loads(certificate.read_text())["status"] == "verified"
assert json.loads(validator.read_text())["status"] == "pass"
assert len(json.loads(symmetry.read_text())["checks"]) == 15

by_curve = {}
for p in CURVES:
    with (RESULTS / f"p{p}-peel-vs-online.csv").open(newline="") as stream:
        rows = list(csv.DictReader(stream))
    band = {}
    for native in ("0", "0.1", "1"):
        matching = [row for row in rows if row["S_per_M"] == "0.5"
                    and row["I_per_M"] == "10" and row["native_GAE_per_logical"] == native]
        assert len(matching) == 1
        row = matching[0]
        band[native] = {"ic": float(row["candidate_GAE"]), "rho": float(row["rho_GAE"])}
        assert band[native]["ic"] > band[native]["rho"]
    by_curve[str(p)] = band

gate = {
    "schema": "validation-gate/v1",
    "scope": "degree-peeling point-driven pencil, nine 60-target frozen pilot curves",
    "replay_certificate": pin(certificate),
    "validator_report": pin(validator),
    "rho_reference": {
        "kind": "negation",
        "protocol": "same-process source-negation-rho on each frozen target; PROTOCOL.md",
        "unit": "GAE",
        "binary_or_process": "pencil-sparse process runs candidate and source rho per target",
        "matches_protocol": True,
        "strongest_available": True,
        "same_unit": True,
        "same_binary_or_process": True,
        "symmetry_check": pin(symmetry),
        "gae_band": by_curve["739"],
        "gae_band_scope": "p=739, 60 targets, S/M=0.5, I/M=10; native paired CSV",
        "gae_band_by_curve": by_curve,
        "order_holds_across_band": True,
        "ordering": "candidate GAE > matched rho GAE on each of the nine curves at all three native prices",
    },
}
(ROOT / "VALIDATION.json").write_text(json.dumps(gate, indent=2, sort_keys=True) + "\n")
print("VALIDATION.json: verified nine-curve gate, with p739 representative band")
