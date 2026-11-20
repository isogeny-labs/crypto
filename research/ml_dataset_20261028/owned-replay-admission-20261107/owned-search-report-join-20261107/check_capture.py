#!/usr/bin/env python3
"""Read-only full committed-prefix and owned-search snapshot verifier."""
import argparse, hashlib, json, re
from pathlib import Path
from inspect_wire import Inspector, Limits, Invalid
SHA = re.compile(r'[0-9a-f]{64}\Z')
ROLE = 'boolean-original-owned-search-snapshot-chunk-v1'
CHUNK = re.compile(r'payloads/boolean-owned-pattern-(0|[1-9][0-9]*)-snapshot-(0|[1-9][0-9]*)-chunk-(0|[1-9][0-9]*)\.bin\.part\Z')
def require(ok, why):
    if not ok: raise Invalid(why)
def digest(b): return hashlib.sha256(b).hexdigest()
def integer(v): return type(v) is int and 0 <= v < 2**64
def pairs(items):
    d = {}
    for k,v in items:
        require(k not in d, 'duplicate JSON field');d[k]=v
    return d
def parse(b):
    try: return json.loads(b, object_pairs_hook=pairs, parse_constant=lambda x: (_ for _ in ()).throw(Invalid('nonfinite JSON')))
    except (ValueError,RecursionError) as e: raise Invalid('invalid JSON: '+str(e)) from e
def safe(root, name):
    p=Path(name)
    require(not p.is_absolute() and all(x not in ('..','.') for x in p.parts), 'unsafe path')
    q=root
    for part in p.parts:
        q=q/part;require(not q.is_symlink(),'symlink rejected')
    require(q.is_file(),'missing regular file');return q
class BoundInspector(Inspector):
    def __init__(self,*args):
        super().__init__(*args);self.metadata={};self.map_sha=None
    def value(self,path=(),depth=0):
        old=self.r.raw
        if path==('identity','map_bytes'):
            def raw(n,retain=False):
                h=hashlib.sha256()
                while n:
                    k=min(n,65536);h.update(self.r.read(k));n-=k
                self.map_sha=h.hexdigest()
            self.r.raw=raw
        try: result=super().value(path,depth)
        finally:self.r.raw=old
        if path[:1]==('identity',) and (len(path)==2 or len(path)==3):self.metadata[path[1:]]=result
        return result

def verify(root, binding, expected_sources, max_bytes=128*1024*1024, max_events=100000, *, inspector_factory=BoundInspector, on_snapshot=None):
    root=Path(root);require(not root.is_symlink() and root.is_dir(),'invalid root')
    require(bool(SHA.fullmatch(binding)),'invalid trusted binding hash')
    require(set(expected_sources)=={'public_sha256','factor_sha256','launch_sha256','binding_sha256'} and all(isinstance(x,str) and SHA.fullmatch(x) for x in expected_sources.values()),'four trusted source hashes required')
    require(type(max_bytes) is int and 0<max_bytes<=1024**3 and type(max_events) is int and 0<max_events<=1000000,'invalid resource bounds')
    hp=safe(root,'committed.json');require(hp.stat().st_size<=65536,'head too large');head=parse(hp.read_bytes())
    keys={'schema','binding_sha256','event_count','byte_end','last_event_sha256','payload_count','payload_bytes','stop'}
    require(isinstance(head,dict) and set(head)==keys,'head fields')
    require(head['schema']=='crypto.ml-dataset.capture-head/v1' and head['binding_sha256']==binding,'head binding/schema')
    require(all(integer(head[k]) for k in ['event_count','byte_end','payload_count','payload_bytes']),'head counts')
    require(head['event_count']<=max_events and head['byte_end']+head['payload_bytes']<=max_bytes,'budget exceeded')
    previous=digest(('crypto.ml-dataset.capture-genesis/v1\n'+binding+'\n').encode());offset=0;payloads={};groups={};positions={};count=0;total=0
    with safe(root,'events.jsonl').open('rb') as f:
        while offset<head['byte_end']:
            line=f.readline(min(1024*1024+1,head['byte_end']-offset+1))
            require(line.endswith(b'\n') and len(line)<=1024*1024 and offset+len(line)<=head['byte_end'],'event length or watermark')
            r=parse(line);count+=1;require(count<=head['event_count'] and count<=max_events,'event count exceeded')
            require(isinstance(r,dict) and set(r)=={'schema','binding_sha256','sequence','previous_event_sha256','event'},'event fields')
            require(r['schema']=='crypto.ml-dataset.live-event/v2' and r['binding_sha256']==binding and integer(r['sequence']) and r['sequence']==count and r['previous_event_sha256']==previous,'event chain')
            previous=digest(line);offset+=len(line);positions[count]={'binding_sha256':binding,'event_count':count,'byte_end':offset,'last_event_sha256':previous}
            event=r['event'];require(isinstance(event,dict),'event object')
            if event.get('kind')!='capture_payload_committed':continue
            a=event.get('artifact');require(isinstance(a,dict) and set(a)=={'path','role','encoding','sha256','bytes'},'artifact fields')
            name=a['path'];require(isinstance(name,str) and name.startswith('payloads/') and len(name[9:].encode())<=255 and name[9:] not in ('','.','..') and '/' not in name[9:] and '\\' not in name,'artifact path')
            require(name not in payloads and integer(a['bytes']) and isinstance(a['sha256'],str) and SHA.fullmatch(a['sha256']) and all(isinstance(a[k],str) and a[k].strip() for k in ['role','encoding']),'artifact metadata')
            total+=a['bytes'];require(total<=head['payload_bytes'],'payload budget')
            p=safe(root,name);require(p.stat().st_size==a['bytes'],'payload size');h=hashlib.sha256()
            with p.open('rb') as stream:
                for block in iter(lambda:stream.read(65536),b''):h.update(block)
            require(h.hexdigest()==a['sha256'],'payload hash');payloads[name]=(count,a)
            if a['role']==ROLE:
                match=CHUNK.fullmatch(name);require(match is not None and a['encoding']=='owned-search-wire-v1' and a['bytes']>0,'chunk schema')
                pattern,snapshot,index=map(int,match.groups());g=groups.setdefault((pattern,snapshot),[])
                require(index==len(g) and (not g or count==g[-1][0]+1),'chunk order or interleaving');g.append((count,p))
        # Bytes after the committed watermark are uncommitted and deliberately ignored.
    require(count==head['event_count'] and previous==head['last_event_sha256'] and len(payloads)==head['payload_count'] and total==head['payload_bytes'],'head mismatch')
    snapshots=[];pattern_ends={}
    for (pattern,index),chunks in groups.items():
        def streams():
            for _,p in chunks:
                with p.open('rb') as f:yield f
        it=streams()
        try:
            inspector=inspector_factory(it,Limits(bytes=max_bytes));result=inspector.inspect()
        finally:it.close()
        derived=digest(b'ml-dataset.sat-original-owner/v1\0'+binding.encode()+bytes.fromhex(expected_sources['public_sha256'])+bytes.fromhex(expected_sources['factor_sha256']))
        require(expected_sources['binding_sha256']==derived,'owner binding derivation')
        require(result['snapshot_index']==index and result['source_hashes']==expected_sources,'snapshot source identity')
        md=inspector.metadata
        require(type(md.get(('pattern',))) is int and md[('pattern',)]==pattern and md.get(('publication_attempted',)) is True and md.get(('map_published',)) is True,'snapshot map state')
        mapname=f'payloads/sat-original-pattern-{pattern}.gate.json';require(mapname in payloads,'missing original map')
        seq,a=payloads[mapname]
        prior_index,prior_event=pattern_ends.get(pattern,(-1,seq))
        require(index==prior_index+1 and chunks[0][0]==prior_event+1,'snapshot index or publication boundary')
        pattern_ends[pattern]=(index,chunks[-1][0])
        ack={k:md.get(('map_ack',k)) for k in positions[seq]}
        require(integer(ack['event_count']) and integer(ack['byte_end']),'map receipt numeric types')
        require(a['role']=='sat-original-gate-map' and a['encoding']=='json' and a['sha256']==inspector.map_sha and ack==positions[seq] and seq<chunks[0][0],'map receipt binding')
        if on_snapshot is not None: on_snapshot(inspector, {'pattern':pattern,'snapshot':index,'sha256':result['sha256'],'chunks':len(chunks),'ack':dict(positions[chunks[-1][0]])})
        snapshots.append({'pattern':pattern,'snapshot':index,'chunks':len(chunks),'bytes':result['bytes'],'sha256':result['sha256'],'producer_status':result['producer_status']})
    return {'schema':'owned-search-capture-offline-check/v1','committed_prefix_verified':True,'snapshot_count':len(snapshots),'snapshots':snapshots,'event_count':count,'payload_bytes':total,'trusted_binding_sha256':binding,'trusted_source_hashes':expected_sources,'mathematical_replay':False,'producer_authentication_verified':False,'s3_custody_verified':False,'uncommitted_suffix_ignored':True}
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('live',type=Path);p.add_argument('--binding',required=True);p.add_argument('--sources',required=True,type=Path);a=p.parse_args()
    require(a.sources.stat().st_size<=65536,'source metadata too large');print(json.dumps(verify(a.live,a.binding,parse(a.sources.read_bytes())),sort_keys=True))
if __name__=='__main__':main()
