#!/usr/bin/env python3
"""V9 rho replay (own driver): re-run a subset of the N4v2 frozen rho jobs with the rebuilt binary, exactly the commands of
research/quartic_n4v2_20261020/results/grid-20261025/joblist.txt, and compare every counter with the N4v2 raw records.
usage: rho_replay.py <binary> <outdir> <N4v2 raw units dir> <tag> [job ...]   job names as in the joblist (rho_65479_1_all, ...)"""
import os, sys, subprocess, json, time, resource
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import compare
binp, outdir, raw, tag = sys.argv[1:5]; jobs = sys.argv[5:]
os.makedirs(outdir, exist_ok=True)
SPEC = {  # job name -> argv after the binary (the joblist's own commands)
    'rho_65479_1_all': ['rho', '65479', '1', '0', '16', 'plain,neg'],
    'rho_262111_1_all': ['rho', '262111', '1', '0', '6', 'plain,neg'],
    'rho_2097091_1_0_neg': ['rho', '2097091', '1', '0', '1', 'neg'],
    'rho_2097091_1_0_plain': ['rho', '2097091', '1', '0', '1', 'plain'],
    'rho_4194271_1_0_neg': ['rho', '4194271', '1', '0', '1', 'neg'],
    'rho_4194271_1_0_plain': ['rho', '4194271', '1', '0', '1', 'plain'],
}
def limit(): resource.setrlimit(resource.RLIMIT_AS, (6_000_000 * 1024, 6_000_000 * 1024)); os.nice(5)
for j in jobs:
    t0 = time.time(); t0s = time.strftime('%FT%TZ', time.gmtime())
    with open(os.path.join(outdir, j + '.jsonl'), 'w') as fo, open(os.path.join(outdir, j + '.err'), 'w') as fe:
        p = subprocess.run([binp] + SPEC[j], stdout=fo, stderr=fe, preexec_fn=limit)
    a = compare.recs(os.path.join(raw, j + '.jsonl')); b = compare.recs(os.path.join(outdir, j + '.jsonl'))
    na, nb, nf, d = compare.diff(a, b)
    rec = dict(job=j, rc=p.returncode, wall_s=round(time.time() - t0, 1), start=t0s, n4v2_records=na, replay_records=nb, fields_compared=nf, identical=(not d and na > 0), diffs=d[:6])
    with open(os.path.join(outdir, 'rho_compare_%s.log' % tag), 'a') as lg: lg.write(json.dumps(rec) + '\n')
with open(os.path.join(outdir, 'rho_compare_%s.log' % tag), 'a') as lg: lg.write(json.dumps(dict(done=time.strftime('%FT%TZ', time.gmtime()))) + '\n')
