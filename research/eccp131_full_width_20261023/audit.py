#!/usr/bin/env python3
"""Cross-check retained receipts, raw GP output, replay JSON, and support bound."""
import hashlib
import json
import re
from pathlib import Path

here = Path(__file__).parent
runs = here/'runs'
raw = (runs/'search.stdout').read_bytes()
lines = raw.decode().splitlines()
orders = [(int(m[1]),int(m[2])) for line in lines
          if (m := re.fullmatch(r'ORDER (\d+) (\d+)',line))]
replay = json.loads((runs/'replay.json').read_text())
search_receipt = json.loads((runs/'search.receipt.json').read_text())
replay_receipt = json.loads((runs/'replay.receipt.json').read_text())
gp_receipt = json.loads((runs/'planted_replay.receipt.json').read_text())
identity_receipt = json.loads((runs/'identity_check.receipt.json').read_text())
assert lines[-1] == 'FOUND 1'
assert len(orders) == 301 and [b for b,_ in orders] == list(range(1,302))
assert [(b,n) for b,n in orders if n%256 == 0] == [orders[277],orders[300]]
assert replay['input_sha256'] == hashlib.sha256(raw).hexdigest()
assert int(replay['auxiliary']['b'],16) == 301
assert len(replay['tower']['x']) == len(set(replay['tower']['x'])) == 256
assert len(replay['tower']['steps']) == 8 and replay['tower']['lifted_columns'] == 115
screen = replay['screen']
assert len(screen['planted_controls']) == 8
assert screen['distinct_pair_points'] == 26450
assert not screen['setup_relations'] and not screen['target_relations']
assert screen['setup_rank'] == screen['combined_rank'] == 0
assert (runs/'planted_replay.stdout').read_text().strip() == 'PLANTED_REPLAY_OK 8'
assert (runs/'identity_check.stdout').read_text().strip() == 'IDENTITY_OK'
for receipt,limit in ((search_receipt,120),(replay_receipt,60),(gp_receipt,30),(identity_receipt,30)):
    assert receipt['exit_code'] == 0 and receipt['stop_reason'] is None
    assert receipt['wall_seconds'] < limit and receipt['max_rss_kib_sampled'] < 2048*1024
assert 230*(26450+1) == 6083730
print('AUDIT_OK exact curve and target; 301 auxiliary orders; 256 tower x; 8 maps; 115 columns; 8 GP-replayed planted; 0 natural rows; 4 watchdog receipts')
