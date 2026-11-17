#!/usr/bin/env python3
"""Validator replay worker. Claims units from a fixed ordered list via mkdir locks, runs the solver binary on each,
stores stdout/stderr, never reuses a producer file. usage: replay_worker.py <binary> <outdir> <tag>
Unit command is the producer's: <bin> ic <q> <seed> D 3.0 <c> 4 4 (c printed with two decimals)."""
import os, subprocess, sys, time, json, resource
binp, outdir, tag = sys.argv[1], sys.argv[2], sys.argv[3]
os.makedirs(outdir, exist_ok=True)
# ordered unit list: six mandated units first, then held-out, then grid (large q first is not needed)
SIX = [(2097091, 4, '1.60'), (2097091, 4, '2.00'), (4194271, 4, '1.60'), (4194271, 4, '2.00'), (2097091, 5, '1.75'), (4194271, 5, '1.75')]
units = list(SIX)
for q in (2097091, 4194271):
    for sd in (4, 5, 6, 7, 8):
        for c in ('1.60', '1.75', '2.00'):
            if (q, sd, c) not in units: units.append((q, sd, c))
for q in (65479, 262111, 1048507):
    for sd in (1, 2, 3):
        for c in ('1.40', '1.50', '1.60', '1.75', '2.00'):
            units.append((q, sd, c))
for sd in (1, 2, 3):
    for c in ('1.60', '1.75', '2.00'):
        units.append((4194271, sd, c))
assert len(units) == 84 and len(set(units)) == 84, len(units)
def limit():
    resource.setrlimit(resource.RLIMIT_AS, (6_000_000 * 1024, 6_000_000 * 1024))
    os.nice(5)
for (q, sd, c) in units:
    name = f'ic_{q}_{sd}_c{c}'
    lock = os.path.join(outdir, name + '.lock')
    try: os.mkdir(lock)
    except FileExistsError: continue
    t0 = time.time(); t0s = time.strftime('%FT%TZ', time.gmtime())
    with open(os.path.join(outdir, name + '.jsonl'), 'w') as fo, open(os.path.join(outdir, name + '.err'), 'w') as fe:
        p = subprocess.run([binp, 'ic', str(q), str(sd), 'D', '3.0', c, '4', '4'], stdout=fo, stderr=fe, preexec_fn=limit)
    with open(os.path.join(outdir, 'worker_%s.log' % tag), 'a') as lg:
        lg.write(json.dumps(dict(unit=name, rc=p.returncode, wall_s=round(time.time() - t0, 2), start=t0s)) + '\n')
with open(os.path.join(outdir, 'worker_%s.log' % tag), 'a') as lg:
    lg.write(json.dumps(dict(done=time.strftime('%FT%TZ', time.gmtime()))) + '\n')
