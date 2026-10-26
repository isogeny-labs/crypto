#!/usr/bin/env python3
"""Restate the band verdicts of the planner studies as integer facts checked by
Lean's `decide` (PROTOCOL.md, "What is proven in Lean", item 3).

Input: a config (``lean_certificates.json``) naming committed ``ecbench band``
JSON files and, per study, the reference arms and the base-size families.
Output: ``Certificates.lean`` (one theorem per verdict, each closed by
``decide``) and ``certificates.json`` (the same verdicts, with theorem names).

Every number in the Lean file is copied from a band JSON's exact sums:
``charged_sum`` (Σ (G − C) over verified runs) and ``native_sum`` (Σ (U + n_C)).
An arm's mean priced cost at weight ``w = p/q`` is ``(q·c + p·n) / (q·v)``, with
``v`` its verified run count, so "A cheaper than B at w" is the integer fact
``v_B · scaled(c_A, n_A, p, q) < v_A · scaled(c_B, n_B, p, q)``
(``Band.scaled``). A charged sum that is not an integer (calibration-priced
canonicalisation) is taken as the exact dyadic rational of its f64 value, and
every sum of that band is multiplied by the common power-of-two denominator.

``m*(w)``: G(m, w) is the geometric mean over a family's curves of
S_w(m) / S_w(plan), so its argmin is the argmin of the product over curves of
mean priced cost (the plan and sqrt(r) factors are common to every m). Ties go
to the smaller m, as in the studies' ``derive.jq``.

Nothing here is a statistic: the bootstrap intervals stay in ``ecbench compare``.
"""
import hashlib
import json
import subprocess
import sys
from fractions import Fraction
from math import lcm
from pathlib import Path


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def ident(*parts):
    out = "_".join(parts)
    for a, b in (("-", "_"), (".", "_"), ("/", "_")):
        out = out.replace(a, b)
    return out


class BandFile:
    def __init__(self, repo, rel):
        self.rel = rel
        path = repo / rel
        self.sha = sha256(path)
        data = json.loads(path.read_text())
        if data.get("schema") != "ecbench.band/v1":
            sys.exit(f"{rel}: not an ecbench.band/v1 report")
        self.weights = [Fraction(str(w)) for w in data["weights"]]
        curves = {r["curve"] for r in data["rows"]}
        if len(curves) != 1:
            sys.exit(f"{rel}: expected one curve, found {sorted(curves)}")
        self.curve = curves.pop()
        self.arms = {}
        dens = [1]
        for r in data["rows"]:
            if r["verified"] != r["runs"] or r["verified"] == 0:
                sys.exit(f"{rel}: arm {r['arm']} verified {r['verified']} of {r['runs']}")
            if r.get("charged_sum") is None:
                sys.exit(f"{rel}: arm {r['arm']} has no charged_sum (band built before the exact sums)")
            c = Fraction(r["charged_sum"])  # exact value of the f64
            dens.append(c.denominator)
            n = r.get("native_sum")
            self.arms[r["arm"]] = {"c": c, "n": None if n is None else int(n), "v": int(r["verified"])}
        self.scale = lcm(*dens)
        self.nonintegral = [a for a, x in self.arms.items() if x["c"].denominator != 1]
        for x in self.arms.values():
            x["C"] = int(x["c"] * self.scale)
            x["N"] = None if x["n"] is None else x["n"] * self.scale
            assert Fraction(x["C"]) == x["c"] * self.scale

    def priced(self, arm, w):
        """(numerator, v) with mean priced cost = numerator / (q·v·scale), or None."""
        x = self.arms[arm]
        if w != 0 and x["N"] is None:
            return None
        n = 0 if x["N"] is None else x["N"]
        return (w.denominator * x["C"] + w.numerator * n, x["v"])


def safe(text):
    """Keep doc text from opening or closing a Lean comment."""
    return text.replace("/-", "/ -").replace("-/", "- /")


def lit(z):
    return f"({z} : Int)"


def scaled_expr(x, w):
    n = 0 if x["N"] is None else x["N"]
    return f"scaled {lit(x['C'])} {lit(n)} {lit(w.numerator)} {lit(w.denominator)}"


def wtag(w):
    return f"w{w.numerator}_{w.denominator}"


def main():
    if len(sys.argv) != 4:
        sys.exit("usage: lean_certificates.py CONFIG OUT.lean OUT.json")
    cfg_path, out_lean, out_json = map(Path, sys.argv[1:])
    repo = Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True,
                               text=True, check=True).stdout.strip())
    cfg = json.loads(cfg_path.read_text())

    thms, verdicts, sources = [], [], []

    def theorem(name, prop, doc, record):
        thms.append(f"/-- {safe(doc)} -/\ntheorem {name} :\n    {prop} := by decide\n")
        record["theorem"] = name
        verdicts.append(record)

    for study in cfg["studies"]:
        sid = study["id"]
        bands = {tag: BandFile(repo, rel) for tag, rel in study["bands"].items()}
        for tag, b in bands.items():
            sources.append({"study": sid, "band": tag, "path": b.rel, "sha256": b.sha,
                            "curve": b.curve, "scale": b.scale, "nonintegral_charged": b.nonintegral})

        # A. Pairwise orders, every arm against each reference, at every weight.
        for tag, b in bands.items():
            for ref in study["refs"]:
                if ref not in b.arms:
                    continue
                for arm in sorted(b.arms):
                    if arm == ref:
                        continue
                    orders = {}
                    for w in b.weights:
                        pa, pb = b.priced(arm, w), b.priced(ref, w)
                        if pa is None or pb is None:
                            continue
                        lhs, rhs = pb[1] * pa[0], pa[1] * pb[0]
                        rel = "<" if lhs < rhs else (">" if lhs > rhs else "=")
                        orders[w] = rel
                        A, B = b.arms[arm], b.arms[ref]
                        el = f"{lit(B['v'])} * {scaled_expr(A, w)}"
                        er = f"{lit(A['v'])} * {scaled_expr(B, w)}"
                        prop = {"<": f"{el} < {er}", ">": f"{er} < {el}", "=": f"{el} = {er}"}[rel]
                        word = {"<": "cheaper than", ">": "dearer than", "=": "tied with"}[rel]
                        theorem(ident(sid, tag, arm, {"<": "lt", ">": "gt", "=": "eq"}[rel], ref, wtag(w)), prop,
                                f"{sid} {tag}: `{arm}` is {word} `{ref}` at w = {w} (mean priced cost; {b.rel}).",
                                {"study": sid, "band": tag, "kind": "order", "a": arm, "b": ref,
                                 "w": str(w), "order": rel})
                    # B. A strict reversal between w = 0 and w = 1 has an exact crossing weight.
                    w0, w1 = Fraction(0), Fraction(1)
                    o0, o1 = orders.get(w0), orders.get(w1)
                    if o0 in ("<", ">") and o1 in ("<", ">") and o0 != o1:
                        A, B = b.arms[arm], b.arms[ref]
                        cA, nA = B["v"] * A["C"], B["v"] * A["N"]
                        cB, nB = A["v"] * B["C"], A["v"] * B["N"]
                        ws = Fraction(cB - cA, nA - nB)
                        assert 0 < ws < 1
                        p, q = ws.numerator, ws.denominator
                        prop = (f"{lit(B['v'])} * scaled {lit(A['C'])} {lit(A['N'])} {lit(p)} {lit(q)} = "
                                f"{lit(A['v'])} * scaled {lit(B['C'])} {lit(B['N'])} {lit(p)} {lit(q)} ∧ "
                                f"0 < {lit(p)} ∧ {lit(p)} < {lit(q)}")
                        theorem(ident(sid, tag, arm, "crosses", ref),
                                prop,
                                f"{sid} {tag}: `{arm}` and `{ref}` reverse inside the band; their mean priced "
                                f"costs are equal exactly at w* = {p}/{q} ≈ {float(ws):.6f} (Band.sides: "
                                f"one order below, the other above).",
                                {"study": sid, "band": tag, "kind": "crossing", "a": arm, "b": ref,
                                 "w_star": f"{p}/{q}", "w_star_float": float(ws),
                                 "below": orders[w0], "above": orders[w1]})

        # C. m*(w): argmin over a family's grid of the product over its curves.
        for fam in study.get("families", []):
            ftags = fam["bands"]
            grid = [(Fraction(str(g["m"])), g["arms"]) for g in fam["grid"]]
            weights = bands[ftags[0]].weights
            expect = {}
            if fam.get("expect"):
                d = json.loads((repo / fam["expect"]["path"]).read_text())
                for row in eval_path(d, fam["expect"]["rule"]):
                    expect[Fraction(str(row["w"]))] = Fraction(str(row["m_star"]))
            for w in weights:
                vals = []
                for m, arms in grid:
                    ps = [bands[t].priced(arm_for(arms, t), w) for t in ftags]
                    if any(x is None for x in ps):
                        vals = None
                        break
                    num, den = 1, 1
                    for x in ps:
                        num *= x[0]
                        den *= x[1]
                    vals.append((m, arms, num, den))
                if vals is None:
                    continue
                best = vals[0]
                for v in vals[1:]:
                    if v[2] * best[3] < best[2] * v[3]:
                        best = v
                ms = best[0]
                if w in expect and expect[w] != ms:
                    sys.exit(f"{sid} {fam['name']} w={w}: certificate m* = {ms}, study reported {expect[w]}")

                def prod(arms, side):
                    parts = []
                    for t in ftags:
                        x = bands[t].arms[arm_for(arms, t)]
                        parts.append(f"{scaled_expr(x, w)}" if side == "s" else lit(x["v"]))
                    return " * ".join(f"({e})" for e in parts)

                for m, arms, num, den in vals:
                    if m == ms:
                        continue
                    lhs = f"({prod(best[1], 's')}) * ({prod(arms, 'v')})"
                    rhs = f"({prod(arms, 's')}) * ({prod(best[1], 'v')})"
                    strict = m < ms
                    prop = f"{lhs} {'<' if strict else '≤'} {rhs}"
                    theorem(ident(sid, fam["name"], wtag(w), "mstar", str(ms), "vs", str(m)), prop,
                            f"{sid} {fam['name']}: at w = {w}, m* = {ms} is "
                            f"{'strictly cheaper than' if strict else 'no dearer than'} m = {m} "
                            f"(product over {', '.join(ftags)} of mean priced cost).",
                            {"study": sid, "family": fam["name"], "kind": "argmin", "w": str(w),
                             "m_star": str(ms), "versus": str(m), "strict": strict,
                             "reported": None if w not in expect else str(expect[w])})

    header = f"""/-!
# Certificates: the band verdicts of the planner studies, in integers

Generated by `scripts/lean_certificates.sh` from the committed `ecbench band`
JSON files listed below (path, sha256). Do not edit by hand. Each theorem is a
point verdict restated with the band's exact per-arm sums and closed by
`decide`; `Band.lean` proves what such facts imply across the band (crossing
weights, the direction the optimum moves). {len(verdicts)} verdicts.

Sources:
""" + "\n".join(safe(f"* {s['study']} {s['band']}: `{s['path']}` sha256 `{s['sha256']}`"
               + (f" (scale {s['scale']}: non-integral charged sums of {', '.join(s['nonintegral_charged'])})"
                  if s["scale"] != 1 else "")) for s in sources) + "\n-/\n"
    out_lean.write_text("import Band\n\n" + header + "\nnamespace Certificates\nopen Band\n\n"
                        + "\n".join(thms) + "\nend Certificates\n")
    out_json.write_text(json.dumps({"schema": "ic_planner_exhaustive.certificates/v1",
                                    "sources": sources, "verdicts": verdicts}, indent=1) + "\n")
    print(f"{len(verdicts)} verdicts from {len(sources)} band files")


def arm_for(arms, tag):
    return arms[tag] if isinstance(arms, dict) else arms


def eval_path(d, path):
    for k in path.split("."):
        d = d[k]
    return d


if __name__ == "__main__":
    main()
