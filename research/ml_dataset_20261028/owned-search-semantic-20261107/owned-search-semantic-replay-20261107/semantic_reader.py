#!/usr/bin/env python3
"""Bounded selected retained-state decoding; no algebraic verdict."""
import argparse
from dataclasses import dataclass
import hashlib, json
from pathlib import Path
from inspect_wire import Limits, Inspector, Invalid
from check_capture import BoundInspector, verify, parse, require

@dataclass(frozen=True)
class RetentionLimits:
    nodes: int = 100_000
    allocation_units: int = 16 * 1024 * 1024

class RetentionBudget:
    """Conservative allocation accounting units, not measured Python RSS."""
    def __init__(self, limits):
        require(type(limits.nodes) is int and 0 < limits.nodes <= 1_000_000, 'invalid retained node limit')
        require(type(limits.allocation_units) is int and 0 < limits.allocation_units <= 256*1024*1024, 'invalid retained allocation limit')
        self.limits=limits;self.nodes=0;self.units=0
    def charge(self,nodes,units):
        require(nodes <= self.limits.nodes-self.nodes and units <= self.limits.allocation_units-self.units, 'retention limit exceeded')
        self.nodes+=nodes;self.units+=units
    def summary(self):
        return {'nodes':self.nodes,'allocation_units':self.units,'limits':vars(self.limits)}

def inspect_selected(streams, limits=Limits(), retention=RetentionLimits()):
    budget=RetentionBudget(retention);reader=Inspector(streams,limits);reader.retention=budget
    result=reader.inspect()
    return {'schema':'owned-search-selected-reader/v1','wire':result,'retained':reader.retained,
            'retention':budget.summary(),'capture_chain_verified':False,
            'original_algebra_replayed':False,'mathematical_verdict':None}

def verify_and_extract(root,binding,expected_sources,*,max_bytes=128*1024*1024,max_events=100000,retention=RetentionLimits()):
    """Return only after all full-prefix, source, map and snapshot checks pass.

    All snapshots share one retention budget. Input directory must be immutable.
    Source authentication means equality with caller-supplied hashes only; original
    public input, map algebra, equations and proofs remain unreplayed.
    """
    budget=RetentionBudget(retention);snapshots=[]
    def factory(streams,limits):
        reader=BoundInspector(streams,limits);reader.retention=budget;return reader
    def collect(reader,metadata):
        # Callback receives internal verified metadata. Nothing is exported until
        # verify succeeds for every snapshot and returns its full-prefix verdict.
        budget.charge(1,512)
        snapshots.append({'binding':metadata,'retained':reader.retained})
    checked=verify(root,binding,expected_sources,max_bytes,max_events,inspector_factory=factory,on_snapshot=collect)
    return {'schema':'owned-search-bound-selected-reader/v1','capture':checked,'snapshots':snapshots,
            'retention':budget.summary(),'source_identity_matches_supplied_hashes':True,
            'original_algebra_replayed':False,'mathematical_verdict':None}

def structural_summary(data):
    """Small bounded summary for CLI; selected objects stay in Python API."""
    out={k:v for k,v in data.items() if k not in ('retained','snapshots')}
    def summary(r):
        continuation=r.get('continuation');expansion=r.get('expansion')
        partitions=continuation.get('partitions') if isinstance(continuation,dict) else None
        normalized=r.get('normalized')
        return {'selected_keys':sorted(r),'normalized_equations':len(normalized) if isinstance(normalized,list) else None,
                'variables':expansion.get('variables') if isinstance(expansion,dict) else None,
                'partitions':len(partitions) if isinstance(partitions,list) else None}
    if 'retained' in data:out['selected_summary']=summary(data['retained'])
    if 'snapshots' in data:out['snapshots']=[{'binding':s['binding'],'selected_summary':summary(s['retained'])} for s in data['snapshots']]
    return out

def main():
    p=argparse.ArgumentParser(description=__doc__);sub=p.add_subparsers(dest='command',required=True)
    raw=sub.add_parser('wire');raw.add_argument('chunks',nargs='+',type=Path)
    cap=sub.add_parser('capture');cap.add_argument('live',type=Path);cap.add_argument('--binding',required=True);cap.add_argument('--sources',required=True,type=Path)
    args=p.parse_args()
    if args.command=='wire':
        require(len(args.chunks)<=4096,'too many chunks')
        def streams():
            for path in args.chunks:
                with path.open('rb') as f:yield f
        iterator=streams()
        try:result=inspect_selected(iterator)
        finally:iterator.close()
    else:
        require(args.sources.stat().st_size<=65536,'source metadata bound')
        result=verify_and_extract(args.live,args.binding,parse(args.sources.read_bytes()))
    print(json.dumps(structural_summary(result),sort_keys=True))
if __name__=='__main__':main()
