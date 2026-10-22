#!/usr/bin/env python3
"""Audit the admitted K4300 IC1 identity against executed flags and receipts."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent/'k4300_identity'
ROOT=Path('/Volumes/SSD990/crypto/worktrees/n83-root4-frontier-20261018')
PILOT=HERE.parent/'k4300_16_pilot'


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()


def main():
    row=json.loads((HERE/'pilot_ic1_row.json').read_text())
    candidate_id=row['candidate_id']
    record=json.loads((HERE/f'candidate_{candidate_id}.json').read_text())
    source=json.loads((PILOT/'source.json').read_text())
    analysis=json.loads((PILOT/'analysis.json').read_text())
    summary=json.loads((PILOT/'stdout.jsonl').read_text())
    body={k:v for k,v in record.items() if k!='candidate_id'}
    prefix='IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h'
    if (candidate_id!=prefix+hashlib.sha256(canonical(body)).hexdigest()[:12]
            or record['candidate_id']!=candidate_id
            or row['candidate_manifest_sha256']!=sha(HERE/f'candidate_{candidate_id}.json')
            or row['analysis_sha256']!=sha(PILOT/'analysis.json')
            or record['implementation']['source_sha256']['ic']!=sha(ROOT/'examples/koblitz_orbit_dlp_fast.rs')
            or record['implementation']['source_sha256']['cargo_lock']!=sha(ROOT/'Cargo.lock')
            or record['factor_base']['actual_usable_points']!=analysis['actual_factor_base_points']
            or record['factor_base']['effective_columns']!=analysis['effective_columns']
            or record['factor_base']['ordered_base_dump_sha256']!=analysis['base_dump_sha256']
            or record['factor_base']['source_base_hash_blake3']!=analysis['base_hash']
            or record['point_decomposition']['root_slot']['allocated_bytes']!=summary['root_table_allocated_bytes']
            or record['point_decomposition']['root_bloom_filter']['allocated_bytes']!=summary['root_bloom_bytes']
            or record['point_decomposition']['root_bloom_filter']['hashes']!=summary['root_bloom_hashes']):
        raise ValueError('candidate hash, source or construction differs')
    flags=record['implementation']['algorithm_flags'];executed=source['algorithm_flags']
    mapping={'compact_state26':'KIC_STATE26','compact_state28':'KIC_STATE28',
             'dense_request':'KIC_PACKED_ROOT_TABLE_DENSE',
             'indexed4_fixed31_request':'KIC_INDEXED4_FIXED31',
             'indexed5_fixed30_request':'KIC_INDEXED5_FIXED30',
             'indexed5_id30_request':'KIC_INDEXED5_ID30',
             'indexed5_load75_request':'KIC_INDEXED5_LOAD75',
             'indexed_root4':'KIC_INDEXED_ROOT4','indexed_root5':'KIC_INDEXED_ROOT5',
             'load75_request':'KIC_PACKED_ROOT_TABLE_LOAD75',
             'packed_root_table':'KIC_PACKED_ROOT_TABLE',
             'root_batch':'KIC_INDEX_ROOT_BATCH','root_bloom':'KIC_ROOT_BLOOM',
             'root_only11':'KIC_ROOT_ONLY11','root_only21':'KIC_ROOT_ONLY21',
             'root_only22':'KIC_ROOT_ONLY22','sparse_canonical':'KIC_SPARSE_CANONICAL',
             'symmetric_s3_states':'KIC_SYMMETRIC_S3_STATES',
             'target_cancel':'KIC_TARGET_CANCEL'}
    for key,environment in mapping.items():
        if key=='symmetric_s3_states':environment='KIC_SYMMETRIC_S3'
        if flags[key]!=(executed[environment]=='1'):
            raise ValueError(f'algorithm flag differs: {key}')
    for key,environment in (('rank_threads','KIC_RANK_THREADS'),
                            ('target_threads','KIC_TARGET_THREADS')):
        if flags[key]!=int(executed[environment]):
            raise ValueError(f'worker count differs: {key}')
    if flags['root_bloom_profile']!=executed['KIC_ROOT_BLOOM_PROFILE']:
        raise ValueError('Bloom profile differs')
    checked=set(mapping.values())|{'KIC_RANK_THREADS','KIC_TARGET_THREADS',
                                  'KIC_ROOT_BLOOM_PROFILE','KIC_DUMP_BASE'}
    checked.discard('KIC_SYMMETRIC_S3_STATES');checked.add('KIC_SYMMETRIC_S3')
    if set(executed)!=checked:
        raise ValueError(f'unmapped executed flags: {set(executed)^checked}')
    result={'schema':'n83.k4300-candidate-audit/v1',
            'status':'PASS_EXACT_CANDIDATE_IDENTITY',
            'candidate_id':candidate_id,
            'candidate_manifest_sha256':sha(HERE/f'candidate_{candidate_id}.json'),
            'pilot_analysis_sha256':sha(PILOT/'analysis.json'),
            'executed_algorithm_flag_count':len(executed)-1,
            'base_dump_sha256':analysis['base_dump_sha256'],
            'root_table_entries':analysis['root_table_entries']}
    print(json.dumps(result,sort_keys=True))


if __name__=='__main__':main()
