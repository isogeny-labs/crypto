"""Layout only: draws recorded values from results/all.jsonl into figure.svg. Computes nothing."""
import json
rows = [json.loads(l) for l in open("results/all.jsonl")]
t4 = [r for r in rows if r["test"] == "T4"]
t5 = [r for r in rows if r["test"] == "T5" and r["m"] == 3 and r["planted"]]
W, H = 980, 420
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12">',
     f'<rect width="{W}" height="{H}" fill="white"/>']
# panel A: trace/standard yield per curve, measured vs (lift ratio)^4
x0, y0, pw, ph = 60, 50, 400, 300
s.append(f'<text x="{x0}" y="30" font-weight="bold">A. T4: four-summand relation yield, trace line / standard line</text>')
pairs = []
for i in range(0, len(t4), 2):
    st, tr = t4[i], t4[i + 1]
    pairs.append((st["d"], tr["trace_over_standard"], (tr["lifting_x"] / st["lifting_x"]) ** 4))
ymax = 45
def Y(v): return y0 + ph - v / ymax * ph
s.append(f'<line x1="{x0}" y1="{y0+ph}" x2="{x0+pw}" y2="{y0+ph}" stroke="#333"/><line x1="{x0}" y1="{y0}" x2="{x0}" y2="{y0+ph}" stroke="#333"/>')
for v in range(0, ymax + 1, 5):
    s.append(f'<text x="{x0-8}" y="{Y(v)+4}" text-anchor="end">{v}</text><line x1="{x0}" y1="{Y(v)}" x2="{x0+pw}" y2="{Y(v)}" stroke="#eee"/>')
s.append(f'<line x1="{x0}" y1="{Y(16)}" x2="{x0+pw}" y2="{Y(16)}" stroke="#c33" stroke-dasharray="4 3"/><text x="{x0+4}" y="{Y(16)+14}" fill="#c33">16x (model)</text>')
bw = pw / len(pairs)
for j, (d, meas, pred) in enumerate(pairs):
    cx = x0 + bw * (j + 0.5)
    s.append(f'<rect x="{cx-bw*0.3}" y="{Y(meas)}" width="{bw*0.6}" height="{Y(0)-Y(meas)}" fill="#3a6ea5"/>')
    s.append(f'<circle cx="{cx}" cy="{Y(pred)}" r="4" fill="none" stroke="#e08a00" stroke-width="2"/>')
    s.append(f'<text x="{cx}" y="{y0+ph+15}" text-anchor="middle" font-size="10">d={d}</text>')
s.append(f'<text x="{x0}" y="{y0+ph+38}" font-size="11">bars: measured exact ratio; circles: (lifting x, trace / standard)^4; 12 curves, N = 4d</text>')
# panel B: linear rank by degree, split S3 system (m = 3), with and without L
x1 = 540
s.append(f'<text x="{x1}" y="30" font-weight="bold">B. T5: linear rank of Macaulay span, m = 3 split S3</text>')
ymaxb = 14
def YB(v): return y0 + ph - v / ymaxb * ph
def XB(D): return x1 + (D - 1) / 4 * 380
s.append(f'<line x1="{x1}" y1="{y0+ph}" x2="{x1+380}" y2="{y0+ph}" stroke="#333"/><line x1="{x1}" y1="{y0}" x2="{x1}" y2="{y0+ph}" stroke="#333"/>')
for v in range(0, ymaxb + 1, 2):
    s.append(f'<text x="{x1-8}" y="{YB(v)+4}" text-anchor="end">{v}</text>')
for D in range(1, 6):
    s.append(f'<text x="{XB(D)}" y="{y0+ph+15}" text-anchor="middle">D={D}</text>')
cols = ["#3a6ea5", "#2a9d8f"]
for k, r in enumerate(t5):
    bd = r["by_degree"]
    for key, dash in (("lin_rank", ""), ("lin_rank_with_L", "5 3")):
        pts = " ".join(f'{XB(b["D"])},{YB(b[key])}' for b in bd)
        s.append(f'<polyline points="{pts}" fill="none" stroke="{cols[k]}" stroke-width="2" stroke-dasharray="{dash}"/>')
    for b in bd:
        if b["L_in_span"]:
            s.append(f'<text x="{XB(b["D"])}" y="{YB(b["lin_rank"])-8}" text-anchor="middle" fill="{cols[k]}">L enters</text>')
            break
    s.append(f'<text x="{x1+10}" y="{y0+14+16*k}" fill="{cols[k]}">{r["case"]} ({r["vars"]} vars)</text>')
s.append(f'<text x="{x1}" y="{y0+ph+38}" font-size="11">solid: system; dashed: system + trace form L (always +1 until L enters, never more)</text>')
s.append("</svg>")
open("figure.svg", "w").write("\n".join(s))
