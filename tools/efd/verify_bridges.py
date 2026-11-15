#!/usr/bin/env python3
"""Symbolic verification of the model-to-Weierstrass bridges.

For every model in a pinned catalogue, the record states `toweierstrass` and
`fromweierstrass` relations. This script composes them and checks, over the
field of rational functions, that the composition is the identity on the
model's coordinates (curve equations imposed when needed).

It complements the Lean chart/cache proofs and the formula verifier: bridges
are rational maps, so the obligation is a rational-function identity.

Usage:
    python3 tools/efd/verify_bridges.py [--catalogue PATH] [--out REPORT.json]
"""
import argparse
import json
import sys
from pathlib import Path

import sympy as sp
from sympy.parsing.sympy_parser import (
    auto_number,
    auto_symbol,
    convert_xor,
    factorial_notation,
    implicit_multiplication,
    lambda_notation,
    parse_expr,
    rationalize,
)

ROOT = Path(__file__).resolve().parents[2]
TRANSFORMS = (lambda_notation, auto_symbol, auto_number, factorial_notation,
              convert_xor, implicit_multiplication, rationalize)

# coordinates per model, and the names used on the Weierstrass side
COORDS = {
    "shortw": ("x", "y", ("weierx", "weiery")),
    "montgom": ("x", "y", ("weierx", "weiery")),
    "edwards": ("x", "y", ("u", "v")),
    "twisted": ("x", "y", ("u", "v")),
    "twistedhessian": ("x", "y", ("u", "v")),
    "hessian": ("x", "y", ("u", "v")),
    "jintersect": ("s", "c", "d", ("x", "y")),
    "jquartic": ("x", "y", ("u", "v")),
    "2dik": ("x", "y", ("u", "v")),
    "3dik": ("x", "y", ("u", "v")),
}


def parse(text, env):
    return parse_expr(text, local_dict=env, transformations=TRANSFORMS)


def relations(model, prefix, env):
    out = {}
    for line in model["source"]["text"].splitlines():
        if not line.startswith(prefix):
            continue
        body = line[len(prefix):]
        name, value = body.split("=", 1)
        out[name.strip()] = parse(value, env)
    return out


def model_equations(model, point, env):
    coords = COORDS[model["id"]][0]
    eqs = []
    sub = dict(env)
    for name, value in zip(coords, point):
        sub[name] = value
    for line in model["source"]["text"].splitlines():
        if not line.startswith("satisfying "):
            continue
        body = line[len("satisfying "):]
        left, right = body.split("=", 1)
        eqs.append(sp.expand(parse(left, sub) - parse(right, sub)))
    return eqs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--catalogue", default=str(ROOT / "src/prime_ecc/efd/catalogue.json"))
    ap.add_argument("--out", default=str(ROOT / "docs/efd/prime-validation/bridges.json"))
    args = ap.parse_args()
    catalogue = json.load(open(args.catalogue))
    report = []
    for model in catalogue["models"]:
        entry = COORDS[model["id"]]
        coords, wcoords = entry[:-1], entry[-1]
        env = {}
        for line in model["source"]["text"].splitlines():
            if line.startswith("parameter "):
                name = line[len("parameter "):].strip()
                env[name] = sp.Symbol(name)
        for name in coords + wcoords:
            env[name] = sp.Symbol(name)
        # YZ charts use r only at representation level; model bridges do not.
        to = relations(model, "toweierstrass ", env)
        frm = relations(model, "fromweierstrass ", env)
        if not to or not frm:
            report.append({"model": model["id"], "status": "skip", "detail": "no bridge"})
            continue
        point = [env[name] for name in coords]
        try:
            image = [to[name] if name in to else env[name] for name in wcoords]
            sub = dict(env)
            sub.update(dict(zip(wcoords, image)))
            # substitute the Weierstrass image into the from-weierstrass map
            composed = []
            for name in coords:
                expr = frm[name]
                composed.append(expr.subs({sp.Symbol(k): v for k, v in zip(wcoords, image)}))
            obligations = []
            eqs = model_equations(model, point, env)
            for got, want in zip(composed, point):
                diff = sp.cancel(got - want)
                num, _ = sp.fraction(diff)
                num = sp.expand(num)
                ok = num == 0
                if not ok and eqs:
                    try:
                        basis = sp.groebner([sp.expand(e) for e in eqs], *map(sp.Symbol, coords), order="grevlex")
                        ok = sp.expand(basis.reduce(num)[1]) == 0
                    except Exception:
                        ok = None
                obligations.append(ok)
            if any(o is None for o in obligations):
                status = "unknown"
            elif all(obligations):
                status = "pass"
            else:
                status = "fail"
            report.append({"model": model["id"], "status": status,
                           "detail": [bool(o) if o is not None else None for o in obligations]})
        except Exception as e:  # noqa: BLE001
            report.append({"model": model["id"], "status": "error", "detail": f"{type(e).__name__}: {e}"})
        print(report[-1], flush=True)
    Path(args.out).write_text(json.dumps(report, indent=2) + "\n")
    from collections import Counter
    print(Counter(r["status"] for r in report))


if __name__ == "__main__":
    main()
