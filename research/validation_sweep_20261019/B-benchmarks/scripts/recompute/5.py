import json
import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 5
SLUG = '20261017-perf-history-factor-bases'
STUDY = ['docs/perf/PERFORMANCE_INDEX.md']
PI = 'docs/perf/PERFORMANCE_INDEX.md'
CHILDREN = 'docs/perf/ecbench-children.json'
RAW_CANDS = []  # the card lists none

pi_text = open(abspath(PI), encoding='utf-8').read()


def where(s):
    for i, line in enumerate(pi_text.splitlines(), 1):
        if s in line:
            return '%s:%d' % (PI, i)
    return 'absent'


ch = json.load(open(abspath(CHILDREN)))['children']
ic_fb = [c for c in ch if c['id'].startswith('ic_factor_base/')]
ic_pipe = [c for c in ic_fb if c['input']['method']['id'] == 'ic.pipeline']
rho_fb = [c for c in ic_fb if c['input']['method']['id'] != 'ic.pipeline']
perf_hist = subprocess.run(['git', '-C', REPO, 'rev-parse', '--verify', '-q', 'perf-history'],
                           capture_output=True, text=True).stdout.strip()

F = [PI, CHILDREN]
NR = 'no_raw_in_repo'
claims = [
    C('11', len(ic_fb),
      'count of ic_factor_base/ children in docs/perf/ecbench-children.json (%d ic.pipeline + %d rho)' %
      (len(ic_pipe), len(rho_fb)), [CHILDREN]),
    C('9', len(ic_pipe),
      'count of ic.pipeline (index-calculus) children, i.e. factor-base arms, in the same file; the 3 rho children are '
      'the rho references', [CHILDREN]),
    {'claimed': '28.1 M Ir (index calculus, low end)', 'computed': 'string "28.1" in PERFORMANCE_INDEX.md: %s' %
     where('28.1'), 'definition': 'search for a per-run Ir record behind the claim', 'files': F, 'status': NR},
    {'claimed': '34.6 M Ir (index calculus, high end)', 'computed': 'text only at %s (34.6 M against 33.2 M); '
     'no per-run Ir record committed' % where('34.6 M'), 'definition': 'search for per-run Ir record',
     'files': F, 'status': NR},
    {'claimed': '0.22 M Ir (rho, low end)', 'computed': 'string "0.22" in PERFORMANCE_INDEX.md: %s' % where('0.22'),
     'definition': 'search for per-run Ir record', 'files': F, 'status': NR},
    {'claimed': '0.84 M Ir (rho, high end)', 'computed': 'string "0.84" in PERFORMANCE_INDEX.md: %s' % where('0.84'),
     'definition': 'search for per-run Ir record', 'files': F, 'status': NR},
    {'claimed': '38 to 159 times rho', 'computed': 'no ratio string in PERFORMANCE_INDEX.md; ratios need the '
     'Ir records named above', 'definition': 'search for per-run Ir record', 'files': F, 'status': NR},
    {'claimed': '3 times the signed points (192 against 64)', 'computed': '192/64 = %.1f from text at %s' %
     (192 / 64, where('(192')), 'definition': 'text arithmetic; no factor-base run report committed',
     'files': F, 'status': NR},
    {'claimed': '4 % more Ir (34.6 M against 33.2 M)', 'computed': '34.6/33.2 - 1 = %.3f from text; '
     'no per-run Ir record committed' % (34.6 / 33.2 - 1), 'definition': 'text arithmetic',
     'files': F, 'status': NR},
    {'claimed': '75 snapshots of perf history', 'computed': 'perf-history ref in this clone: %s' %
     ('present' if perf_hist else 'absent (no local or origin perf-history ref)'),
     'definition': 'the snapshots are on the perf-history branch named in the study', 'files': F,
     'status': NR},
]

notes = ('Study file is docs/perf/PERFORMANCE_INDEX.md; the card lists no raw candidates, so no per-child Ir record '
         'was read. docs/perf/ecbench-children.json holds the 13 frozen child inputs (no Ir outputs). '
         'benchmarks/20261017-perf-history-factor-bases/ holds renders (README, svg, png, pdf) only; they were not used as raw. '
         'The 9 factor-base count in the headline is not reproduced by the children file: 8 ic.pipeline children '
         'plus 3 rho references make the 11 children. first_raw_git_date is empty because no raw candidates are listed.')

offrepo_files, off, aa, iso = scan_study(STUDY)
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, no_identical(), off + store, aa, iso,
     git_add(PI), '',
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/5.py', notes)
