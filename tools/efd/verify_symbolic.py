#!/usr/bin/env python3
"""Symbolic (computer-algebra) verification of EFD straight-line programs.

For every formula in a pinned catalogue this checks, over the field of rational
functions, that the optimized program's projected output equals the affine
group law stated in the model record. Curve equations are imposed with a
Groebner basis, so the check is an exact polynomial-identity proof for generic
points (denominators nonzero), not a numeric sample.

This complements the Lean chart proofs (which cover the coordinate maps) and the
Rust execution tests (which cover concrete fields). It is CAS-verified, not a
kernel proof: results are written to a JSON report with the exact obligations.

Usage:
    python3 tools/efd/verify_symbolic.py [--catalogue PATH] [--legacy]
                                        [--only PREFIX] [--out REPORT.json]
"""
import argparse
import json
import re
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
ODD_NAME = re.compile(r"\b(\d[A-Za-z_][A-Za-z0-9_]*)\b")


def sanitize(text):
    """Rename identifiers that start with a digit (`2d`, `3overd`)."""
    return ODD_NAME.sub(lambda m: "v_" + m.group(1), text)


def expr(text, env):
    return parse_expr(sanitize(text), local_dict=env, transformations=TRANSFORMS)


# ---------------------------------------------------------------- charts

def encode(rep_id, x, y, z, env, point3=None):
    """Symbolic encoder mirroring src/prime_ecc/efd/formats.rs."""
    out = {}
    z2, z3 = z * z, z * z * z
    if rep_id.startswith("jintersect/"):
        s, c, d = point3
        out.update(S=s * z, C=c * z, D=d * z, Z=z, SC=s * z * c * z, DZ=d * z * z)
        return out
    if rep_id in ("shortw/jacobian", "shortw/jacobian-0", "shortw/jacobian-3", "shortw/modified"):
        out.update(X=x * z2, Y=y * z3, Z=z)
        if rep_id == "shortw/modified":
            out["T"] = env["a"] * z**4
    elif rep_id in ("shortw/projective", "shortw/projective-1", "shortw/projective-3"):
        out.update(X=x * z, Y=y * z, Z=z)
    elif rep_id == "shortw/w12-0":
        out.update(X=x * z, Y=y * z2, Z=z)
    elif rep_id in ("shortw/xyzz", "shortw/xyzz-3"):
        out.update(X=x * z2, Y=y * z3, ZZ=z2, ZZZ=z3)
    elif rep_id in ("shortw/xz", "montgom/xz"):
        out.update(X=x * z, Z=z)
    elif rep_id == "2dik/standard":
        out.update(X=x * z, Y=y * z2, Z=z, ZZ=z2)
    elif rep_id == "3dik/standard":
        out.update(X=x * z2, Y=y * z3, Z=z, ZZ=z2)
    elif rep_id in ("edwards/projective", "twisted/projective", "twistedhessian/projective",
                    "hessian/standard", "hessian/extended"):
        out.update(X=x * z, Y=y * z, Z=z)
        if rep_id == "hessian/extended":
            out.update(XX=(x * z) ** 2, YY=(y * z) ** 2, ZZ=z * z,
                       XY=2 * (x * z) * (y * z), XZ=2 * (x * z) * z, YZ=2 * (y * z) * z)
    elif rep_id in ("twisted/extended", "twisted/extended-1"):
        out.update(X=x * z, Y=y * z, T=x * y * z, Z=z)
    elif rep_id in ("edwards/inverted", "twisted/inverted"):
        out.update(X=z / x, Y=z / y, Z=z)
    elif rep_id == "edwards/yz":
        out.update(Y=env["r"] * y * z, Z=z)
    elif rep_id == "edwards/yzsquared":
        out.update(Y=env["r"] * y * y * z, Z=z)
    elif rep_id in ("jquartic/xyz", "jquartic/2xyz", "jquartic/extended"):
        out.update(X=x * z, Y=y * z2, Z=z)
        if rep_id == "jquartic/extended":
            out.update(U=(x * z) ** 2, V=2 * (x * z) * z, W=z * z, T=(x * z) ** 2 + z * z)
    elif rep_id in ("jquartic/xxyzz", "jquartic/xxyzzr", "jquartic/2xxyzz", "jquartic/2xxyzzr"):
        out.update(X=x * z, Y=y * z2, Z=z, XX=(x * z) ** 2, ZZ=z * z)
        if rep_id.endswith("r"):
            out["R"] = 2 * (x * z) * z
    else:
        raise KeyError(rep_id)
    return out


def project(rep_id, out):
    """Symbolic projector mirroring formats.rs; returns ('full', x, y) etc."""
    if rep_id.startswith("jintersect/"):
        return ("triple", out["S"] / out["Z"], out["C"] / out["Z"], out["D"] / out["Z"])
    if rep_id in ("shortw/xz", "montgom/xz"):
        return ("x", out["X"] / out["Z"])
    if rep_id == "edwards/yz":
        return ("y", out["Y"] / (sp.Symbol("r") * out["Z"]))
    if rep_id == "edwards/yzsquared":
        return ("y2", out["Y"] / (sp.Symbol("r") * out["Z"]))
    if rep_id in ("edwards/inverted", "twisted/inverted"):
        return ("full", out["Z"] / out["X"], out["Z"] / out["Y"])
    if rep_id in ("shortw/jacobian", "shortw/jacobian-0", "shortw/jacobian-3", "shortw/modified"):
        return ("full", out["X"] / out["Z"] ** 2, out["Y"] / out["Z"] ** 3)
    if rep_id == "shortw/w12-0":
        return ("full", out["X"] / out["Z"], out["Y"] / out["Z"] ** 2)
    if rep_id in ("shortw/xyzz", "shortw/xyzz-3"):
        return ("full", out["X"] / out["ZZ"], out["Y"] / out["ZZZ"])
    if rep_id == "2dik/standard":
        return ("full", out["X"] / out["Z"], out["Y"] / out["ZZ"])
    if rep_id == "3dik/standard":
        return ("full", out["X"] / out["Z"] ** 2, out["Y"] / out["Z"] ** 3)
    if rep_id in ("jquartic/xyz", "jquartic/2xyz", "jquartic/extended"):
        return ("full", out["X"] / out["Z"], out["Y"] / out["Z"] ** 2)
    return ("full", out["X"] / out["Z"], out["Y"] / out["Z"])


# ---------------------------------------------------------------- models

MODEL_VARS = {
    "shortw": ["x", "y"],
    "montgom": ["x", "y"],
    "edwards": ["x", "y"],
    "twisted": ["x", "y"],
    "twistedhessian": ["x", "y"],
    "hessian": ["x", "y"],
    "jintersect": ["s", "c", "d"],
    "jquartic": ["x", "y"],
    "2dik": ["x", "y"],
    "3dik": ["x", "y"],
}


def model_equations(model, pts, env):
    """Polynomials (in point variables) that vanish on the model."""
    text = model["source"]["text"]
    eqs = []
    for line in text.splitlines():
        if not line.startswith("satisfying "):
            continue
        body = sanitize(line[len("satisfying "):])
        left, right = body.split("=", 1)
        sub = dict(env)
        for coord, value in zip(MODEL_VARS[model["id"]], pts):
            sub[coord] = value
        eqs.append(sp.expand(expr(left, sub) - expr(right, sub)))
    return eqs


def oracle(model, operation, args, env):
    """Affine law from the model record: list of coordinate expressions."""
    text = model["source"]["text"]
    coords = MODEL_VARS[model["id"]]
    sub = dict(env)
    for i, point in enumerate(args, 1):
        for coord, value in zip(coords, point):
            sub[f"{coord}{i}"] = value
    got = {}
    for line in text.splitlines():
        prefix = operation + " "
        if not line.startswith(prefix):
            continue
        body = sanitize(line[len(prefix):])
        name, value = body.split("=", 1)
        got[name.strip()] = expr(value, sub)
    return [got[c] for c in coords]


# ---------------------------------------------------------------- formulas

def build_env(formula, model, env0, extra_assumptions=()):
    env = dict(env0)
    relations = []
    for assumption in list(extra_assumptions) + list(formula["assumptions"]):
        body = sanitize(assumption)
        left, right = body.split("=", 1)
        lhs = parse_expr(left, local_dict=env, transformations=TRANSFORMS)
        if not isinstance(lhs, sp.Expr):
            continue
        stripped = body.replace(" ", "")
        if stripped in ("Z1=Z2", "Z2=Z1"):
            env["z1"] = sp.Symbol("z1")
            env["z2"] = env["z1"]
            continue
        if lhs.is_Symbol:
            base = str(lhs)
            if base in ("z1", "z2"):
                env[base] = 1
            elif base in ("Z1", "ZZ1", "ZZZ1", "Z2", "ZZ2", "ZZZ2", "Z3", "ZZ3", "ZZZ3"):
                env["z" + base[-1]] = 1 if right.strip() == "1" else expr(right, env)
            elif base == "Z2" and right.strip() == "Z1":
                env["z2"] = env["z1"]
            else:
                env[base] = expr(right, env)
        elif lhs.is_Mul and lhs.args[0].is_number:
            env[str(lhs.args[1])] = expr(right, env) / lhs.args[0]
        elif lhs.is_Mul and lhs.args[-1].is_number:
            env[str(lhs.args[0])] = expr(right, env) / lhs.args[-1]
        else:
            sub = dict(env)
            relations.append(sp.expand(lhs - expr(right, sub)))
    return env, relations


def reduce_zero(num, equations, variables):
    num = sp.expand(num)
    if num == 0:
        return True
    eqs = [sp.expand(e) for e in equations if e != 0]
    if not eqs:
        return num == 0
    # Keep every non-point symbol (parameters, scales, derived caches) as a free
    # generator so sympy does not coerce them into the coefficient domain.
    gens = list(variables)
    for sym in sorted(num.free_symbols, key=str):
        if sym not in gens and not sym.is_number:
            gens.append(sym)
    try:
        basis = sp.groebner(eqs, *gens, order="grevlex")
    except Exception:
        return None
    return sp.expand(basis.reduce(num)[1]) == 0


def check(formula, catalogue):
    model = next(m for m in catalogue["models"] if m["id"] == formula["representation"].split("/")[0])
    rep_id = formula["representation"]
    z1, z2, z3 = sp.symbols("z1 z2 z3")
    env = {"z1": z1, "z2": z2, "z3": z3}
    # Pre-populate every coordinate name so implicit-multiplication parsing
    # cannot split `Z1` into `Z*1`.
    for base in ("X", "Y", "Z", "ZZ", "ZZZ", "S", "C", "D", "SC", "DZ",
                 "U", "V", "W", "T", "R", "XX", "YY", "XY", "XZ", "YZ"):
        for i in (1, 2):
            env[f"{base}{i}"] = sp.Symbol(f"{base}{i}")
    for coord in MODEL_VARS[model["id"]]:
        for i in (1, 2):
            env[f"{coord}{i}"] = sp.Symbol(f"{coord}{i}")
    # model parameters become free symbols
    for line in model["source"]["text"].splitlines():
        if line.startswith("parameter "):
            name = sanitize(line[len("parameter "):].strip())
            env[name] = sp.Symbol(name)
    # chart parameter r with d = r^2 for the YZ charts
    if rep_id in ("edwards/yz", "edwards/yzsquared"):
        env.setdefault("r", sp.Symbol("r"))
        env["d"] = env["r"] ** 2
        env["c"] = 1
    rep = next(r for r in catalogue["representations"] if r["id"] == rep_id)
    env, extra = build_env(formula, model, env, rep.get("assumptions", []))
    variables = [sp.Symbol(v) for v in MODEL_VARS[model["id"]]]

    op = formula["operation"]
    if rep_id.startswith("jintersect/"):
        p = [sp.Symbol("s1"), sp.Symbol("c1"), sp.Symbol("d1")]
        q = [sp.Symbol("s2"), sp.Symbol("c2"), sp.Symbol("d2")]
    else:
        p = [sp.Symbol("x1"), sp.Symbol("y1")]
        q = [sp.Symbol("x2"), sp.Symbol("y2")]
    env.update(dict(zip([f"{c}1" for c in MODEL_VARS[model["id"]]], p)))
    env.update(dict(zip([f"{c}2" for c in MODEL_VARS[model["id"]]], q)))

    inputs = {}
    unit = {a.replace(" ", "") for a in formula["assumptions"]}
    if "X1=1" in unit or "S1=1" in unit:
        p[0] = sp.Integer(1)
        env["z1"] = 1
    if "X2=1" in unit or "S2=1" in unit:
        q[0] = sp.Integer(1)
        env["z2"] = 1
    def encoded(point, scale):
        if rep_id.startswith("jintersect/"):
            return encode(rep_id, None, None, scale, env, point3=point)
        return encode(rep_id, point[0], point[1], scale, env)

    inputs.update({f"{k}1": v for k, v in encoded(p, env["z1"]).items()})
    if op in ("addition", "diffadd", "ladder"):
        inputs.update({f"{k}2": v for k, v in encoded(q, env["z2"]).items()})
    if op in ("diffadd", "ladder"):
        # input 1 is the difference Q-P, input 2 is P, input 3 is Q
        neg = oracle(model, "negation", [q], env)
        diff = oracle(model, "addition", [p, neg], env)
        inputs = {f"{k}1": v for k, v in encoded(diff, env["z1"]).items()}
        inputs.update({f"{k}2": v for k, v in encoded(p, env["z2"]).items()})
        inputs.update({f"{k}3": v for k, v in encoded(q, env["z3"]).items()})
    prog = {k: v for k, v in env.items() if isinstance(v, sp.Expr) and k.isidentifier()}
    prog.update(inputs)

    program_text = formula["program"]["text"]
    if "error:" in program_text:
        program_text = "\n".join(
            line[len("compute "):]
            for line in formula["record"]["text"].splitlines()
            if line.startswith("compute ")
        )
    for line in program_text.splitlines():
        if not line.strip():
            continue
        lhs, rhs = line.split("=", 1)
        prog[sanitize(lhs).strip()] = expr(rhs, {**env, **prog})

    # outputs are index 3 (legacy scaling uses index 2)
    index = "3" if any(f"{b}3" in prog for b in ("X", "Y", "S")) else "2"

    bases = ["X", "Y", "Z", "ZZ", "ZZZ", "S", "C", "D", "SC", "DZ"]
    out = project(rep_id, {k: prog[f"{k}{index}"] for k in bases if f"{k}{index}" in prog})
    if op == "addition":
        expected = oracle(model, "addition", [p, q], env)
    elif op == "doubling":
        expected = oracle(model, "doubling", [p], env)
    elif op == "tripling":
        d = oracle(model, "doubling", [p], env)
        expected = oracle(model, "addition", [p, d], env)
    elif op == "scaling":
        expected = list(p)
    elif op == "diffadd":
        out = project(rep_id, {k: prog[f"{k}5"] for k in bases if f"{k}5" in prog})
        expected = oracle(model, "addition", [p, q], env)
    elif op == "ladder":
        out = project(rep_id, {k: prog[f"{k}5"] for k in bases if f"{k}5" in prog})
        expected = oracle(model, "addition", [p, q], env)
    else:
        return ("skip", "operation")

    eqs = model_equations(model, p, env) + model_equations(model, q, env) if len(p) == 2 else \
        model_equations(model, p, env) + model_equations(model, q, env)
    eqs += extra
    obligations = []
    if out[0] == "triple":
        pairs = [(out[1], expected[0]), (out[2], expected[1]), (out[3], expected[2])]
    elif out[0] == "full":
        pairs = [(out[1], expected[0]), (out[2], expected[1])]
    elif out[0] == "y":
        pairs = [(out[1], expected[1])]
    elif out[0] == "y2":
        pairs = [(out[1], expected[1] ** 2)]
    else:
        pairs = [(out[1], expected[0])]
    for a, b in pairs:
        diff = sp.cancel(a - b)
        num, _ = sp.fraction(diff)
        ok = reduce_zero(num, eqs, variables)
        if ok is None:
            return ("unknown", "groebner")
        obligations.append(bool(ok))
    return ("pass" if all(obligations) else "fail", obligations)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--catalogue", default=str(ROOT / "src/prime_ecc/efd/catalogue.json"))
    ap.add_argument("--out", default=str(ROOT / "docs/efd/prime-validation/symbolic.json"))
    ap.add_argument("--only", default="")
    args = ap.parse_args()
    catalogue = json.load(open(args.catalogue))
    report = []
    for formula in catalogue["formulas"]:
        if args.only and not formula["id"].startswith(args.only):
            continue
        try:
            status, detail = check(formula, catalogue)
        except Exception as e:  # noqa: BLE001
            status, detail = "error", f"{type(e).__name__}: {e}"
        report.append({"formula": formula["id"], "status": status, "detail": detail})
        print(f"{status:8s} {formula['id']}", flush=True)
    Path(args.out).write_text(json.dumps(report, indent=2) + "\n")
    from collections import Counter
    print(Counter(r["status"] for r in report))


if __name__ == "__main__":
    main()
