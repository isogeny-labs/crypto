import json
d=json.load(open('analysis/proj.json'))
cells=['k1n11','k0n13','k0n15']; abl=['memo0','nolin','noinh','splitlow','chainlayout','occmult','alloff']
Wb_lb={'k1n11':2.02e9+4.71e8,'k0n13':2.93e9+6.04e8,'k0n15':6.05e9+7.80e8}
L0={c:d[f'{c}.memo0']['L0'] for c in cells}; Wa={c:d[f'{c}.memo0']['Wa'] for c in cells}
print('| ablation | K_1 n=11 (lab l5) | K_0 n=13 (lab l6) | K_0 n=15 (lab l7) |\n|:--|--:|--:|--:|')
print('| baseline lab (R2W A/A, counted) | '+' | '.join(f'{L0[c]:.2e} ({L0[c]/Wa[c]:.2f})' for c in cells)+' |')
for a in abl:
    row=[]
    for c in cells:
        A=d[f'{c}.{a}']['A']; fa=(A-L0[c])/(Wa[c]-L0[c]); fb=(A-L0[c])/(Wb_lb[c]-L0[c])
        row.append(f'{A:.2e} ({A/Wa[c]:.2f}) [{fa:.2f} / {fb:.2f}]')
    print(f'| {a} (projected) | '+' | '.join(row)+' |')
print('| WDSat (a) `-x -b`, whole run, best l 8 / 8 / 10 (R2W, counted) | '+' | '.join(f'{Wa[c]:.2e} (1.00)' for c in cells)+' |')
