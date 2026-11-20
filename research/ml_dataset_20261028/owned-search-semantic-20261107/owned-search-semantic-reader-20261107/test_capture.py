import hashlib,json,tempfile,unittest
from pathlib import Path
from check_capture import verify,Invalid,digest,ROLE
from test_inspect_wire import fixture,wire,MAGIC
B='ab'*32
S={k:'00'*32 for k in ['public_sha256','factor_sha256','launch_sha256','binding_sha256']}
S['binding_sha256']=digest(b'ml-dataset.sat-original-owner/v1\0'+B.encode()+bytes(64))
def js(x):return json.dumps(x,separators=(',',':')).encode()+b'\n'
def make(root, alter=None, snapshot=0, gap=False, wrong_chunk=False):
    (root/'payloads').mkdir();lines=[];previous=digest(('crypto.ml-dataset.capture-genesis/v1\n'+B+'\n').encode());total=0
    def payload(name,body,role,encoding):
        nonlocal previous,total
        path='payloads/'+name;(root/path).write_bytes(body);total+=len(body)
        e={'schema':'crypto.ml-dataset.live-event/v2','binding_sha256':B,'sequence':len(lines)+1,'previous_event_sha256':previous,'event':{'kind':'capture_payload_committed','artifact':{'path':path,'role':role,'encoding':encoding,'bytes':len(body),'sha256':digest(body)}}}
        line=js(e);lines.append(line);previous=digest(line)
        return {'binding_sha256':B,'event_count':len(lines),'byte_end':sum(map(len,lines)),'last_event_sha256':previous}
    mapbody=b'{"fixture":true}\n';ack=payload('sat-original-pattern-0.gate.json',mapbody,'sat-original-gate-map','json')
    r=fixture();r.update(snapshot_index=snapshot);r['identity'].update({k:bytes.fromhex(v) for k,v in S.items()});r['identity'].update(map_bytes=mapbody,map_ack=ack,pattern=0)
    if alter:alter(r)
    body=MAGIC+wire(r)
    if gap:payload('unrelated.bin',b'x','other','bytes')
    for i,start in enumerate(range(0,len(body),200)):payload(f'boolean-owned-pattern-0-snapshot-{snapshot}-chunk-{i+1 if wrong_chunk else i}.bin.part',body[start:start+200],ROLE,'owned-search-wire-v1')
    (root/'events.jsonl').write_bytes(b''.join(lines));(root/'committed.json').write_bytes(js({'schema':'crypto.ml-dataset.capture-head/v1','binding_sha256':B,'event_count':len(lines),'byte_end':sum(map(len,lines)),'last_event_sha256':previous,'payload_count':len(lines),'payload_bytes':total,'stop':None}))
class Tests(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.root=Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def test_valid(self):
        make(self.root);r=verify(self.root,B,S);self.assertEqual(r['snapshot_count'],1);self.assertFalse(r['producer_authentication_verified'])
    def test_wrong_trusted_source(self):
        make(self.root);s=dict(S,public_sha256='01'*32)
        with self.assertRaises(Invalid):verify(self.root,B,s)
    def test_wrong_attempt(self):
        make(self.root)
        with self.assertRaises(Invalid):verify(self.root,'cc'*32,S)
    def test_chain_edit(self):
        make(self.root);p=self.root/'events.jsonl';p.write_bytes(p.read_bytes().replace(b'"sequence":1',b'"sequence":2',1))
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_payload_tamper(self):
        make(self.root);p=next((self.root/'payloads').glob('*.part'));p.write_bytes(b'x'*p.stat().st_size)
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_ack_mismatch(self):
        make(self.root,lambda r:r['identity']['map_ack'].update(event_count=0))
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_map_bytes_mismatch(self):
        make(self.root,lambda r:r['identity'].update(map_bytes=b'other'))
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_pattern_mismatch(self):
        make(self.root,lambda r:r['identity'].update(pattern=1))
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_incomplete_wire(self):
        make(self.root,lambda r:r.update(encoding_complete=False))
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_uncommitted_suffix(self):
        make(self.root);p=self.root/'events.jsonl';p.write_bytes(p.read_bytes()+b'uncommitted broken bytes');self.assertTrue(verify(self.root,B,S)['committed_prefix_verified'])
    def test_resource_limit(self):
        make(self.root)
        with self.assertRaises(Invalid):verify(self.root,B,S,max_bytes=10)
    def test_symlink(self):
        make(self.root);p=next((self.root/'payloads').glob('*.part'));target=self.root/'outside';p.rename(target);p.symlink_to(target)
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_snapshot_boundary_gap(self):
        make(self.root,gap=True)
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_snapshot_index_gap(self):
        make(self.root,snapshot=1)
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_chunk_index_gap(self):
        make(self.root,wrong_chunk=True)
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_boolean_ack_count(self):
        make(self.root,lambda r:r['identity']['map_ack'].update(event_count=True))
        with self.assertRaises(Invalid):verify(self.root,B,S)
    def test_duplicate_json_key(self):
        make(self.root);p=self.root/'committed.json';p.write_bytes(p.read_bytes().replace(b'{',b'{"schema":"bad",',1))
        with self.assertRaises(Invalid):verify(self.root,B,S)
if __name__=='__main__':unittest.main()
