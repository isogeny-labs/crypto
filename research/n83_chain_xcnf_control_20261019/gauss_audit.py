#!/usr/bin/env python3
"""Audit raw N83 external-Gauss A/B/A control receipts."""
import gzip
import hashlib
import json
import subprocess
import tempfile
from pathlib import Path

from verify import sha256

HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / 'evidence'
FROZEN_COMMIT = 'acf525e936a7fc87b428eb9a1800152536e1e144'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def main():
    protocol = json.loads((HERE / 'gauss_protocol.json').read_text())
    source = subprocess.run(
        ['git', 'show', f'{FROZEN_COMMIT}:research/n83_chain_xcnf_control_20261019/run_external.py'],
        cwd=HERE.parent.parent, check=True, capture_output=True,
    ).stdout
    require(protocol['status'] == 'FROZEN_BEFORE_100K_CALLS' and
            hashlib.sha256(source).hexdigest() == protocol['runner_source_sha256'] and
            protocol['selected_best_total_runtime'] is None,
            'frozen runner or protocol differs')
    with tempfile.TemporaryDirectory(prefix='n83-gauss-audit-') as temporary:
        xcnf = Path(temporary) / 'instance.xcnf'
        with gzip.open(EVIDENCE / 'instance.xcnf.gz', 'rb') as packed, xcnf.open('wb') as output:
            for chunk in iter(lambda: packed.read(1024 * 1024), b''):
                output.write(chunk)
        require(sha256(xcnf) == protocol['xcnf_sha256'], 'input XCNF differs')
        times = {}
        for call in protocol['calls']:
            label = call['label']
            folder = EVIDENCE / 'gauss' / f'gauss_{label}'
            receipt = json.loads((folder / 'run.json').read_text())
            stdout = folder / 'solver.stdout'
            stderr = folder / 'solver.stderr'
            require(receipt['xcnf_sha256'] == protocol['xcnf_sha256'] and
                    receipt['solver_sha256'] == protocol['solver_binary_sha256'] and
                    receipt['gauss_profile'] == call['gauss_profile'] and
                    all(option in receipt['command'] for option in call['options']) and
                    receipt['cap_conflicts'] == 100000 and receipt['cap_seconds'] == 60 and
                    receipt['status'] == 'UNKNOWN_solver_cap' and
                    receipt['solver_status'] == 'INDETERMINATE' and
                    receipt['exit_code'] == 15 and
                    receipt['stdout_sha256'] == sha256(stdout) and
                    receipt['stderr_sha256'] == sha256(stderr) and
                    stdout.read_text().splitlines() == ['s INDETERMINATE'] and
                    receipt['candidate_id'] is None and
                    receipt['selected_best_total_runtime'] is None,
                    f'{label}: raw run receipt differs')
            times[label] = receipt['elapsed_wall_ms']
    print(json.dumps({'status': 'PASS_GAUSS_PANEL_BOUNDED_RECEIPTS',
                      'wall_ms': times, 'selected_best_total_runtime': None}, sort_keys=True))


if __name__ == '__main__':
    main()
