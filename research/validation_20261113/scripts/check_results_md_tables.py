#!/usr/bin/env python3
"""Every arm-row cell of the RESULTS.md tables (group ops, old ratio, new ratio, classes,
calibrated ratio) against the validator's recompute (out/recompute_tables.json), at the
printed precision (3 significant figures, thousands separators)."""
import json, re, sys
R = '/home/user/crypto/research'
md = open(f'{R}/negation_remeasure_20261113/RESULTS.md').read().splitlines()
rc = json.load(open(f'{R}/validation_20261113/out/recompute_tables.json'))
NR = json.load(open(f'{R}/negation_remeasure_20261113/remeasure.json'))['targets']
idx = {(r['target'], r['p']): r for r in rc['rows']}
def num(s):
    return float(s.replace(',', ''))
def agrees(printed, value):
    s = printed.strip().replace(',', '')
    v = float(s)
    if value == 0: return v == 0
    # printed with 3 significant figures or as an integer
    digits = len(re.sub(r'[^0-9]', '', s.lstrip('0').replace('.', '')))
    rel = abs(v - value) / abs(value)
    return rel <= 0.0051 or abs(v - round(value)) < 0.51 or abs(v - value) <= 0.5 * 10 ** (len(s.split('.')[1]) * -1 if '.' in s else 0) * 1.01
section = None
checked = 0; bad = []
for line in md:
    if line.startswith('#### '):
        section = line
        continue
    if not line.startswith('| ') or section is None or '--' in line[:6]:
        continue
    cells = [c.strip() for c in line.strip('|').split('|')]
    if not cells[0].replace(',', '').isdigit():
        continue
    tgt = 'v3' if 'v3' in section else 'cover_pushforward' if 'cover' in section else 'tower_large'
    p = int(cells[0].replace(',', ''))
    arm = cells[1]
    r = idx[(tgt, p)]['arms'][arm]
    raw = [x for x in NR[tgt]['rows'] if x['p'] == p][0]['arms'][arm]
    if 'recorded arms' in section:
        # p | arm | group ops | old ratio | old class | new ratio | new class | cal ratio | class cal | lower bound
        vals = [(cells[2], r['group_ops'])]
        old = raw.get('recorded_ratio_to_old_plain_rho', raw.get('recorded_median_ratio_to_old_plain_rho', raw.get('recorded_ratio_to_old_plain_rho_la_priced')))
        vals += list(zip(cells[3].split('/'), old))
        vals += list(zip(cells[5].split('/'), r['ratio_neg_band']))
        vals += list(zip(cells[7].split('/'), r['ratio_neg_cal']))
        cls = [(cells[4], raw['recorded_class']), (cells[6], r['class_neg_band']), (cells[8], r['class_neg_cal']),
               (cells[9], 'yes' if r['lower_bound'] else 'no')]
    else:
        # p | arm | group ops | native | ratio | class | cal ratio | class cal
        vals = [(cells[2], r['group_ops'])]
        nat = dict(re.findall(r'([a-z_]+) ([0-9][0-9,]*(?:\.[0-9]+)?)', cells[3]))
        for k, v in nat.items():
            vals.append((v, raw['native'][k]))
        vals += list(zip(cells[4].split('/'), r['ratio_neg_band']))
        vals += list(zip(cells[6].split('/'), r['ratio_neg_cal']))
        cls = [(cells[5], r['class_neg_band']), (cells[7], r['class_neg_cal'])]
    for s, v in vals:
        checked += 1
        if not agrees(s, v): bad.append([tgt, p, arm, s.strip(), v])
    for s, v in cls:
        checked += 1
        if s != v: bad.append([tgt, p, arm, s, v])
json.dump({'cells_checked': checked, 'mismatches': bad}, open(sys.argv[1], 'w'), indent=1)
print('cells checked', checked, 'mismatches', len(bad))
for b in bad[:20]: print(b)
