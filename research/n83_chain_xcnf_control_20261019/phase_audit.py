#!/usr/bin/env python3
"""Replay the frozen N83 known-witness phase-prefix solver panel."""
import contextlib
import gzip
import io
import json
import tempfile
from pathlib import Path

from phase_prefix import main as generate_phase_prefix
from verify import main as verify_model, sha256

HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / 'evidence'
DOMAINS = (HERE.parent / 'koblitz_n83_factor_base_sweep_20261008/verification/'
           'known-witness-ordered-sat-20261014/domains.json')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def unzip(source, output):
    with gzip.open(source, 'rb') as packed, Path(output).open('wb') as stream:
        for chunk in iter(lambda: packed.read(1024 * 1024), b''):
            stream.write(chunk)


def main():
    protocol = json.loads((HERE / 'phase_protocol.json').read_text())
    extension = json.loads((HERE / 'phase_extension.json').read_text())
    require(protocol['status'] == 'FROZEN_BEFORE_SOLVER_CALLS' and
            protocol['selected_best_total_runtime'] is None and
            sha256(HERE / 'phase_prefix.py') == protocol['generator_source_sha256'] and
            extension['status'] == 'FROZEN_BEFORE_SOLVER_CALL' and
            extension['phase_prefix'] == 3 and
            extension['conflict_cap'] == 1000000 and
            extension['xcnf_sha256'] == protocol['formulas'][2]['xcnf_sha256'],
            'phase protocol or generator differs')
    with tempfile.TemporaryDirectory(prefix='n83-phase-audit-') as temporary:
        temporary = Path(temporary)
        parent = temporary / 'instance.xcnf'
        unzip(EVIDENCE / 'instance.xcnf.gz', parent)
        require(sha256(parent) == protocol['parent_xcnf_sha256'],
                'parent XCNF bytes differ')
        exports = {}
        formulas = {}
        for row in protocol['formulas']:
            k = row['phase_prefix']
            formula = temporary / f'k{k}.xcnf'
            unzip(EVIDENCE / f'phase/k{k}.xcnf.gz', formula)
            require(sha256(formula) == row['xcnf_sha256'],
                    f'k{k}: XCNF digest differs')
            generated = temporary / f'k{k}_generated.xcnf'
            generated_export = temporary / f'k{k}_generated_export.json'
            with contextlib.redirect_stdout(io.StringIO()):
                generate_phase_prefix(parent, EVIDENCE / 'export.json',
                                      EVIDENCE / 'pinned_sorted_export.json', k,
                                      generated, generated_export)
            require(generated.read_bytes() == formula.read_bytes() and
                    generated_export.read_bytes() ==
                    (EVIDENCE / f'phase/k{k}_export.json').read_bytes(),
                    f'k{k}: source regeneration differs')
            exports[k] = EVIDENCE / f'phase/k{k}_export.json'
            formulas[k] = formula
        for k in (4, 3, 2, 1):
            for cap in ((5000, 100000, 1000000) if k == 3 else
                        (5000, 100000) if k < 4 else (5000,)):
                label = {5000: '5k', 100000: '100k', 1000000: '1m'}[cap]
                folder = EVIDENCE / f'phase/cms_k{k}_{label}'
                receipt = json.loads((folder / 'run.json').read_text())
                sat = k == 4 and cap == 5000
                if sat:
                    stdout = temporary / 'k4_solver.stdout'
                    unzip(folder / 'solver.stdout.gz', stdout)
                else:
                    stdout = folder / 'solver.stdout'
                stderr = folder / 'solver.stderr'
                require(receipt['xcnf_sha256'] == sha256(formulas[k]) and
                        receipt['cap_conflicts'] == cap and
                        receipt['cap_seconds'] == (180 if cap == 1000000 else 60) and
                        receipt['status'] == ('SAT_unverified' if sat else 'UNKNOWN_solver_cap') and
                        receipt['solver_status'] == ('SATISFIABLE' if sat else 'INDETERMINATE') and
                        receipt['exit_code'] == (10 if sat else 15) and
                        receipt['stdout_sha256'] == sha256(stdout) and
                        receipt['stderr_sha256'] == sha256(stderr) and
                        receipt['candidate_id'] is None and
                        receipt['selected_best_total_runtime'] is None,
                        f'k{k}/{label}: raw run receipt differs')
                if not sat:
                    require(stdout.read_text().splitlines() == ['s INDETERMINATE'],
                            f'k{k}/{label}: unexpected model')
        require(verify_model(formulas[4], temporary / 'k4_solver.stdout',
                             DOMAINS, exports[4]) == 0,
                'four-phase model replay failed')
    print('PASS_FROZEN_PHASE_PREFIX_PANEL_AND_CURVE_REPLAY')


if __name__ == '__main__':
    main()
