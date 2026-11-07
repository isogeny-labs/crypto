import copy,json,tempfile,unittest,subprocess,sys
from pathlib import Path
from report_join import *
from test_capture import make,B,S,js,digest,fixture,wire,MAGIC,ROLE
from test_replay_capture import state

def report_for(root):
    e=verify_and_extract(root,B,S);m=e['snapshots'][-1]['binding'];a=m['ack']
    r={'schema':'crypto.ml-dataset.owned-search-report/v1','adapter_id':'original-owned-boolean-search-v1',
       'strategy':'original-owned-boolean-search-v1','status':'partial','scope':'single-pattern-retained-search',
       'accounting_kind':'original-ledger-reservations-not-retired-instructions','pattern':m['pattern'],
       'capture_snapshot_complete':True,'capture_snapshot_index':m['snapshot'],'capture_chunk_count':m['chunks'],
       'capture_head_event_count':a['event_count'],'capture_head_sha256':a['last_event_sha256'],
       'retained_snapshot_count':len(e['snapshots'])}
    r.update({k:False for k in FALSE_FIELDS});r.update({k:None for k in NULL_FIELDS})
    for f,k in [('public_sha256_bytes','public_sha256'),('factor_base_sha256_bytes','factor_sha256'),('launch_sha256_bytes','launch_sha256'),('binding_sha256_bytes','binding_sha256')]:r[f]=list(bytes.fromhex(S[k]))
    return r
class JoinTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.root=Path(self.tmp.name)
        make(self.root,lambda r:r.update(state()));self.report=report_for(self.root)
    def tearDown(self):self.tmp.cleanup()
    def run_join(self,r=None,**kw):return join_report(self.root,B,S,self.report if r is None else r,self.report['pattern'],**kw)
    def test_success_restricted_scope(self):
        x=self.run_join();self.assertEqual(x['status'],'verified_selected_rows');self.assertEqual(x['row_replay']['verified_rows'],1)
        self.assertTrue(x['report_capture_join_verified']);self.assertFalse(x['cell_identity_admitted']);self.assertFalse(x['solver_completed']);self.assertIsNone(x['public_features'])
    def test_snapshot_tamper_each_join_field(self):
        for key in ['capture_snapshot_index','capture_chunk_count','capture_head_event_count','retained_snapshot_count']:
            with self.subTest(key=key):
                r=copy.deepcopy(self.report);r[key]+=1
                with self.assertRaises(ValueError):self.run_join(r)
    def test_bad_head(self):
        r=copy.deepcopy(self.report);r['capture_head_sha256']='0'*64
        with self.assertRaises(ValueError):self.run_join(r)
    def test_report_identity_and_pattern(self):
        for k,v in [('schema','wrong'),('pattern',True),('pattern',999),('status','verified')]:
            r=copy.deepcopy(self.report);r[k]=v
            with self.assertRaises(ValueError):self.run_join(r)
    def test_overclaims(self):
        for k in FALSE_FIELDS+NULL_FIELDS:
            r=copy.deepcopy(self.report);r[k]=True
            with self.assertRaises(ValueError):self.run_join(r)
    def test_source_mutation(self):
        for k in ['public_sha256_bytes','factor_base_sha256_bytes','launch_sha256_bytes','binding_sha256_bytes']:
            r=copy.deepcopy(self.report);r[k][0]^=1
            with self.assertRaises(ValueError):self.run_join(r)
    def test_boolean_is_not_byte_or_count(self):
        for k in ['capture_chunk_count','capture_snapshot_index']:
            r=copy.deepcopy(self.report);r[k]=True
            with self.assertRaises(ValueError):self.run_join(r)
        r=copy.deepcopy(self.report);r['public_sha256_bytes'][0]=True
        with self.assertRaises(ValueError):self.run_join(r)
    def test_capped_rows_do_not_become_success(self):
        x=self.run_join(limits=ReplayLimits(rows=0));self.assertEqual(x['status'],'inconclusive');self.assertEqual(x['row_replay']['verified_rows'],0)
    def test_cli_success_and_capped_exit(self):
        sources=self.root/'sources.json';report=self.root/'report.json'
        sources.write_text(json.dumps(S));report.write_text(json.dumps(self.report))
        command=[sys.executable,str(Path(__file__).with_name('report_join.py')),str(self.root),'--binding',B,'--sources',str(sources),'--report',str(report),'--pattern',str(self.report['pattern'])]
        a=subprocess.run(command,capture_output=True,text=True,timeout=10)
        self.assertEqual(a.returncode,0,a.stderr);self.assertEqual(json.loads(a.stdout)['row_replay']['verified_rows'],1)
        b=subprocess.run(command+['--row-limit','0'],capture_output=True,text=True,timeout=10)
        self.assertEqual(b.returncode,2,b.stderr);self.assertEqual(json.loads(b.stdout)['status'],'inconclusive')
    def append_event(self,event):
        p=self.root/'committed.json';h=json.loads(p.read_bytes())
        e={'schema':'crypto.ml-dataset.live-event/v2','binding_sha256':B,'sequence':h['event_count']+1,'previous_event_sha256':h['last_event_sha256'],'event':event}
        line=js(e)
        with (self.root/'events.jsonl').open('ab') as f:f.write(line)
        h['event_count']+=1;h['byte_end']+=len(line);h['last_event_sha256']=digest(line)
        if event['kind']=='capture_payload_committed':h['payload_count']+=1;h['payload_bytes']+=event['artifact']['bytes']
        p.write_bytes(js(h))
    def test_trailing_committed_event_rejected(self):
        self.append_event({'kind':'unrelated-event'})
        with self.assertRaisesRegex(ValueError,'terminal committed head'):self.run_join()
    def test_second_snapshot_rejects_stale_report(self):
        line=(self.root/'events.jsonl').read_bytes().splitlines(keepends=True)[0]
        ack={'binding_sha256':B,'event_count':1,'byte_end':len(line),'last_event_sha256':digest(line)}
        r=fixture();r.update(state());r['snapshot_index']=1
        r['identity'].update({k:bytes.fromhex(v) for k,v in S.items()})
        r['identity'].update(map_bytes=b'{"fixture":true}\n',map_ack=ack,pattern=0)
        body=MAGIC+wire(r);name='payloads/boolean-owned-pattern-0-snapshot-1-chunk-0.bin.part'
        (self.root/name).write_bytes(body)
        self.append_event({'kind':'capture_payload_committed','artifact':{'path':name,'role':ROLE,'encoding':'owned-search-wire-v1','sha256':digest(body),'bytes':len(body)}})
        with self.assertRaisesRegex(ValueError,'snapshot mismatch'):self.run_join()
        self.report=report_for(self.root);x=self.run_join();self.assertEqual(x['snapshot'],1)
        self.assertEqual(x['row_replay']['verified_rows'],1)
        self.assertEqual(x['row_replay']['rows'][0]['snapshot'],1)
    def test_payload_corruption(self):
        p=next((self.root/'payloads').glob('*.part'));p.write_bytes(b'x'*p.stat().st_size)
        with self.assertRaises(ValueError):self.run_join()
    def test_unchecked_producer_claims_not_exported_as_verified(self):
        self.report['producer_signed_relation_accepted']=True;self.report['reserved_work']=123
        x=self.run_join();self.assertFalse(x['producer_counters_verified']);self.assertFalse(x['producer_lift_classifications_verified']);self.assertNotIn('reserved_work',x)
if __name__=='__main__':unittest.main()
