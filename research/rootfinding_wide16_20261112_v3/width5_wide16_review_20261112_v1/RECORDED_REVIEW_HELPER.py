"""Read recorded evidence only; no solver or mathematical-oracle execution."""
from pathlib import Path
import json, gzip, hashlib, datetime, statistics

root = Path('/workspace/generalized_roots')
review_dir = root / 'research/width5_wide16_review_20261112_v1'
summaries, pins = [], {}
for mode in ['controls', 'profile']:
    eid = 'width5-wide16-' + mode + '-20261112-v1'
    attempt = root / 'lab/ledger/attempts' / eid / '0001'
    evidence = json.loads((attempt / 'scientific_result.json').read_text())['evidence']
    raw = root / 'research/width5_wide16_20261112_v1' / ('raw-' + mode + '.json.gz')
    raw_hash = hashlib.sha256(raw.read_bytes()).hexdigest()
    copied_raw = attempt / ('artifact-000-raw-' + mode + '.json.gz')
    assert raw_hash == evidence['raw_sha256'] == hashlib.sha256(copied_raw.read_bytes()).hexdigest()
    stages = json.loads(gzip.decompress(raw.read_bytes()))
    arms = evidence['arms']
    assert len(arms) == (3 if mode == 'controls' else 9)
    unions = [tuple(tuple(v) for v in arm['conditional_ordered_root_tuples']) for arm in arms]
    assert all(u == unions[0] for u in unions)
    fields = ['P-192', 'P-224', 'P-256', 'P-384', 'P-521'] if mode == 'profile' else ['tiny-' + str(p) for p in [5,7,11,13,17]]
    width = 16 if mode == 'profile' else 9
    for arm, union in zip(arms, unions):
        assert arm['complete_conditional_block']
        assert arm['conditional_ordered_root_count_exact'] == len(union) == len(set(union))
        assert set(arm['masks']) == set(fields)
        assert all(len(mask) == width for mask in arm['masks'].values())
        assert arm['solved_fibers'] + arm['excluded_fibers'] == 5 * width
    recovered = batches = mask_rows = vectors = checks = 0
    for stage in stages:
        name = stage.get('stage', '')
        if '-recover-batch-' in name:
            output = json.loads(stage['stdout'])
            assert output['count'] == len(output['cases']) <= 40
            ids = {row['id'] for row in output['cases']}
            assert len(ids) == output['count']
            assert all(row['complete'] and row['enumeration_complete'] for row in output['cases'])
            recovered += output['count']
            batches += 1
            tag, number = name.rsplit('-recover-batch-', 1)
            exported = next(row for row in stages if row.get('stage') == tag + '-root-input-' + number)
            assert set(exported['expected_ids']) == ids
        elif name.endswith('-discover'):
            output = json.loads(stage['stdout'])
            assert len(output) == 5
            for row in output:
                assert row['lll_complete']
                mask_rows += 1
                vectors += len(row['candidates'])
                checks += sum(len(v['curved_mask']) for v in row['candidates'])
    process = json.loads((attempt / 'process.json').read_text())
    native_rss = [row['peak_process_rss_kib'] for row in stages if any(part in row.get('stage', '') for part in ['-discover', '-recover-batch-'])]
    summary = dict(mode=mode, root_union_size=len(unions[0]), all_arm_unions_identical=True,
                   arms=len(arms), recorded_complete_native_fibers=recovered,
                   native_recovery_batches=batches, mask_rows=mask_rows,
                   stored_mask_vectors=vectors, candidate_u_checks=checks,
                   wall_seconds=process['wall_seconds'],
                   whole_process_peak_rss_kib=process['peak_process_rss_kib'],
                   native_process_rss_range_kib=[min(native_rss), max(native_rss)],
                   raw_sha256=raw_hash)
    if mode == 'controls':
        assert evidence['tiny_direct_checks'] == 1215 and len(unions[0]) == 38
        assert evidence['batch_controls']['real_batch_sizes'] == [40,5]
        assert len(evidence['batch_controls']['faults_rejected']) == 3
        assert evidence['batch_controls']['empty_batch_complete']
        summary['batch_controls'] = evidence['batch_controls']
    else:
        assert not unions[0]
        for arm in arms:
            assert arm['solved_fibers'] == (80 if arm['arm'] == 'all' else 0)
            assert arm['recovery_batch_sizes'] == ([40,40] if arm['arm'] == 'all' else [0])
        medians = {name: statistics.median(arm['paid_component_seconds'] for arm in arms if arm['arm'] == name) for name in ['all','seven','five']}
        assert medians == evidence['median_component_seconds']
        summary.update(medians=medians, observed_all_over_five=medians['all']/medians['five'],
                       observed_seven_over_five=medians['seven']/medians['five'],
                       empty_conditional_blocks_only=True)
    summaries.append(summary)
    for path in [raw, attempt/'scientific_result.json', attempt/'process.json', copied_raw]:
        pins[str(path.relative_to(root))] = hashlib.sha256(path.read_bytes()).hexdigest()

review = dict(schema='independent-recorded-evidence-review/v1',
              created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              decision='PASSED_CONDITIONAL_EMPTY_BLOCK_COMPONENT_ONLY',
              review_operation='Recorded hashes, exact row-ID/batch/tuple/mask coverage and timing medians only; no scientific algorithm rerun',
              results=summaries, qualified_speedup=None, global_original_root_complete=False,
              genuine_F4_F5_F6_measured=False,
              interpretation='Observed 11.555x versus all-h and 3.529x versus 7D measure faster certification of finite EMPTY conditional blocks; global original roots and genuine GB gate remain unresolved.')
result_path = review_dir/'RESULT_REVIEW.json'
with result_path.open('x') as stream:
    json.dump(review, stream, indent=2)
    stream.write('\n')
pins[str(result_path.relative_to(root))] = hashlib.sha256(result_path.read_bytes()).hexdigest()
pins[str(Path(__file__).relative_to(root))] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
with (review_dir/'RESULT_PINS.json').open('x') as stream:
    json.dump(dict(schema='recorded-result-pins/v1', sha256=pins), stream, indent=2)
    stream.write('\n')
print(json.dumps(review))
