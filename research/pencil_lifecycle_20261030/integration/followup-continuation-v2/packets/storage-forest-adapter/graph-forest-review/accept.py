from pathlib import Path
import json,re
D=Path(__file__).parent;S=D/'subject';I=S/'invocation-a7dpq5t0'
modules=['Find','Cycles','Union','Forest','Composition','PriorControls','ForestControls','Readback','Reviewer','ReviewerReadback']
r=json.loads((I/'receipt.json').read_text());assert r['gate_exit']==0 and r['bindings_unchanged'];assert [x['module'] for x in r['modules']]==modules and all(x['native_exit']==0 for x in r['modules'])
for m in modules:assert (I/'snapshot'/(m+'.lean')).read_bytes()==(S/(m+'.lean')).read_bytes()
expected=['GraphForest.'+x for x in ['root_reach','reach_parent','attach_forest','find_preserves_roots','two_finds','branch_preserves','edge_after_two_finds','next_find_exists']]+['ForestControls.'+x for x in ['initial_forest','distinct_attach','later_find_terminates','same_component_cycle','unsafe_parent_not_root','nonroot_child_can_cut','first_find','second_find','composed_either']]+['GraphForest.attachment_roots','GraphForest.component_transferred']+['ForestReviewer.'+x for x in ['initial_forest','initial_valid','first_find','second_find','both_nontrivial','composed_both_orientations','next_find_from_result','first_root_survives_second','descendant_path','transferred_descendant','descendant_cycle_rejected','proposed_parent_not_root','root_components_disjoint','same_component_second_find','same_root_branch_excluded']]
names=[]
for line in ((I/'Readback.stdout').read_text()+(I/'ReviewerReadback.stdout').read_text()).splitlines():
 m=re.fullmatch(r"'([^']+)' (?:depends on axioms: \[([^\]]*)\]|does not depend on any axioms)",line)
 if m:
  names.append(m[1]);assert (set(m[2].split(', ')) if m[2] else set())<={'propext','Classical.choice','Quot.sound'}
assert names==expected
(D/'acceptance.json').write_text(json.dumps({'accepted':True,'named_inventory':expected,'main':10,'producer_controls':9,'reviewer_controls':15,'readbacks':34,'receipt':str(I/'receipt.json')},indent=2)+'\n')
print('accepted34named readbacks')
