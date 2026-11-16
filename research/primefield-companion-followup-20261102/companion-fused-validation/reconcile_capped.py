"""Read-only inspection of capped raw rows; never executes a numerical kernel.

Writes a new terminal custody record, not the full-panel export or analyzer.
"""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json

D = Path(__file__).resolve().parent
ROOT = D.parent


def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda: f.read(1048576), b''):
            h.update(block)
    return h.hexdigest()


def main():
    assert __debug__
    output = D / 'terminal-reconciliation.json'
    assert not output.exists(), 'Terminal record is write-once'
    launch = json.loads((D / 'launch.json').read_text())
    follower = json.loads((D / 'postprocess-launch.json').read_text())
    assert not (Path('/proc') / str(launch['producer_pid'])).exists()
    # The known follower handle is independently checked, not inferred from state.
    assert not (Path('/proc') / str(follower['postprocessor_pid'])).exists()
    codes = {n: int((D / n).read_text()) for n in ['exit-code.txt', 'postprocessor-exit.txt']}
    assert set(codes.values()) == {124}
    plan = json.loads((D / 'plan.json').read_text())
    assert len(plan) == 849
    raw = D / 'runs/cases.jsonl'
    complete_hash = hashlib.sha256()
    complete_bytes = 0
    trailing = 0
    seen = {}
    arms = {a: dict(rows=0, products=0, archived_products=0, carry_products=0,
                    mutations_rejected=0, equal_wrap_rejections=0, radix_guard_rejections=0)
            for a in ['candidate', 'aa']}
    rows = []
    matched = 0
    with raw.open('rb') as f:
        for encoded in f:
            if not encoded.endswith(b'\n'):
                trailing = len(encoded)
                assert not f.read(1)
                break
            z = json.loads(encoded)
            ordinal = len(rows)
            arm = ['candidate', 'aa'][ordinal // 849]
            iid = ordinal % 849
            assert z['arm'] == arm and z['instance_id'] == str(iid)
            assert z['seed'] == plan[iid]['seed'] and z['payload']['plan'] == plan[iid]
            assert z['replicate'] == 0 and z['status'] == 'verified'
            payload = z['payload']
            h = hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()
            assert h == z['aa_sha256']
            if arm == 'candidate':
                seen[iid] = h
            else:
                assert seen[iid] == h
                matched += 1
            tests = payload['tests']
            count = len(tests)
            assert z['products'] == count and z['mutations_rejected'] == 4 * count
            assert z['equal_wrap_rejections'] == 3 * count
            assert z['radix_guard_rejections'] == (2 * count if z['kind'] == 'carry' else 0)
            for t in tests:
                c = t['counts']
                assert c['coefficient_reductions'] == t['n']
                assert c['integer_multiplications'] == 4 * t['s'] + 8
                assert c['operand_bitlength_product_sum'] == sum(b['a_bits'] * b['b_bits'] for b in t['blocks'])
                assert c['product_result_bitlength_sum'] == sum(b['product_bits'] for b in t['blocks'])
            a = arms[arm]
            a['rows'] += 1
            a['products'] += count
            a[z['kind'] + '_products'] += count
            for key in ['mutations_rejected', 'equal_wrap_rejections', 'radix_guard_rejections']:
                a[key] += z[key]
            rows.append(dict(arm=arm, instance_id=str(iid), kind=z['kind'],
                             products=count, offset=complete_bytes, bytes=len(encoded),
                             sha256=hashlib.sha256(encoded).hexdigest(),
                             started_at=z['started_at'], finished_at=z['finished_at']))
            complete_hash.update(encoded)
            complete_bytes += len(encoded)
    locks = {}
    for name in ['source-lock.json', 'input-lock.json']:
        data = json.loads((D / name).read_text())
        for path, expected in data.items():
            assert sha(ROOT / path) == expected, path
        locks[name] = len(data)
    result = dict(status='capped_incomplete_no_restart', observed_at=datetime.now(timezone.utc).isoformat(),
                  exit_codes=codes, producer_and_follower_absent=True, restarted=False,
                  original_deadline=launch['deadline_epoch'], planned_rows=1698,
                  completed_rows=len(rows), without_completion_receipt=1698-len(rows),
                  attempted_rows=None, attempted_scope='Completed rows known; unfinished attempt lacks a start receipt',
                  by_arm=arms, matched_aa_pairs=matched, full_aa_complete=matched == 849,
                  full_panel_export_generated=False, locks_verified=locks,
                  raw_bytes=raw.stat().st_size, raw_sha256=sha(raw),
                  complete_prefix_bytes=complete_bytes, complete_prefix_sha256=complete_hash.hexdigest(),
                  partial_trailing_bytes=trailing, rows=rows,
                  verification_scope='Recomputed row order, payload hashes, recorded fixed counts and descriptor sums; no numerical kernel replay')
    assert result['raw_bytes'] == complete_bytes + trailing
    output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({k: v for k, v in result.items() if k != 'rows'}, indent=2))


if __name__ == '__main__':
    main()
