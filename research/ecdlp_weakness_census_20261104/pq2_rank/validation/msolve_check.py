#!/usr/bin/env python3
"""Validator's own msolve replay (written from scratch; does not use f4/run_msolve.py).

For each dumped instance (PQ2_DUMP format: N, eqs as monomial lists, Z) it writes an
msolve input over F_2 with variables x0 > x1 > ... > x{N-1} (grevlex in that order), the
dumped equations and the field equations x_i^2 + x_i, runs

    msolve -v 2 -g 2 -t 1 -f IN -o OUT

and parses msolve's own output:

* the F4 round table (lines "deg sel pairs R x C density% a new b zero ..."): the step
  degrees and their maximum ("reduce final basis" is not a round and has no degree);
* "Dimension of quotient: d": with the field equations the ideal is radical over F_2, so
  d must equal |Z|, an independent check that the dumped Z is complete;
* the reduced Groebner basis printed by -g 2: its size and the number of elements of total
  degree 1 (affine linear), which must equal k = N + 1 - rank[1 | z].

Invariants checked per instance: msolve exit 0, no invalid equations, characteristic 2,
variable order unchanged, 1 not in the basis, every basis element vanishes on every z in Z.
The result is compared with the census values (run JSON) and with the producer's
f4/msolve_all.json row for the same instance.

Usage: msolve_check.py --dump-dir D --run-json R --producer f4/msolve_all.json --raw-dir RAW
                       --out OUT.json [--only n:i,...] [--timeout S]
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import time
from pathlib import Path

ROUND = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+[\d.]+%\s+(\d+) new\s+(\d+) zero")
QUOT = re.compile(r"Dimension of quotient:\s*(\d+)")


def msolve_input(d: dict) -> str:
    N = d["N"]
    polys = []
    for e in d["eqs"]:
        terms: dict[tuple, int] = {}
        for m in e:
            t = tuple(sorted(m))
            terms[t] = terms.get(t, 0) ^ 1  # F_2 sum: repeated monomials cancel
        ts = [t for t, c in terms.items() if c]
        if not ts:
            continue
        polys.append("+".join("*".join(f"x{v}" for v in t) if t else "1" for t in ts))
    polys += [f"x{v}^2+x{v}" for v in range(N)]
    return ",".join(f"x{v}" for v in range(N)) + "\n2\n" + ",\n".join(polys) + "\n"


def parse_term(t: str, N: int):
    """'1*x3^1*x5^2' -> (coefficient, exponent tuple); '1' -> (1, zeros)."""
    exps = [0] * N
    coef = 1
    for f in t.split("*"):
        f = f.strip()
        if re.fullmatch(r"\d+", f):
            coef = int(f) % 2
        else:
            m = re.fullmatch(r"x(\d+)(?:\^(\d+))?", f)
            if not m:
                raise ValueError(f"unparsed factor {f!r}")
            exps[int(m[1])] += int(m[2] or 1)
    return coef, tuple(exps)


def parse_gb(text: str, N: int):
    body = "\n".join(l for l in text.splitlines() if not l.startswith("#"))
    m = re.search(r"\[(.*)\]\s*:", body, re.S)
    if not m:
        raise ValueError("no basis list in msolve output")
    polys = []
    for p in m[1].split(","):
        p = p.strip()
        if not p:
            continue
        terms = {}
        for t in p.replace("-", "+").split("+"):  # characteristic 2: -1 = +1
            t = t.strip()
            if not t:
                continue
            c, e = parse_term(t, N)
            if c:
                terms[e] = terms.get(e, 0) ^ 1
        polys.append([e for e, c in terms.items() if c])
    var_line = re.search(r"#variable order:\s*(.*)", text)
    order = [v.strip() for v in var_line[1].split(",")] if var_line else None
    return polys, order


def evaluate(poly, z: int) -> int:
    acc = 0
    for e in poly:
        if all(((z >> v) & 1) or k == 0 for v, k in enumerate(e)):
            acc ^= 1
    return acc


def k_from_Z(N: int, Z: list[int]) -> int:
    piv = {}
    for z in Z:
        v = 1 | (z << 1)
        while v:
            t = v.bit_length() - 1
            if t in piv:
                v ^= piv[t]
            else:
                piv[t] = v
                break
    return N + 1 - len(piv)


def run_one(dpath: Path, raw: Path, timeout: int) -> dict:
    d = json.loads(dpath.read_text())
    N = d["N"]
    stem = dpath.stem
    raw.mkdir(parents=True, exist_ok=True)
    inp, outp, logp = raw / f"{stem}.ms", raw / f"{stem}.gb", raw / f"{stem}.stdout"
    inp.write_text(msolve_input(d))
    t0 = time.time()
    with open(logp, "w") as lf:
        pr = subprocess.run(["msolve", "-v", "2", "-g", "2", "-t", "1", "-f", str(inp), "-o", str(outp)],
                            stdout=lf, stderr=subprocess.STDOUT, timeout=timeout)
    secs = time.time() - t0
    log = logp.read_text()
    steps = [int(m[1]) for m in (ROUND.match(l) for l in log.splitlines()) if m]
    q = QUOT.search(log)
    res = {"file": dpath.name, "n": d["n"], "N": N, "msolve_exit": pr.returncode, "seconds": round(secs, 2),
           "steps": steps, "max_step_degree": max(steps) if steps else None,
           "quotient_dim": int(q[1]) if q else None, "Z_size": len(d["Z"]), "k_from_Z": k_from_Z(N, d["Z"])}
    ck = {}
    ck["exit_0"] = pr.returncode == 0
    ck["no_invalid_equations"] = bool(re.search(r"#invalid equations\s+0\b", log))
    ck["characteristic_2"] = bool(re.search(r"field characteristic\s+2\b", log))
    polys, order = parse_gb(outp.read_text(), N)
    ck["variable_order_unchanged"] = order == [f"x{v}" for v in range(N)]
    lin = [p for p in polys if p and max(sum(e) for e in p) == 1]
    res["gb_size"] = len(polys)
    res["linear_in_gb"] = len(lin)
    ck["one_not_in_gb"] = not any(p == [tuple([0] * N)] for p in polys)
    ck["gb_vanishes_on_Z"] = all(evaluate(p, z) == 0 for p in polys for z in d["Z"])
    ck["quotient_dim_eq_Z_size"] = res["quotient_dim"] == len(d["Z"])
    ck["linear_in_gb_eq_k"] = len(lin) == res["k_from_Z"]
    res["checks"] = ck
    res["failed_checks"] = [c for c, v in ck.items() if v is not True]
    res["raw"] = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in (inp, outp, logp)}
    return res


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--dump-dir", required=True)
    ap.add_argument("--run-json", required=True)
    ap.add_argument("--producer", default="", help="producer msolve JSON to compare with ('' = none)")
    ap.add_argument("--raw-dir", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--only", default="")
    ap.add_argument("--timeout", type=int, default=7200)
    a = ap.parse_args()
    census = {(r["n"], r["instance"]): r for r in json.loads(Path(a.run_json).read_text())["rows"]}
    prod = {}
    if a.producer:
        for r in json.loads(Path(a.producer).read_text())["rows"]:
            prod[(r["n"], r["instance"])] = r
    keys = [tuple(int(x) for x in s.split(":")) for s in a.only.split(",") if s] or sorted(census)
    out = Path(a.out)
    results = json.loads(out.read_text())["results"] if out.exists() else []
    done = {(r["n"], r["instance"]) for r in results}
    ver = subprocess.run(["dpkg-query", "-W", "-f=${Version}", "msolve"], capture_output=True, text=True).stdout
    binsha = hashlib.sha256(Path("/usr/bin/msolve").read_bytes()).hexdigest()
    for key in keys:
        if key in done:
            continue
        n, i = key
        r = run_one(Path(a.dump_dir) / f"n{n}_i{i}.json", Path(a.raw_dir), a.timeout)
        r["instance"] = i
        c = census[key]
        p = prod.get(key, {})
        r["census"] = {"k": c["k"], "d_iter": c["d_iter"]}
        r["producer"] = {kk: p.get(kk) for kk in ("max_step_degree", "steps", "gb_size", "linear_in_gb", "k")}
        mism = []
        if r["k_from_Z"] != c["k"]:
            mism.append("k_vs_census")
        for kk in ("max_step_degree", "steps", "gb_size", "linear_in_gb") if a.producer else ():
            if p.get(kk) != r[kk]:
                mism.append(f"{kk}_vs_producer")
        if c["d_iter"].isdigit() and int(c["d_iter"]) != r["max_step_degree"]:
            mism.append("degree_vs_census_d_iter")
        if c["d_iter"].startswith(">") and r["max_step_degree"] is not None and r["max_step_degree"] <= int(c["d_iter"][1:]):
            mism.append("degree_below_census_bound")
        r["mismatches"] = mism
        results.append(r)
        print(f"n={n:2} i={i} deg={r['max_step_degree']} steps={r['steps']} lin={r['linear_in_gb']} k={r['k_from_Z']} "
              f"quot={r['quotient_dim']} |Z|={r['Z_size']} {r['seconds']}s mism={mism} failed={r['failed_checks']}",
              flush=True)
        out.write_text(json.dumps({"schema": "validator.msolve-replay/v1", "msolve_version": ver,
                                   "msolve_sha256": binsha, "command": "msolve -v 2 -g 2 -t 1 -f IN -o OUT",
                                   "results": results}, indent=1))
    s = {"instances": len(results), "with_mismatch": sum(bool(r["mismatches"]) for r in results),
         "with_failed_checks": sum(bool(r["failed_checks"]) for r in results)}
    print(json.dumps(s))
    return 0 if s["with_mismatch"] == 0 and s["with_failed_checks"] == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
