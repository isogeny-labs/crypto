# Layout only: plots values recorded in ../raw/evaluate.txt, eval.kv, null.kv. Computes nothing new
# except reading numbers. Writes graph.svg, graph.png, graph-data.json.
import re, json, os
from PIL import Image, ImageDraw, ImageFont
R = os.path.join(os.path.dirname(__file__), '..', 'raw')
folds = {}
for line in open(f'{R}/evaluate.txt'):
    if line.startswith('FOLD'):
        name = line.split()[1]
        d = {}
        for arm, mae in re.findall(r'\| ?(\w+) mae=([\d.]+)|(?<=\s)(\w+) mae=([\d.]+)', line) and re.findall(r'(?:\||^[^|]*\|)?\s*(a|b|c1|c2|e) mae=([\d.]+)', line):
            d[arm] = float(mae)
        folds[name] = d
kv = {}
for f in ('eval.kv', 'null.kv'):
    for line in open(f'{R}/{f}'):
        if '=' in line:
            k, v = line.strip().split('=', 1); kv[k] = v
arms = ['a', 'b', 'c1', 'e']
label = {'a': 'a Froberg D_reg', 'b': 'b engine median', 'c1': 'c1 boosted trees', 'e': 'e ridge'}
col = {'a': '#c0504d', 'b': '#4f81bd', 'c1': '#2e8b57', 'e': '#8c7ab8'}
cats = list(folds) + ['macro']
macro = {a: float(kv[f'mae_d_{a}']) for a in arms}
vals = {c: (folds[c] if c in folds else macro) for c in cats}
json.dump({'folds': folds, 'macro': macro, 'null': {k: float(kv[k]) for k in ('null_q025','null_q500','null_q975','real3_mae_d_c1')}}, open(os.path.join(os.path.dirname(__file__), 'graph-data.json'), 'w'), indent=1)
W, H = 980, 520
els = []  # (kind, args)
def T(x, y, s, size=12, anchor='start', fill='#222', bold=False, rot=0): els.append(('t', x, y, s, size, anchor, fill, bold, rot))
def Rc(x, y, w, h, fill, stroke=None): els.append(('r', x, y, w, h, fill, stroke))
def L(x1, y1, x2, y2, c='#999', w=1, dash=False): els.append(('l', x1, y1, x2, y2, c, w, dash))
T(W/2, 24, 'Item 12: held-out MAE of the solving degree D, leave-one-family-out (shape-disjoint)', 16, 'middle', bold=True)
T(W/2, 44, 'measured: 7 folds, 345 exact labels, 20 conformal splits; lower is better; source raw/evaluate.txt, eval.kv, null.kv', 11, 'middle', '#555')
# panel 1
x0, y0, pw, ph, ymax = 60, 70, 640, 340, 5.0
for t in range(0, 6):
    y = y0 + ph - ph*t/ymax; L(x0, y, x0+pw, y, '#ddd'); T(x0-6, y+4, str(t), 11, 'end')
T(18, y0+ph/2, 'MAE of D (degrees)', 12, 'middle', rot=90)
gw = pw/len(cats); bw = gw/6
for i, c in enumerate(cats):
    gx = x0 + i*gw + gw*0.1
    for j, a in enumerate(arms):
        v = vals[c][a]; vv = min(v, ymax); h = ph*vv/ymax
        Rc(gx + j*bw, y0+ph-h, bw*0.9, h, col[a])
        T(gx + j*bw + bw*0.45, y0+ph-h-3, f'{v:.2f}', 8, 'middle')
    T(x0 + i*gw + gw/2, y0+ph+16, c, 11, 'middle', bold=(c == 'macro'))
L(x0, y0+ph, x0+pw, y0+ph, '#444')
for j, a in enumerate(arms):
    Rc(x0+10+j*150, y0+ph+34, 12, 12, col[a]); T(x0+27+j*150, y0+ph+45, label[a], 11)
# panel 2: null band
px, py, pw2, ph2 = 760, 70, 190, 340
lo, hi = 0.40, 0.62
def Y(v): return py + ph2 - ph2*(v-lo)/(hi-lo)
T(px+pw2/2, py-6, 'c1 MAE: real vs permutation null', 11, 'middle', bold=True)
for t in (0.40, 0.45, 0.50, 0.55, 0.60):
    L(px, Y(t), px+pw2, Y(t), '#ddd'); T(px-4, Y(t)+4, f'{t:.2f}', 10, 'end')
q025, q500, q975, real = (float(kv[k]) for k in ('null_q025', 'null_q500', 'null_q975', 'real3_mae_d_c1'))
Rc(px+40, Y(q975), 110, Y(q025)-Y(q975), '#d9e4f2', '#4f81bd')
L(px+40, Y(q500), px+150, Y(q500), '#4f81bd', 2)
L(px+20, Y(real), px+170, Y(real), '#2e8b57', 3, True)
T(px+95, Y(q975)-4, f'null 2.5-97.5%: {q025:.3f}-{q975:.3f}', 9, 'middle')
T(px+95, Y(real)-5, f'real {real:.3f} (3 splits)', 10, 'middle', '#2e8b57', True)
T(px+95, Y(q025)+14, 'real inside band: no signal', 10, 'middle', '#555')
T(px+pw2/2, py+ph2+16, '100 within-engine permutations', 10, 'middle', '#555')
T(W/2, H-30, 'Caption: CARD-BLINDSPOT-12-02/03, commit b88ad9129 protocol; verdict MATCHES (primitive). Toy-scale labels, host L0 (shared 4-vCPU). Bars clipped at 5.', 10, 'middle', '#555')
T(W/2, H-14, 'Arm a is the semi-regular reading; arm b the engine-median "levelled" reading; c1 is not separated from b (diff b-c1 95% CI [-0.195, 0.006]).', 10, 'middle', '#555')
# SVG
def esc(s): return s.replace('&', '&amp;').replace('<', '&lt;')
out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif"><rect width="{W}" height="{H}" fill="#fff"/>']
for e in els:
    if e[0] == 't':
        _, x, y, s, size, anc, fill, bold, rot = e
        tr = f' transform="rotate({rot} {x} {y})"' if rot else ''
        out.append(f'<text x="{x:.1f}" y="{y:.1f}" font-size="{size}" text-anchor="{anc}" fill="{fill}"{" font-weight=\"bold\"" if bold else ""}{tr}>{esc(s)}</text>')
    elif e[0] == 'r':
        _, x, y, w, h, f, s = e
        out.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" fill="{f}"' + (f' stroke="{s}"' if s else '') + '/>')
    else:
        _, x1, y1, x2, y2, c, w, d = e
        out.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="{c}" stroke-width="{w}"' + (' stroke-dasharray="6 4"' if d else '') + '/>')
out.append('</svg>')
d = os.path.dirname(__file__)
open(f'{d}/graph.svg', 'w').write('\n'.join(out))
# PNG
sc = 2
img = Image.new('RGB', (W*sc, H*sc), 'white'); dr = ImageDraw.Draw(img)
def font(sz, b=False):
    for p in ('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf' if b else '/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',):
        if os.path.exists(p): return ImageFont.truetype(p, int(sz*sc*0.9))
    return ImageFont.load_default()
for e in els:
    if e[0] == 'r':
        _, x, y, w, h, f, s = e; dr.rectangle([x*sc, y*sc, (x+w)*sc, (y+h)*sc], fill=f, outline=s)
    elif e[0] == 'l':
        _, x1, y1, x2, y2, c, w, dsh = e
        if dsh:
            n = int((x2-x1)//10)
            for k in range(n+1):
                dr.line([(x1+k*10)*sc, y1*sc, min(x1+k*10+6, x2)*sc, y2*sc], fill=c, width=int(w*sc))
        else: dr.line([x1*sc, y1*sc, x2*sc, y2*sc], fill=c, width=int(w*sc))
    else:
        _, x, y, s, size, anc, fill, bold, rot = e
        f = font(size, bold); tw = dr.textlength(s, font=f)
        tx = x*sc - (tw/2 if anc == 'middle' else tw if anc == 'end' else 0)
        if rot:
            tmp = Image.new('RGBA', (int(tw)+4, int(size*sc*1.6)), (255,255,255,0)); ImageDraw.Draw(tmp).text((2, 0), s, font=f, fill=fill)
            tmp = tmp.rotate(rot, expand=True); img.paste(tmp, (int(x*sc-tmp.width/2), int(y*sc-tmp.height/2)), tmp)
        else: dr.text((tx, (y-size*0.8)*sc), s, font=f, fill=fill)
img.save(f'{d}/graph.png')
