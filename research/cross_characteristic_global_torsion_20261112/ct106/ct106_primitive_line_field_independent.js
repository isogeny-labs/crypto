#!/usr/bin/env node
/* CT106 standalone independent verifier. All 11-division recurrence arithmetic
 * is reduced mod 13 before multiplication. Inverses use polynomial Euclid,
 * rather than the Python producer's integer coefficients/Fermat algorithm. */
'use strict';
function ck(c,m){if(!c)throw Error(m);}
const p=13;
const mod=n=>((n%p)+p)%p;
function trim(a){a=a.slice();while(a.length>1&&a[a.length-1]===0)a.pop();return a;}
function poly(a){return trim(a.map(mod));}
function plus(a,b){let c=Array(Math.max(a.length,b.length)).fill(0);for(let i=0;i<c.length;i++)c[i]=mod((a[i]||0)+(b[i]||0));return trim(c);}
function minus(a,b){let c=Array(Math.max(a.length,b.length)).fill(0);for(let i=0;i<c.length;i++)c[i]=mod((a[i]||0)-(b[i]||0));return trim(c);}
function times(a,b){let c=Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]=mod(c[i+j]+a[i]*b[j]);return trim(c);}
function powSmall(a,n){let z=[1];while(n){if(n%2)z=times(z,a);a=times(a,a);n=Math.floor(n/2);}return z;}
function invNum(z){for(let i=1;i<p;i++)if(mod(i*z)===1)return i;throw Error('scalar inverse');}
function qr(a,b){ck(!(b.length===1&&b[0]===0),'zero polynomial divisor');a=poly(a);b=poly(b);let q=Array(Math.max(1,a.length-b.length+1)).fill(0);let inv=invNum(b[b.length-1]);while(a.length>=b.length&&!(a.length===1&&a[0]===0)){let k=a.length-b.length,c=mod(a[a.length-1]*inv);q[k]=c;for(let i=0;i<b.length;i++)a[k+i]=mod(a[k+i]-c*b[i]);a=trim(a);}return [trim(q),a];}
const D=poly([-79,-40,-4,4]);
const f3=poly([-21,-237,-60,-4,3]);
const f4=poly([-5821,-1496,-210,-790,-100,-4,2]);
const f5=minus(times(powSmall(D,2),f4),powSmall(f3,3));
const f6=times(f3,minus(f5,powSmall(f4,2)));
const f7=minus(times(f5,powSmall(f3,3)),times(powSmall(D,2),powSmall(f4,3)));
const F=minus(times(f7,powSmall(f5,3)),times(times(powSmall(D,2),f4),powSmall(f6,3)));
ck(F.length===61&&F[60]===11,'division polynomial');
const rem=a=>qr(a,F)[1];
const addField=(a,b)=>rem(plus(a,b));
const mulField=(a,b)=>rem(times(a,b));
function powField(a,e){let z=[1];while(e>0){if(e%2)z=mulField(z,a);a=mulField(a,a);e=Math.floor(e/2);}return z;}
function invField(a){ // independent extended Euclidean inversion
 ck(!(a.length===1&&a[0]===0),'inverse zero');
 let r0=F,r1=rem(a),t0=[0],t1=[1];
 while(!(r1.length===1&&r1[0]===0)){
   const [q,r]=qr(r0,r1);
   [r0,r1]=[r1,r];
   [t0,t1]=[t1,minus(t0,times(q,t1))];
 }
 ck(r0.length===1&&r0[0]!==0,'noninvertible denominator');
 const ans=rem(times(t0,[invNum(r0[0])]));
 ck(JSON.stringify(mulField(a,ans))==='[1]','euclidean inverse mismatch');
 return ans;
}
// Rabin/Frobenius irreducibility of degree-60 polynomial over F13.
function gcd(a,b){while(!(b.length===1&&b[0]===0)){let r=qr(a,b)[1];a=b;b=r;}return poly(times(a,[invNum(a[a.length-1])]));}
const X=[0,1];let z=X;for(let i=1;i<=60;i++){
 z=powField(z,13);
 if([12,20,30].includes(i))ck(JSON.stringify(gcd(F,minus(z,X)))==='[1]','reducible mod13');
 if(i===60)ck(JSON.stringify(z)==='[0,1]','Frobenius 60 not identity');
}
const num=[X,
 minus(times(X,D),f3),
 minus(times(X,powSmall(f3,2)),times(D,f4)),
 minus(times(X,times(D,powSmall(f4,2))),times(f5,f3)),
 minus(times(X,powSmall(f5,2)),times(D,times(f6,f4)))];
const den=[[1],D,powSmall(f3,2),times(D,powSmall(f4,2)),powSmall(f5,2)];
let s=[0];for(let j=0;j<5;j++)s=addField(s,mulField(rem(num[j]),invField(rem(den[j]))));
ck(s.length===60,'nontrivial invariant');
let conjugates=[s];for(let j=1;j<=12;j++){
 const next=powField(conjugates[conjugates.length-1],13);
 if(j<12){ck(JSON.stringify(next)!==JSON.stringify(s),'early Frobenius period');conjugates.push(next);}
 else ck(JSON.stringify(next)===JSON.stringify(s),'wrong Frobenius period');
}
let coeff=[[1]];
for(const root of conjugates){
 let out=Array.from({length:coeff.length+1},()=>[0]);
 const negRoot=rem(root.map(x=>mod(-x)));
 for(let j=0;j<coeff.length;j++){
   out[j]=addField(out[j],mulField(coeff[j],negRoot));
   out[j+1]=addField(out[j+1],coeff[j]);
 }
 coeff=out;
}
ck(coeff.length===13&&coeff.every(x=>x.length===1),'minpoly coefficients not F13');
let q13=coeff.map(x=>x[0]);
ck(JSON.stringify(q13)==='[2,11,0,1,0,3,4,8,6,2,12,6,1]','unexpected degree-12 mod13 polynomial');

// F59 source kernel polynomial from eleven-torsion roots, no scalar labels.
const p59=59;
function ev59(a,x){let s=0;for(let i=a.length-1;i>=0;i--)s=((s*x+a[i])%p59+p59)%p59;return s;}
// f11 mod59 must be computed independently, because f11 mod13 was already reduced.
function add59poly(a,b){let out=Array(Math.max(a.length,b.length)).fill(0);for(let i=0;i<out.length;i++)out[i]=((a[i]||0)+(b[i]||0))%59;return trim(out);}
function sub59poly(a,b){let out=Array(Math.max(a.length,b.length)).fill(0);for(let i=0;i<out.length;i++)out[i]=((a[i]||0)-(b[i]||0)+59)%59;return trim(out);}
function mul59poly(a,b){let out=Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)out[i+j]=(out[i+j]+a[i]*b[j])%59;return trim(out);}
function pow59poly(a,n){let z=[1];while(n){if(n&1)z=mul59poly(z,a);a=mul59poly(a,a);n>>=1;}return z;}
const d59=[-79,-40,-4,4].map(x=>(x%59+59)%59);
const a59=[-21,-237,-60,-4,3].map(x=>(x%59+59)%59);
const b59=[-5821,-1496,-210,-790,-100,-4,2].map(x=>(x%59+59)%59);
const f559=sub59poly(mul59poly(pow59poly(d59,2),b59),pow59poly(a59,3));
const f659=mul59poly(a59,sub59poly(f559,pow59poly(b59,2)));
const f759=sub59poly(mul59poly(f559,pow59poly(a59,3)),mul59poly(pow59poly(d59,2),pow59poly(b59,3)));
const f1159=sub59poly(mul59poly(f759,pow59poly(f559,3)),mul59poly(mul59poly(pow59poly(d59,2),b59),pow59poly(f659,3)));
ck(f1159.length===61,'f11 over F59');
let roots=[];for(let x=0;x<59;x++)if(ev59(f1159,x)===0)roots.push(x);
ck(JSON.stringify(roots)==='[19,20,22,43,46]','source roots');
let H=[1];for(let x of roots)H=mul59poly(H,[(59-x)%59,1]);
ck(JSON.stringify(H)==='[27,19,45,31,27,1]','source invariant polynomial');
const rhs=x=>((x*x*x-x*x-10*x-20)%59+59)%59;
const mod59=x=>((x%59)+59)%59;
const inv59=x=>{for(let z=1;z<59;z++)if(z*x%59===1)return z;throw Error('inverse59');};
function point59(P,Q){
 if(P===null)return Q;if(Q===null)return P;
 const [x,y]=P,[xx,yy]=Q;
 if(x===xx&&mod59(y+yy+1)===0)return null;
 const lam=x===xx?mod59((3*x*x-2*x-10)*inv59(mod59(2*y+1))):mod59((yy-y)*inv59(mod59(xx-x)));
 const t=mod59(lam*lam+1-x-xx);return [t,mod59(-y-lam*(t-x)-1)];
}
let sourcePoints=[];for(const x of roots)for(let y=0;y<59;y++)if(mod59(y*y+y-rhs(x))===0)sourcePoints.push([x,y]);
ck(sourcePoints.length===10,'ten source points');
for(let P of sourcePoints){let q=null,xs=[];for(let j=1;j<=11;j++){q=point59(q,P);if(j<11)xs.push(q[0]);}ck(q===null,'source order');const uniq=[...new Set(xs)].sort((a,b)=>a-b);ck(JSON.stringify(uniq)===JSON.stringify(roots),'source blind invariant failed');}

// A separate bit-polynomial representation of F_(2^20) and binary curve.
const f2poly=0x130027;
function fm(a,b){let v=0;while(b){if(b&1)v^=a;b>>>=1;a<<=1;if(a&(1<<20))a^=f2poly;}return v&((1<<20)-1);}
function fp(a,e){let v=1;while(e){if(e%2)v=fm(v,a);a=fm(a,a);e=Math.floor(e/2);}return v;}
function point2(P,Q){
 if(P===null)return Q;if(Q===null)return P;
 const [x,y]=P,[xx,yy]=Q;
 if(x===xx&&((y^yy)===1))return null;
 let sl;
 if(x===xx){ck(y===yy,'binary distinct y same x');sl=fm(x,x);}
 else sl=fm(y^yy,fp(x^xx,(2**20)-2));
 const x3=fm(sl,sl)^1^x^xx;
 return [x3,y^fm(sl,x^x3)^1];
}
let pp=null,binroots=[];
for(let i=1;i<=11;i++){pp=point2(pp,[2,0]);if(i<=5)binroots.push(pp[0]);}
ck(pp===null&&new Set(binroots).size===5,'binary order 11');
let B=[1];for(let xx of binroots){let next=Array(B.length+1).fill(0);for(let j=0;j<B.length;j++){next[j]^=fm(B[j],xx);next[j+1]^=B[j];}B=next;}
ck(JSON.stringify(B)==='[471939,4952,65815,1,65814,1]','binary invariant coefficients');
ck(B.every(x=>fp(x,16)===x)&&B.some(x=>fp(x,4)!==x),'binary residue coefficient degree4');
for(const root of binroots){let v=0;for(let j=B.length-1;j>=0;j--)v=fm(v,root)^B[j];ck(v===0,'binary polynomial root');}
const orbit=[binroots[0]];
for(let j=1;j<5;j++){const next=fp(orbit[orbit.length-1],16);ck(!orbit.includes(next),'short binary quintic orbit');orbit.push(next);}
ck(fp(orbit[4],16)===orbit[0]&&JSON.stringify([...orbit].sort((a,b)=>a-b))===JSON.stringify([...binroots].sort((a,b)=>a-b)),'quintic not irreducible over F16');
const sBin=B[4];let minimal4=[1];
for(let j=0;j<4;j++){let v=fp(sBin,2**j),next=Array(minimal4.length+1).fill(0);for(let k=0;k<minimal4.length;k++){next[k]^=fm(minimal4[k],v);next[k+1]^=minimal4[k];}minimal4=next;}
ck(JSON.stringify(minimal4)==='[1,1,0,0,1]','s target F16 minimal polynomial');

const data={case:'ct106-primitive-line-field',global_x_degree:60,global_oriented_degree:120,
 fixed_line_field_degree:12,source_p:59,source_subgroup_polynomial:H,source_all_oriented_same_polynomial:true,source_invariant_s:32,
 target_p:2,target_kernel_coefficients_f2_20:B,target_coefficient_degree:4,
 target_invariant_s_f2_20:sBin,target_s_minpoly_f2:minimal4,relative_quintic_binary_frobenius_orbit:5,
 mod13_primitive_invariant_frobenius_degree:12,mod13_primitive_invariant_minpoly:q13,
 global_char0_minpoly_constructed:false,online_global_public_lift_constructed:false,
 end_to_end_advantage_measured:false};
const keys=Object.keys(data).sort();let output={};for(const k of keys)output[k]=data[k];
process.stdout.write(JSON.stringify(output)+'\n');
