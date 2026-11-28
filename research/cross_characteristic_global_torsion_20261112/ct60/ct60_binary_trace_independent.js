#!/usr/bin/env node
// Independently audit CT60 via the Frobenius companion MATRIX and divisors.
// Unlike the Python verifier, this program never extracts eigenvalue roots.
'use strict';
const ps = [3n,5n,7n,11n,17n,19n,23n,37n,43n,163n,257n,521n,
            65537n,2147483647n,4294967291n];
const ts = [-2n,-1n,0n,1n,2n];
function chk(b,s) { if (!b) throw Error(s); }
function mod(x,p) { x%=p; return x<0n ? x+p : x; }
function factors(n) { let a=[]; for (let d=2n; d*d<=n; d+=d===2n?1n:2n) {
  if (n%d===0n) { let e=0; do {n/=d;e++;} while(n%d===0n); a.push([d,e]); }
} if (n>1n) a.push([n,1]); return a; }
function mm(a,b,p) { return [
  mod(a[0]*b[0]+a[1]*b[2],p), mod(a[0]*b[1]+a[1]*b[3],p),
  mod(a[2]*b[0]+a[3]*b[2],p), mod(a[2]*b[1]+a[3]*b[3],p)]; }
const I = [1n,0n,0n,1n];
function power(a,n,p) { let r=I; while(n) {if(n&1n) r=mm(r,a,p); a=mm(a,a,p); n>>=1n;} return r; }
function isI(a) {return a.every((x,i)=>x===I[i]); }
function divs(fs) {let d=[1n]; for(const [p,e] of fs){ const old=[...d];let x=1n;
  for(let k=1;k<=e;k++){x*=p; for(const v of old)d.push(v*x);}}
  return d.sort((a,b)=>a<b?-1:a>b?1:0);
}
function minDegree(ell,trace) {
  const M=[0n,mod(-2n,ell),1n,mod(trace,ell)];
  let order=ell*(ell*ell-1n); // An upper bound for every GL2(F_ell) element with eigenvalues.
  for(const [factor] of factors(order)) while(order%factor===0n && isI(power(M,order/factor,ell))) order/=factor;
  chk(isI(power(M,order,ell)), 'matrix order certificate');
  const ds=divs(factors(order));
  for(const f of ds){const A=power(M,f,ell);const fixed=mod((1n-A[0])*(1n-A[3])-A[1]*A[2],ell);
    if(fixed===0n){ for(const d of ds){ if(d>=f)break;const B=power(M,d,ell);
        chk(mod((1n-B[0])*(1n-B[3])-B[1]*B[2],ell)!==0n,'nonminimal');}
      return f;}}
  throw Error('missing fixed eigenvector');
}
function witness() {const out=new Map();for(let k=0;k<32;k++){
  const [a1,a2,a3,a4,a6]=[4,3,2,1,0].map(i=>(k>>i)&1);
  if(!a1 && !a3)continue;
  if(a1){const x=a3,y=x^a4;
    if(((y*y+a1*x*y+a3*y-x*x*x-a2*x*x-a4*x-a6)%2+2)%2===0)continue;}
  let n=1;for(let x=0;x<2;x++)for(let y=0;y<2;y++){
    if(((y*y+a1*x*y+a3*y-x*x*x-a2*x*x-a4*x-a6)%2+2)%2===0)n++;}
  const t=3-n;if(!out.has(t))out.set(t,[a1,a2,a3,a4,a6].join(''));
}chk(out.size===5,'trace witnesses');return out; }
const w=witness();
console.log('traces', [...w.keys()].sort((a,b)=>a-b).join(','));
console.log('curve_witnesses',ts.map(t=>String(t)+':'+w.get(Number(t))).join(';'));
for(const ell of ps) {
 chk(factors(ell).length===1 && factors(ell)[0][0]===ell, 'not prime');
 const vals=ts.map(t=>minDegree(ell,t));
 let best=vals.reduce((a,b)=>a<b?a:b);
 console.log(String(ell)+'|'+vals.join(',')+'|'+best);
}
// Distinguish a minimum f from the false statement f divides ALL admissible f.
const M=[0n,35n,1n,36n]; // trace -1, ell=37; F has eigen-orders 12 and 18
for(const f of [12n,18n]) {const A=power(M,f,37n);
 chk(mod((1n-A[0])*(1n-A[3])-A[1]*A[2],37n)===0n,'37 sanity');}
for(let f=1n;f<12n;f++){const A=power(M,f,37n);
 chk(mod((1n-A[0])*(1n-A[3])-A[1]*A[2],37n)!==0n,'37 minimality');}
console.log('counterexample_ell37_trace-1_eigen_orders=12,18');
console.log('PASS exact-order-and-small-degree-recurrence');
