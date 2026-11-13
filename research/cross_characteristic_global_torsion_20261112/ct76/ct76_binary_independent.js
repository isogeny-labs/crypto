#!/usr/bin/env node
'use strict';
// CT76 independent matrix/trace verification for the unchanged 11a1 binary curve.
// This auditor uses exact trace recursion rather than constructing GF(2^f).
const ELLS = [3, 5, 7, 11, 13];
function check(x, msg) { if (!x) throw Error(msg); }
function mod(x, l) { return (x % l + l) % l; }
function matrixTimes(A, B, l) {
  const C = [];
  for (let i=0;i<2;i++) for(let j=0;j<2;j++) {
    let v=0;
    for (let k=0;k<2;k++) v += A[2*i+k]*B[2*k+j];
    C.push(mod(v,l));
  }
  return C;
}
function count(f) {
  let a=2,b=-2;
  for(let n=1;n<f;n++) { const c=-2*b-2*a; a=b; b=c; }
  return 2**f+1-b;
}
function frobenius(l) {
  const F=[0,mod(-2,l),1,mod(-2,l)];
  let M=[1,0,0,1], first=null, full=null;
  for(let n=1;n<=128;n++) {
    M=matrixTimes(F,M,l);
    const det=mod((M[0]-1)*(M[3]-1)-M[1]*M[2],l);
    // Modular integer recurrence avoids large unsafe JavaScript numbers.
    let t0=2%l, t1=mod(-2,l), q=1;
    for(let k=1;k<=n;k++) {
      q=mod(q*2,l);
      if(k>1) { const t2=mod(-2*t1-2*t0,l); t0=t1; t1=t2; }
    }
    check(det===mod(q+1-t1,l),'Frobenius determinant/trace contradiction');
    if(det===0 && first===null) first=n;
    if(M.every((v,i)=>v===[1,0,0,1][i])) { full=n; break; }
  }
  check(first!==null && full!==null, 'unresolved Frobenius order');
  if (l%4===3) check(first===full, 'nonsplit first-point/full mismatch');
  else {
    const roots=[];
    for(let r=0;r<l;r++) if(mod(r*r,l)===l-1) roots.push(r);
    check(roots.length===2, 'split eigenvalues missing');
    function order(a) {
      let z=1;
      for(let k=1;k<l;k++) { z=mod(z*a,l); if(z===1) return k; }
      throw Error('unresolved eigenvalue order');
    }
    function gcd(a,b) { while(b) { const t=a%b; a=b; b=t; } return a; }
    const u=order(mod(-1+roots[0],l)), v=order(mod(-1-roots[0],l));
    check(first===Math.min(u,v) && full===u/gcd(u,v)*v, 'split eigenvalue gate mismatch');
  }
  return [first,full];
}
function main() {
  const expected=[5,5,5,25,25,65,145,225];
  for(let f=1;f<=8;f++) {
    check(count(f)===expected[f-1], 'small binary point-count control');
    // Python uses exhaustive binary-field point enumeration instead.
    console.log(JSON.stringify({f,points:count(f),type:'point_count'}));
  }
  for(let i=0;i<ELLS.length;i++) {
    const ell=ELLS[i];
    const [first_some,first_full]=frobenius(ell);
    check(JSON.stringify([first_some,first_full])===
          JSON.stringify([[8,8],[1,4],[24,24],[40,40],[6,12]][i]),
          'unexpected fixed-model torsion gate');
    console.log(JSON.stringify({ell,first_full,first_some,type:'frobenius'}));
  }
}
main();
