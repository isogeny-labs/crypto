"""Remove only this terminal qualification's disposable claims after readback."""
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parent
state = json.loads((root / 'final-state.json').read_text())
assert state['phase'] == 'failed' and state['restores'] == 1
assert (root / 'verification.json').is_file()
assert len((root / 'fresh-sha256.txt').read_text().splitlines()) == 18
k = ['kubectl', '--kubeconfig', '/home/ec2-user/ops/eks-isolab/kubeconfig']
namespace = 'isolab-recovery-system'
pods = json.loads(subprocess.check_output(k + ['get', 'pods', '-n', namespace, '-o', 'json']))
claims = [('ecbench-periodic-20261022-b-a0', 'ab68d030-7bc0-49af-99fe-225cc3bf03a8'),
          ('ecbench-periodic-20261022-b-a1', 'cc8571f9-9308-4c4a-a63d-54d93b51d42b')]
for name, uid in claims:
    obj = json.loads(subprocess.check_output(k + ['get', 'pvc', name, '-n', namespace, '-o', 'json']))
    assert obj['metadata']['uid'] == uid
    assert obj['metadata']['labels']['isolab.dev/recovery-run'] == state['request']['run_name']
    assert not any(v.get('persistentVolumeClaim', {}).get('claimName') == name
                   for p in pods['items'] for v in p['spec'].get('volumes', []))
    (root / (name + '-before-cleanup.json')).write_text(json.dumps(obj, indent=2) + '\n')
    options = root / (name + '-delete-options.json')
    options.write_text(json.dumps({'apiVersion': 'v1', 'kind': 'DeleteOptions',
                                  'preconditions': {'uid': uid}}))
    result = subprocess.check_output(k + ['delete', '--raw',
        f'/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}', '-f', str(options)])
    (root / (name + '-cleanup.json')).write_bytes(result)
print('Deleted both owned PVCs with UID preconditions; committed snapshots retained.')
