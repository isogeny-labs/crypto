#!/usr/bin/env python3
"""Independent recompute of the IC tool programme's published figures.

Shares no code with `icprog`: it reads the raw `*.price.json` and
`*.isolation.jsonl` records (from a directory or a `runs.tar.xz`) and
recomputes, from the definitions in the protocols, what each round's
`analysis.json`, each rule's `analysis.json` and each exploration's
`pairs*.tsv` / `summary*.tsv` publish.

Definitions used (IC_TOOL_PROGRAM.md section 5; R01 PROTOCOL "Accounting"):

- cold time of a process: the median over its in-process repetitions of
  set-up plus the online interval (`setup_ns + ic_online.wall_ns`);
- a process's figure is its first attempt (`rK`, `rK-retry1`, ...) that the
  isolation record does not mark contended;
- a paired comparison is the geometric mean of base/candidate ratios over
  every (row, round) both arms have, with a 95% Student t interval on the
  log ratios.

    recompute.py round DIR        # rounds/R*/: suite, holdouts, A/A
    recompute.py rule DIR         # rule/v*/: S figures by size
    recompute.py exploration DIR  # explorations/*: pairs and summaries
    recompute.py ... --out FILE

Each mode prints a validator report (docs/VALIDATION_GATE.md) whose
`headline` list pairs every published figure with its recompute.  Stdlib only.
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import io
import json
import math
import re
import statistics as st
import sys
import tarfile
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
LEVEL = 0.95


# ---------------------------------------------------------------- statistics

def _betacf(a: float, b: float, x: float) -> float:
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > 1e-300 else 1e-300)
    h = d
    for m in range(1, 400):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > 1e-300 else 1e-300)
        c = 1.0 + aa / c if abs(c) > 1e-300 else 1e-300
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > 1e-300 else 1e-300)
        c = 1.0 + aa / c if abs(c) > 1e-300 else 1e-300
        delta = d * c
        h *= delta
        if abs(delta - 1.0) < 1e-15:
            break
    return h


def _ibeta(a: float, b: float, x: float) -> float:
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log1p(-x)
    if x < (a + 1.0) / (a + b + 2.0):
        return math.exp(lbt) * _betacf(a, b, x) / a
    return 1.0 - math.exp(lbt) * _betacf(b, a, 1.0 - x) / b


def t_cdf(t: float, df: float) -> float:
    x = df / (df + t * t)
    tail = 0.5 * _ibeta(df / 2.0, 0.5, x)
    return 1.0 - tail if t > 0 else tail


def t_quantile(p: float, df: float) -> float:
    lo, hi = 0.0, 1000.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if t_cdf(mid, df) < p:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def geo_ci(ratios: list[float]) -> dict:
    n = len(ratios)
    if n == 0:
        return {"n": 0}
    logs = [math.log(r) for r in ratios]
    m = st.fmean(logs)
    out = {"n": n, "geomean": math.exp(m), "min": min(ratios), "max": max(ratios)}
    if n > 1:
        h = t_quantile(0.5 + LEVEL / 2, n - 1) * st.stdev(logs) / math.sqrt(n)
        out["lo"], out["hi"] = math.exp(m - h), math.exp(m + h)
    return out


# ------------------------------------------------------------------- records

class Records:
    """Every price record and its isolation verdict, by path relative to the runs root."""

    def __init__(self, root: Path):
        self.root = root
        self.price: dict[str, dict] = {}
        self.contended: dict[str, bool] = {}
        self.inputs: list[dict] = []
        tars = sorted(root.glob("*.tar.xz"))
        if tars:
            for t in tars:
                self._tar(t)
        else:
            self._dir(root)

    def _add(self, name: str, raw: bytes):
        name = re.sub(r"^(\./)?", "", name)
        if name.endswith(".price.json"):
            self.price[name[: -len(".price.json")]] = json.loads(raw)
        elif name.endswith(".isolation.jsonl"):
            lines = [json.loads(l) for l in raw.decode().splitlines() if l.strip()]
            if lines:
                last = lines[-1]
                self.contended[name[: -len(".isolation.jsonl")]] = bool((last.get("run") or {}).get("contended", last.get("contended")))

    def _tar(self, path: Path):
        self.inputs.append({"path": str(path.relative_to(REPO)), "sha256": sha256(path)})
        with tarfile.open(path) as tf:
            for m in tf.getmembers():
                if m.isfile() and (m.name.endswith(".price.json") or m.name.endswith(".isolation.jsonl")):
                    self._add(m.name, tf.extractfile(m).read())

    def _dir(self, path: Path):
        self.inputs.append({"path": str(path.relative_to(REPO)) + "/", "sha256": None})
        for p in sorted(path.rglob("*")):
            if p.is_file() and (p.name.endswith(".price.json") or p.name.endswith(".isolation.jsonl")):
                self._add(str(p.relative_to(path)), p.read_bytes())

    def first_clean(self, stem: str) -> dict | None:
        """The first attempt of process `stem` (no retry suffix) not marked contended."""
        attempts = [stem] + [f"{stem}-retry{i}" for i in range(1, 10)]
        for a in attempts:
            if a not in self.price:
                if a == stem:
                    continue
                break
            if not self.contended.get(a, False):
                return self.price[a]
        return None

    def stems(self, prefix: str) -> list[str]:
        out = set()
        for k in self.price:
            if k.startswith(prefix):
                out.add(re.sub(r"-retry\d+$", "", k))
        return sorted(out)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cold_ns(rec: dict) -> float:
    return st.median([r["setup_ns"] + r["ic_online"]["wall_ns"] for r in rec["repetitions"]])


def size_key(rec: dict) -> str:
    return f"k{rec['a']}n{rec['n']}"


# ------------------------------------------------------------------ headline

class Report:
    def __init__(self, what: str, evidence: Path):
        self.what = what
        self.evidence = evidence
        self.headline: list[dict] = []
        self.raw: list[dict] = []
        self.notes: list[str] = []
        self.extra: dict = {}

    def check(self, name: str, claimed, recomputed, rel_tol: float = 1e-9):
        if claimed is None or recomputed is None:
            self.notes.append(f"{name}: claimed {claimed}, recomputed {recomputed} (not compared)")
            return
        self.headline.append({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": rel_tol})

    def agree(self) -> list[dict]:
        bad = []
        for h in self.headline:
            c, r, tol = h["claimed"], h["recomputed"], h["rel_tol"]
            if not math.isclose(c, r, rel_tol=tol, abs_tol=0 if c else 1e-12):
                bad.append(h)
        return bad

    def doc(self) -> dict:
        bad = self.agree()
        return {
            "schema": "ic-tool-programme-validator-report/v1",
            "what": self.what,
            "tool": "research/ic_tool_program/validation/recompute.py",
            "independent": True,
            "independence": "written for the validation gate; shares no code with icprog or the ic binary; reads only the raw records",
            "status": "pass" if self.headline and not bad else "fail",
            "raw_data": self.raw,
            "headline": self.headline,
            "disagreements": bad,
            "notes": self.notes,
            **self.extra,
        }


def compare_interval(rep: Report, label: str, published: dict, ratios: list[float]):
    ci = geo_ci(ratios)
    rep.check(f"{label} n", published.get("n"), ci["n"], 0.0)
    for k in ("geomean", "lo", "hi"):
        rep.check(f"{label} {k}", published.get(k), ci.get(k))


# --------------------------------------------------------------------- modes

def paired_ratios(recs: Records, base_prefix: str, cand_prefix: str):
    ratios, missing = [], 0
    for stem in recs.stems(base_prefix):
        b = recs.first_clean(stem)
        c = recs.first_clean(cand_prefix + stem[len(base_prefix):])
        if b is None or c is None:
            missing += 1
            continue
        ratios.append(cold_ns(b) / cold_ns(c))
    return ratios, missing


def mode_round(d: Path) -> Report:
    ana = json.loads((d / "analysis.json").read_text())
    rep = Report(f"{d.relative_to(REPO)}: every cold-time interval of analysis.json (A/A, suite rows, holdouts)", d)
    recs = Records(d)
    rep.raw = recs.inputs + [{"path": str((d / "analysis.json").relative_to(REPO)), "sha256": sha256(d / "analysis.json")}]
    for row in ana["aa"]:
        r, _ = paired_ratios(recs, f"runs/aa/A/{row['slug']}/", f"runs/aa/A2/{row['slug']}/")
        compare_interval(rep, f"A/A {row['slug']} cold", row["cold"], r)
    for key, sub in (("suite", "compare"), ("holdouts", "holdout")):
        for row in ana[key]:
            r, miss = paired_ratios(recs, f"runs/{sub}/base/{row['slug']}/", f"runs/{sub}/cand/{row['slug']}/")
            compare_interval(rep, f"{key} {row['slug']} cold", row["cold"], r)
            rep.check(f"{key} {row['slug']} missing pairs", row["cold"].get("missing_pairs"), miss, 0.0)
    rep.extra["records"] = len(recs.price)
    return rep


def mode_rule(d: Path) -> Report:
    ana = json.loads((d / "analysis.json").read_text())
    rep = Report(f"{d.relative_to(REPO)}: the S figures and cold ratios by size (analysis.json `sizes`)", d)
    recs = Records(d / "runs")
    rep.raw = recs.inputs + [{"path": str((d / "analysis.json").relative_to(REPO)), "sha256": sha256(d / "analysis.json")}]
    by_size: dict[str, list[dict]] = {}
    for stem in recs.stems(""):
        m = re.match(r"(k\dn\d+)/T(\d+)-R1$", stem)
        if m:
            rec = recs.first_clean(stem)
            if rec is not None:
                by_size.setdefault(m.group(1), []).append(rec)
    out = []
    for s in ana["sizes"]:
        key = f"k{s['a']}n{s['n']}"
        rows = by_size.get(key, [])
        med = [r["median"] for r in rows]
        setup = st.median([m["s_setup"] for m in med])
        ic_on = st.fmean([m["s_ic_online"] for m in med])
        rho_on = st.fmean([m["s_rho_online"] for m in med])
        rho_setup = st.median([m["s_rho_setup"] for m in med])
        rho_model = st.fmean([m["s_rho_model"] for m in med])
        label = f"{s['curve']}"
        rep.check(f"{label} R1 rows", s["rows"]["R1"], len(rows), 0.0)
        rep.check(f"{label} s_setup", s["s_setup"], setup)
        rep.check(f"{label} s_ic_online_mean", s["s_ic_online_mean"], ic_on)
        rep.check(f"{label} s_rho_online_mean", s["s_rho_online_mean"], rho_on)
        rep.check(f"{label} s_rho_setup", s["s_rho_setup"], rho_setup)
        rep.check(f"{label} s_rho_model_mean", s["s_rho_model_mean"], rho_model)
        cold = (setup + ic_on) / (rho_setup + rho_on)
        rep.check(f"{label} cold ratio IC/rho", s["cold_ratio_ic_over_rho"]["value"], cold)
        rep.check(f"{label} online speedup", s["online_speedup"]["mean_ratio"], rho_on / ic_on)
        canon = st.median([r["rho_policy"]["counters"].get("canonicalisations_uncharged", 0) for r in rows])
        out.append({"curve": s["curve"], "log2_r": s["log2_r"], "r": s["r"], "s_ic_cold": setup + ic_on,
                    "s_rho_as_built": rho_setup + rho_on, "s_rho_canonical": rho_setup + rho_model,
                    "rho_canonicalisations_uncharged_median": canon})
    rep.extra["sizes"] = out
    rep.extra["records"] = len(recs.price)
    return rep


def mode_exploration(d: Path) -> Report:
    rep = Report(f"{d.relative_to(REPO)}: every paired cold time and summary row", d)
    recs = Records(d)
    rep.raw = list(recs.inputs)
    by_tail: dict[str, str] = {}
    for stem in sorted(recs.price):
        tail = stem.split("runs/", 1)[-1]
        # the timed pairs, not a stage-probe tree that reuses the names
        if tail not in by_tail or "probe" in by_tail[tail]:
            by_tail[tail] = stem
    for pf in sorted(d.glob("pairs*.tsv")):
        rep.raw.append({"path": str(pf.relative_to(REPO)), "sha256": sha256(pf)})
        rows = list(csv.DictReader(io.StringIO(pf.read_text()), delimiter="\t"))
        cols = [c[: -len("_cold_ns")] for c in rows[0] if c.endswith("_cold_ns")]
        a, b = cols
        groups: dict[str, list[tuple[float, bool]]] = {}
        for row in rows:
            curve = row["curve"]
            vals = {}
            for arm in (a, b):
                stem = locate(recs, by_tail, arm, curve, row["row"], row["round"])
                if stem is None:
                    rep.notes.append(f"{pf.name}: no raw record for {arm} {curve} {row['row']} {row['round']}")
                    continue
                rec = recs.price[stem]
                vals[arm] = cold_ns(rec)
                rep.check(f"{pf.name} {arm} {curve} {row['row']} {row['round']} cold_ns", float(row[f"{arm}_cold_ns"]), vals[arm])
                cont = recs.contended.get(stem)
                if f"{arm}_contended" in row and cont is not None:
                    rep.check(f"{pf.name} {arm} {curve} {row['row']} {row['round']} contended",
                              float(row[f"{arm}_contended"] == "true"), float(cont), 0.0)
            if len(vals) == 2:
                clean = row.get(f"{a}_contended") == "false" and row.get(f"{b}_contended") == "false"
                groups.setdefault(curve, []).append((vals[a] / vals[b], clean))
        sf = pf.with_name(pf.name.replace("pairs", "summary"))
        if sf.exists():
            rep.raw.append({"path": str(sf.relative_to(REPO)), "sha256": sha256(sf)})
            for srow in csv.DictReader(io.StringIO(sf.read_text()), delimiter="\t"):
                g = groups.get(srow["curve"], [])
                clean = [r for r, c in g if c]
                ci = geo_ci(clean)
                ratio_col = [c for c in srow if "_over_" in c][0]
                # the summaries print three decimals
                rep.check(f"{sf.name} {srow['curve']} clean pairs", float(srow["clean_pairs"]), float(ci["n"]), 0.0)
                rep.check(f"{sf.name} {srow['curve']} {ratio_col}", float(srow[ratio_col]), round(ci["geomean"], 3), 0.0)
                if "lo" in ci:
                    rep.check(f"{sf.name} {srow['curve']} lo95", float(srow["lo95"]), round(ci["lo"], 3), 0.0)
                    rep.check(f"{sf.name} {srow['curve']} hi95", float(srow["hi95"]), round(ci["hi"], 3), 0.0)
                if "all_geomean" in srow:
                    rep.check(f"{sf.name} {srow['curve']} all_geomean", float(srow["all_geomean"]),
                              round(geo_ci([r for r, _ in g])["geomean"], 3), 0.0)
    rep.extra["records"] = len(recs.price)
    return rep


def mode_stages(d: Path) -> Report:
    """scan-stages: stages.tsv and shares.tsv, the mean over each curve's clean processes."""
    rep = Report(f"{d.relative_to(REPO)}: the scan's stages and the phase shares (stages.tsv, shares.tsv)", d)
    recs = Records(d)
    rep.raw = list(recs.inputs)
    per: dict[str, list[dict]] = {}
    for stem, rec in recs.price.items():
        if recs.contended.get(stem, True) or "scan_probes" not in rec:
            continue
        p = rec["scan_probes"]
        hz, n = p["tsc_hz"], p["counts"]["summands"]
        ns = {k: p["stages"][k] / hz * 1e9 / n for k in ("subtract", "key", "filter", "admitted", "trial")}
        reps = rec["repetitions"]
        col = sum(r["setup_phases_ns"]["collect"] for r in reps)
        cold = sum(r["setup_ns"] + r["ic_online"]["wall_ns"] for r in reps)
        per.setdefault(rec["curve"], []).append({
            **ns, "admitted_a_summand": p["counts"]["admitted"] / n,
            "scan_of_collection": sum(p["stages"].values()) / hz * 1e9 / col,
            "collection_of_cold": col / cold,
            "build_of_cold": sum(r["setup_phases_ns"]["build"] for r in reps) / cold,
            "selection_of_cold": sum(r["setup_phases_ns"]["select"] for r in reps) / cold})
    for name, cols, places in (("stages.tsv", ["subtract", "key", "filter", "admitted", "trial", "admitted_a_summand"], None),
                               ("shares.tsv", ["scan_of_collection", "collection_of_cold", "build_of_cold", "selection_of_cold"], 3)):
        f = d / name
        rep.raw.append({"path": str(f.relative_to(REPO)), "sha256": sha256(f)})
        for row in csv.DictReader(io.StringIO(f.read_text()), delimiter="\t"):
            g = per.get(row["curve"], [])
            rep.check(f"{name} {row['curve']} processes", float(row["processes"]), float(len(g)), 0.0)
            means = {c: st.fmean(x[c] for x in g) for c in cols}
            if name == "stages.tsv":
                means["scan"] = sum(means[c] for c in ("subtract", "key", "filter", "admitted"))
                for c, col in (("subtract", "subtract_ns"), ("key", "key_ns"), ("filter", "filter_ns"),
                               ("admitted", "admitted_ns"), ("scan", "scan_ns"), ("trial", "trial_ns")):
                    rep.check(f"{name} {row['curve']} {col}", float(row[col]), round(means[c], 2), 0.0)
                rep.check(f"{name} {row['curve']} admitted_a_summand", float(row["admitted_a_summand"]),
                          round(means["admitted_a_summand"], 4), 0.0)
            else:
                for c in cols:
                    rep.check(f"{name} {row['curve']} {c}", float(row[c]), round(means[c], places), 0.0)
    rep.extra["records"] = len(recs.price)
    return rep


def locate(recs: Records, by_tail: dict, arm: str, curve: str, row: str, rnd: str) -> str | None:
    m = re.match(r"icv1-f2m(\d+)-", curve)
    n = m.group(1) if m else None
    for a in ("0", "1"):
        tail = f"{arm}/k{a}n{n}/{row}/{rnd}"
        if tail in by_tail:
            stem = by_tail[tail]
            if recs.price[stem]["curve"] == curve:
                return stem
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("mode", choices=["round", "rule", "exploration", "stages"])
    ap.add_argument("dir", type=Path)
    ap.add_argument("--out", type=Path)
    a = ap.parse_args()
    d = a.dir.resolve()
    rep = {"round": mode_round, "rule": mode_rule, "exploration": mode_exploration, "stages": mode_stages}[a.mode](d)
    doc = rep.doc()
    text = json.dumps(doc, indent=1)
    if a.out:
        a.out.write_text(text + "\n")
    print(json.dumps({"status": doc["status"], "headline": len(doc["headline"]),
                      "disagreements": len(doc["disagreements"]), "notes": len(doc["notes"])}))
    for b in doc["disagreements"][:12]:
        print("DISAGREE", b, file=sys.stderr)
    for n in doc["notes"][:6]:
        print("NOTE", n, file=sys.stderr)
    return 0 if doc["status"] == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
