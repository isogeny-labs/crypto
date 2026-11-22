#!/usr/bin/env node
/* CT123 independent BigInt Weierstrass division-polynomial evaluator and
   modular Frobenius/gcd irreducibility certificate. No npm dependencies. */
'use strict';
const crypto=require('crypto');
const assert=require('assert');

const clean = a => {let b=a.slice();while(b.length>1&&b[b.length-1]===0n)b.pop();return b;};
const add=(a,b)=>clean(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)+(b[i]||0n)));
const sub=(a,b)=>add(a,b.map(v=>-v));
const mul=(a,b)=>{const r=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)r[i+j]+=a[i]*b[j];return clean(r)};
const square=a=>mul(a,a);
const cube=a=>mul(mul(a,a),a);
const fourth=a=>square(square(a));
function psi11(v){
  const [a1,a2,a3,a4,a6]=v.map(BigInt);
  const b2=a1*a1+4n*a2,b4=2n*a4+a1*a3,b6=a3*a3+4n*a6;
  const b8=a1*a1*a6+4n*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4;
  const C=[b6,2n*b4,b2,4n];
  const u=[b8,3n*b6,3n*b4,b2,3n];
  const v4=[b4*b8-b6*b6,b2*b8-b4*b6,10n*b8,10n*b6,5n*b4,b2,2n];
  const a5=sub(mul(v4,square(C)),cube(u));
  const a6poly=mul(u,sub(a5,square(v4)));
  const a7=sub(mul(a5,cube(u)),mul(cube(v4),square(C)));
  return sub(mul(a7,cube(a5)),mul(mul(v4,cube(a6poly)),square(C)));
}
const pos=(n,p)=>((n%p)+p)%p;
const normalize=a=>{let b=a.slice();while(b.length>1&&b[b.length-1]===0)b.pop();return b;};
const toMod=(a,p)=>normalize(a.map(x=>pos(Number(x%BigInt(p)),p)));
function inv(n,p){let [old,r]=[1,0],[a,b]=[n,p];while(b!==0){let q=Math.floor(a/b);[a,b]=[b,a-q*b];[old,r]=[r,old-q*r];}assert.equal(a,1);return pos(old,p)}
function reduce(a,f,p){const b=normalize(f.map(v=>pos(v,p)));let r=normalize(a.map(v=>pos(v,p)));const leadInv=inv(b[b.length-1],p);
 while(r.length>=b.length && !(r.length===1&&r[0]===0)){
  const shift=r.length-b.length,co=pos(r[r.length-1]*leadInv,p);
  for(let i=0;i<b.length;i++)r[i+shift]=pos(r[i+shift]-co*b[i],p);
  r=normalize(r);
 }
 return r;
}
function mmul(a,b,f,p){const o=Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)if(a[i]!==0)for(let j=0;j<b.length;j++)o[i+j]+=a[i]*b[j];return reduce(o,f,p)}
function powMod(a,e,f,p){let r=[1],v=a.slice();while(e>0){if(e%2===1)r=mmul(r,v,f,p);e=Math.floor(e/2);if(e>0)v=mmul(v,v,f,p)}return r;}
function monicGCD(a,b,p){let u=normalize(a.map(v=>pos(v,p))),v=normalize(b.map(v=>pos(v,p)));
 while(!(v.length===1&&v[0]===0)){let r=reduce(u,v,p);u=v;v=r;}
 const z=inv(u[u.length-1],p);return normalize(u.map(v=>pos(v*z,p)));
}
function frobDegrees(fraw,p,ks){const f=toMod(fraw,p),max=Math.max(...ks);const mark=new Set(ks);const out={};
 let h=[0,1];
 // Direct successive q-th-power Frobenius; no factorization library.
 const derivative=normalize(f.slice(1).map((v,i)=>pos(v*(i+1),p)));
 assert.equal(monicGCD(f,derivative,p).length-1,0);
 for(let k=1;k<=max;k++){
  h=powMod(h,p,f,p);
  if(mark.has(k)){
   const hMinusX=h.slice();while(hMinusX.length<2)hMinusX.push(0);
   hMinusX[1]=pos(hMinusX[1]-1,p);
   out[String(k)]=monicGCD(f,normalize(hMinusX),p).length-1;
  }
 }
 return [out,JSON.stringify(h)===JSON.stringify([0,1])];
}
const curves={source:[0,-1,1,-10,-20],plus:[59,-1,60,-10,39],minus:[59,58,60,-10,39]};
const polys=Object.fromEntries(Object.entries(curves).map(([k,v])=>[k,psi11(v)]));
assert.equal(JSON.stringify(toMod(polys.source,59)),JSON.stringify(toMod(polys.plus,59)));
assert.equal(JSON.stringify(toMod(polys.source,59)),JSON.stringify(toMod(polys.minus,59)));
const roots=[];
for(let x=0;x<59;x++){let v=0;const coeffs=toMod(polys.source,59);for(let i=coeffs.length-1;i>=0;i--)v=pos(v*x+coeffs[i],59);if(v===0)roots.push(x);}
assert.deepEqual(roots,[19,20,22,43,46]);
const output={ct:'CT123',models:{},source_roots_mod59:roots};
for(const [name,f] of Object.entries(polys)){
 assert.equal(f.length,61);assert.equal(f[60],11n);
 const witness=(name==='plus'?73:(name==='minus'?29:null));
 const a=frobDegrees(f,2,[1,2,5,10,20])[0];
 const b=frobDegrees(f,59,[1,5])[0];
 assert.deepEqual(b,{'1':5,'5':60});
 const ex=name==='source'?{'1':0,'2':0,'5':0,'10':0,'20':60}:{'1':0,'2':0,'5':10,'10':60,'20':60};
 assert.deepEqual(a,ex);
 const entry={degree:f.length-1,frob_gcd_degrees_2:a,frob_gcd_degrees_59:b,
  max_coefficient_digits:Math.max(...f.map(v=>v.toString().replace('-','').length)),
  sha256_high_to_low:crypto.createHash('sha256').update(f.slice().reverse().map(v=>v.toString()).join(',')).digest('hex')};
 if(witness){const [c,identity]=frobDegrees(f,witness,[12,20,30,60]);assert.deepEqual(c,{'12':0,'20':0,'30':0,'60':60});assert(identity);entry.irreducible_mod_prime=witness;entry.rabin_gcd_degrees=c;}
 output.models[name]=entry;
}
const canonical=x=>Array.isArray(x)?'['+x.map(canonical).join(',')+']':x&&typeof x==='object'?'{'+Object.keys(x).sort().map(k=>JSON.stringify(k)+':'+canonical(x[k])).join(',')+'}':JSON.stringify(x);
process.stdout.write(canonical(output)+'\n');
