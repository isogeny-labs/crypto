#!/usr/bin/env python3
"""Metadata from retained process outputs only; no mathematical verifier."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent
accepted=[]
for rid in ('jac83-03','jac284-02'):
 p=h/'runs'/rid;log=(p/'native.log').read_text()
 assert (p/'exit.txt').read_text().strip()=='0' and log.rstrip().endswith('JAC32_ALL_CHECKS_PASS')
 assert not re.search(r'^\s*\?',log,re.M)
 rows=[dict(zip(('root','twist','rank','minors3','minors4'),map(int,m))) for m in
 re.findall(r'JAC32_RESULT root=(\d+) twist=(\d+) rank=(\d+) minors3=(\d+) minors4=(\d+) zero_control=proper',log)]
 assert len(rows)==10
 accepted.append({'id':rid,'source_revision':(p/'source-revision.txt').read_text().strip(),'records':rows})
failed=h/'runs/jac83-02';assert '? JAC32_UNIT_IDENTITY' in (failed/'native.log').read_text()
for rid in ('jac83-01','jac284-01'):
 p=h/'runs'/rid;assert not (p/'native.log').exists() and 'missing or empty' in (p/'gate.log').read_text()
p=h/'runs/lean32-01';assert (p/'compile.exit').read_text().strip()=='0' and (p/'audit.exit').read_text().strip()=='0'
audit=(p/'audit.log').read_text().rstrip().splitlines()[-1]
assert re.fullmatch(r'audited 2 theorems in \[.*\]; 0 failing, 0 trusted \(native_decide\)',audit)
r={'schema':'koblitz.round32-results/v1','accepted':accepted,'native_accepted':2,'native_rejected':1,
 'native_rejected_ids':['jac83-02'],'pretrial_refusals':['jac83-01','jac284-01'],'accepted_point_algebras':20,
 'named_Lean_theorems':2,'audited_Lean_theorems':2,'formal_native_matrix_import':False,
 'local_dimension_proved':False,'proper_normalization_completed':False,'nonprincipal_class_proved':False,
 'performance_measured':False,'independent_scientific_replay':False}
(h/'results.json').write_text(json.dumps(r,indent=2)+'\n')
p={'schema':'koblitz.exact-computation-protocol/v1','round':32,'normative_record':'PROTOCOL.md',
 'machine_index_created_after_runs':True,'generic_checker_available':False,'comparative_or_statistical_claim':False,
 'native_limits':{'virtual_memory_KiB':4194304,'timeout_seconds_each':1200},
 'lean_limits':{'version':'4.34.1','mathlib':'d13f23b723b8a846827a245b89c10fc7d3f11612','threads':1,'managed_memory_MiB':4096,'timeout_seconds_each':1200}}
(h/'protocol.json').write_text(json.dumps(p,indent=2)+'\n')
print('Indexed20 accepted point-algebra records, one native rejection, two pretrial refusals and two audited theorems.')
