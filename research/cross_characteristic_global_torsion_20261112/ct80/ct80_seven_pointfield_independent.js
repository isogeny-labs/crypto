#!/usr/bin/env node
'use strict';
/* CT80 independent BigInt polynomial and packed F_(2^24) control.
   Not a general log-free mapping. [pcc:unverified] */
function check(ok,msg){if(!ok)throw Error(msg);}
function trim(a){const r=a.slice();while(r.length>1&&r.at(-1)===0n)r.pop();return r;}
function plus(a,b,sign=1n){const c=[];for(let i=0;i<Math.max(a.length,b.length);i++)c.push((a[i]??0n)+sign*(b[i]??0n));return trim(c);}
function prod(a,b){const c=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]+=a[i]*b[j];return trim(c);}
function power(a,n){let z=[1n];while(n){if(n%2)z=prod(z,a);a=prod(a,a);n=Math.floor(n/2);}return z;}
const D=[-79n,-40n,-4n,4n],P3=[-21n,-237n,-60n,-4n,3n],F4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const P5=plus(prod(power(D,2),F4),power(P3,3),-1n);
const P7=plus(prod(P5,power(P3,3)),prod(power(D,2),power(F4,3)),-1n);
check(P7.length===25&&P7[24]===7n&&P7.some(x=>x===2207441511783053n),'division polynomial malformed');
const pos=(x,p)=>{let a=x%BigInt(p);return Number(a<0n?a+BigInt(p):a);};
const vals=(x,p)=>{let s=0n;for(let i=P7.length-1;i>=0;i--)s=(s*BigInt(x)+P7[i])%BigInt(p);return pos(s,p);};
check([9,27,28].every(x=>vals(x,37)===0),'missing source linear root');
check(Array.from({length:37},(_,x)=>x).filter(x=>vals(x,37)===0).join(',')==='9,27,28','unexpected source root');
function modN(x,p){return((x%p)+p)%p;}
function mtrim(a,p){let z=a.map(x=>modN(x,p));while(z.length>1&&z.at(-1)===0)z.pop();return z;}
function invN(a,p){for(let i=1;i<p;i++)if(modN(a*i,p)===1)return i;throw Error('modular inverse missing');}
function rem(a,b,p){a=mtrim(a,p);b=mtrim(b,p);check(b.some(Boolean),'zero polynomial denominator');const ib=invN(b.at(-1),p);while(a.length>=b.length&&a.some(Boolean)){const j=a.length-b.length,q=modN(a.at(-1)*ib,p);for(let i=0;i<b.length;i++)a[i+j]=modN(a[i+j]-q*b[i],p);a=mtrim(a,p);}return a;}
function mmul(a,b,p){let z=Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)z[i+j]=modN(z[i+j]+a[i]*b[j],p);return mtrim(z,p);}
function mgcd(a,b,p){while(b.some(Boolean)){const q=rem(a,b,p);a=b;b=q;}const z=invN(a.at(-1),p);return mtrim(a.map(x=>x*z),p);}
function xFrob(n,f,p){let x=[0,1];for(let i=0;i<n;i++){let z=[1],base=x,k=p;while(k){if(k&1)z=rem(mmul(z,base,p),f,p);base=rem(mmul(base,base,p),f,p);k=Math.floor(k/2);}x=z;}return x;}
function msub(a,b,p){return mtrim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>modN((a[i]??0)-(b[i]??0),p)),p);}
const p3=P7.map(x=>pos(x,3));
check(JSON.stringify(xFrob(24,p3,3))==='[0,1]','mod3 Frobenius');
for(const k of [12,8])check(JSON.stringify(mgcd(p3,msub(xFrob(k,p3,3),[0,1],3),3))==='[1]','mod3 reducible');
const f1=(1<<12)|(1<<9)|(1<<5)|(1<<2)|1;
const f2=(1<<12)|(1<<9)|(1<<5)|(1<<4)|(1<<2)|(1<<1)|1;
function bdegree(a){return a?31-Math.clz32(a):-1;}
function brem(a,b){check(b!==0,'zero bitpoly denominator');while(bdegree(a)>=bdegree(b))a^=b<<(bdegree(a)-bdegree(b));return a;}
function bprod(a,b){let z=0;while(b){if(b&1)z^=a;a<<=1;b>>>=1;}return z;}
const parity=P7.reduce((a,c,i)=>a|(Number(c&1n)<<i),0);
check(bprod(f1,f2)===parity,'bad mod2 factorization');
for(const f of [f1,f2])for(let d=1;d<=6;d++)for(let u=0;u<(1<<d);u++)check(brem(f,(1<<d)|u)!==0,'reducible degree12 factor');
const MASK=4095;
function cmul(a,b){let z=0;while(b){if(b&1)z^=a;b>>>=1;a<<=1;if(a&4096)a^=f1;}return z&MASK;}
function cpow(a,n){let z=1;while(n){if(n&1)z=cmul(z,a);a=cmul(a,a);n=Math.floor(n/2);}return z;}
const G=cmul(cmul(2,2),2)^cmul(2,2);
let tr=0,t=G;for(let i=0;i<12;i++){tr^=t;t=cmul(t,t);}check(tr===1,'Artin-Schreier polynomial split');
const EONE=1,EX=2,EY=4096;
function emul(a,b){const a0=a&MASK,a1=a>>>12,b0=b&MASK,b1=b>>>12;
 const bd=cmul(a1,b1);return(cmul(a0,b0)^cmul(bd,G))|((cmul(a0,b1)^cmul(a1,b0)^bd)<<12);}
function einv(a){const a0=a&MASK,a1=a>>>12,n=cmul(a0,a0)^cmul(a0,a1)^cmul(cmul(a1,a1),G);
 check(n!==0,'zero inverse');const z=cpow(n,4094);
 return cmul(a0^a1,z)|(cmul(a1,z)<<12);}
function epow(a,n){let o=EONE;while(n){if(n%2)o=emul(o,a);a=emul(a,a);n=Math.floor(n/2);}return o;}
function on2(P){if(P===null)return true;const[x,y]=P;return(emul(y,y)^y)===(emul(emul(x,x),x)^emul(x,x));}
function add2(U,V){if(U===null)return V;if(V===null)return U;const[x,y]=U,[z,t]=V;
 if(x===z&&(y^t)===EONE)return null;
 let lam,nu;
 if(x===z){lam=emul(x,x);nu=y^emul(lam,x);}else{lam=emul(t^y,einv(z^x));nu=y^emul(lam,x);}
 const xx=emul(lam,lam)^EONE^x^z,yy=emul(lam,xx)^nu^EONE;
 check(on2([xx,yy]),'bad binary sum');return[xx,yy];}
function on37(P){if(P===null)return true;const[x,y]=P;return modN(y*y+y-x*x*x+x*x+10*x+20,37)===0;}
function add37(U,V){if(U===null)return V;if(V===null)return U;const[x,y]=U,[z,t]=V;
 if(x===z&&modN(y+t+1,37)===0)return null;
 let lam;if(x===z)lam=modN((3*x*x-2*x-10)*invN(modN(2*y+1,37),37),37);
 else lam=modN((t-y)*invN(modN(z-x,37),37),37);
 const nu=modN(y-lam*x,37),xx=modN(lam*lam+1-x-z,37),yy=modN(-lam*xx-nu-1,37);
 check(on37([xx,yy]),'bad source sum');return[xx,yy];}
function multiples(P,add){let a=null,arr=[null];for(let i=1;i<=7;i++){a=add(a,P);arr.push(a);}
 check(arr[7]===null&&new Set(arr.slice(0,7).map(JSON.stringify)).size===7,'wrong point order');return arr.slice(0,7);}
const S=multiples([9,4],add37),B=multiples([EX,EY],add2);
check(on37(S[1])&&on2(B[1]),'invalid generator');
check(epow(EX,2**12)===EX&&epow(EY,2**12)===(EY^EONE)&&epow(EY,2**24)===EY,'wrong target degree');
function display(P){return P===null?null:[P[0]&MASK,P[0]>>>12,P[1]&MASK,P[1]>>>12];}
let sc=0,tc=0;const rows=[];
for(let j=0;j<7;j++){
 const Q=S[j],idx=S.findIndex(p=>JSON.stringify(p)===JSON.stringify(Q));
 check(idx===j,'source scan failed');sc+=idx;
 const dst=B[idx],log=B.findIndex(p=>JSON.stringify(p)===JSON.stringify(dst));
 check(log===j,'target DLP mismatch');tc+=log;
 const v=display(dst);
 rows.push({source:Q,target:v===null?null:[[v[0],v[1]],[v[2],v[3]]],recovered:log,candidates:j+1});
}
check(JSON.stringify(B[1].map(v=>epow(v,2**12)))===JSON.stringify(B[6]),'wrong-Frobenius negative control');
const result={experiment:'ct80-seven-point-field',ell:7,global_x_degree:24,global_point_degree:48,source_prime:37,source_linear_x_roots:[9,27,28],source_factor_degrees:{'1':3,'3':7},target_field_degree:24,target_x_factor_degrees:[12,12],source_generator:[9,4],target_generator:[[2,0],[0,1]],coefficient_height_bits:51,source_scan_additions_if_repeated:sc,target_scan_additions_if_repeated:tc,candidate_enumeration_used:true,global_field_precomputed:true,rows};
function stable(a){if(Array.isArray(a))return a.map(stable);if(a&&typeof a==='object')return Object.fromEntries(Object.keys(a).sort().map(k=>[k,stable(a[k])]));return a;}
process.stdout.write(JSON.stringify(stable(result))+'\n');
