#!/usr/bin/env python3
"""Join a partial worker report to independently checked capture and row identities.

Caller-provided identities remain a dependency; this is not native cell admission.
Only the final acknowledged snapshot supplies exported row labels.
"""
import argparse
import json
from pathlib import Path
from check_capture import require, parse
from semantic_reader import verify_and_extract
from replay_capture import replay_snapshots, ReplayLimits

FALSE_FIELDS=('all_patterns_covered','solver_completed','ecdlp_pipeline_completed',
              'scalar_recovered_without_witness','full_span_search',
              'independent_original_source_replay','s3_custody_verified')
NULL_FIELDS=('first_fall_degree','last_fall_degree','degree_of_regularity','first_stop')

def join_report(root,binding,sources,report,pattern,limits=ReplayLimits()):
    require(type(pattern) is int and 0<=pattern<2**32,'invalid trusted pattern')
    require(type(report) is dict,'report object required')
    for key,value in {'adapter_id':'original-owned-boolean-search-v1',
                      'strategy':'original-owned-boolean-search-v1',
                      'schema':'crypto.ml-dataset.owned-search-report/v1',
                      'status':'partial','scope':'single-pattern-retained-search',
                      'accounting_kind':'original-ledger-reservations-not-retired-instructions'}.items():
        require(report.get(key)==value,'report identity or scope mismatch: '+key)
    for key in FALSE_FIELDS:require(report.get(key) is False,'report overclaim or missing field: '+key)
    for key in NULL_FIELDS:require(key in report and report[key] is None,'report unsupported label: '+key)
    require(report.get('capture_snapshot_complete') is True,'report snapshot incomplete')
    require(type(report.get('pattern')) is int and report['pattern']==pattern,'report pattern mismatch')
    for field,key in [('public_sha256_bytes','public_sha256'),('factor_base_sha256_bytes','factor_sha256'),
                      ('launch_sha256_bytes','launch_sha256'),('binding_sha256_bytes','binding_sha256')]:
        raw=report.get(field)
        require(type(raw) is list and len(raw)==32 and all(type(x) is int and 0<=x<=255 for x in raw),'invalid source bytes')
        require(bytes(raw).hex()==sources.get(key),'report source identity mismatch')
    # Full committed-prefix validation completes before any join or algebraic label.
    extracted=verify_and_extract(root,binding,sources)
    snapshots=extracted['snapshots']
    require(bool(snapshots),'no retained snapshot')
    require(all(s['binding']['pattern']==pattern for s in snapshots),'mixed report patterns')
    final=snapshots[-1];meta=final['binding'];ack=meta['ack']
    require(ack['event_count']==extracted['capture']['event_count'],'report snapshot is not terminal committed head')
    for key,expected in [('capture_snapshot_index',meta['snapshot']),('capture_chunk_count',meta['chunks']),
                         ('capture_head_event_count',ack['event_count']),('retained_snapshot_count',len(snapshots))]:
        require(type(report.get(key)) is int and report[key]==expected,'report snapshot mismatch: '+key)
    require(report.get('capture_head_sha256')==ack['last_event_sha256'],'report capture head mismatch')
    result=replay_snapshots([final],limits)
    for row in result['rows']:
        row['snapshot']=meta['snapshot']
    for lineage in result['lineages']:
        lineage['snapshot']=meta['snapshot']
    return {'schema':'owned-search-restricted-row-record/v1','visibility':'restricted',
            'status':result['status'],'pattern':pattern,'snapshot':meta['snapshot'],
            'snapshot_sha256':meta['sha256'],'capture_ack':ack,
            'report_capture_join_verified':True,'source_identity_matches_supplied_hashes':True,
            'cell_identity_admitted':False,'public_features':None,
            'capture':extracted['capture'],'retention':extracted['retention'],
            'row_replay':result,'original_descent_verified':False,'solver_completed':False,
            'first_fall_degree':None,'last_fall_degree':None,'degree_of_regularity':None,
            'producer_counters_verified':False,'producer_lift_classifications_verified':False}

def read_bounded(path,cap):
    require(not path.is_symlink() and path.is_file(),'metadata must be regular file')
    with path.open('rb') as f:raw=f.read(cap+1)
    require(len(raw)<=cap,'metadata size limit')
    return parse(raw)

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('live',type=Path);p.add_argument('--binding',required=True)
    p.add_argument('--sources',required=True,type=Path);p.add_argument('--report',required=True,type=Path)
    p.add_argument('--pattern',required=True,type=int)
    p.add_argument('--work-limit',type=int,default=ReplayLimits.work)
    p.add_argument('--row-limit',type=int,default=ReplayLimits.rows)
    a=p.parse_args()
    out=join_report(a.live,a.binding,read_bounded(a.sources,65536),read_bounded(a.report,8192),a.pattern,
                    ReplayLimits(work=a.work_limit,rows=a.row_limit))
    print(json.dumps(out,sort_keys=True))
    return 0 if out['status']=='verified_selected_rows' else 2
if __name__=='__main__':raise SystemExit(main())
