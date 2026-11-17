#!/usr/bin/env python3
"""Replay committed Boolean basis identities relative to captured equations."""
import argparse
from dataclasses import dataclass, replace
import json
from pathlib import Path
from semantic_reader import verify_and_extract, inspect_selected
from check_capture import parse, require
from equation_lineage import check_lineage, Limits as LineageLimits
from polynomial_replay import verify_row, Limits as RowLimits

@dataclass(frozen=True)
class ReplayLimits:
    work: int = 5_000_000
    allocation_units: int = 2_000_000
    rows: int = 4096

def replay_snapshots(snapshots, limits=ReplayLimits(), *, wire_variables=None):
    for name,maximum in [('work',100_000_000),('allocation_units',20_000_000),('rows',65536)]:
        value=getattr(limits,name)
        require(type(value) is int and 0<=value<=maximum,'invalid replay limit')
    work=allocated=0;rows=[];lineages=[];terminal=None
    for snapshot_index,entry in enumerate(snapshots):
        state=entry['retained'];expansion=state.get('expansion')
        if type(expansion) is not dict and wire_variables is None:
            terminal='inconclusive';lineages.append({'snapshot':snapshot_index,'status':'inconclusive','reason':'no retained expansion'});break
        variables=expansion.get('variables') if type(expansion) is dict else wire_variables
        require(wire_variables is None or (type(wire_variables) is int and wire_variables==variables),'explicit variable count mismatch')
        continuation=state.get('continuation')
        lineage=check_lineage(state.get('normalized'),continuation,variables,limits=LineageLimits(work=max(0,limits.work-work)))
        work+=lineage['work'];lineages.append({'snapshot':snapshot_index,**lineage})
        if lineage['status'] not in ('verified_lineage','inconclusive_partial'):
            terminal='mismatch' if lineage['status']=='lineage_mismatch' else 'inconclusive';break
        for location in lineage['verified_nodes']:
            node=continuation['partitions'][location['partition']]['search']['nodes'][location['node']]
            native=node.get('native')
            if native is None:continue
            if type(native) is not dict or type(native.get('basis')) is not list:
                terminal='invalid';break
            # Committed basis is updated atomically by the actual scheduler.
            # Current/staged/scratch rows may be partial and are not verdict inputs.
            for row_index,row in enumerate(native['basis']):
                if len(rows)>=limits.rows or work>=limits.work or allocated>=limits.allocation_units:
                    terminal='inconclusive';break
                caps=replace(RowLimits(),work=limits.work-work,allocation_units=limits.allocation_units-allocated)
                verdict=verify_row(variables,node['equations'],row,caps)
                work+=verdict['work'];allocated+=verdict['allocation_units']
                rows.append({'snapshot':snapshot_index,**location,'basis_row':row_index,**verdict})
                if verdict['status']!='verified':terminal=verdict['status'];break
            if terminal:break
        if terminal:break
    status=terminal or ('verified_selected_rows' if rows else 'inconclusive')
    return {'schema':'owned-search-scoped-matrix-replay/v1','status':status,
            'scope':'committed basis identities relative to captured normalized equations and branch constraints',
            'rows':rows,'verified_rows':sum(r['status']=='verified' for r in rows),'lineages':lineages,
            'work':work,'allocation_units':allocated,'limits':vars(limits),
            'work_scope':'lineage plus row replay; capture decoding has separate limits',
            'allocation_scope':'row replay only; selected decoding has separate retention budget',
            'original_descent_verified':False,'signed_lifts_replayed':False,
            'complete_search_replayed':False,'groebner_basis_complete':False,
            'first_fall_degree':None,'last_fall_degree':None,'degree_of_regularity':None}

def replay_capture(root,binding,sources,limits=ReplayLimits()):
    extracted=verify_and_extract(root,binding,sources)
    result=replay_snapshots(extracted['snapshots'],limits)
    result['capture']=extracted['capture'];result['retention']=extracted['retention']
    result['source_identity_matches_supplied_hashes']=True
    return result

def main():
    p=argparse.ArgumentParser(description=__doc__);sub=p.add_subparsers(dest='command',required=True)
    c=sub.add_parser('capture');c.add_argument('live',type=Path);c.add_argument('--binding',required=True);c.add_argument('--sources',required=True,type=Path)
    w=sub.add_parser('wire');w.add_argument('chunks',nargs='+',type=Path);w.add_argument('--variables',type=int,help='explicit diagnostic wire variable count when expansion is absent')
    for parser in (c,w):
        parser.add_argument('--work-limit',type=int,default=ReplayLimits.work)
        parser.add_argument('--row-limit',type=int,default=ReplayLimits.rows)
    a=p.parse_args();limits=ReplayLimits(work=a.work_limit,rows=a.row_limit)
    if a.command=='capture':
        require(a.sources.stat().st_size<=65536,'source metadata bound')
        result=replay_capture(a.live,a.binding,parse(a.sources.read_bytes()),limits)
    else:
        require(len(a.chunks)<=4096,'too many chunks')
        def streams():
            for path in a.chunks:
                with path.open('rb') as f:yield f
        iterator=streams()
        try:extracted=inspect_selected(iterator)
        finally:iterator.close()
        result=replay_snapshots([{'retained':extracted['retained']}],limits,wire_variables=a.variables)
        result['retention']=extracted['retention'];result['wire_variables_argument']=a.variables;result['wire']=extracted['wire'];result['capture_chain_verified']=False
        result['source_identity_matches_supplied_hashes']=False
    print(json.dumps(result,sort_keys=True))
    return 0 if result['status']=='verified_selected_rows' else 2
if __name__=='__main__':raise SystemExit(main())
