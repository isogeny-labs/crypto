#!/usr/bin/env python3
"""Macaulay2 / ComprehensiveGBs twin of ``scripts/ic_semaev_msolve.py``.

Solve the Semaev S4 + membership decomposition system over F_p with Macaulay2's
``ComprehensiveGBs`` package (Suzuki-Sato ISSAC'06 / Kapur-Sun-Wang JSC'13):

    S4(x1,x2,x3,xQ) = 0,   h(x1) = h(x2) = h(x3) = 0

with ``h`` the degree-k membership polynomial of the base x-set.  This is the
same object as ``examples/ic_semaev_gb.rs`` and ``scripts/ic_semaev_msolve.py``,
but handed to Macaulay2.

Two modes:

* ``specialized`` -- the target abscissa xQ is a *constant*; compute an ordinary
  Groebner basis (directly comparable to the msolve / in-repo F4 solve).
* ``parametric``  -- xQ is a free *parameter* ``t`` in the tower ring
  ``ZZ/p[t][x1,x2,x3]``; compute ONE comprehensive Groebner system
  (``parametricGBMain`` / ``comprehensiveGBMain``) whose branches cover every
  target at once.  This is the amortization a comprehensive GB offers: the fixed
  curve/base system is solved once over all targets, not re-solved per trial.

``verify`` runs both and checks, inside Macaulay2, that the parametric branch
selected by ``t = xQ`` generates the same ideal as the specialized solve.

External dependencies, located at runtime (NOTHING is vendored -- Macaulay2 is
GPL, this repo is MIT; this mirrors how msolve/Singular/Magma are treated):

* ``M2`` on ``PATH``.
* the ``ComprehensiveGBs.m2`` package file, via env ``CGB_PACKAGE_PATH`` (it is
  not shipped with released Macaulay2; fetch it from the Macaulay2/M2 repo,
  ``M2/Macaulay2/packages/ComprehensiveGBs.m2``, or the Warwick-2026 CGBs branch).

Both are detected up front and the script exits with a clear, actionable message
when either is missing, so a host without them fails cleanly instead of wrongly.
"""
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
import time


# ───────────────────────── dependency location ──────────────────────────
def locate_m2() -> str:
    m2 = shutil.which("M2")
    if m2 is None:
        sys.exit(
            "error: Macaulay2 ('M2') not found on PATH.\n"
            "  Install it (e.g. `nix profile install nixpkgs#macaulay2`, or your\n"
            "  platform's package), then re-run.  This backend shells out to M2."
        )
    return m2


def locate_package() -> str:
    path = os.environ.get("CGB_PACKAGE_PATH")
    if not path:
        sys.exit(
            "error: CGB_PACKAGE_PATH is not set.\n"
            "  Point it at the ComprehensiveGBs.m2 package file, e.g.\n"
            "    export CGB_PACKAGE_PATH=/path/to/M2/Macaulay2/packages/ComprehensiveGBs.m2\n"
            "  (from the Macaulay2/M2 repository; the package is not vendored here)."
        )
    if not os.path.isfile(path):
        sys.exit(f"error: CGB_PACKAGE_PATH={path!r} is not a file.")
    return path


# ───────────────────────── toy curve arithmetic ─────────────────────────
# Self-contained (p passed explicitly), used only to pick a decomposable target
# and a base x-set.  The algebra of S4/membership is done by M2, not here.
# ic_semaev_m2.py is a TOY-INPUT correctness driver with planted targets, not an
# admitted benchmark. A real M31-scale (p = 2^31-1) cold measurement needs frozen
# public inputs, subgroup provenance and the gated benchmark path, not this; so p
# is capped and the x-scan is bounded (no range(p) materialization).
TOY_P_MAX = 2 ** 24


def curve_points(p: int, a: int, b: int, want_x: int | None = None):
    """On-curve (x, y) points, scanning x upward; stops once `want_x` distinct
    on-curve x-values have been seen (bounded work, no full range(p) scan)."""
    try:
        from sympy import sqrt_mod
    except ImportError:
        sqrt_mod = None
    pts = []
    seen_x = set()
    for x in range(p):
        r = (x * x * x + a * x + b) % p
        if sqrt_mod is not None:
            y = sqrt_mod(r, p)
        else:
            y = next((yy for yy in range(p) if (yy * yy) % p == r), None)
        if y is not None:
            pts.append((x, y))
            if y:
                pts.append((x, (-y) % p))
            seen_x.add(x)
            if want_x is not None and len(seen_x) >= want_x:
                break
    return pts


def eadd(P1, P2, p: int, a: int):
    if P1 is None:
        return P2
    if P2 is None:
        return P1
    x1, y1 = P1
    x2, y2 = P2
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def base_set_and_target(p: int, a: int, b: int, k: int, force_xq=None):
    if p > TOY_P_MAX:
        sys.exit(
            f"error: p={p} exceeds the toy bound {TOY_P_MAX}. This is a toy-input\n"
            "  correctness driver with planted targets, not an admitted M31-scale\n"
            "  benchmark; a cold measurement needs frozen public inputs, subgroup\n"
            "  provenance and the gated benchmark path (PR #5250 defers that A/B)."
        )
    aff = curve_points(p, a, b, want_x=k)
    by_x = {}
    for q in aff:
        by_x.setdefault(q[0], q)
    xs_all = sorted(by_x)
    if len(xs_all) < k:
        sys.exit(f"error: only {len(xs_all)} on-curve x-values < k={k} for p={p}.")
    xset = xs_all[:k]
    if force_xq is not None:
        return xset, int(force_xq) % p
    pts = [by_x[x] for x in xset]
    for i in range(len(pts)):
        for j in range(len(pts)):
            for l in range(len(pts)):
                s = eadd(eadd(pts[i], pts[j], p, a), pts[l], p, a)
                if s is not None:
                    return xset, s[0]
    sys.exit("error: no decomposable target found on this base.")


# ───────────────────────── Macaulay2 code generation ────────────────────
_HEADER = 'loadPackage("ComprehensiveGBs", FileName => "{pkg}", Reload => true)\n'

# S4 via the two-quadratic resultant; curve coeffs ca, cb; M2 does the algebra.
_S4_DEFS = """ca = {a}_R; cb = {b}_R;
abc = (u, v) -> (
    Aq := (u - v)^2;
    Bq := -2 * ((u + v) * (u * v + ca) + 2 * cb);
    Cq := (u * v - ca)^2 - 4 * cb * (u + v);
    {{Aq, Bq, Cq}}
);
s4 = (p1, p2, p3, p4) -> (
    q1 := abc(p1, p2);
    q2 := abc(p3, p4);
    ac := q1#0 * q2#2 - q2#0 * q1#2;
    ab := q1#0 * q2#1 - q2#0 * q1#1;
    bc := q1#1 * q2#2 - q2#1 * q1#2;
    ac^2 - ab * bc
);
xvars = gens R;
memb = i -> product({xset}, w -> xvars#i - w);
"""


def _m2_xset(xset):
    return "{" + ", ".join(str(int(w)) for w in xset) + "}"


def gen_specialized(p, a, b, xset, xq, pkg):
    s = _HEADER.format(pkg=pkg)
    s += f"R = ZZ/{p}[x1, x2, x3, MonomialOrder => GRevLex];\n"
    s += _S4_DEFS.format(a=a % p, b=b % p, xset=_m2_xset(xset))
    s += f"S4 = s4(x1, x2, x3, {xq % p}_R);\n"
    s += "F = {S4, memb 0, memb 1, memb 2};\n"
    s += "I = ideal F;\n"
    s += "G = flatten entries gens gb I;\n"
    s += '<< "MODE specialized" << endl;\n'
    s += '<< "ONE_IN_IDEAL " << (if 1 % I == 0 then 1 else 0) << endl;\n'
    s += '<< "DIM " << dim I << endl;\n'
    s += '<< "GBSIZE " << #G << endl;\n'
    s += 'scan(G, g -> << "GB " << toString g << endl);\n'
    s += '<< "DONE" << endl;\n'
    return s


def gen_parametric(p, a, b, xset, pkg, method):
    s = _HEADER.format(pkg=pkg)
    s += f"U = ZZ/{p}[t, MonomialOrder => GRevLex];\n"
    s += f"R = U[x1, x2, x3, MonomialOrder => GRevLex];\n"
    s += _S4_DEFS.format(a=a % p, b=b % p, xset=_m2_xset(xset))
    s += "S4 = s4(x1, x2, x3, t);\n"
    s += "F = {S4, memb 0, memb 1, memb 2};\n"
    s += f"L = {method} F;\n"
    s += 'emit = LL -> concatenate between(" ;; ", apply(toList LL, toString));\n'
    s += '<< "MODE parametric" << endl;\n'
    s += '<< "NBRANCHES " << #L << endl;\n'
    s += ("scan(L, br -> (\n"
          '    << "BRANCH" << endl;\n'
          '    << "E " << emit br#0 << endl;\n'
          '    << "N " << emit br#1 << endl;\n'
          '    << "G " << emit br#2 << endl;\n'
          "));\n")
    s += '<< "DONE" << endl;\n'
    return s


def gen_verify(p, a, b, xset, xq, pkg, method):
    s = _HEADER.format(pkg=pkg)
    s += f"U = ZZ/{p}[t, MonomialOrder => GRevLex];\n"
    s += f"R = U[x1, x2, x3, MonomialOrder => GRevLex];\n"
    s += _S4_DEFS.format(a=a % p, b=b % p, xset=_m2_xset(xset))
    s += "S4 = s4(x1, x2, x3, t);\n"
    s += "F = {S4, memb 0, memb 1, memb 2};\n"
    s += f"L = {method} F;\n"
    s += f"xq = {xq % p};\n"
    # Specialize t -> xq and coerce into a true field-coefficient ring R0 so that
    # ideal equality is a standard 0-dim comparison, not a GB over ZZ/p[t].
    s += f"R0 = ZZ/{p}[x1, x2, x3, MonomialOrder => GRevLex];\n"
    s += "spec = f -> sub(sub(f, {t => xq}), R0);\n"
    s += "Ispec = ideal apply(F, spec);\n"
    s += ("matched = select(L, br -> all(toList br#0, e -> sub(e, {t => xq}) == 0) "
          "and (#(toList br#1) == 0 or any(toList br#1, nn -> sub(nn, {t => xq}) != 0)));\n")
    s += '<< "NBRANCHES " << #L << endl;\n'
    s += '<< "NMATCH " << #matched << endl;\n'
    s += ("if #matched >= 1 then (\n"
          "    Gbr := (matched#0)#2;\n"
          "    Jbr := ideal apply(toList Gbr, spec);\n"
          '    << "VERIFY " << (if Ispec == Jbr then "PASS" else "FAIL") << endl;\n'
          ") else << \"VERIFY NOBRANCH\" << endl;\n")
    s += '<< "DONE" << endl;\n'
    return s


# ───────────────────────── run + parse ──────────────────────────────────
def run_m2(script: str, m2: str, timeout: int):
    with tempfile.TemporaryDirectory() as d:
        path = os.path.join(d, "prog.m2")
        with open(path, "w") as fh:
            fh.write(script)
        t0 = time.time()
        try:
            r = subprocess.run([m2, "--script", path], capture_output=True,
                               text=True, timeout=timeout)
        except subprocess.TimeoutExpired:
            return {"timeout": True, "wall": timeout}
        wall = time.time() - t0
        return {"timeout": False, "wall": round(wall, 3), "rc": r.returncode,
                "stdout": r.stdout, "stderr": r.stderr}


def parse(stdout: str) -> dict:
    out = {"branches": [], "gb": []}
    cur = None
    for line in stdout.splitlines():
        if line == "BRANCH":
            cur = {"E": [], "N": [], "G": []}
            out["branches"].append(cur)
            continue
        if " " in line:
            key, _, val = line.partition(" ")
        else:
            key, val = line, ""
        if key in ("MODE",):
            out["mode"] = val
        elif key in ("ONE_IN_IDEAL", "DIM", "GBSIZE", "NBRANCHES", "NMATCH"):
            out[key.lower()] = int(val) if val.lstrip("-").isdigit() else val
        elif key == "VERIFY":
            out["verify"] = val
        elif key == "GB":
            out["gb"].append(val)
        elif key in ("E", "N", "G") and cur is not None:
            cur[key] = [t for t in val.split(" ;; ") if t]
        elif key == "DONE":
            out["done"] = True
    return out


# ───────────────────────── result validation ────────────────────────────
def validate_result(parsed: dict, mode: str):
    """Return (ok, reason). Reject incomplete or semantically-invalid M2 output
    so the CLI never fail-opens on an rc==0 run: completion (DONE), count
    consistency, and -- for verify -- exactly one matched stratum and PASS."""
    if not parsed.get("done"):
        return False, "output did not reach DONE (truncated or aborted mid-run)"
    if mode in ("specialized", "parametric") and parsed.get("mode") != mode:
        return False, f"mode marker {parsed.get('mode')!r} != requested {mode!r}"
    if mode == "specialized":
        if not isinstance(parsed.get("dim"), int):
            return False, "no DIM in specialized output"
        if parsed.get("gbsize") != len(parsed.get("gb", [])):
            return False, f"GBSIZE {parsed.get('gbsize')} != {len(parsed.get('gb', []))} GB lines"
    elif mode == "parametric":
        nb = parsed.get("nbranches")
        if not isinstance(nb, int):
            return False, "no NBRANCHES in parametric output"
        if nb != len(parsed.get("branches", [])):
            return False, f"NBRANCHES {nb} != {len(parsed.get('branches', []))} emitted branches"
    elif mode == "verify":
        if not isinstance(parsed.get("nbranches"), int):
            return False, "no NBRANCHES in verify output"
        if parsed.get("nmatch") != 1:
            return False, f"NMATCH {parsed.get('nmatch')} != 1 (target must hit exactly one disjoint stratum)"
        if parsed.get("verify") != "PASS":
            return False, f"VERIFY {parsed.get('verify')!r} (parametric branch != direct solve)"
    return True, ""


# ───────────────────────── CLI ──────────────────────────────────────────
# Exit codes: 0 ok; 1 M2 process error; 2 timeout; 3 output invalid/incomplete.
def main(argv=None):
    ap = argparse.ArgumentParser(description="Semaev S4 decomposition via Macaulay2 ComprehensiveGBs")
    ap.add_argument("mode", choices=["specialized", "parametric", "verify"])
    ap.add_argument("--p", type=int, default=4099)
    ap.add_argument("--a", type=int, default=1)
    ap.add_argument("--b", type=int, default=1)
    ap.add_argument("--k", type=int, default=5, help="base x-set size")
    ap.add_argument("--target", type=int, default=None, help="force target xQ")
    ap.add_argument("--method", choices=["parametricGBMain", "comprehensiveGBMain"],
                    default="parametricGBMain")
    ap.add_argument("--timeout", type=int, default=600)
    ap.add_argument("--print-script", action="store_true")
    args = ap.parse_args(argv)

    m2 = locate_m2()
    pkg = locate_package()
    xset, xq = base_set_and_target(args.p, args.a, args.b, args.k, args.target)

    if args.mode == "specialized":
        script = gen_specialized(args.p, args.a, args.b, xset, xq, pkg)
    elif args.mode == "parametric":
        script = gen_parametric(args.p, args.a, args.b, xset, pkg, args.method)
    else:
        script = gen_verify(args.p, args.a, args.b, xset, xq, pkg, args.method)

    if args.print_script:
        sys.stdout.write(script)
        return 0

    res = run_m2(script, m2, args.timeout)
    if res.get("timeout"):
        print(f"timeout after {res['wall']}s (p={args.p} k={args.k} mode={args.mode})")
        return 2
    if res["rc"] != 0:
        sys.stderr.write(res.get("stderr", ""))
        print(f"M2 exited {res['rc']}")
        return 1
    parsed = parse(res["stdout"])
    ok, reason = validate_result(parsed, args.mode)
    if not ok:
        # Fail closed: an rc==0 run whose output is incomplete or semantically
        # wrong is a failure, not a pass. Retain the raw streams for triage.
        sys.stderr.write(res.get("stdout", ""))
        sys.stderr.write("\n" + res.get("stderr", ""))
        print(f"invalid result ({args.mode}): {reason}")
        return 3
    parsed.update(p=args.p, a=args.a, b=args.b, k=args.k, target=xq,
                  base_set=xset, wall=res["wall"], method=args.method)
    import json
    print(json.dumps(parsed, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
