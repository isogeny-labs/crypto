#!/usr/bin/env python3
"""Exact rational construction through threshold-box coverage or frozen cap."""

import hashlib
import json
import resource
import signal
import time
from decimal import Decimal, getcontext
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, ZZ

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def bit_height(P):
    return max(max(abs(int(v.numerator())).bit_length(),int(v.denominator()).bit_length())
               for v in (P[0],P[1]))


def timeout(_signum,_frame):
    raise TimeoutError("per-cell wall cap")


def main():
    raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(raw)
    source_raw = (ROOT / protocol["source_threshold_result"]).read_bytes()
    assert sha(source_raw) == protocol["source_threshold_result_raw_sha256"]
    source = json.loads(source_raw)
    panel = json.loads((ROOT / protocol["source_panel_protocol"]).read_bytes())
    E = EllipticCurve(QQ,panel["rational_curve_coefficients"])
    Q = E(panel["Q"])
    R = E([QQ(z) for z in panel["R"]])
    pairing = E.height_pairing_matrix([Q,R],precision=panel["height_pairing_precision_bits"])
    assert [[str(pairing[i,j]) for j in range(2)] for i in range(2)] == panel["height_pairing_matrix"]
    getcontext().prec = 50
    hqq,hqr,hrr = [Decimal(z) for z in
                   (panel["height_pairing_matrix"][0][0],
                    panel["height_pairing_matrix"][0][1],
                    panel["height_pairing_matrix"][1][1])]
    def score(label):
        a,b = label
        return hqq*a*a+2*hqr*a*b+hrr*b*b
    rows = []
    for cell,source_cell in zip(protocol["cells"],source["thresholds"]):
        p,q,lam,B = [cell[k] for k in ("p","q","R_over_Q","radius")]
        assert (source_cell["p"],source_cell["q"],source_cell["first_full_radius"]) == (p,q,B)
        assert ZZ(p).is_prime() and ZZ(q).is_prime() and ZZ(E.discriminant())%p
        Ep = EllipticCurve(GF(p),[GF(p)(z) for z in panel["rational_curve_coefficients"]])
        assert Ep.cardinality() == q
        G = Ep([GF(p)(Q[0]),GF(p)(Q[1])])
        Rbar = Ep([GF(p)(R[0].numerator())/GF(p)(R[0].denominator()),
                   GF(p)(R[1].numerator())/GF(p)(R[1].denominator())])
        assert G.order() == q and Rbar == lam*G
        labels = [(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda x:(score(x),x[0],x[1]))
        seen = {}
        first_relation = None
        peak_bits = 0
        evaluations = 0
        attempts_started = 0
        status = "coefficient_cap_exhausted"
        signal.signal(signal.SIGALRM,timeout)
        start = time.monotonic()
        signal.setitimer(signal.ITIMER_REAL,300.0)
        try:
            for label in labels:
                a,b = label
                attempts_started += 1
                P = a*Q+b*R
                assert P != E(0)
                peak_bits = max(peak_bits,bit_height(P))
                if int(P[0].denominator())%p == 0:
                    reduction = Ep(0)
                else:
                    reduction = Ep([GF(p)(P[0].numerator())/GF(p)(P[0].denominator()),
                                    GF(p)(P[1].numerator())/GF(p)(P[1].denominator())])
                assert reduction == a*G+b*Rbar
                evaluations += 1
                if first_relation is None:
                    if reduction == Ep(0):
                        first_relation = {"kind":"zero_reduction","index":evaluations,"labels":[list(label)],
                                          "delta":[a,b],"prefix_max_coordinate_bits":peak_bits}
                    elif reduction in seen:
                        old = seen[reduction]
                        da,db = a-old[0],b-old[1]
                        assert db%q and -da*pow(db,-1,q)%q == lam
                        first_relation = {"kind":"finite_collision","index":evaluations,
                                          "labels":[list(old),list(label)],"delta":[da,db],
                                          "prefix_max_coordinate_bits":peak_bits}
                if reduction != Ep(0):
                    seen.setdefault(reduction,label)
                if len(seen) == q-1:
                    status = "full_coverage"
                    break
        except TimeoutError:
            status = "wall_cap_exhausted"
        finally:
            signal.setitimer(signal.ITIMER_REAL,0)
        elapsed = time.monotonic()-start
        row = {"p":p,"q":q,"R_over_Q":lam,"radius":B,"candidate_cap":len(labels),
               "candidate_evaluations":evaluations,"status":status,
               "attempts_started":attempts_started,
               "nonidentity_coverage":len(seen),"first_relation":first_relation,
               "peak_rational_coordinate_bits":peak_bits,
               "construction_reduction_wall_seconds":elapsed,
               "process_max_rss_bytes":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}
        rows.append(row)
        (HERE / "progress.jsonl").write_text("".join(json.dumps(r,sort_keys=True)+"\n" for r in rows))
    rows_raw = "".join(json.dumps(r,sort_keys=True)+"\n" for r in rows).encode()
    (HERE / "rows.jsonl").write_bytes(rows_raw)
    return {"schema":"xedni-threshold-rational-cost-result-v1","status":"PASS",
            "protocol_sha256":sha(raw),"script_sha256":sha((HERE/"run_sage.py").read_bytes()),
            "source_threshold_result_raw_sha256":sha(source_raw),
            "rows_sha256":sha(rows_raw),"cells":len(rows),
            "full_coverage_cells":sum(r["status"]=="full_coverage" for r in rows)}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
