// Frozen known-answer/record-shape checker. This is not the arithmetic verifier:
// subsequent Lean certificates must check the finite-ring execution witnesses.
import {readFileSync,existsSync} from 'node:fs';
import {join} from 'node:path';
const unsigned=/^(0|[1-9][0-9]*)$/;
const signed=/^(0|-?[1-9][0-9]*)$/;
const fields=['a','b','G_y_lift','Q_y_lift','G_hensel_correction','Q_hensel_correction',
 'G_hensel_inverse','Q_hensel_inverse','G_inverse_Y','Q_inverse_Y','G_phi','Q_phi',
 'G_coefficient','Q_coefficient'];
function assess(s,rc,spec) {
 if(spec.kind==='rejected')return {ok:rc===2&&s.status==='rejected'&&
   s.reason==='input point off curve',rejected:1,nondegenerate:0,degenerate:0,steps:0};
 if(rc!==0||s.schema!=='smart-known-answer/v1'||s.status!=='processed'||
   s.p_bits!==spec.bits||!Array.isArray(s.lifts)||s.lifts.length!==4)
   return {ok:false,rejected:0,nondegenerate:0,degenerate:0,steps:0};
 let ok=true,nondegenerate=0,degenerate=0,steps=0;
 for(let i=0;i<4;i++) {
   const l=s.lifts[i];
   ok&&=l.lift===i&&l.jacobian_steps===spec.steps_per_lift;
   for(const f of fields)ok&&=typeof l[f]==='string'&&unsigned.test(l[f]);
   for(const f of ['G_hensel_quotient','Q_hensel_quotient'])
     ok&&=typeof l[f]==='string'&&signed.test(l[f]);
   ok&&=l.G_trace===`lift-${i}-G.json`&&l.Q_trace===`lift-${i}-Q.json`;
   steps+=Number.isSafeInteger(l.jacobian_steps)?l.jacobian_steps:0;
   if(l.status==='nondegenerate') {
     nondegenerate++;
     ok&&=l.G_coefficient!=='0'&&l.result===spec.scalar&&
       typeof l.G_inverse_coefficient==='string'&&unsigned.test(l.G_inverse_coefficient);
   } else if(l.status==='degenerate') {
     degenerate++;
     ok&&=l.G_coefficient==='0'&&l.result===null&&l.G_inverse_coefficient===null;
   } else ok=false;
 }
 ok&&=nondegenerate>0&&s.nondegenerate===nondegenerate;
 return {ok,rejected:0,nondegenerate,degenerate,steps};
}
if(process.argv[2]==='--selftest') {
 const spec={kind:'valid',bits:3,steps_per_lift:6,scalar:'2'};
 const s={schema:'smart-known-answer/v1',status:'processed',p_bits:3,nondegenerate:4,lifts:[]};
 for(let i=0;i<4;i++) {
   const l=Object.fromEntries(fields.map(f=>[f,'1']));
   Object.assign(l,{lift:i,jacobian_steps:6,G_hensel_quotient:'-1',Q_hensel_quotient:'0',
    G_trace:`lift-${i}-G.json`,Q_trace:`lift-${i}-Q.json`,status:'nondegenerate',result:'2',G_inverse_coefficient:'1'});
   s.lifts.push(l);
 }
 const wrong=structuredClone(s);wrong.lifts[2].result='3';
 const duplicate=structuredClone(s);duplicate.lifts[2].lift=1;
 const reject={status:'rejected',reason:'input point off curve'};
 const ok=assess(s,0,spec).ok&&!assess(wrong,0,spec).ok&&!assess(duplicate,0,spec).ok&&
   !assess(s,124,spec).ok&&assess(reject,2,{kind:'rejected'}).ok&&
   !assess(reject,0,{kind:'rejected'}).ok;
 process.stdout.write(JSON.stringify({calibration:'synthetic_known_answer_shape_only',passed:ok})+'\n');
 process.exit(ok?0:1);
}
const study=process.argv[2];if(!study)throw Error('study directory required');
const expected=JSON.parse(readFileSync(join(study,'expected.json'),'utf8'));
let all=true;
for(const arm of ['small','offcurve','large']) {
 try {
   const prefix=join(study,'evidence/runs',arm);
   const rc=Number(readFileSync(prefix+'.exit','utf8').trim());
   const s=JSON.parse(readFileSync(prefix+'.stdout','utf8'));
   const result=assess(s,rc,expected[arm]);
   if(expected[arm].kind==='valid')for(let i=0;i<4;i++)for(const point of ['G','Q'])
     result.ok&&=existsSync(join(prefix+'-data',`lift-${i}-${point}.json`));
   all&&=result.ok;
   process.stdout.write(JSON.stringify({arm,exit:rc,...result,
     verdict:result.ok?'fixture_pass':'discrepancy'})+'\n');
 } catch(e) {
   all=false;process.stderr.write(arm+': '+e.message+'\n');
   process.stdout.write(JSON.stringify({arm,verdict:'not_validated'})+'\n');
 }
}
process.exit(all?0:1);
