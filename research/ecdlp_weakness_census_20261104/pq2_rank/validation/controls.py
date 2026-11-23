#!/usr/bin/env python3
"""Negative controls for the validator's own checkers.

Each control plants one defect into an otherwise passing input and requires the
checker to flag it. A checker that cannot fail is not evidence.

  python3 -I validation/controls.py   (from the pq2_rank directory)
"""
from __future__ import annotations

import copy
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import closure_check as cc  # noqa: E402  (the validator's own module)
import recompute as rc      # noqa: E402

run = json.loads((HERE / "dump_n15_17/pq2_rank.json").read_text())["rows"]
row = next(r for r in run if r["n"] == 16)
dump = json.loads((HERE / "dump_n15_17/inst/n16_i0.json").read_text())
results = []


def control(name, fn, expect):
    out = fn()
    ok = expect(out)
    results.append({"control": name, "caught": bool(ok)})
    print(("caught " if ok else "MISSED ") + name)


def base():
    return cc.check_instance(row, dump, iter4=True, low4=False, iter3=True, seed=1)


def flip_monomial():
    d = copy.deepcopy(dump)
    eq = d["eqs"][3]
    t = [0, d["N"] // 2]  # a_0 b_0
    if t in eq:
        eq.remove(t)
    else:
        eq.append(t)
    return cc.check_instance(row, d, iter4=False, low4=False, iter3=True, seed=1)


def drop_solution():
    d = copy.deepcopy(dump)
    d["Z"] = d["Z"][1:]
    return cc.check_instance(row, d, iter4=False, low4=False, iter3=True, seed=1)


def off_by_one():
    r = dict(row)
    r["iter3"] = r["iter3"] - 1
    return cc.check_instance(r, dump, iter4=False, low4=False, iter3=True, seed=1)


def planted_generator():
    # x_0 + 1 does not vanish on Z unless every z has bit 0 set; pick a variable that
    # takes value 0 at some z, so the planted generator is outside the ideal
    v = next(v for v in range(dump["N"]) if any(((z >> v) & 1) == 0 for z in dump["Z"]))
    d = copy.deepcopy(dump)
    d["eqs"] = d["eqs"] + [[[v]]]
    r = cc.check_instance(row, d, iter4=False, low4=False, iter3=True, seed=1)
    return r


b = base()
control("baseline passes (no planted defect)", lambda: b,
        lambda o: not o["failed_checks"] and not o["value_mismatches"])
control("flipped monomial in a dumped equation", flip_monomial,
        lambda o: "descent_anf_equals_dump" in o["failed_checks"])
control("dropped solution from dumped Z", drop_solution,
        lambda o: "Z_bruteforce_equals_dump" in o["failed_checks"])
control("producer iter3 off by one", off_by_one, lambda o: "iter3" in o["value_mismatches"])
control("generator outside the ideal (closure must stop vanishing on Z)", planted_generator,
        lambda o: "closure3_vanishes_on_Z" in o["failed_checks"])

line = ("n=16 n'= 8 N=16 inst=0 |Z|= 2 affdim=1 k=15 rho=15 low2= 1 low3= 1 low4=8 d_lin=>4 | iter3=15 "
        "(5 rounds) iter4=15 (3 rounds) d_iter=3 (0 ms Z, 0 ms deg2, 1 ms deg3, 54 ms deg4, 38 ms iter3, 3688 ms iter4)")
p = rc.parse_line(line)
control("parser reads a known line exactly", lambda: p,
        lambda o: o["iter3"] == 15 and o["k"] == 15 and o["low4"] == 8 and o["d_lin"] == ">4" and o["ms"]["iter4"] == 3688)
control("parser sees a changed value", lambda: rc.parse_line(line.replace("iter3=15", "iter3=1")),
        lambda o: o["iter3"] == 1)


def malformed():
    try:
        rc.parse_line(line.replace("rho=15 ", ""))
    except ValueError:
        return True
    return False


control("parser rejects a line with a missing field", malformed, lambda o: o is True)

(HERE / "controls.json").write_text(json.dumps({"schema": "validator.controls/v1", "results": results}, indent=1) + "\n")
sys.exit(0 if all(r["caught"] for r in results) else 1)
