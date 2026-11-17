import copy,unittest
from equation_lineage import *

def node(eqs,parent=None,origins=None,decisions=None,status='Ready'):
 return dict(parent=parent,equations=eqs,origins=origins if origins is not None else [f'Original({i})' for i in range(len(eqs))],decisions=decisions or [],status=status,children=[None,None],candidate=[],candidate_valid=None)
def fixture():
 g=[[[0],[1]]];r=node(copy.deepcopy(g),status='Branched');r['children']=[2,1]
 a=node(g+[constraint(0,True)],0,['Original(0)','Branch { parent: 0, variable: 0, value: true }'],[[0,True]])
 b=node(g+[constraint(0,False)],0,['Original(0)','Branch { parent: 0, variable: 0, value: false }'],[[0,False]])
 p=dict(parent=None,prefix=[],equations=copy.deepcopy(g),status='Running',children=[],search={'nodes':[r,a,b],'witness_node':None})
 return g,dict(partitions=[p]),2
class Controls(unittest.TestCase):
 def check(self,f):return check_lineage(*f)
 def test_actual_builder_order(self):
  r=self.check(fixture());self.assertEqual(r['status'],'verified_lineage');self.assertEqual(len(r['verified_nodes']),3);self.assertFalse(r['solution_or_unsat_verified'])
 def test_wrong_equation(self):
  f=fixture();f[1]['partitions'][0]['search']['nodes'][1]['equations'][-1]=[[0]];self.assertEqual(self.check(f)['status'],'lineage_mismatch')
 def test_wrong_origin(self):
  f=fixture();f[1]['partitions'][0]['search']['nodes'][1]['origins'][-1]='Branch { parent: 0, variable: 1, value: true }';self.assertEqual(self.check(f)['status'],'lineage_mismatch')
 def test_wrong_decision(self):
  f=fixture();f[1]['partitions'][0]['search']['nodes'][1]['decisions']=[[1,True]];self.assertEqual(self.check(f)['status'],'lineage_mismatch')
 def test_root_substitution(self):
  f=fixture();f[1]['partitions'][0]['search']['nodes'][0]['equations']=[[[1]]];self.assertEqual(self.check(f)['status'],'lineage_mismatch')
 def test_noncanonical(self):
  f=fixture();f[0][0].reverse();self.assertEqual(self.check(f)['status'],'lineage_mismatch')
 def test_partial_node(self):
  f=fixture();n=f[1]['partitions'][0]['search']['nodes'][1];n.update(status='Building',equations=[],origins=[],decisions=[])
  r=self.check(f);self.assertEqual(r['status'],'inconclusive_partial');self.assertEqual(r['skipped_nodes'],[{'partition':0,'node':1,'reason':'builder incomplete'}]);self.assertEqual(len(r['verified_nodes']),2)
 def test_partial_partition(self):
  g,c,n=fixture();p=c['partitions'][0];p.update(status='Building',equations=[],search=None);self.assertEqual(check_lineage(g,c,n)['status'],'inconclusive_partial')
 def test_boolean_not_index(self):
  f=fixture();f[1]['partitions'][0]['search']['nodes'][1]['parent']=False;self.assertEqual(self.check(f)['status'],'lineage_mismatch')
 def test_wrong_backlink(self):
  f=fixture();f[1]['partitions'][0]['search']['nodes'][0]['children']=[1,1];self.assertEqual(self.check(f)['status'],'lineage_mismatch')
 def test_cap(self):
  r=check_lineage(*fixture(),limits=Limits(work=2));self.assertEqual(r['status'],'inconclusive');self.assertLessEqual(r['work'],2);self.assertEqual(r['verified_nodes'],[])
 def test_invalid_limits(self):
  for lim in [Limits(work=True),Limits(work=-1),Limits(nodes=999999),Limits(work=100000001)]:
   with self.assertRaises(ValueError):check_lineage(*fixture(),limits=lim)
 def test_late_mismatch_clears_previous_nodes(self):
  f=fixture();f[1]['partitions'][0]['search']['nodes'][2]['equations']=[[]];r=self.check(f);self.assertEqual(r['status'],'lineage_mismatch');self.assertEqual(r['verified_nodes'],[])
 def test_partition_split(self):
  g=[[[0],[1]]];root=node(g,status='Witness');root['candidate']=[True,True]
  p=dict(parent=None,prefix=[],equations=g,status='Excluded',children=[1,2],search={'nodes':[root],'witness_node':0})
  children=[dict(parent=0,prefix=pre,equations=g+[constraint(i,b) for i,b in enumerate(pre)],status='Ready',children=[],search=None) for pre in [[False],[True,False]]]
  c={'partitions':[p]+children};self.assertEqual(check_lineage(g,c,2)['status'],'verified_lineage')
  c['partitions'][2]['prefix']=[False,False];self.assertEqual(check_lineage(g,c,2)['status'],'lineage_mismatch')
 def test_missing_continuation(self):self.assertEqual(check_lineage([],None,2)['status'],'inconclusive')
if __name__=='__main__':unittest.main()
