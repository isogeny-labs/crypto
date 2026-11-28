import json, re
R = open('/home/user/crypto/research/curve25519_wave7_untried_20261122/L3_translated_involution/README.md').read()
V = json.load(open('verdicts_val.json'))
cells = V['cells']
names = {'E x-frame (null)':'E.x','E translated frame, T = 2S (control)':'E.iota_even','E Montgomery y/x, T = T0 (control)':'E.iota_T0',
 'E odd T, no rational half':'E.iota_odd',"E' x-frame (null)":'Ep.x',"E' T ∈ 2E' (control)":'Ep.iota_2E',
 "E' class (0,1), no rational half":'Ep.iota_c01',"E' class (1,0), no rational half":'Ep.iota_c10',"E' class (1,1), no rational half":'Ep.iota_c11',
 "E' Huff x":'Ep.huff_x',"E' Huff y":'Ep.huff_y',"E' Jacobi-intersection s (degree 4)":'Ep.ji_s',"E' Jacobi-intersection c (degree 4)":'Ep.ji_c',
 "E' Jacobi-intersection d (degree 4)":'Ep.ji_d'}
bad = 0; n = 0
for line in R.splitlines():
    m = re.match(r'\| (.+?) \| ([\d.]+) \| ([\d.]+) \| ([\d.]+) \| ([\d.]+) \| ([\d.]+) \|$', line)
    if m and m.group(1) in names:
        arm = names[m.group(1)]
        for t, v in zip((4,5,6,7,8), m.groups()[1:]):
            mine = cells[f'20|{t}|{arm}']['mean_D']
            n += 1
            if f'{mine:.1f}' != v:
                bad += 1; print('MISMATCH', arm, t, v, mine)
print('k=20 table cells', n, 'mismatches', bad)
# ratio table
P1, P2, P3 = V['P1_cells'], V['P2_cells'], V['P3_cells']
rows = {'E odd T / E x-frame': lambda t: P1[f'20|{t}|E.iota_odd'], "E' Huff x / E' x-frame": lambda t: P1[f'20|{t}|Ep.huff_x'],
        "E' Jacobi-intersection s / E' x-frame": lambda t: P3[f'20|{t}|Ep.ji_s'], "E' x-frame / E x-frame": lambda t: P2[f'20|{t}']}
for line in R.splitlines():
    for name, f in rows.items():
        if line.startswith('| ' + name + ' |'):
            parts = [p.strip() for p in line.strip('|').split('|')][1:]
            for t, cell in zip((4,5,6,8), parts):
                m = re.match(r'([\d.]+) \(([\d.]+)–([\d.]+)\) \[([\d.]+)\]', cell)
                r_, lo, hi, pm = map(float, m.groups())
                mine = f(t)
                flag = '' if (abs(r_-mine[0])<=0.0006 and abs(pm-mine[3])<=0.0006 and abs(lo-mine[1])<0.006 and abs(hi-mine[2])<0.006) else '  <-- CHECK'
                print(name, t, (r_, lo, hi, pm), 'mine', mine, flag)
