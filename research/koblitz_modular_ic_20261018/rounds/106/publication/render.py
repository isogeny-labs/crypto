#!/usr/bin/env python3
"""Quantitative R106 valuation chart as SVG and one-page PDF."""
from pathlib import Path

root = Path(__file__).resolve().parent
bars = [("y=0 z", 2, "#2c679f"), ("y=0 t", 3, "#2c679f"),
        ("y=1 z", 3, "#b6662a"), ("y=1 t", 4, "#b6662a")]
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="300" viewBox="0 0 760 300">',
       '<rect width="760" height="300" fill="white"/>',
       '<text x="35" y="32" font-size="20" font-family="sans-serif">R106: local base uniformizer valuations</text>',
       '<text x="35" y="278" font-size="14" font-family="sans-serif">Order-32 exact series; R105 source and R106 fresh replay</text>']
for i, (name, val, color) in enumerate(bars):
    x = 65 + i*165
    svg += [f'<rect x="{x}" y="{230-40*val}" width="90" height="{40*val}" fill="{color}"/>',
            f'<text x="{x+30}" y="{222-40*val}" font-size="17" font-family="sans-serif">{val}</text>',
            f'<text x="{x+6}" y="251" font-size="15" font-family="sans-serif">{name}</text>']
svg.append('<path d="M45 230 L710 230" stroke="black"/></svg>')
(root / "diagram.svg").write_text("\n".join(svg)+"\n")


def label(x,y,size,value):
    value=value.replace("\\","\\\\").replace("(","\\(").replace(")","\\)")
    return f"BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n"


c="0 0 0 rg\n"+label(42,748,18,"R106: base local uniformizers at x=1")
c+=label(42,723,10,"5 November 2026 | marked-23 Koblitz source-order experiment")
for i,line in enumerate([
    "Set t=x+1 and z=y or y+1. Then s=t/z gives a simple-root",
    "transformed equation in both base completions. Hensel lifting",
    "constructs z(s); the source equation vanishes through s^32.",
]): c+=label(42,683-18*i,11,line)
c+=label(42,600,12,"Verified valuations in the two base completions")
for i,(name,val,_) in enumerate(bars):
    x=55+i*137
    c+=("0.18 0.40 0.62 rg\n" if i<2 else "0.71 0.40 0.16 rg\n")
    c+=f"{x} 345 72 {35*val} re f\n"
    c+="0 0 0 rg\n"+label(x+29,360+35*val,12,str(val))+label(x+6,324,10,name)
c+="0 0 0 RG 45 345 m 595 345 l S\n"
for i,line in enumerate([
    "Fresh replay matched; changed R105 hash rejected.",
    "Independent bit evaluator and Lean checked both residuals.",
    "Next: substitute the base series into full G51 and N51.",
    "Raw archive SHA-256:",
    "315e624c9045f92392cbccf6a2b24d880761b7df8693da99fb38c8e6aab1eb50",
]): c+=label(42,280-18*i,10,line)
stream=c.encode("ascii")
objects=[b"<< /Type /Catalog /Pages 2 0 R >>",
 b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
 b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>",
 b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
 f"<< /Length {len(stream)} >>\nstream\n".encode()+stream+b"endstream"]
pdf=bytearray(b"%PDF-1.4\n");offsets=[0]
for n,obj in enumerate(objects,1):
 offsets.append(len(pdf));pdf+=f"{n} 0 obj\n".encode()+obj+b"\nendobj\n"
xref=len(pdf);pdf+=f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode()
for off in offsets[1:]:pdf+=f"{off:010d} 00000 n \n".encode()
pdf+=f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
(root/"report.pdf").write_bytes(pdf)
