"""Extract committed values for the fm figures and tables from origin/main via git only.

    python3 extract_fm_data.py <crypto checkout> <output dir>   # writes data-fm-{glance,opcost,phases}.json
"""
import json, re, subprocess, collections, statistics, datetime, csv, io, sys
W = sys.argv[1]; OUT = sys.argv[2]
def git(*a, inp=None):
    return subprocess.run(['git', '-C', W, *a], input=inp, capture_output=True, check=True).stdout
def show(p): return git('show', f'origin/main:{p}')
def sha(p): return git('rev-parse', f'origin/main:{p}').decode().strip()
HEAD = git('rev-parse', 'origin/main').decode().strip()
def src(p, what): return {"path": p, "blob": sha(p), "what": what}
months = ["2026-07", "2026-08", "2026-09", "2026-10", "2026-11"]

# ---------- glance ----------
G = {"_comment": "Counts by month, copied or counted from committed files at origin/main %s. Counting rules are in each block's 'rule'." % HEAD, "commit": HEAD, "months": months, "sources": []}
S = json.loads(show('research/ecdlp_jul_sep_index_20261029/SUMMARY.json'))
G["sources"].append(src('research/ecdlp_jul_sep_index_20261029/SUMMARY.json', 'crypto-autoresearcher ledger decisions, experiments and run directories, 19 Jul - 30 Sep'))
G["ledger"] = {"rule": "copied from SUMMARY.json; the index covers 19 Jul-30 Sep; its 7 October-dated rows are late-September files and are not a count of October work", "decisions": S["decisions_by_month"], "experiments": S["distinct_experiments_by_month"], "run_dirs": S["runs_by_month"]}
# research study dirs
dirs = git('ls-tree', '-d', '--name-only', 'origin/main', 'research/').decode().split()
c = collections.Counter(); und = 0
for d in dirs:
    m = re.search(r'_2026(\d\d)\d\d$', d) or re.search(r'2026(\d\d)\d\d', d)
    if m: c['2026-' + m.group(1)] += 1
    else: und += 1
G["sources"].append({"path": "research/", "tree": sha('research'), "what": "top-level study directories; month from the YYYYMMDD in the directory name"})
G["studies"] = {"rule": "top-level directories under research/ at origin/main, month from the 2026MMDD in the name", "by_month": dict(sorted(c.items())), "undated": und, "total": len(dirs)}
# ecbench sessions
paths = git('ls-tree', '-r', '--name-only', 'origin/main').decode().split('\n')
by = collections.defaultdict(set)
for p in paths:
    d, _, f = p.rpartition('/'); by[d].add(f)
ses = sorted(d for d, fs in by.items() if {'session.json', 'plan.json', 'records.jsonl'} <= fs and d.startswith('research/'))
out = git('cat-file', '--batch', inp=''.join(f'origin/main:{d}/session.json\n' for d in ses).encode())
i = 0; sc = collections.Counter(); rc = collections.Counter(); ids = set(); st = collections.Counter()
for d in ses:
    e = out.index(b'\n', i); size = int(out[i:e].split()[2]); body = out[e+1:e+1+size]; i = e + 2 + size
    o = json.loads(body); assert o['session_id'].startswith('ECBS1h')
    mo = datetime.datetime.fromtimestamp(o['started_unix_ms']/1000, datetime.timezone.utc).strftime('%Y-%m')
    sc[mo] += 1; rc[mo] += o.get('records_written') or 0; st[(mo, o['status'])] += 1; ids.add(o['session_id'])
G["ecbench"] = {"rule": "directories under research/ holding session.json, plan.json and records.jsonl (an ecbench session capsule); month of started_unix_ms (UTC); records = records_written", "sessions": dict(sorted(sc.items())), "records": dict(sorted(rc.items())), "status": {f"{a}|{b}": n for (a, b), n in sorted(st.items())}, "directories": len(ses), "distinct_session_ids": len(ids)}
G["sources"].append({"path": "research/**/session.json", "tree": sha('research'), "what": "ecbench session capsules (session_id ECBS1h...)"})
# benchmarks README admitted rows
R = show('benchmarks/README.md').decode().split('\n')
cut = next(k for k, l in enumerate(R) if l.startswith('## Studies awaiting'))
rows = [l for l in R[:cut] if re.match(r'^\| 2026-\d\d-\d\d \|', l)]
bc = collections.Counter(l[2:9] for l in rows)
valdirs = set()
for p in paths:
    if p.endswith('/VALIDATION.json') and p.startswith('benchmarks/'): valdirs.add(p.split('/')[1])
rowdirs = [re.search(r'\[`([^`]+)`\]', l).group(1) for l in rows]
withval = collections.Counter(l[2:9] for l, d in zip(rows, rowdirs) if d in valdirs)
G["benchmarks"] = {"rule": "rows of the admitted results table in benchmarks/README.md (before 'Studies awaiting benchmark admission'), month from the date column; with_validation_json = the row's directory holds a VALIDATION.json", "admitted_rows": dict(sorted(bc.items())), "with_validation_json": dict(sorted(withval.items()))}
G["sources"].append(src('benchmarks/README.md', 'admitted benchmark rows'))
A = json.loads(show('benchmarks/ARCHIVE.json'))
ac = collections.Counter(('2026-' + re.search(r'2026(\d\d)', r['directory']).group(1), r['disposition']) for r in A['rows'])
G["archive"] = {"rule": "rows of benchmarks/ARCHIVE.json (removed from the admitted table), month from the directory name", "by_month_disposition": {f"{a}|{b}": n for (a, b), n in sorted(ac.items())}, "total": len(A['rows'])}
ad = collections.Counter((r['directory'][:8], r['disposition']) for r in A['rows'])
G["archive"]["by_day_disposition"] = {f"{a}|{b}": n for (a, b), n in sorted(ad.items())}
G["archive"]["rows"] = [{"directory": r['directory'], "disposition": r['disposition']} for r in A['rows']]
G["sources"].append(src('benchmarks/ARCHIVE.json', 'archived (removed) benchmark rows and dispositions'))
P = json.loads(show('benchmarks/PRE_GATE.json'))
G["pre_gate"] = {"rows": len(P['rows']), "frozen": P['frozen']}
G["sources"].append(src('benchmarks/PRE_GATE.json', 'rows frozen as unvalidated (pre-gate)'))
# VALIDATION.json gate files
vc = collections.Counter(); vund = 0; vtot = 0
for p in paths:
    if p.endswith('VALIDATION.json') and 'fixtures' not in p:
        o = json.loads(show(p))
        if o.get('schema') != 'validation-gate/v1': continue
        vtot += 1
        m = re.search(r'2026(\d\d)\d\d', p)
        if m: vc['2026-' + m.group(1)] += 1
        else: vund += 1
G["validation_files"] = {"rule": "files named VALIDATION.json with schema validation-gate/v1, test fixtures excluded; month from the 2026MMDD in the path; benchmark copies of a study's file count separately", "by_month": dict(sorted(vc.items())), "undated": vund, "total": vtot}
# leaderboard
LB = json.loads(show('docs/ic/leaderboard.json'))
fam = collections.Counter((r['family'], r['validation']['status']) for r in LB['board'])
G["leaderboard"] = {"status_counts": LB['validation_gate']['status_counts'], "by_family": {f"{a}|{b}": n for (a, b), n in sorted(fam.items())}}
G["sources"].append(src('docs/ic/leaderboard.json', 'validation_gate.status_counts and board[*].validation.status'))
# pcc
L = [json.loads(l) for l in show('pcc/ledger.jsonl').decode().split('\n') if l.strip()]
G["pcc_ledger"] = {"entries": len(L), "by_type": dict(collections.Counter(e['type'] for e in L)), "first": L[0]['at'], "last": L[-1]['at']}
G["sources"].append(src('pcc/ledger.jsonl', 'hash-chained pcc ledger: claim, bypass and genesis entries; no verdict entries'))
cl = [p for p in paths if p.startswith('pcc/claims/PCC-')]
out = git('cat-file', '--batch', inp=''.join(f'origin/main:{p}\n' for p in cl).encode())
i = 0; kc = collections.Counter(); pk = collections.Counter(); dc = collections.Counter()
for p in cl:
    e = out.index(b'\n', i); size = int(out[i:e].split()[2]); o = json.loads(out[e+1:e+1+size]); i = e + 2 + size
    kc[o['kind']] += 1; dc[o['created_at'][:10]] += 1
    kinds = sorted(set(x['kind'] for x in o.get('proofs', [])))
    pk[(o['kind'], '+'.join(kinds) or 'none')] += 1
G["pcc_claims"] = {"rule": "pcc/claims/PCC-*.json; proofs = the set of proof kinds the record carries", "files": len(cl), "by_kind": dict(kc), "by_date": dict(sorted(dc.items())), "by_kind_proofs": {f"{a}|{b}": n for (a, b), n in sorted(pk.items())}}
G["sources"].append({"path": "pcc/claims/", "tree": sha('pcc/claims'), "what": "pcc.claim/v1 records"})
D = [l for l in show('docs/ecbench/DEFECTIVE_SESSIONS.tsv').decode().split('\n') if l and not l.startswith('#')]
G["defective_sessions"] = len(D)
G["sources"].append(src('docs/ecbench/DEFECTIVE_SESSIONS.tsv', 'sessions with an owner-dispositioned integrity defect'))
json.dump(G, open(f'{OUT}/data-fm-glance.json', 'w'), indent=1)

# ---------- op costs on the ledger host ----------
C = {"_comment": "Per-operation costs copied from committed files. Medians/min/max over the three frozen boundary-ledger runs are computed by extract_fm_data.py (median as docs/ic/calibration.json does for its ratios).", "commit": HEAD, "sources": []}
runs = ['ic-boundary-ledger-round2-2026-09-21', 'ic-boundary-ledger-round3-rotation-2026-09-21', 'ic-boundary-ledger-round3-2026-09-21']
per = collections.defaultdict(lambda: collections.defaultdict(list)); meta = {}
for r in runs:
    p = f'docs/ic/runs/{r}.json'; o = json.loads(show(p)); C["sources"].append(src(p, 'ledger.instances[*].calibration ns_per_* (host %s)' % o['host']['cpu']))
    C["host"] = o['host']
    for inst in o['ledger']['instances']:
        k = (inst['regime'], inst['curve']['name'] if isinstance(inst['curve'], dict) else inst['curve']); meta[k] = inst['log2_r']
        for f, v in inst['calibration'].items():
            if f.startswith('ns_per') and v is not None: per[k][f].append(v)
inst = []
for (reg, cur), d in per.items():
    nb = re.search(r'(?:n|2\^|bit)(\d+)', cur); 
    m = re.search(r'GF\(2\^(\d+)\)', cur) or re.search(r'-n(\d+)-', cur) or re.search(r'(\d+)bit', cur)
    inst.append({"regime": reg, "curve": cur, "field_bits": int(m.group(1)), "log2_r": meta[(reg, cur)],
                 "ns": {f: {"median": statistics.median(v), "min": min(v), "max": max(v), "n": len(v)} for f, v in d.items()}})
C["ledger_host"] = inst
cal = json.loads(show('docs/ic/calibration.json')); C["sources"].append(src('docs/ic/calibration.json', 'pinned ratios ns_per_<op>/ns_per_add'))
C["calibration_ratios"] = cal['instances']
b2 = json.loads(show('research/prime_fast_bench_20261008/bench2.json'))
C["prime_fast_micro"] = {"host": "Apple M4 Pro, 14 threads, load average about 50 (second sample); contended", "rows": [r for r in b2['results'] if r['section'] == 'micro']}
C["sources"].append(src('research/prime_fast_bench_20261008/bench2.json', 'Fp64/FastCurve vs BigUint micro-benchmarks, ns per operation (second, lower-load sample)'))
C["sources"].append(src('research/prime_fast_bench_20261008/HOST.md', 'host and load'))
C["gf2_97_ptx"] = {"rule": "static PTX instruction counts (ptx_stats2k.sh, clang sm_90); never run on a GPU", "mul": 304, "sqr": 78, "class_weight": 165, "tau_k_windowed": 296, "secp256k1_mul": 472, "secp256k1_mul_inline_ptx": 210,
                   "inversion": {"mul": 7, "sqr": 96, "fermat_prime_mul": 270},
                   "per_step": [{"W": w, "mul": a, "sqr": b, "ptx_before": c_, "ptx_after": d} for w, a, b, c_, d in [(1, 9.00, 108.90, 12425, 11395), (2, 7.00, 61.23, 7844, 7069), (4, 6.00, 37.15, 5534, 4887), (8, 5.50, 25.33, 4397, 3813), (16, 5.25, 19.32, 3820, 3268), (32, 5.12, 16.49, 3543, 3008)]]}
C["sources"].append(src('gpu/ecc2k/README.md', "'What the field arithmetic actually costs' and the per-step table"))
C["gf2_itoh_tsujii"] = {"n": 53, "fermat_mul": 51, "itoh_tsujii_mul": 7}
C["sources"].append(src('RESEARCH_KOBLITZ_INDEX_CALCULUS.md', 'Itoh-Tsujii Gf2::inv, 2026-09-23'))
J = list(csv.DictReader(io.StringIO(show('research/jacobian_op_count_20261014/summary.csv').decode())))
C["cantor"] = [r for r in J if r['kind'] == 'genus']
C["sources"].append(src('research/jacobian_op_count_20261014/summary.csv', 'counted Cantor addition: mean multiplications and inversions per genus and prime, against 10 g^2'))
r12 = show('benchmarks/20261026-r12-rho-ns-per-step/README.md').decode()
C["rho_ns_per_step"] = {"rule": "Apple arm64, L0 (busy, unpinned): descriptive", "rows": [{"n": 41, "tuned": 95.0, "strong": 519.9, "ratio": 5.48}, {"n": 53, "tuned": 92.2, "strong": 861.5, "ratio": 9.34}, {"n": 61, "tuned": 102.5, "strong": 979.7, "ratio": 9.56}]}
assert '| 95.0 | 519.9 | 5.48' in r12 and '| 92.2 | 861.5 | 9.34' in r12 and '| 102.5 | 979.7 | 9.56' in r12
C["sources"].append(src('benchmarks/20261026-r12-rho-ns-per-step/README.md', 'rho ns per step'))
g = show('gpu/ecc2k/README.md').decode()
for s_ in ['| F(2^97) multiply | 304', '| F(2^97) squaring | 78', '| secp256k1 multiply | 472', '| 8 | 5.50 | 25.33 | 4397 | **3813**', 'about 7\n  multiplications and 96 squarings, against 270']:
    assert s_ in g, s_
k = show('RESEARCH_KOBLITZ_INDEX_CALCULUS.md').decode(); assert '(7 vs 51 at n = 53)' in k
e = show('docs/ecbench/README.md').decode(); assert 'S/M=0.75, I/M=10' in e
a4 = show('research/ic_acct4_inversion_20261021/README.md').decode(); assert 'each cost about 2.1\nfield multiplications' in a4
isf = show('RESEARCH_ISOGENY_FIBERS_ECDLP.md').decode(); assert 'I = 91 M' in isf
C["declared"] = [
 {"what": "ecbench three-leads GAE conversion", "S_over_M": 0.75, "I_over_M": 10, "kind": "declared conversion, not a host-time calibration", "source": "docs/ecbench/README.md"},
 {"what": "bitop/v1 price of a field inversion and a residue test at b = 6-16 bits", "I_over_M": 2.1, "kind": "price of a binary extended Euclid, in field multiplications", "source": "research/ic_acct4_inversion_20261021/README.md"},
 {"what": "isogeny-fibres round 7 cost convention", "I_over_M": 91, "kind": "counted convention, inversions not executed in the tally", "source": "RESEARCH_ISOGENY_FIBERS_ECDLP.md"}]
for p_ in ['docs/ecbench/README.md', 'research/ic_acct4_inversion_20261021/README.md', 'RESEARCH_ISOGENY_FIBERS_ECDLP.md']:
    C["sources"].append(src(p_, 'declared or priced I/M, S/M'))
json.dump(C, open(f'{OUT}/data-fm-opcost.json', 'w'), indent=1)

# ---------- phase shares ----------
PH = {"_comment": "Per-row phase shares of S for every leaderboard row (best recipe), computed by extract_fm_data.py as phases_s[p]/s; ratio_rho and standing copied.", "commit": HEAD, "sources": [src('docs/ic/leaderboard.json', 'board[*].best.phases_s, .s, .ratio_rho; board[*].validation.status')], "phase_names": {p['id']: p['name'] for p in LB['phases']}, "rows": []}
for r in LB['board']:
    b = r['best']
    PH["rows"].append({"family": r['family'], "slug": r['slug'], "regime": r['regime'], "log2_r": r['log2_r'], "variant": b['variant'], "s": b['s'], "ratio_rho": b.get('ratio_rho'), "standing": r['validation']['status'], "phases_s": b['phases_s']})
json.dump(PH, open(f'{OUT}/data-fm-phases.json', 'w'), indent=1)
print('ok', HEAD)
