"""Enumerate the frozen coefficient grid and preserve every rank result."""

import hashlib
import json
import subprocess
import time
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_direct_word2_lifts_20261121" / "result.json"
SAGE = "/Volumes/SSD990/cryptanalysis/sage"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def mod(x, p):
    return (x.numerator*pow(x.denominator, -1, p)) % p


def run_arm(payload, method, cap):
    start = time.monotonic_ns()
    try:
        cp = subprocess.run([SAGE, "-python", str(HERE / "rank_worker_sage.py"),
                             json.dumps(payload, sort_keys=True), method],
                            capture_output=True, text=True, timeout=cap)
        row = {"status": "complete" if cp.returncode == 0 else "error",
               "exit_code": cp.returncode, "elapsed_ns": time.monotonic_ns()-start,
               "stdout": cp.stdout, "stderr": cp.stderr}
        if cp.returncode == 0:
            row["result"] = json.loads(cp.stdout)
        return row
    except subprocess.TimeoutExpired as ex:
        return {"status": "wall_cap", "elapsed_ns": time.monotonic_ns()-start,
                "stdout": (ex.stdout or b"").decode(errors="replace") if isinstance(ex.stdout, bytes) else (ex.stdout or ""),
                "stderr": (ex.stderr or b"").decode(errors="replace") if isinstance(ex.stderr, bytes) else (ex.stderr or "")}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha(SOURCE) == protocol["source_result_sha256"]
    source = {r["source_index"]: r for r in json.loads(SOURCE.read_text())["rows"]}
    rows = []
    for index in protocol["source_indices"]:
        xp, yp = map(F, source[index]["P"])
        xq, yq = map(F, source[index]["Q"])
        lam = (yp-yq)/(xp-xq)
        for u in protocol["offset_order"]:
            for v in protocol["offset_order"]:
                a1, a3 = F(47*u), F(47*v)
                a2 = lam**2+a1*lam-2*xq-xp
                a4 = lam*(2*yq+a1*xq+a3)-3*xq*xq-2*a2*xq+a1*yq
                a6 = yq*yq+a1*xq*yq+a3*yq-xq**3-a2*xq*xq-a4*xq
                coeffs = [a1,a2,a3,a4,a6]
                payload = {"source_index": index, "u": u, "v": v,
                           "P": source[index]["P"], "Q": source[index]["Q"],
                           "a_invariants": list(map(str, coeffs))}
                assert [mod(x,47) for x in coeffs] == [0,0,0,46,45]
                assert yq*yq+a1*xq*yq+a3*yq == xq**3+a2*xq*xq+a4*xq+a6
                assert yp*yp+a1*xp*yp+a3*yp == xp**3+a2*xp*xp+a4*xp+a6
                payload["simon"] = run_arm(payload, "simon_two_descent", protocol["per_method_wall_cap_seconds"])
                if payload["simon"]["status"] == "complete" and payload["simon"]["result"]["rank_upper"] <= 1:
                    payload["pari"] = run_arm(payload, "pari_proof_rank", protocol["per_method_wall_cap_seconds"])
                else:
                    payload["pari"] = {"status": "not_selected_by_simon_gate"}
                rows.append(payload)
                print(json.dumps(payload, sort_keys=True), flush=True)
    winners = {}
    for index in protocol["source_indices"]:
        wins = [r for r in rows if r["source_index"] == index and r["pari"]["status"] == "complete"
                and r["pari"]["result"]["rank"] == 1]
        winners[str(index)] = [{"u":r["u"],"v":r["v"],"a_invariants":r["a_invariants"]} for r in wins]
    (HERE / "summary.json").write_text(json.dumps({"schema": "xedni-bridge-rank1-replacements-summary-v1",
        "source_sha256": sha(SOURCE), "protocol_sha256": sha(HERE / "protocol.json"),
        "cells": len(rows), "rank_one_witnesses": winners}, indent=2, sort_keys=True)+"\n")


if __name__ == "__main__":
    main()
