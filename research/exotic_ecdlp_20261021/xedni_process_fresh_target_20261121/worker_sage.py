"""One new checked Sage process: rebuild table and run one matched pair."""

import hashlib
import importlib.util
import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE=Path(__file__).resolve().parent
SOURCE=HERE.parent/"xedni_fixed_curve_target_accounting_20261121"/"run_sage.py"


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest()==p["source_table_runner_sha256"]
    spec=importlib.util.spec_from_file_location("xedni_frozen_table",SOURCE)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    rep=int(sys.argv[1])
    assert 1<=rep<=p["repetitions"]
    k=1+int.from_bytes(hashlib.sha256(p["target_seed"].encode()).digest()[:8],"big")%42
    fixture_curve=EllipticCurve(GF(47),p["reduced_curve_a_invariants"])
    fixture_G=fixture_curve([GF(47)(v) for v in p["rational_point_Q"]])
    fixture_T=k*fixture_G
    target=tuple(map(int,fixture_T.xy()))
    config={"rational_curve_a_invariants":p["rational_curve_a_invariants"],
            "rational_point_Q":p["rational_point_Q"],
            "reduced_curve_a_invariants":p["reduced_curve_a_invariants"],
            "rational_multiple_prefix":p["relation_prefix"],
            "relation_coefficients":p["relation_coefficients"]}
    E,G,table,setup=module.build_table(config)
    T=E(target)
    arms=[]
    for position,arm in enumerate(p["arm_order"][rep-1].split(","),1):
        result=module.table_solve(G,T,table) if arm=="table" else module.rho_solve(E,G,T,rep)
        arms.append({"position":position,"arm":arm,**result})
    assert all(r["status"]=="verified" and r["scalar"]==k for r in arms)
    print(json.dumps({"schema":"xedni-process-fresh-worker-v1","status":"PASS",
                      "repetition":rep,"target":target,"fixture_scalar_for_replay":k,
                      "setup":setup,"arms":arms},sort_keys=True))


if __name__=="__main__":main()
