#!/usr/bin/env node
'use strict';
// Independent implementation: Frobenius matrix powers -> fixed-point dimensions
// -> arithmetic Möbius inversion for primitive point periods. No point enumeration.
const primes = [3, 5, 7, 11, 13];
const traces = [-2, -1, 0, 1, 2];
const winners = {
  3: [0, 1, 2, [[1, 2], [2, 3]]],
  5: [-2, 1, 4, [[1, 4], [4, 5]]],
  7: [1, 3, 21, [[3, 2], [21, 2]]],
  11: [-1, 5, 10, [[5, 2], [10, 11]]],
  13: [2, 3, 12, [[3, 4], [12, 13]]]
};
function check(ok, msg) { if (!ok) throw Error(msg); }
function mod(x, p) { return ((x % p) + p) % p; }
function multiply(a, b, p) {
  return [mod(a[0]*b[0]+a[1]*b[2],p),mod(a[0]*b[1]+a[1]*b[3],p),
          mod(a[2]*b[0]+a[3]*b[2],p),mod(a[2]*b[1]+a[3]*b[3],p)];
}
function dimFixed(m, p) {
  const a=mod(m[0]-1,p),b=m[1],c=m[2],d=mod(m[3]-1,p);
  if (mod(a*d-b*c,p) !== 0) return 0;
  if (a===0 && b===0 && c===0 && d===0) return 2;
  return 1;
}
function record(p,t) {
  const F=[0,mod(-2,p),1,mod(t,p)];
  const I=[1,0,0,1];let M=I.slice();const fixed=[1];let period=0;
  for(let d=1;d<=p*p*p;d++) {
    M=multiply(M,F,p);
    fixed.push(p ** dimFixed(M,p));
    if(M.every((v,i)=>v===I[i])) {period=d;break;}
  }
  check(period>0,'failed matrix period');
  let cycles=[];const exact=Array(period+1).fill(0);
  let some=0, atSome=0;
  for(let d=1;d<=period;d++) {
    // Number of points of exact period d from the fixed-point counts,
    // including the zero point for d=1 only.
    if(period%d!==0)continue;
    let count=fixed[d];
    for(let k=1;k<d;k++)if(d%k===0) count-=exact[k];
    exact[d]=count;
    if(d===1)exact[d]-=1; // remove zero (its only period is one)
    // For d>1 the already subtracted exact[1] omits zero; correct it separately.
    if(d>1)exact[d]-=1;
    check(exact[d]>=0 && exact[d]%d===0,'nonintegral orbit count');
    if(exact[d]){
      cycles.push([d,exact[d]/d]);
      if(some===0){some=d;atSome=fixed[d]-1;}
    }
  }
  check(cycles.reduce((n,c)=>n+c[0]*c[1],0)===p*p-1,'point partition');
  check(atSome===p-1 || atSome===p*p-1,'fixed eigenspace');
  return {cycles,ell:p,f_full:period,f_some:some,fixed_at_some:atSome,
          total_nonzero:p*p-1,trace:t,type:'cycle'};
}
function stringify(record){
  // Explicit canonical key ordering matching Python sort_keys=True.
  return JSON.stringify(record,Object.keys(record).sort());
}
for(const p of primes){
  const results=traces.map(t=>record(p,t));
  for(const r of results)console.log(stringify(r));
  const best=results.slice().sort((a,b)=>(a.f_some-b.f_some)||(a.trace-b.trace)||(a.f_full-b.f_full))[0];
  const e=winners[p];
  check(JSON.stringify([best.trace,best.f_some,best.f_full,best.cycles])===JSON.stringify(e), 'winner mismatch');
  check(best.fixed_at_some===p-1, 'not 1D winner');
  console.log(stringify({...best,type:'winner'}));
}
