from pathlib import Path
import json,re,hashlib
D=Path(__file__).parent
main=['put_good','put_decode','allocation_bound','current_index_bound','loop_refines','loop_good','finish_decode','finish_good','word_projection','word_good','word_exact','finish_normal','word_normal','word_success','word_total']
controls=['empty_input','all_zero_input','empty_divisor','exact_words','nonexact_words','leading_zero_input','constant_divisor','insufficient_fuel','repeated_write','zero_capacity_guard','largest_usize_input','empty_divisor_overflow_boundary','allocation_is_nonzero']
expected=['QuotientStorage.'+x for x in main]+['StorageControls.'+x for x in controls]
gates={'QuotientStorage':'storage1','StoredLoop':'loop1','StoredDivision':'division2','StorageEndpoint':'endpoint1','StorageControls':'controls1','StorageReadback':'readback1'}
for module,tag in gates.items():
 r=json.loads((D/tag/'receipt.json').read_text());g=json.loads((D/tag/'gate-exit.json').read_text())
 assert r['exit_code']==0 and g['gate_exit_code']==0 and g['input_bindings_unchanged']
 assert r['source_sha256']==hashlib.sha256((D/(module+'.lean')).read_bytes()).hexdigest()
names=[]
for line in (D/'readback1/stdout').read_text().splitlines():
 m=re.fullmatch(r"'([^']+)' depends on axioms: \[([^\]]*)\]",line)
 if m:
  names.append(m[1]);assert set(m[2].split(', '))<={'propext','Classical.choice','Quot.sound'}
assert names==expected
(D/'acceptance.json').write_text(json.dumps({'accepted':True,'named_inventory':expected,'gates':gates,'main':len(main),'controls':len(controls),'readbacks':len(names)},indent=2)+'\n')
print('accepted',len(names),'named declarations')
