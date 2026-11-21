#!/usr/bin/env python3
"""Exact coefficient-box scalar-image sweep frozen in protocol.json."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    source_raw = (ROOT / protocol["source_panel"]).read_bytes()
    assert sha(source_raw) == protocol["source_panel_raw_sha256"]
    source = {r["p"]:r for r in map(json.loads,source_raw.splitlines())}
    all_rows = []
    thresholds = []
    lo,hi = protocol["radius_range_inclusive"]
    assert lo == 0 and hi == 128
    for cell in protocol["cells"]:
        p,q,lam,prior = [cell[k] for k in ("p","q","R_over_Q","prior_radius")]
        old = source[p]
        assert (old["q"],old["coefficient_cap"],old["first_relation"]["derived_R_over_G"]) == (q,prior,lam)
        first = None
        for B in range(lo,hi+1):
            image = {(a+lam*b)%q for a in range(-B,B+1) for b in range(-B,B+1)}
            missing = sorted(set(range(q))-image)
            row = {"p":p,"q":q,"R_over_Q":lam,"radius":B,
                   "coefficient_pairs_including_zero":(2*B+1)**2,
                   "attained_residues_including_zero":len(image),
                   "attained_nonidentity_residues":len(image)-1,
                   "missing_residues":len(missing),
                   "first_missing_residue":missing[0] if missing else None,
                   "status":"full_coverage" if not missing else "incomplete"}
            all_rows.append(row)
            if not missing and first is None:
                first = B
            if B == prior:
                assert row["attained_nonidentity_residues"] == old["distinct_nonidentity_reductions"]
                assert (row["status"] == "full_coverage") == (old["status"] == "full_coverage")
        thresholds.append({"p":p,"q":q,"first_full_radius":first,
                           "status":"full_coverage_found" if first is not None else "radius_cap_exhausted"})
    raw = "".join(json.dumps(row,sort_keys=True)+"\n" for row in all_rows).encode()
    (HERE / "radius-rows.jsonl").write_bytes(raw)
    return {"schema":"xedni-radius-threshold-result-v1","status":"PASS",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE/"run.py").read_bytes()),
            "source_panel_raw_sha256":sha(source_raw),"rows_sha256":sha(raw),
            "radius_rows":len(all_rows),"thresholds":thresholds}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
