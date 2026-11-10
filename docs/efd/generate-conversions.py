#!/usr/bin/env python3
"""Generate the EFD conversion diagrams and table from the pinned catalogues.

Usage (from the repository root):

    python3 docs/efd/generate-conversions.py
    dot -Tsvg docs/efd/conversions.dot -o docs/efd/conversions.svg

Both catalogues are the source of truth: the model graph comes from the
`toweierstrass`/`fromweierstrass` records, and the chart list from the pinned
representations. No hand-maintained edge list.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SECTIONS = [
    (
        "g1p",
        "GF(p), p odd — EFD g1p",
        ROOT / "src/prime_ecc/efd/catalogue.json",
        "#e8f0ff",
        "#4a6fa5",
    ),
    (
        "g12o",
        "GF(2^m) — EFD g12o",
        ROOT / "src/binary_ecc/efd/catalogue.json",
        "#eaf7ea",
        "#3f7a3f",
    ),
]

EQUATIONS = {
    "shortw": "y^2=x^3+ax+b",
    "montgom": "b*y^2=x^3+a*x^2+x",
    "edwards": "x^2+y^2=c^2(1+d*x^2*y^2)",
    "twisted": "a*x^2+y^2=1+d*x^2*y^2",
    "twistedhessian": "a*x^3+y^3+1=d*x*y",
    "hessian": "x^3+y^3+1=3*d*x*y",
    "jintersect": "s^2+c^2=1, a*s^2+d^2=1",
    "jquartic": "y^2=x^4+2*a*x^2+1",
    "2dik": "y^2=x^3+a*x^2+16*a*x",
    "3dik": "y^2=x^3+3*a*(x+1)^2",
}


def load(path):
    return json.load(open(path))


def charts_by_model(catalogue):
    counts = {}
    for rep in catalogue["representations"]:
        model = rep["id"].split("/")[0]
        counts.setdefault(model, []).append(rep["id"])
    return counts


def projection_kind(rep):
    rid = rep["id"]
    if rid.startswith("jintersect/"):
        return "(s,c,d)"
    if rid == "edwards/yz":
        return "y"
    if rid == "edwards/yzsquared":
        return "y^2"
    if rid in ("shortw/xz", "montgom/xz"):
        return "x"
    return "(x,y)"


def main():
    data = []
    for section, label, path, fill, border in SECTIONS:
        catalogue = load(path)
        charts = charts_by_model(catalogue)
        data.append((section, label, catalogue, charts, fill, border))

    # ---- Graphviz ----
    lines = [
        "digraph efd_conversions {",
        '  graph [rankdir=LR, fontname="Helvetica", fontsize=11, bgcolor="white",',
        '         label="EFD model conversions: every curve shape bridges to its short Weierstrass model",',
        "         labelloc=t, pad=0.3];",
        '  node [shape=box, style="rounded,filled", fontname="Helvetica", fontsize=10];',
        '  edge [fontname="Helvetica", fontsize=8, dir=both];',
    ]
    for section, label, catalogue, charts, fill, border in data:
        lines += [
            f"  subgraph cluster_{section} {{",
            f'    label="{label} — {len(catalogue["formulas"])} formulas, '
            f'{len(catalogue["representations"])} charts";',
            f'    style="rounded"; color="{border}"; fontcolor="{border}";',
        ]
        hub = f"m_shortw_{section}"
        for model in catalogue["models"]:
            mid = model["id"]
            node = f"m_{mid}_{section}"
            equation = EQUATIONS.get(mid, "")
            count = len(charts.get(mid, []))
            plural = "chart" if count == 1 else "charts"
            lines.append(
                f'    {node} [label="{mid}\\n{equation}\\n{count} {plural}", fillcolor="{fill}"];'
            )
        for model in catalogue["models"]:
            mid = model["id"]
            if mid == "shortw":
                continue
            identity = model["id"] in ("montgom", "2dik", "3dik")
            style = "dashed" if identity else "solid"
            what = "identity (u=x, v=y)" if identity else "rational map"
            lines.append(
                f'    m_{mid}_{section} -> {hub} [style={style}, label="{what}"];'
            )
        lines.append("  }")
    lines += [
        f'  m_shortw_g1p -> m_shortw_g12o [style=invis, constraint=false];',
        '  note [shape=note, fillcolor="#fff8dc", label="Within a section all shapes are mutually convertible\\n'
        'through the hub. Across sections (GF(p) vs GF(2^m)) no conversion exists.\\n'
        'Chart-to-affine conversion is separate: exact for full charts, lossy for quotient charts."];',
        "}",
    ]
    dot = "\n".join(lines) + "\n"
    (ROOT / "docs/efd/conversions.dot").write_text(dot)

    # ---- Mermaid ----
    mm = ["flowchart LR"]
    for section, label, catalogue, charts, fill, border in data:
        mm.append(f'  subgraph {section}["{label} — {len(catalogue["formulas"])} formulas, {len(catalogue["representations"])} charts"]')
        mm.append("    direction LR")
        for model in catalogue["models"]:
            mid = model["id"]
            count = len(charts.get(mid, []))
            plural = "chart" if count == 1 else "charts"
            mm.append(f'    m_{mid}_{section}["{mid}<br/>{EQUATIONS.get(mid, "")}<br/>{count} {plural}"]')
        for model in catalogue["models"]:
            mid = model["id"]
            if mid == "shortw":
                continue
            identity = mid in ("montgom", "2dik", "3dik")
            arrow = "-.->" if identity else "-->"
            what = "identity u=x, v=y" if identity else "rational map"
            mm.append(f"    m_{mid}_{section} {arrow}|{what}| m_shortw_{section}")
        mm.append("  end")
    mm.append("  m_shortw_g1p -. no conversion (different characteristic) .- m_shortw_g12o")
    (ROOT / "docs/efd/conversions.mmd").write_text("\n".join(mm) + "\n")

    # ---- Markdown ----
    md = []
    md.append("# EFD model and chart conversions")
    md.append("")
    md.append("Generated by `docs/efd/generate-conversions.py` from the two pinned")
    md.append("catalogues. Regenerate after any catalogue change:")
    md.append("")
    md.append("```sh")
    md.append("python3 docs/efd/generate-conversions.py")
    md.append("dot -Tsvg docs/efd/conversions.dot -o docs/efd/conversions.svg")
    md.append("```")
    md.append("")
    md.append("![EFD conversion graph](conversions.svg)")
    md.append("")
    md.append("## What can be converted")
    md.append("")
    md.append("- **Within one section, every curve shape converts to and from short")
    md.append("  Weierstrass.** Each model record carries `toweierstrass` and")
    md.append("  `fromweierstrass` lines; therefore any two shapes in the same section")
    md.append("  are mutually convertible by composing through the hub. The maps are")
    md.append("  rational and can have exceptional points (denominators that vanish at")
    md.append("  the identity, 2-torsion, or chart boundaries).")
    md.append("- **`shortw` is the identity hub.** `montgom`, `2dik`, and `3dik` are")
    md.append("  already Weierstrass equations with the same `x, y` coordinates (only")
    md.append("  the parameter names differ), so their bridge is the identity.")
    md.append("- **`edwards`, `twisted`, `twistedhessian`, `hessian`, `jintersect`,")
    md.append("  and `jquartic` need a non-trivial rational map.**")
    md.append("- **No conversion across characteristic.** `g1p` is for odd prime")
    md.append("  fields; `g12o` is for GF(2^m). Nothing bridges the two hubs.")
    md.append("")
    md.append("## Chart-level conversions")
    md.append("")
    md.append("Every chart converts to its model's affine coordinates with")
    md.append("`Representation::encode_affine` (or `encode_jintersection`) and back with")
    md.append("`Representation::project`. Full charts round-trip exactly at any nonzero")
    md.append("projective scale. Quotient charts are one-way: they keep only the listed")
    md.append("projection, and no affine reconstruction is provided.")
    md.append("")
    md.append("### GF(p) — g1p")
    md.append("")
    md.append("| Chart | Variables | Affine map | `project` returns |")
    md.append("| --- | --- | --- | --- |")
    prime = data[0][2]
    for rep in prime["representations"]:
        rels = [
            r
            for r in rep["relations"]
            if r.split("=")[0].strip() in ("x", "y", "s", "c", "d")
        ]
        if rep["id"] == "edwards/yz":
            rels = ["r*y = Y/Z"]
        if rep["id"] == "edwards/yzsquared":
            rels = ["r*y^2 = Y/Z"]
        md.append(
            f"| `{rep['id']}` | {', '.join(rep['variables'])} | "
            f"{'; '.join(rels)} | {projection_kind(rep)} |"
        )
    md.append("")
    md.append("### GF(2^m) — g12o")
    md.append("")
    md.append("| Chart | Variables | `project` returns |")
    md.append("| --- | --- | --- |")
    binary = data[1][2]
    for rep in binary["representations"]:
        kind = "(x,y)"
        if rep["variables"] == ["w"]:
            kind = "w = x+y"
        elif "W" in rep["variables"]:
            kind = "W"
        elif rep["id"] == "shortw/xz":
            kind = "x"
        md.append(
            f"| `{rep['id']}` | {', '.join(rep['variables'])} | {kind} |"
        )
    md.append("")
    md.append("## Totals")
    md.append("")
    md.append("| Section | Models | Charts | Formulas |")
    md.append("| --- | ---: | ---: | ---: |")
    for section, label, catalogue, charts, fill, border in data:
        md.append(
            f"| `{section}` | {len(catalogue['models'])} | "
            f"{len(catalogue['representations'])} | {len(catalogue['formulas'])} |"
        )
    md.append(
        f"| **EFD total** | **{sum(len(d[2]['models']) for d in data)}** | "
        f"**{sum(len(d[2]['representations']) for d in data)}** | "
        f"**{sum(len(d[2]['formulas']) for d in data)}** |"
    )
    md.append("")
    (ROOT / "docs/efd/conversions.md").write_text("\n".join(md) + "\n")
    print("wrote docs/efd/conversions.{dot,mmd,md}")


if __name__ == "__main__":
    main()
