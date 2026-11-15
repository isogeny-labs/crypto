#!/usr/bin/env python3
"""Pin the eight-class collision check against the dated local LMFDB index."""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

P = Path(__file__).resolve().parent
helper = Path('/Volumes/data/workspaces/lmfdb-agent-query-20261019/query.py')
cache = Path('/Volumes/SSD990/llm/tmp/lmfdb-query-cache-20261025')
manifest = cache / 'av_fq_manifest.json'
target = P / 'lmfdb_snapshot.json'
if target.exists():
    raise SystemExit('refusing to overwrite immutable database receipt')
traces = (-26,-22,-14,-10,10,14,22,26)
polys = [f"'1 0 0 0 0 {-t} 0 0 0 0 243 '" for t in traces]
sql = ('SELECT label, poly_str, has_jacobian, is_simple, curve_count '
       'FROM av_fq WHERE g=5 AND q=3 AND poly_str IN (' + ', '.join(polys) + ') '
       'ORDER BY poly_str')
cmd = [sys.executable, str(helper), '--table', 'av_fq', '--sql', sql,
       '--dataset-dir', str(cache)]
run = subprocess.run(cmd, capture_output=True, text=True, timeout=180)
if run.returncode != 0:
    out = {'query': sql, 'exit_code': run.returncode, 'stderr': run.stderr,
           'stdout': run.stdout}
    target.write_text(json.dumps(out, indent=2)+'\n')
    raise SystemExit('local LMFDB query failed; receipt retained')
raw = json.loads(run.stdout)
rows = raw['rows']
assert len(rows) == 8
by_trace = {int(-int(row['poly_str'].split()[5])): row for row in rows}
assert set(by_trace) == set(traces)
assert all(row['is_simple'] is True for row in rows)
assert all(by_trace[t]['has_jacobian'] == (0 if abs(t) == 26 else 1)
           for t in traces)
metadata = raw['available_datasets']['av_fq']
assert metadata['row_count'] == 2945722
out = {'schema': 'ghs-odd5-lmfdb-snapshot/v1',
       'script_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
       'manifest_sha256': hashlib.sha256(manifest.read_bytes()).hexdigest(),
       'query': sql, 'helper': str(helper), 'helper_exit_code': run.returncode,
       'source': metadata['source'], 'source_ended_utc': metadata['ended_utc'],
       'source_row_count': metadata['row_count'], 'rows_by_trace':
       {str(t): by_trace[t] for t in traces}}
target.write_text(json.dumps(out, indent=2)+'\n')
print(json.dumps({'returned_rows': len(rows),
                  'unknown_jacobian_traces': [t for t in traces if by_trace[t]['has_jacobian'] == 0]}))
