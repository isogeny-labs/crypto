#!/usr/bin/env python3
"""Validate and render the exhaustive small-field weighted-law gate."""
from __future__ import annotations

import csv
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "src/bin/projective_law_gate.rs"
TESTS = HERE / "gate_tests.rs"
RUNNER = HERE / "run.py"
ORACLE = ROOT.parent.parent / "src/cryptanalysis/three_leads.rs"
LOG = HERE / "cargo-test.log"
PREFIX = "EXHAUSTIVE_GATE_JSON "
SCHEMA = "projective-law-exhaustive-small-field/v1"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_rows() -> list[dict]:
    rows = [json.loads(line[len(PREFIX):]) for line in LOG.read_text().splitlines()
            if line.startswith(PREFIX)]
    assert [r["p"] for r in rows] == [7, 11]
    assert "test result: ok. 1 passed; 0 failed" in LOG.read_text()
    for row in rows:
        p = row["p"]
        assert row["curve"] == [2, 1, 0, 0, 0, 1]
        # A common root of f=x^5+x+2 and f'=5x^4+1 must be x=-5/2;
        # substituting gives f'(-5/2)=3141/16, nonzero in these fields.
        assert p not in (2, 5) and 3141 % p != 0
        assert row["ordered_pairs"] == row["divisors"] ** 2
        assert row["checked"] + row["unresolved"] == row["ordered_pairs"] * row["weight_variants"]
        assert sum(row["labels"].values()) == row["checked"] + row["unresolved"]
        assert row["unresolved"] == 0
        assert row["weight_variants"] == 2
    return rows


def payload(rows: list[dict]) -> dict:
    return {
        "schema": SCHEMA,
        "scope": "every canonical reduced Mumford pair over two stated smooth prime-field quintics, two weight assignments; finite test only",
        "source_sha256": sha(SOURCE),
        "test_module_sha256": sha(TESTS),
        "runner_sha256": sha(RUNNER),
        "cantor_oracle_source_sha256": sha(ORACLE),
        "test_log_sha256": sha(LOG),
        "command": "python3 research/projective_law_followup_20261022/exhaustive_small_field/run.py",
        "model": "y^2=x^5+x+2",
        "smoothness": "f'(-5/2)=3141/16 != 0 modulo 7 and 11",
        "rows": rows,
        "universal_rust_semantics": "not established",
        "pic0_class_equation": "not established",
        "collector_cost_change": None,
    }


def render(rows: list[dict]) -> None:
    data = payload(rows)
    (HERE / "RESULT.json").write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
    with (HERE / "branch-counts.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, lineterminator="\n", fieldnames=["schema", "p", "curve", "divisors",
            "ordered_pairs", "weight_variants", "checked", "unresolved", "branch", "count"])
        writer.writeheader()
        for row in rows:
            for name, count in sorted(row["labels"].items()):
                writer.writerow({"schema": SCHEMA, "p": row["p"], "curve": "x^5+x+2",
                    "divisors": row["divisors"], "ordered_pairs": row["ordered_pairs"],
                    "weight_variants": row["weight_variants"], "checked": row["checked"],
                    "unresolved": row["unresolved"], "branch": name, "count": count})
    names = sorted({k for row in rows for k in row["labels"]})
    svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="690" viewBox="0 0 1100 690">',
           '<rect width="1100" height="690" fill="white"/>',
           '<text x="30" y="35" font-family="sans-serif" font-size="20">Complete weighted-law finite branch census</text>',
           '<text x="30" y="58" font-family="sans-serif" font-size="13">All reduced divisor pairs on y²=x⁵+x+2; 2 weight choices per pair. Bar length is count / total checks.</text>']
    for i, name in enumerate(names):
        y = 88 + 36 * i
        svg.append(f'<text x="30" y="{y+12}" font-family="monospace" font-size="13">{name}</text>')
        for j, row in enumerate(rows):
            x = 350 + 340*j
            width = round(280 * row["labels"].get(name, 0) / row["checked"])
            color = "#2879b8" if j == 0 else "#d47e29"
            svg.append(f'<rect x="{x}" y="{y}" width="{width}" height="16" fill="{color}"/>')
            svg.append(f'<text x="{x+width+5}" y="{y+13}" font-family="monospace" font-size="12">{row["labels"].get(name, 0)}</text>')
    svg.extend(['<text x="350" y="670" font-family="sans-serif" font-size="13" fill="#2879b8">F7: 8,712 checks</text>',
                '<text x="690" y="670" font-family="sans-serif" font-size="13" fill="#d47e29">F11: 25,992 checks</text>', '</svg>'])
    (HERE / "branch-counts.svg").write_text("\n".join(svg) + "\n")
    body = """\\documentclass[11pt]{article}
\\usepackage[a4paper,margin=22mm]{geometry}
\\usepackage{longtable}
\\usepackage{amssymb}
\\begin{document}
\\section*{Exhaustive finite arithmetic gate for the weighted genus-2 law}
The test enumerates every monic reduced Mumford pair satisfying
\\(u\\mid(f-v^2)\\) over each stated field for
\\(f=x^5+x+2\\). The curve is smooth: a common root of
\\(f\\) and \\(f'=5x^4+1\\) would be \\(x=-5/2\\),
but \\(f'(-5/2)=3141/16\\ne0\\) in either field.
It compares every ordered pair with the repository's separate Cantor
implementation under unit weights and non-unit weights \\( (2,3),(3,5)\\).
This is a finite arithmetic test. It does not establish the universal
Rust-to-formula theorem, smooth projective or Picard-class transport,
nor a revised collector cost.
\\begin{longtable}{rrrrr}
Field & Divisors & Ordered pairs & Weighted checks & Unresolved\\\\\\hline
"""
    for row in rows:
        body += f"$\\mathbb{{F}}_{{{row['p']}}}$ & {row['divisors']} & {row['ordered_pairs']} & {row['checked']} & {row['unresolved']}\\\\\n"
    body += "\\end{longtable}\n\\paragraph{Branch counts.} The SVG and standardized CSV record every returned branch label.\n"
    tex_schema = SCHEMA.replace('_', '\\_')
    body += f"\\paragraph{{Reproduction.}} Schema \\texttt{{{tex_schema}}}; run the command recorded in \\texttt{{RESULT.json}}, then \\texttt{{python3 render.py --check}}. The JSON binds the exact Rust law, independent Cantor source, and raw test log by SHA-256.\n"
    body += "\\end{document}\n"
    (HERE / "REPORT.tex").write_text(body)
    subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error",
                    "-output-directory", str(HERE), str(HERE / "REPORT.tex")],
                   check=True, stdout=subprocess.DEVNULL)
    optimized = HERE / "REPORT-optimized.pdf"
    subprocess.run(["gs", "-q", "-dNOPAUSE", "-dBATCH", "-sDEVICE=pdfwrite",
                    "-dPDFSETTINGS=/screen", f"-sOutputFile={optimized}",
                    str(HERE / "REPORT.pdf")], check=True)
    optimized.replace(HERE / "REPORT.pdf")


def check(rows: list[dict]) -> None:
    expected = payload(rows)
    assert json.loads((HERE / "RESULT.json").read_text()) == expected
    with (HERE / "branch-counts.csv").open(newline="") as stream:
        actual = list(csv.DictReader(stream))
    assert len(actual) == sum(len(r["labels"]) for r in rows)
    assert sum(int(r["count"]) for r in actual) == sum(r["checked"] for r in rows)
    for row in actual:
        source = next(r for r in rows if r["p"] == int(row["p"]))
        assert int(row["count"]) == source["labels"][row["branch"]]
    assert (HERE / "branch-counts.svg").read_text().startswith("<svg ")
    assert (HERE / "REPORT.pdf").read_bytes().startswith(b"%PDF")


if __name__ == "__main__":
    records = read_rows()
    if sys.argv[1:] == ["--check"]:
        check(records)
    elif len(sys.argv) == 1:
        render(records)
        check(records)
    else:
        raise SystemExit("usage: render.py [--check]")
    print(f"{SCHEMA}: {sum(r['checked'] for r in records)} checks, 0 unresolved")
