// Formats untrusted C++ strings as Lean proof obligations. No curve arithmetic.
import {readFileSync,writeFileSync} from 'node:fs';
const [input,output,limitText]=process.argv.slice(2);
if(!input||!output)throw Error('input.json output.lean [probe-node-count]');
const data=JSON.parse(readFileSync(input,'utf8'));
const limit=limitText?Number(limitText):data.nodes.length;
if(!Number.isSafeInteger(limit)||limit<1||limit>data.nodes.length)throw Error('bad limit');
const nodes=data.nodes.slice(0,limit);
for(let i=0;i<nodes.length;i++) {
 const n=nodes[i];
 for(const k of ['scalar','x','y','slope'])if(!/^[0-9]+$/.test(n[k]))throw Error('bad numeral');
 if(i&&(!Number.isSafeInteger(n.left)||!Number.isSafeInteger(n.right)||
   n.left<0||n.right<0||n.left>=i||n.right>=i))throw Error('bad dependency');
}
const out=['import AffineCertificates','','/-! Untrusted candidate coordinates and slopes are bound to actual Mathlib',
 'points. Private theorems package the finite certificate; the final theorem',
 'depends transitively on every referenced proof and is axiom-audited. -/',
 'namespace AnomalousPointTraceAudit','noncomputable section',
 'open AnomalousPrimeAudit AnomalousCurveDataAudit AnomalousAffineCertificateAudit',''];
for(let i=0;i<nodes.length;i++) {
 const n=nodes[i];
 out.push(`private def node_${i} : formulaCurve.Point :=`);
 if(n.infinity)out.push('  0');
 else out.push(`  .some (${n.x} : F) (${n.y} : F) (by`,
  '    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero',
  '      formula_discriminant_ne_zero).mp',
  '    rw [WeierstrassCurve.Affine.equation_iff]',
  '    simp only [formulaCurve]','    unfold F p','    reduce_mod_char)');
 out.push('');
 if(i===0){out.push('def G : formulaCurve.Point := node_0',
   'private theorem nsmul_0 : (1 : ℕ) • G=node_0 := by simp [G]','');continue;}
 const l=n.left,r=n.right,defs=[...new Set([l,r,i])].map(j=>`node_${j}`).join(',');
 out.push(`private theorem add_${i} : node_${l}+node_${r}=node_${i} := by`,
  `  dsimp only [${defs}]`);
 if(n.kind==='vertical')out.push('  apply WeierstrassCurve.Affine.Point.add_of_Y_eq',
   '  · rfl','  · simp only [formulaCurve,WeierstrassCurve.Affine.negY]',
   '    unfold F p','    reduce_mod_char','');
 else {
 out.push(`  apply ${n.kind==='tangent'?'tangent_double':'secant_add'} (ℓ:=(${n.slope} : F))`);
 out.push('  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,',
   '    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,',
   '    WeierstrassCurve.Affine.negY]',
   '  all_goals unfold F p','  all_goals reduce_mod_char',
   '  all_goals decide +kernel','');
 }
 const rewrites=[...new Set([l,r])].map(j=>`nsmul_${j}`).join(',');
 out.push(`private theorem nsmul_${i} : (${n.scalar} : ℕ) • G=node_${i} := by`,
  `  rw [show (${n.scalar} : ℕ)=${nodes[l].scalar}+${nodes[r].scalar} by norm_num,`,
  `    add_nsmul,${rewrites}]`,
  `  exact add_${i}`,'');
}
if(limitText){const i=limit-1;out.push(`theorem probe_result : (${nodes[i].scalar} : ℕ) • G=node_${i} := nsmul_${i}`,
  '#print axioms probe_result');}
else {
 const o=data.order_node,t=data.target_node;
 if(!nodes[o].infinity)throw Error('order witness is not infinity');
 out.push(`def Q : formulaCurve.Point := node_${t}`,
  `def knownScalar : ℕ := ${nodes[t].scalar}`,
  'theorem G_ne_zero : G≠0 := by simp [G,node_0]',
  'theorem p_nsmul_G : p • G=0 := by',
  `  exact nsmul_${o}`,
  'theorem G_order : addOrderOf G=p := by',
  '  apply addOrderOf_eq_prime','  · exact p_nsmul_G','  · exact G_ne_zero',
  'theorem known_answer_relation : knownScalar • G=Q := by',
  `  exact nsmul_${t}`,
  'theorem large_order_audit_certificate :',
  '    Nat.Prime p ∧ 2^159≤p ∧ p<2^160 ∧ formulaCurve.Δ≠0 ∧',
  '    addOrderOf G=p ∧ G≠0 ∧ knownScalar • G=Q ∧',
  '    formulaCurve.j=(-32768 : F) ∧ printedCurve.j≠(-32768 : F) := by',
  '  exact ⟨p_prime,p_bit_bounds.1,p_bit_bounds.2,formula_discriminant_ne_zero,',
  '    G_order,G_ne_zero,known_answer_relation,formula_j,printed_j_not_claimed⟩',
  '#print axioms G_order','#print axioms known_answer_relation',
  '#print axioms large_order_audit_certificate');
}
out.push('end','end AnomalousPointTraceAudit','');
writeFileSync(output,out.join('\n'));
