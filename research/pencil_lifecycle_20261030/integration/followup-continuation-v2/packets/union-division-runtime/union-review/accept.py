from pathlib import Path
import json,re,hashlib
D=Path(__file__).parent;S=D/'subject';I=S/'invocation-e68t8xgd'
rec=json.loads((I/'receipt.json').read_text())
assert rec['gate_exit']==0 and rec['bindings_unchanged']
assert [x['module'] for x in rec['modules']]==['Find','Cycles','Union','Controls','Readback','Reviewer','ReviewerReadback']
assert all(x['native_exit']==0 for x in rec['modules'])
for n in ['Find','Cycles','Union','Controls','Readback','Reviewer','ReviewerReadback']:assert (I/'snapshot'/(n+'.lean')).read_bytes()==(S/(n+'.lean')).read_bytes()
expected=['GraphUnion.'+x for x in ['join_preserves','join_result','both_fixed_zero_consistent','forward_global','backward_global']]+['UnionControls.'+x for x in ['initial_valid','global_after','descendant_frame','transferred_anchor','stale_child_anchor','both_returns','reverse_preserves','wrong_potential_breaks']]+['UnionReviewer.'+x for x in ['initial_valid','forward_global_check','reverse_global_check','reverse_changes_offsets','grandchild_unchanged','all_stored_anchors','both_valid','both_answer','zero_consistent_result','inconsistent_prestate_rejected','invalid_zero_constraint_model_boundary','wrong_reverse_target_sign']]
names=[]
for line in ((I/'Readback.stdout').read_text()+(I/'ReviewerReadback.stdout').read_text()).splitlines():
 m=re.fullmatch(r"'([^']+)' depends on axioms: \[([^\]]*)\]",line)
 if m:
  names.append(m[1]);assert set(m[2].split(', '))<={'propext','Classical.choice','Quot.sound'}
assert names==expected
(D/'acceptance.json').write_text(json.dumps({'accepted':True,'receipt':str(I/'receipt.json'),'named_inventory':expected,'main':5,'producer_controls':8,'reviewer_controls':12,'readbacks':25},indent=2)+'\n')
print('accepted25named readbacks')
