from pathlib import Path
import json,re,hashlib
D=Path(__file__).parent
main=['write_at','write_other','fresh_write','zero_fresh','next_fresh','quotient_step','success_correct','success_exists','remainder_projection','checked_projection','checked_correct','checked_exists','empty_sound','exact_iff_divisible','quotient_unique','quotient_index_bound','returns_given_exact']
controls=['assignment_replaces','addition_wrong_at_used_slot','empty_divisor_rejected','nonmonic_divisor_rejected','zero_input','no_fuel','exact_two_steps','nonexact_remainder','insufficient_one_step']
expected=['NativeDivision.'+x for x in main]+['DivisionControls.'+x for x in controls]
gates={'QuotientWrite':'write2','DivisionLoop':'loop1','ExactDivision':'exact2','DivisionInterface':'interface1','DivisionControls':'controls2','DivisionReadback':'readback1'}
for module,tag in gates.items():
 rec=json.loads((D/tag/'receipt.json').read_text());gate=json.loads((D/tag/'gate-exit.json').read_text())
 assert gate['gate_exit_code']==0 and gate['input_bindings_unchanged'] and rec['exit_code']==0
 assert rec['source_sha256']==hashlib.sha256((D/(module+'.lean')).read_bytes()).hexdigest()
text=(D/'readback1/stdout').read_text();names=[]
for line in text.splitlines():
 m=re.fullmatch(r"'([^']+)' depends on axioms: \[([^\]]*)\]",line)
 if m:
  names.append(m[1]);assert set(m[2].split(', '))<={'propext','Classical.choice','Quot.sound'}
assert names==expected
(D/'acceptance.json').write_text(json.dumps({'accepted':True,'named_inventory':expected,'gates':gates,'main':len(main),'controls':len(controls),'readbacks':len(names)},indent=2)+'\n')
print('accepted',len(names),'named readbacks')
