from pathlib import Path
import hashlib
import importlib.util
import json
import subprocess
import yaml

repo = Path.cwd()
base = Path('research/factor_base_chart_complements_20261019/cards')
card_path = base / 'CARD-chart-complements-20261019-1.yaml'
card = yaml.safe_load(card_path.read_text())
cli_argv = ['python3', 'tools/task_card.py', 'check-relay', str(card_path), '--report', str(base / 'executor-relay.txt')]
cli = subprocess.run(cli_argv, capture_output=True, text=True)
record = {'argv': cli_argv, 'exit_code': cli.returncode, 'stdout': cli.stdout, 'stderr': cli.stderr}
with (base / 'check-relay-cli.json').open('x') as stream:
    json.dump(record, stream, indent=2)
    stream.write('\n')

module_spec = importlib.util.spec_from_file_location('canonical_task_card', repo / 'tools/task_card.py')
module = importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(module)
check = module.check_relay(card, (base / 'executor-relay.txt').read_text(), repo)
receipt_path = Path(card['receipt'])
receipt = json.loads(receipt_path.read_text())
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
assert receipt['card_sha256'] == digest(card_path)
assert receipt['commit'] == card['commit']
assert all(digest(item['path']) == item['sha256'] for item in card['binds'])
assert len(receipt['commands']) == len(card['commands']) == 1
for specified, observed in zip(card['commands'], receipt['commands']):
    assert observed['argv'] == specified['argv']
    assert observed['cwd'] == specified.get('cwd', '.')
    assert observed['attempts'] == 1
    assert observed['exit_code'] in specified['success_exit_codes']
    assert not observed['timed_out'] and observed['succeeded']
    assert set(observed['outputs']) == set(specified['outputs'])
    assert all(digest(path) == sha for path, sha in observed['outputs'].items())
    for stream in ('stdout', 'stderr'):
        assert digest(specified[stream]) == observed[stream + '_sha256']
for path in set(receipt['dirty_after']) - set(receipt['dirty_before']):
    assert any(path.startswith(scope) for scope in card['write_scope'])
status = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=all'], text=True)
assert all(any(line[3:].startswith(scope) for scope in card['write_scope']) for line in status.splitlines())
assert check['relay_verbatim'] and not check['verify_findings']
print(json.dumps({'canonical_cli_exit_code': cli.returncode, 'canonical_function_check_relay': check,
                  'bindings_outputs_streams_argv_scope_verified': True,
                  'receipt_sha256': digest(receipt_path)}, indent=2))
