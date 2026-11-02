"""Prepare hashes and enumerate metadata; does not freeze or launch a panel."""
from pathlib import Path
import hashlib
import json
from schedule import expanded

D = Path(__file__).resolve().parent
ROOT = D.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(name, value):
    (D / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def main():
    if not __debug__:
        raise SystemExit('Assertions must be enabled')
    assert not (D / 'runs').exists(), 'Metadata preparation is predata only'
    assert not (D / 'protocol.json').exists(), 'Do not modify a frozen panel'
    plan = json.loads((D / 'plan.json').read_text())
    assert sha(D / 'plan.json') == sha(ROOT / 'companion-schur-validation/plan.json')
    table = expanded(plan)
    assert len(table) == 642
    for first, second in zip(table[:321], table[321:]):
        assert {k: v for k, v in first.items() if k != 'arm'} == {
            k: v for k, v in second.items() if k != 'arm'}
    sources = [
        'companion-schur-fused-validation/' + name for name in
        ['check.py', 'analyze.py', 'audit.py', 'lift_checks.py', 'schedule.py', 'prepare.py', 'PREPARATION.md']]
    sources += ['companion-schur-fused/adapter.py', 'companion-schur-fused/certificates.py',
                'companion-schur-lift/lift.py', 'companion-schur/schur.py',
                'companion-blocks/operators.py', 'structured-block-kernel/kernel.py',
                'structured-block-kernel/features.py', 'companion-fused/multiply.py',
                'companion-fused/reference.py', 'companion-blocks-validation/check.py',
                'companion-schur-validation/schur_reference.py',
                'tools/experiment_protocol.py', 'schemas/experiment-protocol.v1.json',
                'schemas/experiment-export.v1.json']
    inputs = ['companion-schur-fused-validation/plan.json',
              'direct-companion-validation/runs/cases.jsonl']
    write('source-lock.json', {p: sha(ROOT / p) for p in sources})
    write('input-lock.json', {p: sha(ROOT / p) for p in inputs})
    report = dict(status='metadata_prepared_not_frozen', cells=len(table),
                  instances_per_arm=321, arms=['candidate', 'aa'],
                  plan_sha256=sha(D / 'plan.json'),
                  expanded_schedule_sha256=hashlib.sha256(json.dumps(table, sort_keys=True).encode()).hexdigest(),
                  source_lock_entries=len(sources), input_lock_entries=len(inputs),
                  source_lock_sha256=sha(D / 'source-lock.json'),
                  input_lock_sha256=sha(D / 'input-lock.json'),
                  numerical_trials=0, protocol_frozen=False,
                  conditional_cases='Only reference rank determines valid/obstructed and normalized branches; all contexts retained')
    write('seed-preparation.json', report)
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
