#!/usr/bin/env python3
"""Layout only: render REPORT.md text and the recorded ratios (evidence/summary.json) into REPORT.pdf.
Computes nothing; every number is read from the files. Dependency-free PDF writer (Helvetica/Courier)."""
import json, re, textwrap, zlib, os
HERE = os.path.dirname(os.path.abspath(__file__)) + "/.."
SUB = {"−": "-", "²": "^2", "³": "^3", "⁸": "^8", "⁴": "^4", "⁷": "^7", "ⁿ": "^n", "≥": ">=", "≤": "<=", "≈": "~", "×": "x", "·": "*",
       "ℤ": "Z", "ℚ": "Q", "π": "pi", "√": "sqrt", "μ": "mu", "₆": "6", "₃": "3", "₄": "4", "ζ": "zeta", "β": "beta",
       "λ": "lambda", "ψ": "psi", "Δ": "Delta", "ℓ": "l", "∤": "!|", "∉": "not in", "∈": "in", "⊋": "strictly contains",
       "→": "->", "–": "-", "—": "-", "…": "...", "′": "'", "ⁱ": "^i", "½": "1/2", "≡": "=", "é": "e", "§": "S",
       "**": "", "`": ""}
def clean(s):
    for k, v in SUB.items(): s = s.replace(k, v)
    s = s.encode("latin-1", "replace").decode("latin-1")
    return s.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")

W, H, M = 595, 842, 48
pages, cur, y = [], [], H - M
def newpage():
    global cur, y
    if cur: pages.append(cur)
    cur, y = [], H - M
def line(txt, size=9, font="F1", dy=None):
    global y
    dy = dy or size * 1.35
    if y - dy < M: newpage()
    y -= dy
    cur.append(f"BT /{font} {size} Tf {M} {y:.1f} Td ({clean(txt)}) Tj ET")

md0 = open(HERE + "/REPORT.md", encoding="utf-8").read().splitlines()
# join soft-wrapped paragraph lines (continuations start with spaces or plain text, not '-', '|', '#', '```')
md, incode0 = [], False
for raw in md0:
    if raw.startswith("```"): incode0 = not incode0; md.append(raw); continue
    cont = (not incode0 and md and raw.strip() and md[-1].strip() and not md[-1].startswith(("#", "|", "```"))
            and not raw.lstrip().startswith(("-", "|", "#")) )
    if cont: md[-1] = md[-1].rstrip() + " " + raw.strip()
    else: md.append(raw)
incode = False
for raw in md:
    if raw.startswith("```"): incode = not incode; continue
    if incode: line(raw, 7.5, "F2"); continue
    if raw.startswith("# "):
        for t in textwrap.wrap(raw[2:], 62): line(t, 14, "F3", 20)
        continue
    if raw.startswith("## ") or raw.startswith("### "): line(raw.lstrip("# "), 11, "F3", 18); continue
    if raw.startswith("|"):
        if set(raw) <= set("|-: "): continue
        cells = [c.strip() for c in raw.strip("|").split("|")]
        for chunk in textwrap.wrap(" | ".join(cells), 124): line(chunk, 6.5, "F2", 8.5)
        continue
    if not raw.strip(): y -= 4; continue
    ind = len(raw) - len(raw.lstrip())
    for k, chunk in enumerate(textwrap.wrap(raw.strip(), 108 - ind)): line(" " * (ind + (2 if k and raw.lstrip().startswith("-") else 0)) + chunk, 9)
newpage()
# figure page from recorded summary
s = json.load(open(HERE + "/evidence/summary.json"))
cur.append(f"BT /F3 12 Tf {M} {H-M-10} Td (Paired ratios, 95% bootstrap CI \\(2000 pairs each\\)) Tj ET")
cur.append(f"BT /F1 9 Tf {M} {H-M-26} Td (blue = group operations; orange = walk field multiplications; red tick = heuristic prediction) Tj ET")
left, right, top = 230, W - 40, H - M - 50
X = lambda v: left + v / 1.3 * (right - left)
for t in [0, 0.25, 0.5, 0.75, 1.0, 1.25]:
    g = 0.55 if t == 1.0 else 0.88
    cur.append(f"{g} G 0.5 w {X(t):.1f} {top+10} m {X(t):.1f} {top-30*len(s['comparisons'])} l S")
    cur.append(f"BT /F1 8 Tf {X(t)-8:.1f} {top-30*len(s['comparisons'])-12} Td ({t:.2f}) Tj ET")
for i, c in enumerate(s["comparisons"]):
    yy = top - 30 * i
    cur.append(f"0 g BT /F1 8 Tf {M} {yy-3} Td ({clean(c['comparison'])}) Tj ET")
    p, lo, hi = c["steps"]; cur.append(f"0.12 0.37 0.66 RG 0.12 0.37 0.66 rg 2 w {X(lo):.1f} {yy} m {X(hi):.1f} {yy} l S {X(p)-2.5:.1f} {yy-2.5} 5 5 re f")
    p, lo, hi = c["walk_mul"]; cur.append(f"0.82 0.48 0 RG 0.82 0.48 0 rg 2 w {X(lo):.1f} {yy-8} m {X(hi):.1f} {yy-8} l S {X(p)-2.5:.1f} {yy-10.5} 5 5 re f")
    pr = c["prediction"]; cur.append(f"0.7 0 0.3 RG 1.5 w {X(pr):.1f} {yy+6} m {X(pr):.1f} {yy-14} l S")
cur.append(f"0 g BT /F1 9 Tf {M} {top-30*len(s['comparisons'])-40} Td (ratio of means, arm A / arm B; < 1 = arm A cheaper. Source: evidence/summary.json; SVG: figures/ratios.svg) Tj ET")
newpage()
objs = ["<< /Type /Catalog /Pages 2 0 R >>", None,
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>",
        "<< /Type /Font /Subtype /Type1 /BaseFont /Courier /Encoding /WinAnsiEncoding >>",
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>"]
kids = []
for pg in pages:
    data = zlib.compress("\n".join(pg).encode("latin-1"))
    objs.append(f"<< /Length {len(data)} /Filter /FlateDecode >>stream\n".encode("latin-1") + data + b"\nendstream")
    objs.append(f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {W} {H}] /Resources << /Font << /F1 3 0 R /F2 4 0 R /F3 5 0 R >> >> /Contents {len(objs)} 0 R >>")
    kids.append(len(objs))
objs[1] = f"<< /Type /Pages /Kids [{' '.join(f'{k} 0 R' for k in kids)}] /Count {len(kids)} >>"
out = bytearray(b"%PDF-1.4\n"); offs = []
for i, o in enumerate(objs, 1):
    offs.append(len(out)); b = o if isinstance(o, bytes) else o.encode("latin-1")
    out += f"{i} 0 obj\n".encode() + b + b"\nendobj\n"
x = len(out)
out += f"xref\n0 {len(objs)+1}\n0000000000 65535 f \n".encode() + b"".join(f"{o:010d} 00000 n \n".encode() for o in offs)
out += f"trailer << /Size {len(objs)+1} /Root 1 0 R >>\nstartxref\n{x}\n%%EOF\n".encode()
open(HERE + "/REPORT.pdf", "wb").write(out)
print("pages", len(kids), "bytes", len(out))
