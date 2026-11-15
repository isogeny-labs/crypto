#!/usr/bin/env python3
"""V0 (provenance) and V8 (held-out discipline) checks. usage: v0_v8_checks.py <repo> <N4v2 units.tar.gz> <out.json>"""
import json, os, sys, hashlib, subprocess, tarfile, glob
repo, n4tar, OUT = sys.argv[1:4]
def sha(p):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        for b in iter(lambda: f.read(1 << 20), b''): h.update(b)
    return h.hexdigest()
def git(*a): return subprocess.run(['git', '-C', repo] + list(a), capture_output=True, text=True).stdout.strip()
R = {}
J = os.path.join(repo, 'research/leads_chase_20261112/j2')
R['commit'] = git('rev-parse', 'HEAD')
R['porcelain_producer_paths'] = git('status', '--porcelain', '--', 'research/leads_chase_20261112', 'research/quartic_n4v2_20261020', 'research/quartic_n4_20261019', 'src/cryptanalysis/plane_quartic_pic.rs', 'benchmarks/records/20261112T0348Z-j2-percolation-fb-size.json')
man = json.load(open(os.path.join(J, '.artifacts.json')))['files']['v2.bin']
build = open(os.path.join(J, 'BUILD.txt')).read().split()
rec = json.load(open(os.path.join(repo, 'benchmarks/records/20261112T0348Z-j2-percolation-fb-size.json')))
R['v2.bin'] = dict(sha256=sha(os.path.join(J, 'v2.bin')), build_txt=build[-2] if len(build) >= 2 else None, manifest=man['sha256'], bytes_ok=os.path.getsize(os.path.join(J, 'v2.bin')) == man['bytes'])
R['v2.bin']['all_equal'] = len({R['v2.bin']['sha256'], R['v2.bin']['manifest'], [w for w in build if len(w) == 64][0]}) == 1
R['raw_units_tar'] = dict(sha256=sha(os.path.join(J, 'raw-units.tar.gz')), bench_record=rec['raw'][0]['sha256'])
t = tarfile.open(os.path.join(J, 'raw-units.tar.gz')); same = 0; bad = []
for m in t.getmembers():
    if m.isfile():
        p = os.path.join(repo, 'research/leads_chase_20261112', m.name)
        (same := same + 1) if (os.path.exists(p) and hashlib.sha256(t.extractfile(m).read()).hexdigest() == sha(p)) else bad.append(m.name)
R['raw_units_tar']['members_identical_to_committed'] = same; R['raw_units_tar']['members_not_identical'] = bad
n4m = json.load(open(os.path.join(repo, 'research/quartic_n4v2_20261020/results/grid-20261025/.artifacts.json')))['files']['units.tar.gz']
R['n4v2_units_tar'] = dict(sha256=sha(n4tar), manifest=n4m['sha256'], equal=(sha(n4tar) == n4m['sha256']), bytes=os.path.getsize(n4tar), manifest_bytes=n4m['bytes'])
inst = [json.loads(l) for l in open(os.path.join(repo, 'research/quartic_n4v2_20261020/results/instances.jsonl'))]
R['n4v2_instances_sha256'] = dict(computed=sha(os.path.join(repo, 'research/quartic_n4v2_20261020/results/instances.jsonl')), recorded=open(os.path.join(repo, 'research/quartic_n4v2_20261020/results/instances.sha256')).read().split()[0])
# V8 -------------------------------------------------------------
am = '0056acedae'
R['amendment_commit'] = dict(sha=git('rev-parse', am), committed_utc=git('log', '-1', '--format=%cI', am), subject=git('log', '-1', '--format=%s', am))
cmds = rec['commands']; R['heldout_launches_utc'] = [c['started_utc'] for c in cmds if 'j2_heldout' in ' '.join(c['argv'])]
gl = [l.split()[0] for l in open(os.path.join(J, 'heldout', 'grid.log')) if l[:2] == '20' and 'rc=' in l]
R['first_heldout_completion_utc_in_grid_log'] = min(gl)
R['amendment_precedes_first_heldout_launch'] = R['amendment_commit']['committed_utc'] < min(R['heldout_launches_utc'])
used = sorted({(r['q'], r['seed']) for r in inst})
R['heldout_seeds_in_n4v2_instances'] = [list(x) for x in used if x[0] in (2097091, 4194271) and x[1] >= 4]
R['n4v2_seeds_at_k21_k22'] = sorted({x[1] for x in used if x[0] in (2097091, 4194271)})
gridseeds = sorted({int(os.path.basename(f)[3:-6].split('_')[1]) for f in glob.glob(os.path.join(J, 'units', 'ic_*_c*.jsonl'))})
R['j2_grid_seeds'] = gridseeds
hs = sorted({int(os.path.basename(f)[3:-6].split('_')[1]) for f in glob.glob(os.path.join(J, 'heldout', 'ic_*_c*.jsonl'))})
R['j2_heldout_seeds'] = hs
fd = os.path.join(J, 'heldout', 'failed-binary-missing')
fj = sorted(f for f in os.listdir(fd) if f.endswith('.jsonl')); fe = sorted(f for f in os.listdir(fd) if f.endswith('.err'))
R['failed_dir'] = dict(jsonl=len(fj), all_empty=all(os.path.getsize(os.path.join(fd, f)) == 0 for f in fj), err_files=len(fe),
                       err_says_binary_missing=all('No such file or directory' in open(os.path.join(fd, f)).read() for f in fe),
                       each_has_final_unit_in_heldout=all(os.path.exists(os.path.join(J, 'heldout', f)) and os.path.getsize(os.path.join(J, 'heldout', f)) > 0 for f in fj))
R['heldout_units_with_data'] = len([f for f in glob.glob(os.path.join(J, 'heldout', 'ic_*_c*.jsonl')) if os.path.getsize(f) > 0])
json.dump(R, open(OUT, 'w'), indent=1)
print(json.dumps(R, indent=1))
