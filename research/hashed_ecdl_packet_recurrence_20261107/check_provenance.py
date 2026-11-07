#!/usr/bin/env python3
"""Check source metadata and locators only; performs no research computation."""
import hashlib
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parent
repo = root.parents[1]
sources = json.loads((root / 'sources.json').read_text())
recurrence = json.loads((root / 'recurrence-map.json').read_text())

def blob(revision, path):
    return subprocess.check_output(['git', '-C', str(repo), 'show', revision + ':' + path])

for record in sources['sources']:
    data = blob(record['revision'], record['path'])
    assert len(data) == record['bytes'], record['path']
    assert hashlib.sha256(data).hexdigest() == record['sha256'], record['path']
    actual = subprocess.check_output(['git', '-C', str(repo), 'rev-parse',
                                      record['revision'] + ':' + record['path']], text=True).strip()
    assert actual == record['git_blob'], record['path']
for site in recurrence['anchors']:
    lines = blob(recurrence['source_revision'], site['path']).decode().splitlines()
    assert lines[site['line'] - 1].strip() == site['anchor'], site['id']
for item in sources['current_native_comparisons']:
    equal = blob(sources['source_revision'], item['path']) == blob(item['current_main'], item['path'])
    assert equal == item['byte_equal'], item['path']
print(f"Metadata only: {len(sources['sources'])} source records; "
      f"{len(recurrence['anchors'])} anchors; "
      f"{len(sources['current_native_comparisons'])} pinned native comparisons consistent.")
print('No semantic refinement, theorem, complexity bound, PCC standing or scientific result checked.')
