"""Read-only archive and derived-result verification; never launches a worker."""
from pathlib import Path
import hashlib,json,tarfile
p=Path(__file__).resolve().parent
manifest=json.loads((p/'raw-manifest.json').read_text())
with tarfile.open(p/'native-receipts.tar.gz') as archive:
    assert sorted(archive.getnames())==sorted(r['path'] for r in manifest['files'])
    for r in manifest['files']:
        b=archive.extractfile(r['path']).read()
        assert len(b)==r['bytes'] and hashlib.sha256(b).hexdigest()==r['sha256']
    native=json.load(archive.extractfile('worker/receipt.json'))
    debit=json.load(archive.extractfile('campaign-debit.json'))
result=json.loads((p/'result.json').read_text())
assert native['status']==result['status']=='failed'
assert native['elapsed_seconds']==result['recorded_elapsed_seconds']
assert native['files']==result['recorded_files']
assert native['compiler_invoked']==result['compiler_invoked']==False
assert result['campaign_accounting']==debit
assert debit['cumulative_charged_seconds']==debit['prior_charged_seconds']+debit['this_attempt_conservative_charge_seconds']
assert debit['remaining_worker_seconds']==debit['campaign_total_seconds']-debit['cumulative_charged_seconds']
assert not result['historical_terminal_custody_verified']
print('Verified 11 archived members and derived receipt/accounting fields; no trial replay.')
