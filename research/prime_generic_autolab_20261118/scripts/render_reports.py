#!/usr/bin/env python3
"""Render SVG/PNG charts and Typst PDFs from recorded JSON outputs only.

Follows .agents/skills/research-visuals/SKILL.md: reads recorded JSON files,
performs no ECDLP or statistical computation beyond formatting recorded numbers,
writes SVG + 150-dpi PNG charts, and compiles Typst PDFs.
"""
from __future__ import annotations

import json
import math
import shutil
import subprocess
from pathlib import Path

HOME = Path(__file__).resolve().parent.parent
FIG_DIR = HOME / "figures"
RUN_REPORT_DIR = HOME / "reports" / "run-20261119T0718Z"


def esc(s: str) -> str:
    return (
        str(s)
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace('"', "&quot;")
    )


def typ_esc(s: str) -> str:
    return (
        str(s)
        .replace("\\", "\\\\")
        .replace("#", "\\#")
        .replace("$", "\\$")
        .replace("@", "\\@")
        .replace("*", "\\*")
        .replace("_", "\\_")
        .replace("<", "\\<")
        .replace(">", "\\>")
    )


def render_png(svg_path: Path, png_path: Path) -> None:
    subprocess.run(
        ["rsvg-convert", "-d", "150", "-p", "150", str(svg_path), "-o", str(png_path)],
        check=True,
    )


def build_band_comparison_svg(ledger: dict, alg_band: dict) -> str:
    w, h = 920, 430
    ml, mr, mt, mb = 68, 30, 56, 86
    pw = (w - ml - mr - 36) / 3.0
    ph = h - mt - mb
    sizes = ["20", "24", "28", "32"]
    weights = [(0, "w = 0 (group ops)"), (1, "w = 0.1 GAE / native op"), (2, "w = 1.0 GAE / native op")]

    # Recorded Round-0 mitm2 S from ledger.seed.json
    seed = json.loads((HOME / "ledger.seed.json").read_text())
    r0 = {k: seed["tracks"]["multibase"][k]["s"] for k in sizes}
    rho = {k: seed["tracks"]["multibase"][k]["rho_s"] for k in sizes}
    mb_inc = {k: ledger["tracks"]["multibase"][k]["s"] for k in sizes}
    gen_inc = {k: ledger["tracks"]["generic"][k]["s"] for k in sizes}

    max_y = 85.0
    lines = [
        f"<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='11'>",
        "<rect width='100%' height='100%' fill='#ffffff'/>",
        f"<text x='{ml}' y='22' font-size='14' font-weight='bold' fill='#111'>Generic Prime-Order E(F_p) Index Calculus: Recorded Mean S Across §5a Pricing Band</text>",
        f"<text x='{ml}' y='40' font-size='10.5' fill='#555'>Curves: prime_search seed 59297 (k=20..32, cofactor 1). Sessions: Round-0 ECBS1h6a7cf080ed52..cb28a02cb5f9; Rounds 1-6 holdouts (48 pairs/size, all verified).</text>",
    ]

    series_meta = [
        ("Round-0 mitm2", "#8c96a6", r0),
        ("Multibase C=3 (wsum)", "#2563eb", mb_inc),
        ("Generic C=2 (tw2l/wwalk)", "#0d9488", gen_inc),
        ("Matched rho.negation", "#16a34a", rho),
    ]

    for wi, (_, wlabel) in enumerate(weights):
        x0 = ml + wi * (pw + 18)
        lines.append(
            f"<rect x='{x0:.1f}' y='{mt}' width='{pw:.1f}' height='{ph}' fill='#fafafa' stroke='#e5e7eb'/>"
        )
        lines.append(
            f"<text x='{x0 + pw/2:.1f}' y='{mt - 8}' text-anchor='middle' font-size='11.5' font-weight='bold' fill='#222'>{esc(wlabel)}</text>"
        )
        for tick in [0, 20, 40, 60, 80]:
            yy = mt + ph * (1.0 - tick / max_y)
            lines.append(
                f"<line x1='{x0:.1f}' x2='{x0 + pw:.1f}' y1='{yy:.1f}' y2='{yy:.1f}' stroke='#e5e7eb' stroke-dasharray='2 2'/>"
            )
            if wi == 0:
                lines.append(
                    f"<text x='{x0 - 6:.1f}' y='{yy + 3.5:.1f}' text-anchor='end' font-size='9.5' fill='#555'>{tick}</text>"
                )

        group_w = (pw - 16) / len(sizes)
        bar_w = group_w / 4.6
        for si, k in enumerate(sizes):
            gx = x0 + 8 + si * group_w
            for bi, (_, color, data) in enumerate(series_meta):
                val = data[k][wi]
                bh = min(ph, ph * (val / max_y))
                bx = gx + bi * (bar_w + 1.5)
                by = mt + ph - bh
                lines.append(
                    f"<rect x='{bx:.1f}' y='{by:.1f}' width='{bar_w:.1f}' height='{max(bh, 1.2):.1f}' fill='{color}' rx='1'/>"
                )
                if wi in (0, 2) and bi in (0, 1, 2):
                    lines.append(
                        f"<text x='{bx + bar_w/2:.1f}' y='{by - 3:.1f}' text-anchor='middle' font-size='7.5' fill='#222'>{val:.1f}</text>"
                    )
            lines.append(
                f"<text x='{gx + 2*bar_w:.1f}' y='{mt + ph + 15:.1f}' text-anchor='middle' font-size='10' fill='#222'>k={k}</text>"
            )

    lines.append(
        f"<text transform='translate(18,{mt + ph/2:.1f}) rotate(-90)' text-anchor='middle' font-size='11' fill='#333'>Mean S = priced GAE / sqrt(r)</text>"
    )

    # Legend
    lx = ml
    ly = h - 42
    for label, color, _ in series_meta:
        lines.append(f"<rect x='{lx}' y='{ly - 9}' width='12' height='12' fill='{color}' rx='2'/>")
        lines.append(f"<text x='{lx + 17}' y='{ly + 1}' font-size='10.5' fill='#222'>{esc(label)}</text>")
        lx += 205

    alg_rows = {r["arm"]: r["mean_s"] for r in alg_band["rows"]}
    alg_s = alg_rows["inc"]
    lines.append(
        f"<text x='{ml}' y='{h - 14}' font-size='9.5' fill='#555'>Note: Round-0 algebraic track at k=20 (pkm-iso-t6-g4-s1093, session ECBS1h7ed674fd42dd, 8/8 verified) recorded S = {alg_s[0]:.2f} / {alg_s[1]:.0f} / {alg_s[2]:.0f} across w = 0 / 0.1 / 1 (off-scale at w &gt;= 0.1).</text>"
    )
    lines.append("</svg>\n")
    return "\n".join(lines)


def build_sweeper_svg(t_show: dict, rf_show: dict, alg_show: dict) -> str:
    w, h = 880, 360
    ml, mr, mt, mb = 64, 28, 54, 68
    pw = (w - ml - mr - 36) / 3.0
    ph = h - mt - mb

    panels = [
        (
            "prime-table (rungs 16..22)",
            [16, 18, 20, 22],
            t_show["top"][0]["rung_log2_ratio"],
            f"best id={t_show['top'][0]['id']} (m=3): +{t_show['top'][0]['objective_log2_ratio']:.2f} @80b",
            "#2563eb",
            (-6.0, 6.0),
        ),
        (
            "prime-rootfind (rungs 16..22)",
            [16, 18, 20, 22],
            rf_show["top"][0]["rung_log2_ratio"],
            f"best id={rf_show['top'][0]['id']} (m=3): {rf_show['top'][0]['objective_log2_ratio']:.2f} @80b",
            "#0d9488",
            (-6.0, 6.0),
        ),
        (
            "prime-algebraic (rungs 10,12)",
            [10, 12],
            alg_show["top"][0]["rung_log2_ratio"],
            f"best id={alg_show['top'][0]['id']} (m=3, deg=7): +{alg_show['top'][0]['objective_log2_ratio']:.2f}",
            "#9333ea",
            (-6.0, 6.0),
        ),
    ]

    lines = [
        f"<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='11'>",
        "<rect width='100%' height='100%' fill='#ffffff'/>",
        f"<text x='{ml}' y='22' font-size='14' font-weight='bold' fill='#111'>ic sweep TPE Studies on Generic Prime Curves: Best Trial log2(S_trial / S_baseline) Across Rungs</text>",
        f"<text x='{ml}' y='40' font-size='10.5' fill='#555'>Below 0.0 (dashed line) beats each space's frozen baseline across the worst case of the §5a band (seed 20261118; 66 total TPE trials).</text>",
    ]

    for pi, (title, rungs, vals, sub, col, (ylo, yhi)) in enumerate(panels):
        x0 = ml + pi * (pw + 18)
        lines.append(
            f"<rect x='{x0:.1f}' y='{mt}' width='{pw:.1f}' height='{ph}' fill='#fafafa' stroke='#e5e7eb'/>"
        )
        lines.append(
            f"<text x='{x0 + pw/2:.1f}' y='{mt - 8}' text-anchor='middle' font-size='11' font-weight='bold' fill='#222'>{esc(title)}</text>"
        )
        for tick in [-6, -4, -2, 0, 2, 4, 6]:
            yy = mt + ph * (1.0 - (tick - ylo) / (yhi - ylo))
            dash = "4 3" if tick == 0 else "2 2"
            scol = "#374151" if tick == 0 else "#e5e7eb"
            lines.append(
                f"<line x1='{x0:.1f}' x2='{x0 + pw:.1f}' y1='{yy:.1f}' y2='{yy:.1f}' stroke='{scol}' stroke-dasharray='{dash}'/>"
            )
            if pi == 0:
                lines.append(
                    f"<text x='{x0 - 6:.1f}' y='{yy + 3.5:.1f}' text-anchor='end' font-size='9.5' fill='#555'>{tick:+d}</text>"
                )

        pts = []
        for idx, (rg, v) in enumerate(zip(rungs, vals)):
            xx = x0 + 28 + (pw - 56) * (idx / max(1, len(rungs) - 1))
            vv = max(ylo, min(yhi, v))
            yy = mt + ph * (1.0 - (vv - ylo) / (yhi - ylo))
            pts.append((xx, yy, rg, v))
        poly = " ".join(f"{x:.1f},{y:.1f}" for x, y, _, _ in pts)
        lines.append(f"<polyline fill='none' stroke='{col}' stroke-width='2.4' points='{poly}'/>")
        for xx, yy, rg, v in pts:
            lines.append(f"<circle cx='{xx:.1f}' cy='{yy:.1f}' r='4' fill='{col}'/>")
            lines.append(
                f"<text x='{xx:.1f}' y='{yy - 8:.1f}' text-anchor='middle' font-size='9' fill='#111'>{v:+.2f}</text>"
            )
            lines.append(
                f"<text x='{xx:.1f}' y='{mt + ph + 15:.1f}' text-anchor='middle' font-size='9.5' fill='#333'>{rg}b</text>"
            )
        lines.append(
            f"<text x='{x0 + pw/2:.1f}' y='{mt + ph + 34:.1f}' text-anchor='middle' font-size='9.5' fill='{col}' font-weight='bold'>{esc(sub)}</text>"
        )

    lines.append(
        f"<text transform='translate(18,{mt + ph/2:.1f}) rotate(-90)' text-anchor='middle' font-size='11' fill='#333'>Worst-band log2(S_trial / S_baseline)</text>"
    )
    lines.append(
        f"<text x='{ml}' y='{h - 10}' font-size='9.5' fill='#555'>Note: In prime-algebraic, rungs 14 and 16 exceeded the 240 s safety cap on the naive degree-12 F4 baseline and were dropped automatically by ic sweep.</text>"
    )
    lines.append("</svg>\n")
    return "\n".join(lines)


def compile_round_pdfs() -> None:
    for r in range(1, 7):
        rdir = HOME / f"round-{r:03d}"
        render_png(rdir / "round.svg", rdir / "round.png")
        res = json.loads((rdir / "RESULT.json").read_text())
        man = json.loads((rdir / "MANIFEST.json").read_text())
        track = res["track"]
        claims = json.loads((HOME / "claims.json").read_text())
        cid = Path(claims[f"round_{r}"]).stem

        rows_typ = []
        for k in sorted(res["sizes"].keys(), key=int):
            row = res["sizes"][k]
            cls = row["class"]
            if cls == "no_challenger":
                inc_s = " / ".join(f"{x:.3f}" for x in row["incumbent_s"])
                rows_typ.append(
                    f"[{k}], [{typ_esc(cls)}], [-], [-], [-], [{inc_s}], [-],"
                )
            else:
                sel = row["sel"]["name"]
                w0 = row["ratio"]["w0"]
                r_str = f"{w0['ratio']:.3f} [{w0['ci95'][0]:.3f}, {w0['ci95'][1]:.3f}]"
                bv = row.get("band_verdict") or "-"
                ssel = " / ".join(f"{x:.3f}" for x in row["s"]["sel"])
                sinc = " / ".join(f"{x:.3f}" for x in row["s"]["inc"])
                d = row["distance_to_rho"]["sel"]
                d_str = " / ".join(f"{x:.1f}x" for x in d) if d else "-"
                rows_typ.append(
                    f"[{k}], [*{typ_esc(cls)}*], [`{typ_esc(sel)}`], [{typ_esc(r_str)}], [{typ_esc(bv)}], [{ssel} (was {sinc})], [{d_str}],"
                )

        typ = f"""#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 9.5pt)
#show heading: set block(above: 10pt, below: 5pt)

= ic_autolab Round {r} ({typ_esc(track)} track) — Generic Prime-Order E(F_p)

*Claim ID:* `{cid}` (`[pcc:unverified]`) \\
*Frozen:* `{man.get("frozen", "")}` at git `{man.get("git_head", "")}` · *Route:* `{man.get("route", "local")}` (counted GAE across §5a band `w in {{0, 0.1, 1}}`, L0, no wall-time claim).

== Holdout Decision Summary

#table(
  columns: (auto, auto, auto, auto, auto, 1fr, auto),
  inset: 4pt,
  align: (right, left, left, left, left, left, left),
  [*k*], [*Class*], [*Selected*], [*sel/inc w=0 (95% CI)*], [*§5a Band*], [*Mean S (w=0 / 0.1 / 1)*], [*sel / rho*],
  {chr(10).join("  " + x for x in rows_typ)}
)

== Recorded Train Screen (`round.svg`)
#image("round.svg", width: 100%)

== Cumulative Track Ratchet (`ratchet-{track}.svg`)
#image("../ratchet-{track}.svg", width: 88%)
"""
        typ_path = rdir / "REPORT.typ"
        typ_path.write_text(typ)
        subprocess.run(
            ["typst", "compile", "--root", str(HOME), str(typ_path), str(rdir / "REPORT.pdf")],
            check=True,
        )


def main() -> None:
    FIG_DIR.mkdir(parents=True, exist_ok=True)
    RUN_REPORT_DIR.mkdir(parents=True, exist_ok=True)

    ledger = json.loads((HOME / "ledger.json").read_text())
    alg_band = json.loads((HOME / "baseline/analysis/band-alg-k20.json").read_text())
    t_show = json.loads((HOME / "sweeps/prime-table-generic.show.json").read_text())
    rf_show = json.loads((HOME / "sweeps/prime-rootfind-generic.show.json").read_text())
    alg_show = json.loads((HOME / "sweeps/prime-algebraic-generic.show.json").read_text())

    band_svg = build_band_comparison_svg(ledger, alg_band)
    (FIG_DIR / "band_comparison.svg").write_text(band_svg)
    render_png(FIG_DIR / "band_comparison.svg", FIG_DIR / "band_comparison.png")

    sweep_svg = build_sweeper_svg(t_show, rf_show, alg_show)
    (FIG_DIR / "sweeper_rungs.svg").write_text(sweep_svg)
    render_png(FIG_DIR / "sweeper_rungs.svg", FIG_DIR / "sweeper_rungs.png")

    render_png(HOME / "ratchet-multibase.svg", FIG_DIR / "ratchet-multibase.png")
    render_png(HOME / "ratchet-generic.svg", FIG_DIR / "ratchet-generic.png")

    shutil.copy2(FIG_DIR / "band_comparison.svg", RUN_REPORT_DIR / "band_comparison.svg")
    shutil.copy2(FIG_DIR / "band_comparison.png", RUN_REPORT_DIR / "band_comparison.png")
    shutil.copy2(FIG_DIR / "sweeper_rungs.svg", RUN_REPORT_DIR / "sweeper_rungs.svg")
    shutil.copy2(FIG_DIR / "sweeper_rungs.png", RUN_REPORT_DIR / "sweeper_rungs.png")

    compile_round_pdfs()
    print("rendered all figures and round PDFs")


if __name__ == "__main__":
    main()
