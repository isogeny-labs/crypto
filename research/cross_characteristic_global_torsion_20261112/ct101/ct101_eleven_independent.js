#!/usr/bin/env node
'use strict';
// CT101 independent F59, F2[20]/F2[40], elliptic group-law verification.
function assert(b,msg='check failed'){if(!b)throw Error(msg);}
const P=59;
const trim=a=>{a=a.slice();while(a.length>1&&a[a.length-1]===0)a.pop();return a;};
const add=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>((a[i]||0)+(b[i]||0))%P));
const sub=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>((a[i]||0)-(b[i]||0)+P)%P));
function mul(a,b){const z=Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)z[i+j]=(z[i+j]+a[i]*b[j])%P;return trim(z);}
function powp(a,n){let r=[1];for(;n;n=Math.floor(n/2),a=mul(a,a))if(n%2)r=mul(r,a);return r;}
function invp(a){a=(a%P+P)%P;assert(a);let u=0,v=1,b=P,c=a;while(c){let q=Math.floor(b/c);[b,c]=[c,b-q*c];[u,v]=[v,u-q*v];}assert(b===1);return (u%P+P)%P;}
function mod(a,b){a=trim(a);b=trim(b);let inv=invp(b.at(-1));while(a.length>=b.length&&!(a.length===1&&a[0]===0)){let off=a.length-b.length,k=a.at(-1)*inv%P;for(let i=0;i<b.length;i++)a[i+off]=(a[i+off]-k*b[i]%P+P)%P;a=trim(a);}return a;}
function same(a,b){return a.length===b.length&&a.every((x,i)=>x===b[i]);}
function gcdp(a,b){while(!(b.length===1&&b[0]===0)){[a,b]=[b,mod(a,b)];}let t=invp(a.at(-1));return a.map(z=>z*t%P);}
function modpow(a,k,m){let r=[1];for(;k;k=Math.floor(k/2),a=mod(mul(a,a),m))if(k%2)r=mod(mul(r,a),m);return r;}
const D=[-79,-40,-4,4].map(n=>(n%P+P)%P);
const F3=[-21,-237,-60,-4,3].map(n=>(n%P+P)%P);
const F4=[-5821,-1496,-210,-790,-100,-4,2].map(n=>(n%P+P)%P);
const d2=powp(D,2), f5=sub(mul(d2,F4),powp(F3,3));
const f6=mul(F3,sub(f5,powp(F4,2)));
const f7=sub(mul(f5,powp(F3,3)),mul(d2,powp(F4,3)));
const f11=sub(mul(f7,powp(f5,3)),mul(mul(d2,F4),powp(f6,3)));
assert(f11.length===61&&f11[60]===11);
const roots=[];for(let x=0;x<P;x++)if(same(mod(f11,[(P-x)%P,1]),[0]))roots.push(x);
assert(JSON.stringify(roots)===JSON.stringify([19,20,22,43,46]));
const x59=[0,1];const q1=gcdp(f11,sub(modpow(x59,59,f11),x59));
const q5=gcdp(f11,sub(modpow(x59,59**5,f11),x59));
assert(q1.length===6&&q5.length===61);
const der=Array.from({length:60},(_,i)=>((i+1)*f11[i+1])%P);
assert(same(gcdp(f11,der),[1]));

// Distinct from F59 list algebra: carryless BigInt F2 polynomial operations.
function bitmul(a,b){let z=0n;while(b){if(b&1n)z^=a;a<<=1n;b>>=1n;}return z;}
function bitpow(a,n){let z=1n;while(n){if(n&1)z=bitmul(z,a);a=bitmul(a,a);n>>=1;}return z;}
function bitdeg(a){return a.toString(2).length-1;}
function bitrem(a,b){let d=bitdeg(b);while(a!==0n&&bitdeg(a)>=d)a^=b<<BigInt(bitdeg(a)-d);return a;}
function bitgcd(a,b){while(b){[a,b]=[b,bitrem(a,b)];}return a;}
const s3=0x13n, s5=1n^bitpow(s3,3), s6=bitpow(s3,4), s7=bitmul(s5,bitpow(s3,3))^1n;
const s11=bitmul(s7,bitpow(s5,3))^bitpow(s6,3);
const fact=[0x130027,0x131203,0x131211];
assert(s11===fact.reduce((r,f)=>bitmul(r,BigInt(f)),1n));
for(const factor of fact){let t=2n,f=BigInt(factor);for(let j=1;j<=20;j++){t=bitrem(bitmul(t,t),f);if(j===4||j===10)assert(bitgcd(t^2n,f)===1n);}assert(t===2n);}

// Each x-coordinate is in F2^20, while its oriented ordinate is Y with
// Y^2 + Y + (x^3 + x^2)=0 and trace(x^3+x^2)=1: an F2^40 extension.
const M=(1<<20)-1;
function kmul(a,b,f){let r=0;for(let i=0;i<20;i++){if(b&1)r^=a;b>>>=1;a<<=1;if(a&(1<<20))a^=f;}assert(b===0);return r&M;}
function kpow(a,n,f){let z=1;for(;n;n=Math.floor(n/2),a=kmul(a,a,f))if(n&1)z=kmul(z,a,f);return z;}
function sq(a,f){return kmul(a,a,f);}
function inv(a,f){assert(a);let v=kpow(a,(1<<20)-2,f);assert(kmul(a,v,f)===1);return v;}
function rhs(x,f){return kmul(sq(x,f),x,f)^sq(x,f);}
function on(A,f,rad){return A===null||(sq(A[1],f)^A[1]^rad)===rhs(A[0],f);}
function eql(A,B){return A===null||B===null?A===B:A[0]===B[0]&&A[1]===B[1];}
function plus(A,B,f,rad){if(A===null)return B;if(B===null)return A;let [a,b]=A,[c,d]=B;
let l;if(a===c){if((b^d)===1)return null;assert(b===d);l=sq(a,f);}else l=kmul(b^d,inv(a^c,f),f);
let x=sq(l,f)^1^a^c,y=b^kmul(l,a^x,f)^1,Z=[x,y];assert(on(Z,f,rad));return Z;}
let bs=[];
for(const factor of fact){let t=2,rad=rhs(t,factor),trace=0,u=rad;
 for(let i=0;i<20;i++){trace^=u;u=sq(u,factor);}assert(trace===1&&u===rad);
 let base=[t,0],multiples=[null];assert(on(base,factor,rad));
 for(let i=1;i<=11;i++)multiples.push(plus(multiples[i-1],base,factor,rad));
 assert(multiples[11]===null&&new Set(multiples.slice(0,11).map(v=>JSON.stringify(v))).size===11);
 for(let i=0;i<11;i++)for(let j=0;j<11;j++)assert(eql(plus(multiples[i],multiples[j],factor,rad),multiples[(i+j)%11]));
 let seen=new Set(),xx=t,yy=0;
 for(let i=0;i<40;i++){seen.add(xx+':'+yy);assert(on([xx,yy],factor,rad));xx=sq(xx,factor);yy=sq(yy,factor)^rad;if(i===19)assert(xx===t&&yy===1);}
 assert(seen.size===40&&xx===t&&yy===0);
 bs.push({factor:'0x'+factor.toString(16),group_pairs:121,order11_verified:true,oriented_orbit:40,trace_rhs:trace,x_orbit:20});
}
function curve(x){return (((x*x%P)*x-x*x-10*x-20)%P+P)%P;}
let count=1;for(let x=0;x<P;x++)for(let y=0;y<P;y++)if((y*y+y-curve(x)+2*P)%P===0)count++;assert(count===55);
function sum59(A,B){if(A===null)return B;if(B===null)return A;let [x,y]=A,[z,w]=B,lam;
 if(x===z){if((y+w+1)%P===0)return null;lam=(3*x*x-2*x-10)*invp(2*y+1)%P;}
 else lam=(w-y+P)*invp(z-x)%P;
 let tx=((lam*lam+1-x-z)%P+P)%P,ty=((-y-lam*(tx-x)-1)%P+P)%P;return [tx,ty];}
let src=[null];for(let i=1;i<=11;i++)src.push(sum59(src[i-1],[19,22]));
assert(src[11]===null&&new Set(src.slice(0,11).map(v=>JSON.stringify(v))).size===11);
let xs=[...new Set(src.slice(1,11).map(v=>v[0]))].sort((a,b)=>a-b);assert(JSON.stringify(xs)===JSON.stringify(roots));
for(let i=0;i<11;i++)for(let j=0;j<11;j++)assert(eql(sum59(src[i],src[j]),src[(i+j)%11]));
const obj={binary_factors:bs,case:'ct101-eleven-field-gate',cross_characteristic_lift_evaluated:false,ell:11,required_oriented_target_degree:40,source_11_generator:[19,22],source_group_pairs:121,source_order:count,source_p:59,source_x_factor_degrees:[1,1,1,1,1,...Array(11).fill(5)],source_x_roots:roots,target_group_pairs:363,target_p:2,target_x_factor_degrees:[20,20,20]};
function sorted(o){if(Array.isArray(o))return o.map(sorted);if(o&&typeof o==='object')return Object.fromEntries(Object.keys(o).sort().map(k=>[k,sorted(o[k])]));return o;}
process.stdout.write(JSON.stringify(sorted(obj))+'\n');
