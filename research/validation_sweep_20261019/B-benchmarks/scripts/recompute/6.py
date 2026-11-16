import csv
import json
import os
import sys
from decimal import Decimal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 6
SLUG = '20261017-photo-volcano-restart'
STUDY = ['research/photo_volcano_restart_20261017']
Q = 'research/photo_volcano_restart_20261017/reports/quantitative_1452'
PCSV = Q + '/per_curve.csv'
ICJ = Q + '/ic_per_problem.json'
SUM = Q + '/summary.json'
PROTOCOL = 'research/photo_volcano_restart_20261017/PROTOCOL.md'
RAW_CANDS = ['research/photo_volcano_restart_20261017/reports/quantitative_1452/ic_per_problem.json',
             'research/photo_volcano_restart_20261017/reports/quantitative_1452/per_curve.csv',
             'research/photo_volcano_restart_20261017/reports/quantitative_1452/per_curve.json',
             'research/photo_volcano_restart_20261017/data/ic_support_audit.json',
             'research/photo_volcano_restart_20261017/artifacts/public_ic_metrics_and_replay.tar.xz',
             'research/photo_volcano_restart_20261017/artifacts/public_inputs.tar.xz']

rows = list(csv.DictReader(open(abspath(PCSV))))
allrows = [r for r in rows if r['generator'] == 'all']
rho = [r for r in allrows if r['algorithm'] == 'rho']
bsgs = [r for r in allrows if r['algorithm'] == 'bsgs']
rho_n = sum(int(r['n']) for r in rho)
rho_ver = sum(int(r['n']) - int(r['failures']) for r in rho)
bsgs_req = sum(int(r['n']) for r in bsgs)
bsgs_ver = sum(int(r['n']) - int(r['failures']) for r in bsgs)
means = [float(r['ops_mean']) for r in rho]
mn, mx = min(means), max(means)

ic = json.load(open(abspath(ICJ)))
probes = sum(p['metrics']['gb_wall_ns']['n'] for p in ic)
fm_key = 'gb.field_multiply_including_inverse_power'
fm_min = min(p['metrics'][fm_key]['min'] for p in ic)
fm_max = max(p['metrics'][fm_key]['max'] for p in ic)
wall_min_ms = min(p['metrics']['gb_wall_ns']['min'] for p in ic) / 1e6
wall_max_ms = max(p['metrics']['gb_wall_ns']['max'] for p in ic) / 1e6

summ = json.load(open(abspath(SUM)))
F_pc = [PCSV]
F_ic = [ICJ]

def fmt_ms(x):
    return '%.6f' % x

claims = [
    C('4600', rho_ver, 'sum of n minus failures over the rho generator=all rows of per_curve.csv (%d rows); '
      'summary.json verified_rho = %s' % (len(rho), summ['verified_rho']), F_pc, 'partial'),
    C('3297', bsgs_ver, 'sum of n minus failures over the bsgs generator=all rows of per_curve.csv; '
      'summary.json verified_bsgs = %s' % summ['verified_bsgs'], F_pc, 'partial'),
    C('107895789.39', '%.2f' % mn, 'minimum over the %d curve rows of rho ops_mean (generator=all)' % len(rho),
      F_pc, 'partial'),
    C('134494184.67', '%.2f' % mx, 'maximum over the same rho ops_mean values', F_pc, 'partial'),
    C('1.246519', '%.6f' % (mx / mn), 'max/min of the same rho ops_mean values', F_pc, 'partial'),
    C('217410', fm_min, 'minimum over the 100 instances of the per-instance min of %s (ic_per_problem.json)' % fm_key,
      F_ic, 'partial'),
    C('217410', fm_max, 'maximum over the 100 instances of the per-instance max of %s' % fm_key, F_ic, 'partial'),
    C('11.519667', fmt_ms(wall_min_ms), 'minimum over the 100 instances of per-instance gb_wall_ns min, in ms', F_ic,
      'partial'),
    C('266.013969', fmt_ms(wall_max_ms), 'maximum over the 100 instances of per-instance gb_wall_ns max, in ms', F_ic,
      'partial'),
    C('4600', probes, 'sum over the 100 instances of gb_wall_ns n (IC target GB probes)', F_ic, 'partial'),
    {'claimed': 'zero full IC recoveries', 'computed': 'summary.json full_ic_recoveries = %s; no per-probe recovery '
     'field in ic_per_problem.json' % summ['full_ic_recoveries'], 'definition': 'search for per-probe recovery records',
     'files': [SUM, ICJ], 'status': 'no_raw_in_repo'},
    C('14:50 UTC', summ['generated_utc'], 'summary.json generated_utc (snapshot time)', [SUM],
      'match' if summ['generated_utc'][11:16] == '14:50' else 'mismatch'),
]

notes = ('Per-run rho and BSGS records and per-probe IC records are not committed in this repository: the README says raw '
         'attempts and RNG journals remain local. The committed per-curve (per_curve.csv, n and failures per curve) and '
         'per-instance (ic_per_problem.json, min/mean/max over 46 curves) aggregates were used, so those claims are '
         'partial. The archive public_ic_metrics_and_replay.tar.xz holds metrics/per_curve.csv (not extracted here). '
         'Counts: rho rows %d, bsgs rows %d, IC instances %d. The headline "rho 4600/4600" is the requested count of 46 curves x 100 cases.'
         % (len(rho), len(bsgs), len(ic)))

offrepo_files, off, aa, iso = scan_study(STUDY)
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, no_identical(), off + store, aa, iso,
     git_add(PROTOCOL), first_raw_date(RAW_CANDS),
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/6.py', notes)
