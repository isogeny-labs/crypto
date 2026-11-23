// Postdata source formatter. It copies decimal strings, never computes EC arithmetic.
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const raw=fs.readFileSync(path.join(root,'inputs/large.txt'),'utf8').trim().split(/\s+/);
const [p,,,gx,gy,qx,qy]=raw;
const summary=JSON.parse(fs.readFileSync(path.join(root,'evidence/runs/large.stdout'),'utf8'));
const nat=s=>{if(!/^\d+$/.test(s))throw Error('not a natural decimal');return s;};
const int=s=>{if(!/^-?\d+$/.test(s))throw Error('not an integer decimal');return `(${s}:ℤ)`;};
for(const lift of summary.lifts){
 for(const tag of ['G','Q']){
  const i=lift.lift;
  const name=`SmartLargeLift${i}${tag}`;
  const tr=JSON.parse(fs.readFileSync(path.join(root,'evidence/runs/large-data',lift[`${tag}_trace`]),'utf8'));
  const x=tag==='G'?gx:qx,y=tag==='G'?gy:qy;
  const lines=[`import ${i===0?"SmartSpecification":`SmartLargeLift${i-1}${tag}`}`,`namespace ${name}`,`open SmartArithmeticAudit SmartSpecificationAudit`,`def modulus : ℕ := ${nat(p)}`,`abbrev R := ZMod (modulus^2)`,`def a : ℕ := ${nat(lift.a)}`,`def b : ℕ := ${nat(lift.b)}`];
  for(let j=0;j<tr.nodes.length;j++){
   const n=tr.nodes[j];
   lines.push(`private def node_${j} : J R := ⟨${nat(n.X)},${nat(n.Y)},${nat(n.Z)}⟩`);
   if(j===0){lines.push('private def ops_0 : List Op := []','private theorem exec_0 : Trace (a:R) node_0 ops_0 node_0 := .initial');continue;}
   const op=n.op==='double'?'.double':n.op==='add'?'.addBase':null;if(!op)throw Error('bad op');
   lines.push(`private def ops_${j} : List Op := ops_${j-1}++[${op}]`,`private theorem step_${j} : step (a:R) node_0 node_${j-1} ${op}=node_${j} := by decide +kernel`,`private theorem exec_${j} : Trace (a:R) node_0 ops_${j} node_${j} := .append exec_${j-1} ${op} step_${j}`);
  }
  const last=tr.nodes.length-1,n=tr.nodes[last];
  lines.push(`private theorem operations : ops_${last}=binaryOps modulus := by decide +kernel`,
   `private theorem initial_lift : lifted modulus a b ${x} ${y}=node_0 := by`,
   `  have h := henselValue_of_witness modulus a b ${x} ${y} ${nat(lift[`${tag}_hensel_correction`])} ${nat(lift[`${tag}_hensel_inverse`])} ${int(lift[`${tag}_hensel_quotient`])}`,
   '    (by decide +kernel) (by decide +kernel) (by decide +kernel) (by decide +kernel)',
   '  unfold lifted','  rw [h]','  decide +kernel',
   `def output : J R := node_${last}`,
   `theorem execution : run (a:R) modulus (lifted modulus a b ${x} ${y})=output := by`,
   '  rw [initial_lift]','  exact trace_run (by decide +kernel) operations exec_'+last,
   `theorem extracted_coefficient : coefficient modulus output=${nat(lift[`${tag}_coefficient`])} := by`,
   `  exact coefficient_of_inverse_witness modulus ${nat(n.X)} ${nat(n.Y)} ${nat(n.Z)} ${nat(lift[`${tag}_inverse_Y`])} ${nat(lift[`${tag}_phi`])} ${nat(lift[`${tag}_coefficient`])}`,
   '    (by decide +kernel) (by decide +kernel) (by decide +kernel) (by decide +kernel)',
   'theorem initial_on_curve : curveEquation (a:R) (b:R) node_0 := by unfold curveEquation; decide +kernel',
   'theorem output_on_curve : curveEquation (a:R) (b:R) output := by unfold curveEquation; decide +kernel',
   '#print axioms execution','#print axioms extracted_coefficient',`end ${name}`);
  fs.writeFileSync(path.join(root,'certificate-lean',name+'.lean'),lines.join('\n')+'\n');
 }
}
const lines=['import SmartLargeLift0G','import SmartLargeLift0Q','import SmartLargeLift1G','import SmartLargeLift1Q','import SmartLargeLift2G','import SmartLargeLift2Q','import SmartLargeLift3G','import SmartLargeLift3Q','namespace SmartLargeRecoveryAudit','open SmartArithmeticAudit SmartSpecificationAudit AnomalousPointTraceAudit'];
for(const lift of summary.lifts){
 const i=lift.lift,ns=`SmartLargeLift${i}`;
 lines.push(`theorem attempt_${i} : attempt ${p} ${lift.a} ${lift.b} ${gx} ${gy} ${qx} ${qy}=some knownScalar := by`,
 `  apply attempt_of_witness _ _ _ _ _ _ _ ${lift.G_coefficient} ${lift.Q_coefficient} _ ${ns}G.output ${ns}Q.output`,
 `  · exact ${ns}G.execution`,`  · exact ${ns}Q.execution`,`  · exact ${ns}G.extracted_coefficient`,`  · exact ${ns}Q.extracted_coefficient`,'  · decide +kernel',
 `  · exact recovery_value_of_witness _ _ _ ${lift.G_inverse_coefficient} _`,
 '      (by decide +kernel) (by decide +kernel) (by decide +kernel)');
}
lines.push('theorem four_lift_recovery :',...summary.lifts.map((l,i)=>`    attempt ${p} ${l.a} ${l.b} ${gx} ${gy} ${qx} ${qy}=some knownScalar${i===3?' := by':' ∧'}`),'  exact ⟨attempt_0,attempt_1,attempt_2,attempt_3⟩',
 'theorem concrete_log_correct : knownScalar • G=Q := known_answer_relation',
 '#print axioms four_lift_recovery','#print axioms concrete_log_correct','end SmartLargeRecoveryAudit');
fs.writeFileSync(path.join(root,'certificate-lean/SmartLargeRecovery.lean'),lines.join('\n')+'\n');
