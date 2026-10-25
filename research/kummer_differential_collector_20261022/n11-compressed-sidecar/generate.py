#!/usr/bin/env python3
"""Derive the N11 label-state comparison from the frozen N7 model list."""
import csv
import hashlib
import json
import math
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
INPUT = HERE.parent / "orbit-proof" / "frozen-inputs.json"
WALK_SOURCE = HERE.parent / "src" / "walk_impl.rs"
RESULTS = HERE / "results"
SCHEMA = "kummer-compressed-sidecar/v1"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prime(n):
    if n < 2:
        return False
    for d in range(2, math.isqrt(n) + 1):
        if n % d == 0:
            return False
    return True


def main():
    frozen = json.loads(INPUT.read_text())
    assert frozen["schema"] == "kummer-fixed-offset-orbit-input/v1"
    assert len(frozen["models"]) == 21
    rows = []
    for model in frozen["models"]:
        r = model["r"]
        assert prime(r) and r > 2 and len(model["jumps"]) == 16
        for jump in model["jumps"]:
            j = jump["jump_scalar"]
            assert 0 < j < r and math.gcd(j, r) == 1
            rows.append({
                "schema": SCHEMA,
                "bits": model["bits"], "p": model["p"], "r": r,
                "jump_index": jump["jump_index"], "jump_scalar": j,
                "fixed_offset_classes": r,
                "fixed_offset_explicit_bytes": 32 * r,
                "pair_classes": 2,
                "pair_bytes": 64,
                "pair_plus_label_bytes_external_step": 72,
                "pair_plus_label_and_step_bytes": 80,
                "minimum_label_bits_for_all_histories": (r - 1).bit_length(),
                "regular_pair_step_xadds": 3,
                "regular_pair_step_field_m": 72,
                "regular_pair_step_field_a": 108,
                "pair_sign_ambiguities": 0,
                "coordinate_xadd_exception_bound": None,
                "cohort_eligible": False,
            })
    assert len(rows) == 336
    RESULTS.mkdir(exist_ok=True)
    fields = list(rows[0])
    with (RESULTS / "bounds.csv").open("w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    (RESULTS / "bounds.json").write_text(json.dumps({
        "schema": SCHEMA, "study_id": "KUMMER-N11-COMPRESSED-SIDECAR-20261025",
        "input_sha256": digest(INPUT),
        "walk_source_sha256": digest(WALK_SOURCE),
        "proof_scope": "odd-prime cyclic group and exact Kummer quotient",
        "coordinate_xadd_exception_bound": None,
        "cohort_eligible": False,
        "rows": rows,
    }, indent=2) + "\n")
    unique = [rows[i] for i in range(0, len(rows), 16)]
    x0, x1, y0, y1 = 88, 720, 44, 400
    lo, hi = min(x["bits"] for x in unique), max(x["bits"] for x in unique)
    pmax = max(x["fixed_offset_explicit_bytes"] for x in unique)
    def xx(v): return x0 + (v - lo) * (x1 - x0) / (hi - lo)
    def yy(v): return y1 - math.log10(v / 72) / math.log10(pmax / 72) * (y1 - y0)
    svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="800" height="460" viewBox="0 0 800 460">',
           '<rect width="800" height="460" fill="white"/>',
           '<text x="55" y="22" font-size="17">Explicit state by subgroup size (log byte axis)</text>',
           f'<path d="M{x0} {y0}V{y1}H{x1}" fill="none" stroke="black"/>']
    for power in [2, 4, 6, 8, 10, 12, 14]:
        y = yy(10 ** power)
        if y0 <= y <= y1:
            svg += [f'<path d="M{x0-5} {y:.1f}H{x1}" stroke="#ddd"/>',
                    f'<text x="20" y="{y+4:.1f}" font-size="12">10^{power}</text>']
    for b in [16, 20, 24, 28, 32, 36, 40]:
        svg.append(f'<text x="{xx(b)-9:.1f}" y="420" font-size="12">{b}</text>')
    svg.append('<text x="362" y="444" font-size="12">subgroup bits</text>')
    svg.append(f'<path d="M{x0} {yy(72):.1f}H{x1}" stroke="#1965a3" stroke-width="2"/>')
    for x in unique:
        svg.append(f'<circle cx="{xx(x["bits"]):.1f}" cy="{yy(x["fixed_offset_explicit_bytes"]):.1f}" r="4" fill="#b34b27"/>')
    svg += ['<text x="525" y="60" fill="#b34b27" font-size="12">N7 fixed-offset explicit state</text>',
            '<text x="525" y="389" fill="#1965a3" font-size="12">pair + label, external step: 72 B</text>',
            '</svg>']
    (HERE / "state-graph.svg").write_text("\n".join(svg) + "\n")
    subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "-output-directory", str(HERE), str(HERE / "REPORT.tex")],
                   cwd=HERE, check=True, stdout=subprocess.DEVNULL)
    for suffix in ("aux", "log"):
        (HERE / f"REPORT.{suffix}").unlink(missing_ok=True)
    names = ["../orbit-proof/frozen-inputs.json", "../src/walk_impl.rs", "generate.py", "validate.py", "REPORT.md", "REPORT.tex", "PROTOCOL.md", "state-graph.svg", "REPORT.pdf", "results/bounds.csv", "results/bounds.json"]
    (HERE / "SHA256SUMS").write_text("".join(f"{digest(HERE / name)}  {name}\n" for name in names))


if __name__ == "__main__":
    main()
