#!/usr/bin/env python3
"""Replay the immutable N83 external-XCNF controls and raw solver receipts."""
import gzip
import json
import tempfile
from pathlib import Path

from verify import main as verify_model, sha256

HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / 'evidence'
DOMAINS = (HERE.parent / 'koblitz_n83_factor_base_sweep_20261008/verification/'
           'known-witness-ordered-sat-20261014/domains.json')
XCNF_SHA256 = {
    'instance': 'c2cc36eee7a72dbc583a217a0d42d02c74d5d3fde241536941b6614fc565b81e',
    'pinned_choices': '0dd57244e4ae56abd221dfc4b61513ef0bd01ba4e3e676bdfd68facf73650def',
    'pinned_sorted': '8569c449f00b0aab6e1b7457ec1383e663e1ae3260c456c56b47dd032dd7a4e5',
    'selector_only': 'e70ba46885f35a8b0088e3f5cc15bc626e032ff7fe821580a46ded945cca916d',
    'phase_only': '89348c5797536847d558596c13144262ae5bce010b19c3dc84c223226ad52379',
}
RUNS = {
    'cms_180s_5k': ('instance', 5000, 'UNKNOWN_solver_cap', 'INDETERMINATE', 15),
    'cms_180s_100k': ('instance', 100000, 'UNKNOWN_solver_cap', 'INDETERMINATE', 15),
    'cms_180s_1m': ('instance', 1000000, 'UNKNOWN_solver_cap', 'INDETERMINATE', 15),
    'cms_pinned_30s_5k': ('pinned_choices', 5000, 'UNSAT_unverified', 'UNSATISFIABLE', 20),
    'cms_pinned_sorted_30s_5k': ('pinned_sorted', 5000, 'SAT_unverified', 'SATISFIABLE', 10),
    'cms_selector_only_5k': ('selector_only', 5000, 'UNKNOWN_solver_cap', 'INDETERMINATE', 15),
    'cms_selector_only_100k': ('selector_only', 100000, 'UNKNOWN_solver_cap', 'INDETERMINATE', 15),
    'cms_phase_only_5k': ('phase_only', 5000, 'SAT_unverified', 'SATISFIABLE', 10),
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def main():
    with tempfile.TemporaryDirectory(prefix='n83-xcnf-audit-') as temporary:
        files = {}
        for name, expected in XCNF_SHA256.items():
            archive = EVIDENCE / f'{name}.xcnf.gz'
            path = Path(temporary) / f'{name}.xcnf'
            with gzip.open(archive, 'rb') as source, path.open('wb') as output:
                for chunk in iter(lambda: source.read(1024 * 1024), b''):
                    output.write(chunk)
            require(sha256(path) == expected, f'{name}: XCNF SHA-256 differs')
            files[name] = path
        exports = {name: json.loads((EVIDENCE / filename).read_text()) for name, filename in
                   [('instance', 'export.json'), ('pinned_choices', 'pinned_export.json'),
                    ('pinned_sorted', 'pinned_sorted_export.json'),
                    ('selector_only', 'selector_only_export.json'),
                    ('phase_only', 'phase_only_export.json')]}
        for name, receipt in exports.items():
            require(receipt['xcnf_bytes'] == files[name].stat().st_size,
                    f'{name}: export byte count differs')
        require(exports['pinned_choices']['parent_xcnf_sha256'] == XCNF_SHA256['instance'] and
                exports['pinned_sorted']['parent_xcnf_sha256'] == XCNF_SHA256['instance'] and
                exports['pinned_sorted']['added_choice_unit_clauses'] == 40 and
                exports['selector_only']['parent_xcnf_sha256'] == XCNF_SHA256['instance'] and
                exports['selector_only']['added_choice_unit_clauses'] == 5 and
                exports['phase_only']['parent_xcnf_sha256'] == XCNF_SHA256['instance'] and
                exports['phase_only']['added_choice_unit_clauses'] == 35,
                'choice pins or parent differ')
        stdout_files = {}
        for name, (formula, cap, status, solver_status, code) in RUNS.items():
            folder = EVIDENCE / name
            receipt = json.loads((folder / 'run.json').read_text())
            if status == 'SAT_unverified':
                stdout = Path(temporary) / f'{name}.stdout'
                with gzip.open(folder / 'solver.stdout.gz', 'rb') as source, stdout.open('wb') as output:
                    for chunk in iter(lambda: source.read(1024 * 1024), b''):
                        output.write(chunk)
            else:
                stdout = folder / 'solver.stdout'
            stdout_files[name] = stdout
            stderr = folder / 'solver.stderr'
            require(receipt['xcnf_sha256'] == XCNF_SHA256[formula] and
                    receipt['xcnf_bytes'] == files[formula].stat().st_size and
                    receipt['cap_conflicts'] == cap and receipt['status'] == status and
                    receipt['solver_status'] == solver_status and receipt['exit_code'] == code and
                    receipt['stdout_sha256'] == sha256(stdout) and
                    receipt['stderr_sha256'] == sha256(stderr) and
                    receipt['candidate_id'] is None and
                    receipt['selected_best_total_runtime'] is None,
                    f'{name}: raw run receipt differs')
            lines = stdout.read_text().splitlines()
            require(sum(line.startswith('s ') for line in lines) == 1,
                    f'{name}: solver status count differs')
            if formula not in ('pinned_sorted', 'phase_only'):
                require(not any(line.startswith('v ') for line in lines),
                        f'{name}: unexpected assignment')
        require(verify_model(files['pinned_sorted'],
                             stdout_files['cms_pinned_sorted_30s_5k'],
                             DOMAINS, EVIDENCE / 'pinned_sorted_export.json') == 0,
                'independent model replay failed')
        require(verify_model(files['phase_only'],
                             stdout_files['cms_phase_only_5k'],
                             DOMAINS, EVIDENCE / 'phase_only_export.json') == 0,
                'independent phase-only model replay failed')
    print('PASS_ARCHIVED_XCNF_FIRST_ATTEMPTS_AND_CURVE_REPLAY')


if __name__ == '__main__':
    main()
