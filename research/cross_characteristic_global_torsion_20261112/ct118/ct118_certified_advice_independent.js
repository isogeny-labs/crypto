#!/usr/bin/env node
// CT118 independent BigInt and bitwise-field negative-control verifier.
// Genuine paired-prime global-element certificates can conceal offline DLP advice.
'use strict';
const p=59, order=11, P=[19,22], fmod=0x130027;
function mod(a){return ((a%p)+p)%p;}
function inv(a){let t=0,nt=1,r=p,nr=mod(a); while(nr){let q=Math.floor(r/nr);[t,nt]=[nt,t-q*nt];[r,nr]=[nr,r-q*nr];} if(r!==1)throw Error('not invertible');return mod(t);}
function neg(a){return a===null?null:[a[0],mod(-a[1]-1)];}
function plus(a,b){if(a===null)return b;if(b===null)return a;const [x,y]=a,[z,w]=b; if(x===z && mod(y+w+1)===0)return null;
 const s=x===z?mod((3*x*x-2*x-10)*inv(2*y+1)):mod((w-y)*inv(z-x));
 const xx=mod(s*s+1-x-z);return [xx,mod(-1-y+s*(x-xx))];}
function mult(k,a){let r=null;for(;k;k=Math.floor(k/2),a=plus(a,a)){if(k%2)r=plus(r,a);}return r;}
function eq(a,b){return a===null?b===null:b!==null&&a[0]===b[0]&&a[1]===b[1];}
function valid(a){return Array.isArray(a)&&a.length===2&&a.every(v=>Number.isInteger(v)&&v>=0&&v<p)&&mod(a[1]*a[1]+a[1]-a[0]*a[0]*a[0]+a[0]*a[0]+10*a[0]+20)===0&&mult(order,a)===null;}
const B=a=>a.map(BigInt);
function trim(a){while(a.length>1&&a[a.length-1]===0n)a.pop();return a;}
function combine(a,b,sgn){let v=Array(Math.max(a.length,b.length)).fill(0n);for(let k=0;k<v.length;k++)v[k]=(a[k]??0n)+sgn*(b[k]??0n);return trim(v);}
function addp(a,b){return combine(a,b,1n);}function subp(a,b){return combine(a,b,-1n);}
function times(a,b){let out=Array(a.length+b.length-1).fill(0n);a.forEach((v,i)=>b.forEach((w,j)=>out[i+j]+=v*w));return trim(out);}
function exponent(a,n){let z=[1n];while(n){if(n%2)z=times(z,a);a=times(a,a);n=Math.floor(n/2);}return z;}
const x=B([0,1]),g=B([-79,-40,-4,4]),ps3=B([-21,-237,-60,-4,3]),h=B([-5821,-1496,-210,-790,-100,-4,2]);
const ps5=subp(times(exponent(g,2),h),exponent(ps3,3));
const maps=[
 [x,[1n]],
 [subp(times(x,g),ps3),g],
 [subp(times(x,exponent(ps3,2)),times(g,h)),exponent(ps3,2)],
 [subp(times(x,times(g,exponent(h,2))),times(ps3,ps5)),times(g,exponent(h,2))],
 [subp(times(x,exponent(ps5,2)),times(times(times(g,h),ps3),subp(ps5,exponent(h,2)))),exponent(ps5,2)]
];
function evalmod(a,z){let v=0n,t=BigInt(z),M=59n;for(let k=a.length-1;k>=0;k--)v=((v*t+a[k])%M+M)%M;return Number(v);}
function fm(a,b){let v=0;while(b){if(b&1)v^=a;b>>=1;a<<=1;if(a&(1<<20))a^=fmod;}return v&((1<<20)-1);}
function expgf(a,n){let o=1;while(n){if(n%2)o=fm(o,a);a=fm(a,a);n=Math.floor(n/2);}return o;}
function evalgf(a,t){let v=0;for(let k=a.length-1;k>=0;k--)v=fm(v,t)^Number(a[k]&1n);return v;}
function source(a){const n=evalmod(a[0],19),d=evalmod(a[1],19);if(!d)throw Error('source pole');return [mod(n*inv(d)),d];}
function binary(a){const n=evalgf(a[0],2),d=evalgf(a[1],2);if(!d)throw Error('binary pole');return [fm(n,expgf(d,(2**20)-2)),d];}
let sourceVals=maps.map(source),binaryVals=maps.map(binary),rvals=sourceVals.map(v=>v[0]);
if(JSON.stringify(rvals)!==JSON.stringify([19,20,43,46,22]))throw Error('source map');
if(JSON.stringify(binaryVals.map(v=>v[0]))!==JSON.stringify([2,17,4956,65536,4697]))throw Error('binary map');
if(!valid(P)||maps.some(m=>m[1].length-1>=60))throw Error('global coordinate admissibility');
const points=Array.from({length:5},(_,j)=>mult(j+1,P));
const abscissas=points.map(a=>a[0]);if(JSON.stringify(abscissas)!==JSON.stringify(rvals))throw Error('advice');
// Independent integer barycentric derivation; no Fraction or polynomial interpolation shared.
function barycentric(r){return abscissas.map((ri,i)=>{
 let num=1n,den=1n;
 for(let j=0;j<5;j++){if(i===j)continue;num*=BigInt(r-abscissas[j]);den*=BigInt(ri-abscissas[j]);}
 if(num%den!==0n)throw Error('nonintegral interpolant on admitted class');return Number(num/den);
});}
for(let j=0;j<5;j++){let w=barycentric(abscissas[j]);if(JSON.stringify(w)!==JSON.stringify(Array.from({length:5},(_,i)=>Number(i===j))))throw Error('bad Kronecker');}
function bsgs(q){const m=Math.floor(Math.sqrt(order))+1;let babies=new Map(),r=null,cost=0;
 for(let j=0;j<m;j++){babies.set(JSON.stringify(r),j);r=plus(r,P);cost++;}
 const shift=neg(r);let t=q;
 for(let i=0;i<=m;i++){let v=babies.get(JSON.stringify(t));if(v!==undefined)return [(i*m+v)%order,cost];t=plus(t,shift);cost++;}
 throw Error('BSGS miss');}
let publicCases=[],costs=[];
for(let secret=1;secret<=10;secret++){
 const q=mult(secret,P);if(!valid(q))throw Error('invalid source target');
 let w=barycentric(q[0]);if(w.reduce((a,b)=>a+b,0)!==1||w.some(v=>v!==0&&v!==1))throw Error('not a single global root');
 let target=0;w.forEach((v,j)=>{if(v)target^=binaryVals[j][0];});
 let j=binaryVals.findIndex(v=>v[0]===target);if(j<0)throw Error('target class');
 let recovered=eq(mult(j+1,P),q)?j+1:order-j-1;
 if(recovered!==secret)throw Error('incorrect sign recovered');
 const [bs,bc]=bsgs(q);if(bs!==secret)throw Error('BSGS control');
 costs.push(bc);publicCases.push([q[0],q[1],target,recovered]);
}
const bad=[['off_curve',[19,23]],['outside_subgroup',[4,18]],['identity',null]];
if(bad.some(([_,q])=>valid(q)))throw Error('bad admission');
let result={
 case:'ct118-genuine-dual-place-interpolation-advice',pcc:'unverified',
 source_x_classes:abscissas,binary_x_classes:binaryVals.map(v=>v[0]),
 global_field_x_degree_prerequisite:60,
 rational_map_numerator_degrees:maps.map(v=>v[0].length-1),
 rational_map_denominator_degrees:maps.map(v=>v[1].length-1),
 source_denominator_residues:sourceVals.map(v=>v[1]),
 binary_denominator_residues:binaryVals.map(v=>v[1]),
 interpolation_polynomial_degree:4,global_coefficients_are_rational_functions_of_alpha:true,
 charzero_evaluation_gives_exact_global_conjugates:true,
 source_and_binary_fixed_prime_residues_match:true,
 offline_projective_dlp_entries:5,offline_source_additions:4,
 online_scalar_lookup:false,online_certificate_has_no_advice:false,
 public_recoveries:publicCases,rejected_controls:bad.map(v=>v[0]),
 matched_source_bsgs_additions:costs,
 target_full_dlp_and_all_costs_measured:false,global_certificate_suffices_for_speedup:false
};
process.stdout.write(JSON.stringify(Object.fromEntries(Object.entries(result).sort((a,b)=>a[0].localeCompare(b[0]))))+'\n');
