#!/usr/bin/env node
// Independent CT117 oracle: Newton divided differences over rational BigInt.
// No source-x dictionary is used in the online evaluator.
'use strict';
const MOD=59, ORDER=11, GEN=[19,22], POLY=0x130027n;
function inv(a) { let z=((a%MOD)+MOD)%MOD; if(z===0)throw Error('division by 0');
  let old=MOD,t=0,nt=1; while(z){const q=Math.floor(old/z);[old,z]=[z,old-q*z];[t,nt]=[nt,t-q*nt]}
  if(old!==1)throw Error('nonunit');return ((t%MOD)+MOD)%MOD;
}
function mod(a) {return ((a%MOD)+MOD)%MOD}
function valid(p) {if(p===null)return true; const [x,y]=p;return mod(y*y+y-x*x*x+x*x+10*x+20)===0}
function negative(p){return p===null?null:[p[0],mod(-1-p[1])]}
function plus(p,q){if(p===null)return q;if(q===null)return p;
  const [x,y]=p,[u,v]=q;let slope;
  if(x===u){if(mod(y+v+1)===0)return null;if(y!==v)throw Error('bad duplicate x');
    slope=mod((3*x*x-2*x-10)*inv(2*y+1));
  }else slope=mod((v-y)*inv(u-x));
  const xr=mod(slope*slope+1-x-u),yr=mod(-1-y+slope*(x-xr)); return [xr,yr];
}
function times(k,p){let out=null;for(;k>0;k=Math.floor(k/2)){if(k%2)out=plus(out,p);p=plus(p,p)}return out}
function equal(a,b){return JSON.stringify(a)===JSON.stringify(b)}
function mulBinary(a,b){let n=BigInt(a),m=BigInt(b),z=0n;
  for(let j=0;j<20;j++){if(m&1n)z^=n;m>>=1n;n<<=1n;if(n&(1n<<20n))n^=POLY}
  if(m)throw Error('binary width');return Number(z & ((1n<<20n)-1n))}
function frob4(a){for(let i=0;i<4;i++)a=mulBinary(a,a);return a}
function gcd(a,b){a=a<0n?-a:a;b=b<0n?-b:b;while(b){[a,b]=[b,a%b]}return a}
function frac(n,d=1n){n=BigInt(n);d=BigInt(d);if(d===0n)throw Error('zero fraction divisor');
  if(d<0n){n=-n;d=-d}const g=gcd(n,d);return [n/g,d/g]}
const fadd=(a,b)=>frac(a[0]*b[1]+b[0]*a[1],a[1]*b[1]);
const fneg=a=>[-a[0],a[1]];
const fsub=(a,b)=>fadd(a,fneg(b));
const fmul=(a,b)=>frac(a[0]*b[0],a[1]*b[1]);
const fdiv=(a,b)=>frac(a[0]*b[1],a[1]*b[0]);
function ffrom(a){return frac(BigInt(a))}
function newtonCoefficients(pairs){
  const xs=pairs.map(p=>BigInt(p[0]));let dd=pairs.map(p=>ffrom(p[1]));const n=pairs.length;
  for(let j=1;j<n;j++)for(let i=n-1;i>=j;i--)
    dd[i]=fdiv(fsub(dd[i],dd[i-1]),ffrom(xs[i]-xs[i-j]));
  let power=[ffrom(1)],coeff=Array.from({length:n},()=>ffrom(0));
  for(let j=0;j<n;j++){
    for(let k=0;k<power.length;k++)coeff[k]=fadd(coeff[k],fmul(dd[j],power[k]));
    if(j<n-1){const grow=Array.from({length:power.length+1},()=>ffrom(0));
      for(let k=0;k<power.length;k++){
        grow[k]=fsub(grow[k],fmul(power[k],ffrom(xs[j])));
        grow[k+1]=fadd(grow[k+1],power[k]);
      }power=grow;}
  }
  return coeff;
}
function evalPoly(coeff,x){let z=ffrom(0);for(let j=coeff.length-1;j>=0;j--)z=fadd(fmul(z,ffrom(x)),coeff[j]);return z}
function bsgs(q){const bound=Math.floor(Math.sqrt(ORDER))+1;
  const babies=[];let cur=null,cost=0;
  for(let j=0;j<bound;j++){babies.push(cur);cur=plus(cur,GEN);cost++}
  const step=negative(cur);cur=q;
  for(let i=0;i<=bound;i++){
    const j=babies.findIndex(v=>equal(v,cur));
    if(j!==-1)return [(i*bound+j)%ORDER,cost];
    cur=plus(cur,step);cost++;
  }throw Error('BSGS missed');
}
function mod11(v){return ((v%11)+11)%11}
if(!valid(GEN)||times(ORDER,GEN)!==null)throw Error('invalid generator');
const orbit=[2];for(let i=0;i<4;i++)orbit.push(frob4(orbit.at(-1)));
if(new Set(orbit).size!==5||frob4(orbit.at(-1))!==orbit[0])throw Error('bad orbit');
const exponents=[];let c=1;for(let j=0;j<5;j++){exponents.push(c);c=(c*7)%ORDER}
if(!equal(exponents,[1,7,5,2,3]))throw Error('bad exponent orbit');
const pairs=[];let r=GEN;
for(let k=1;k<=5;k++){
  if(k>1)r=plus(r,GEN);
  const j=exponents.findIndex(e=>e===k||mod11(-e)===k);
  if(j<0)throw Error('missing exponent');
  pairs.push([r[0],orbit[j]]);
}
if(new Set(pairs.map(p=>p[0])).size!==5)throw Error('bad abscissae');
const coeff=newtonCoefficients(pairs);
function lcm(a,b){return a/gcd(a,b)*b}
const den=coeff.reduce((acc,f)=>lcm(acc,f[1]),1n);
const num=coeff.map(f=>f[0]*(den/f[1]));
if(den!==1162512n||!equal(num.map(String),['1822729272584','-280573814794','15582488805','-368168372','3131989']))
  throw Error('coefficient certificate mismatch');
for(const [x,y] of pairs)if(!equal(evalPoly(coeff,x).map(String),[String(y),'1']))throw Error('interpolation failed');
let t=den,v2=0;while(t%2n===0n){v2++;t/=2n}if(v2!==4)throw Error('wrong v2');
function online(q){
  if(!Array.isArray(q)||q.length!==2||!q.every(v=>Number.isInteger(v)&&v>=0&&v<MOD))throw new RangeError('input');
  if(!valid(q)||times(11,q)!==null)throw new RangeError('not admitted subgroup');
  const value=evalPoly(coeff,q[0]);
  if(value[1]!==1n||value[0]<0n||value[0]>(1n<<20n)-1n)throw new RangeError('invalid bitmask');
  const binary=Number(value[0]);const matches=orbit.flatMap((x,j)=>x===binary?[j]:[]);
  if(matches.length!==1)throw new RangeError('not in Frobenius orbit');
  const e=exponents[matches[0]];
  const rec=equal(times(e,GEN),q)?e:mod11(-e);
  if(!equal(times(rec,GEN),q))throw Error('sign recovery mismatch');
  return [binary,matches[0],rec];
}
const publicCases=[],costs=[];
for(let hidden=1;hidden<11;hidden++){
  const q=times(hidden,GEN),[binary,j,rec]=online(q);
  if(rec!==hidden)throw Error('public log mismatch');
  const [control,cost]=bsgs(q);if(control!==hidden)throw Error('BSGS mismatch');
  costs.push(cost);publicCases.push([...q,binary,j,rec]);
}
const rejected=[];
for(const [name,bad] of [['off_curve',[19,23]],['outside_subgroup',[4,18]],['identity',null],['wrong_type',[19,22,0]]]){
  try{online(bad);throw Error('accepted bad '+name)}catch(e){if(!(e instanceof RangeError))throw e;rejected.push(name)}
}
const mutation=coeff.slice();mutation[0]=fadd(mutation[0],ffrom(1));
if(evalPoly(mutation,GEN[0])[0]===2n)throw Error('mutation undetected');
const output={
  all_public_logs_recovered:publicCases.length,
  binary_integer_reduction_is_field_bitmask:false,
  binary_orbit:orbit,
  case:'ct117-scalar-blind-interpolation-negative-control',
  complete_cold_cost_available:false,
  denominator_v2:v2,
  global_x_lift_certificate:false,
  mutation_detected:true,
  offline_source_class_additions:4,
  online_cold_interpolation:false,
  online_public_cases:publicCases,
  online_source_class_lookup:false,
  pcc:'unverified',
  rational_polynomial_common_denominator:Number(den),
  rational_polynomial_numerator_ascending:num.map(Number),
  rejected_controls:rejected,
  source_bsgs_costs:costs,
  source_x_target_bitmask_pairs:pairs,
  target_dlp_shortcut:false
};
console.log(JSON.stringify(output));
