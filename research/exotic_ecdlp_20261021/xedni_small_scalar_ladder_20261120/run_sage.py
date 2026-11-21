#!/usr/bin/env python3
"""Frozen small-scalar public-lift ladder on four prior selected curves."""

import hashlib
import importlib.util
import json
import os
import platform
import resource
import signal
from math import isqrt
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def load_source(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def timeout(_sig, _frame):
    raise TimeoutError("per-arm wall cap")


def main():
    raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(raw)
    for name in ("source_family_result", "source_family_selection",
                 "source_lift_search", "source_rho"):
        assert sha((ROOT / protocol[name]).read_bytes()) == protocol[name + "_sha256"]
    selection = [json.loads(x) for x in (ROOT / protocol["source_family_selection"]).read_bytes().splitlines()]
    selected = [(x["p"],x["t"],x["q"]) for x in selection if x["status"] == "selected"]
    assert selected == [(x["p"],x["t"],x["q"]) for x in protocol["curves"]]
    lift_source = load_source(ROOT / protocol["source_lift_search"], "xedni_family_lift_source")
    rho_source = load_source(ROOT / protocol["source_rho"], "xedni_ladder_rho_source")
    signal.signal(signal.SIGALRM, timeout)
    fixtures, rows = [], []
    for curve in protocol["curves"]:
        p,t,q = curve["p"],curve["t"],curve["q"]
        E = EllipticCurve(QQ,[0,0,0,t,1])
        Q = E([0,1])
        Ep = EllipticCurve(GF(p),[0,0,0,GF(p)(t),GF(p)(1)])
        assert int(Ep.cardinality()) == q
        G = Ep([GF(p)(0),GF(p)(1)])
        assert int(G.order()) == q
        for k in protocol["scalars"]:
            rational = k * Q
            x = rational[0]
            a, d = int(x.numerator()), int(x.denominator())
            b = isqrt(d)
            assert b*b == d
            known_height = max(abs(a), b)
            T = k * G
            target = [int(T[0]),int(T[1])]
            fixture = {"p":p,"t":t,"q":q,"k":k,"target":target,
                       "known_rational_x_height":known_height,
                       "known_rational_x_numerator_bits":abs(a).bit_length(),
                       "known_rational_x_denominator_bits":d.bit_length()}
            fixtures.append(fixture)
            order = ("lift","rho") if k%2 else ("rho","lift")
            for position,arm in enumerate(order,1):
                signal.setitimer(signal.ITIMER_REAL,
                                 protocol["lift_wall_cap_seconds"] if arm == "lift"
                                 else protocol["rho_wall_cap_seconds"])
                try:
                    if arm == "lift":
                        result = lift_source.lift_search(target,p,t,protocol["height_cap"])
                    else:
                        result = rho_source.rho_solve(Ep,G,T,q,p,k,f"xedni-ladder:{t}")
                except Exception as e:
                    result = {"status":"error","error":type(e).__name__,"detail":str(e)}
                finally:
                    signal.setitimer(signal.ITIMER_REAL,0)
                if arm == "rho" and result.get("status") == "verified":
                    assert result["scalar"] == k and result["scalar"]*G == T
                rows.append({"p":p,"t":t,"q":q,"k":k,"target":target,
                             "position":position,"arm":arm,**result})
                (HERE / "progress.jsonl").write_text("".join(
                    json.dumps(x,sort_keys=True)+"\n" for x in rows))
    output_hashes = {}
    for name, content in (("target-fixtures.jsonl",fixtures),("arm-rows.jsonl",rows)):
        value = "".join(json.dumps(x,sort_keys=True)+"\n" for x in content).encode()
        (HERE / name).write_bytes(value)
        output_hashes[name] = sha(value)
    good = all(x["status"] in ("found","height_cap_exhausted","wall_cap_exhausted")
               if x["arm"] == "lift" else x["status"] == "verified" for x in rows)
    return {"schema":"xedni-small-scalar-ladder-result-v1",
            "status":"PASS" if good else "PARTIAL",
            "protocol_sha256":sha(raw),
            "script_sha256":sha((HERE / "run_sage.py").read_bytes()),
            "output_hashes":output_hashes,
            "targets":len(fixtures),"arms":len(rows),
            "lift_found":sum(x["arm"] == "lift" and x["status"] == "found" for x in rows),
            "lift_height_cap_exhausted":sum(x["arm"] == "lift" and x["status"] == "height_cap_exhausted" for x in rows),
            "lift_wall_cap_exhausted":sum(x["arm"] == "lift" and x["status"] == "wall_cap_exhausted" for x in rows),
            "rho_verified":sum(x["arm"] == "rho" and x["status"] == "verified" for x in rows),
            "host":{"system":platform.system(),"machine":platform.machine(),
                    "cpu_count":os.cpu_count(),
                    "process_max_rss_bytes":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
