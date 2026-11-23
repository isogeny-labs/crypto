import copy,tempfile,unittest
from pathlib import Path
from replay_capture import *
from test_capture import make,B,S

def state():
    g=[[[0],[1]]]
    row={'polynomial':copy.deepcopy(g[0]),'proof':[[0,[]]]}
    node={'parent':None,'equations':copy.deepcopy(g),'origins':['Original(0)'],'decisions':[],
          'status':'Ready','children':[None,None],'candidate':[],'candidate_valid':None,
          'native':{'basis':[row],'current':{'polynomial':[[]],'proof':[]}}}
    part={'parent':None,'prefix':[],'equations':copy.deepcopy(g),'status':'Running','children':[],
          'search':{'nodes':[node],'witness_node':None}}
    return {'expansion':{'variables':2},'normalized':g,'continuation':{'partitions':[part]}}
def entry(s=None):return {'retained':state() if s is None else s}
class IntegrationTests(unittest.TestCase):
    def test_valid_committed_basis_and_uncommitted_diagnostic(self):
        r=replay_snapshots([entry()]);self.assertEqual(r['status'],'verified_selected_rows');self.assertEqual(r['verified_rows'],1)
        self.assertFalse(r['original_descent_verified']);self.assertIsNone(r['last_fall_degree'])
    def test_wrong_row(self):
        s=state();s['continuation']['partitions'][0]['search']['nodes'][0]['native']['basis'][0]['polynomial']=[[]]
        r=replay_snapshots([entry(s)]);self.assertEqual(r['status'],'mismatch');self.assertEqual(r['verified_rows'],0)
    def test_substituted_equations_before_row_replay(self):
        s=state();n=s['continuation']['partitions'][0]['search']['nodes'][0];n['equations']=[[[0]]];n['native']['basis'][0]['polynomial']=[[0]]
        r=replay_snapshots([entry(s)]);self.assertEqual(r['status'],'mismatch');self.assertEqual(r['rows'],[])
    def test_shared_row_cap(self):
        r=replay_snapshots([entry(),entry()],ReplayLimits(rows=1));self.assertEqual(r['status'],'inconclusive');self.assertEqual(r['verified_rows'],1)
    def test_shared_allocation_cap(self):
        used=replay_snapshots([entry()])['allocation_units']
        r=replay_snapshots([entry(),entry()],ReplayLimits(allocation_units=used))
        self.assertEqual(r['status'],'inconclusive');self.assertEqual(r['verified_rows'],1);self.assertEqual(r['allocation_units'],used)
    def test_shared_work_cap(self):
        r=replay_snapshots([entry()],ReplayLimits(work=1));self.assertEqual(r['status'],'inconclusive');self.assertLessEqual(r['work'],1)
    def test_empty_is_not_verified(self):self.assertEqual(replay_snapshots([])['status'],'inconclusive')
    def test_no_continuation(self):
        s=state();s['continuation']=None;self.assertEqual(replay_snapshots([entry(s)])['status'],'inconclusive')
    def test_invalid_basis(self):
        s=state();s['continuation']['partitions'][0]['search']['nodes'][0]['native']['basis']=True
        self.assertEqual(replay_snapshots([entry(s)])['status'],'invalid')
    def test_explicit_wire_variables_only_when_matched(self):
        s=state();s['expansion']=None
        self.assertEqual(replay_snapshots([entry(s)])['status'],'inconclusive')
        self.assertEqual(replay_snapshots([entry(s)],wire_variables=2)['status'],'verified_selected_rows')
        with self.assertRaises(ValueError):replay_snapshots([entry()],wire_variables=3)
    def test_invalid_limits(self):
        with self.assertRaises(ValueError):replay_snapshots([entry()],ReplayLimits(work=True))
    def test_verified_capture_to_math(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);make(root,lambda r:r.update(state()))
            r=replay_capture(root,B,S);self.assertEqual(r['status'],'verified_selected_rows');self.assertTrue(r['capture']['committed_prefix_verified']);self.assertEqual(r['verified_rows'],1)
    def test_capture_tamper_prevents_math(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);make(root,lambda r:r.update(state()));f=next((root/'payloads').glob('*.part'));f.write_bytes(b'x'*f.stat().st_size)
            with self.assertRaises(ValueError):replay_capture(root,B,S)
if __name__=='__main__':unittest.main()
