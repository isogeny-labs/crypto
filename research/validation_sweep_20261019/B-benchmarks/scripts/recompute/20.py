import sys, os, json, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _common import *

BASE = 'research/n83_e2e_precision_20261016/'
AN = BASE + 'analysis_38970960376.json'
OUT = p('research/validation_sweep_20261019/B-benchmarks/recompute/rows/row20.json')
STUDY = [BASE + 'RESULT.md', BASE + 'render_reports.py']

with open(p(AN)) as fh:
    A = json.load(fh)

cold = A['cold_process_ms_by_point']
k1104 = [c['cold_process_ms']['k1104'] for c in cold]
k1120 = [c['cold_process_ms']['k1120'] for c in cold]
m1104 = sum(k1104) / len(k1104) / 1000.0
m1120 = sum(k1120) / len(k1120) / 1000.0

diffs = [a - b for a, b in zip(k1104, k1120)]
n = len(diffs)
md = sum(diffs) / n
sd = math.sqrt(sum((d - md) ** 2 for d in diffs) / (n - 1))
try:
    from scipy.stats import t as _t
    tcrit = float(_t.ppf(0.975, n - 1))
    tsrc = 'scipy'
except Exception:
    tcrit = 2.131449545559  # t_{0.975,15}, hardcoded
    tsrc = 'hardcoded t_0.975,15'
lo = md - tcrit * sd / math.sqrt(n)
hi = md + tcrit * sd / math.sqrt(n)

pi = A['paired_interval']
claims = []
claims.append({'claimed': 'K1120 mean cold process 119.701 s', 'computed': f"{m1120:.3f} s (n={len(k1120)} points)",
               'definition': 'mean over 16 points of analysis cold_process_ms_by_point[].cold_process_ms.k1120 / 1000',
               'files': [AN], 'status': 'match' if round(m1120, 3) == 119.701 else 'mismatch'})
claims.append({'claimed': 'K1104 mean cold process 119.847 s', 'computed': f"{m1104:.3f} s (n={len(k1104)} points)",
               'definition': 'mean over 16 points of cold_process_ms_by_point[].cold_process_ms.k1104 / 1000',
               'files': [AN], 'status': 'match' if round(m1104, 3) == 119.847 else 'mismatch'})
claims.append({'claimed': 'paired K1104-minus-K1120 mean 145.454 ms', 'computed': f"{md:.3f} ms (recomputed); stored paired_interval.mean_ms={pi['mean_ms']:.3f}",
               'definition': 'mean of per-point differences k1104 - k1120 (ms), n=16',
               'files': [AN], 'status': 'match' if round(md, 3) == 145.454 else 'mismatch'})
claims.append({'claimed': '95% interval [-92.776, 383.684] ms', 'computed': f"[{lo:.3f}, {hi:.3f}] ms via mean +/- {tsrc} * sample_sd/sqrt(n); stored lower={pi['lower_95_ms']:.3f} upper={pi['upper_95_ms']:.3f}",
               'definition': 'paired t-interval, df=n-1=15, sample sd of the 16 differences',
               'files': [AN], 'status': 'match' if round(lo, 3) == -92.776 and round(hi, 3) == 383.684 else 'mismatch'})
claims.append({'claimed': 'frozen selection BOUNDED_UNKNOWN', 'computed': f"status={A['status']}, selected_candidate_id={A['selected_candidate_id']}, selected_pipeline={A['selected_pipeline']}",
               'definition': 'top-level status field of analysis JSON', 'files': [AN],
               'status': 'match' if A['status'] == 'BOUNDED_UNKNOWN' else 'mismatch'})
claims.append({'claimed': '64 fresh IC processes (IC arms)', 'computed': f"ic_arm_count={A['ic_arm_count']}; arms per point in point_rows={[len(r['arms']) for r in A['point_rows']][:1]} x {len(A['point_rows'])} points",
               'definition': 'ic_arm_count field of analysis JSON; cross-check of arms list length',
               'files': [AN], 'status': 'match' if A['ic_arm_count'] == 64 else 'mismatch'})
claims.append({'claimed': '16 same-point rho solves', 'computed': f"rho_arm_count={A['rho_arm_count']}; point_count={A['point_count']}",
               'definition': 'rho_arm_count field of analysis JSON', 'files': [AN],
               'status': 'match' if A['rho_arm_count'] == 16 else 'mismatch'})

study_off = grep_lines(STUDY, OFFREPO_RX)
study_aa = grep_lines(STUDY, AA_RX)
study_iso = grep_lines(STUDY, ISO_RX)
aa_json = [f"{AN}: point_rows[{r['point_index']}].aa_control present" for r in A['point_rows'][:1] if 'aa_control' in r]
proto = BASE + 'RESULT.md'
first_raw = AN

row = {
    'row_no': 20, 'slug': '20261017-n83-e2e-precision',
    'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'},
    'offrepo_refs': study_off, 'aa_lines': study_aa + aa_json, 'isolation_lines': study_iso,
    'protocol_git_date': git_added(proto), 'first_raw_git_date': git_added(first_raw),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/20.py',
    'notes': (f"no PROTOCOL file in study dir; RESULT.md used as spec-bearing file for protocol_git_date. t-interval critical value from {tsrc}. "
              "The A/A control data is in analysis JSON point_rows[].aa_control (not in RESULT.md text); aa_lines lists the md grep matches plus the JSON key presence. "
              "The ZIP path /Volumes/SSD990/... is quoted from RESULT.md; the ZIP itself is not in the repo (its SHA-256 is in provider_38970960376.json)."),
}
write_row(20, row, OUT)
