#!/usr/bin/env python3
"""fig_split.svg from results.json (analyze.py output): stacked per-call time, arm A vs arm B, at 24 and 26 unknowns."""
import json, os
D = os.path.dirname(os.path.abspath(__file__)) + "/"
d = json.load(open(D + "results.json"))
rows = []
for inst, label in (("koblitz-n23", "24 unknowns (Koblitz n=23)"), ("char2-n25", "26 unknowns (char-2 n=25)")):
    A, B, C = d[inst]["A"], d[inst]["B"], d[inst]["C"]
    wa, wb = A["solver_wall_ms_per_call"]["mean"], B["solver_wall_ms_per_call"]["mean"]
    prepA = A["forms_ms"]["mean"] + A["pack_ms"]["mean"]; rtA = A["round_trip_ms"]["mean"]; verA = A["verify_ms"]["mean"]
    pack = B["pack_ms"]["mean"]; kern = B["kernel_gpu_us"]["mean"] / 1e3; busy = B["busy_us"]["mean"] / 1e3
    floor = C["round_trip_ms"]["mean"]; rt = B["round_trip_ms"]["mean"]; ver = B["verify_ms"]["mean"]
    rows.append((label, wa, wb,
                 [("Rust prep", prepA), ("round trip (A)", rtA), ("verify", verA), ("other", wa - prepA - rtA - verA)],
                 [("Rust prep", pack), ("kernel (GPU)", kern), ("copies + reply", busy - kern), ("pipe floor", floor),
                  ("unlocated", rt - busy - floor), ("verify", ver), ("other", wb - pack - rt - ver)]))
cols = {"Rust prep": "c1", "round trip (A)": "c2", "kernel (GPU)": "c3", "copies + reply": "c4", "pipe floor": "c5", "unlocated": "c6", "verify": "c7", "other": "c8"}
W, H, x0 = 860, 330, 250
scale = 520 / 0.65
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">fes-f2-gpu per-call time on an A100: before and now, and where the current call goes</title>',
     '<desc id="d">Stacked horizontal bars of the mean whole-call solver time in milliseconds at 24 and 26 unknowns for the pre-direct-pack text-protocol build (A) and current main (B); the current call is split into Rust prep, GPU kernel, worker copies and reply, the pipe floor measured with a null worker, an unlocated remainder, verification and other.</desc>',
     '<style>.bg{fill:#fcfcfb}.c1{fill:#eb6834}.c2{fill:#c9a227}.c3{fill:#2a78d6}.c4{fill:#7fb3f0}.c5{fill:#1baf7a}.c6{fill:#8b6fd6}.c7{fill:#9d9c97}.c8{fill:#d6d5d0}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.c1{fill:#d95926}.c2{fill:#b8921a}.c3{fill:#3987e5}.c4{fill:#5a96d8}.c5{fill:#199e70}.c6{fill:#a58cf0}.c7{fill:#797873}.c8{fill:#52514e}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">fes-f2-gpu on an A100 80GB PCIe: where a call spends its time</text>',
     '<text class="q" x="16" y="42" font-size="12">mean of 3 interleaved runs per arm, ms per call; A = ic before the direct pack (text protocol), B = current main (binary protocol)</text>']
for t in range(7):
    x = x0 + t * 0.1 * scale
    o.append(f'<line class="g" x1="{x:.1f}" y1="56" x2="{x:.1f}" y2="236"/><text class="q" x="{x:.1f}" y="252" font-size="11" text-anchor="middle">{t * 0.1:.1f}</text>')
o.append(f'<text class="q" x="{x0 + 0.3 * scale:.1f}" y="270" font-size="11" text-anchor="middle">ms per call</text>')
y = 62
for label, wa, wb, Aseg, Bseg in rows:
    o.append(f'<text class="p" x="16" y="{y + 12}" font-size="12" font-weight="600">{label}</text>')
    for tag, total, segs in (("A", wa, Aseg), ("B", wb, Bseg)):
        o.append(f'<text class="q" x="{x0 - 8}" y="{y + 16}" font-size="12" text-anchor="end">{tag}  {total:.3f}</text>')
        x = x0
        for name, v in segs:
            w = v * scale
            o.append(f'<rect class="{cols[name]}" x="{x:.1f}" y="{y}" width="{w:.1f}" height="22"><title>{name}: {v:.4f} ms</title></rect>')
            if w > 34:
                o.append(f'<text x="{x + w / 2:.1f}" y="{y + 15}" font-size="10" text-anchor="middle" style="fill:#fff">{v:.3f}</text>')
            x += w
        y += 28
    y += 26
xx = 16
for n, c in (("Rust prep", "c1"), ("round trip (A)", "c2"), ("kernel (GPU)", "c3"), ("copies + reply", "c4"), ("pipe floor", "c5"), ("unlocated", "c6"), ("verify", "c7"), ("other", "c8")):
    o.append(f'<rect class="{c}" x="{xx}" y="{H - 40}" width="10" height="10"/><text class="q" x="{xx + 14}" y="{H - 31}" font-size="10">{n}</text>')
    xx += 14 + len(n) * 6 + 18
o.append(f'<text class="q" x="16" y="{H - 10}" font-size="10">pipe floor = ic main against a null worker; unlocated = round trip minus worker busy minus pipe floor (derived)</text>')
o.append("</svg>")
open(D + "fig_split.svg", "w").write("\n".join(o))
