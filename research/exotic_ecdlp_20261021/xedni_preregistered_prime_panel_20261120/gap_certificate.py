#!/usr/bin/env python3
"""Exact modular coverage certificate for the first capped prime cell."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def main():
    rows = [json.loads(line) for line in (HERE / "prime-rows.jsonl").read_text().splitlines()]
    row = next(r for r in rows if r["p"] == 1531)
    q = row["q"]
    B = row["coefficient_cap"]
    lam = row["first_relation"]["derived_R_over_G"]
    assert (q,B,lam) == (1553,41,11)
    image = {(a+lam*b)%q for a in range(-B,B+1) for b in range(-B,B+1)}
    interval = {z%q for z in range(-492,493)}
    assert image == interval
    assert len(image) == 985 and len(image-{0}) == row["distinct_nonidentity_reductions"] == 984
    missing = set(range(q))-image
    assert 500 in missing and len(missing) == 568
    raw = "".join(f"{x}\n" for x in sorted(image)).encode()
    return {"status":"PASS","p":1531,"q":q,"coefficient_radius":B,
            "R_reduced_scalar":lam,"attained_scalar_interval":[-492,492],
            "attained_residues_including_zero":len(image),
            "attained_nonidentity_residues":len(image)-1,
            "missing_residues":len(missing),"example_missing_residue":500,
            "attained_residue_list_sha256":hashlib.sha256(raw).hexdigest()}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
