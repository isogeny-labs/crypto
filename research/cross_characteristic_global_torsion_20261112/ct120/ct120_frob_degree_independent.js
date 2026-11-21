#!/usr/bin/env node
'use strict';
// CT120 independent 2x2-matrix power implementation.
function need(cond, why) { if (!cond) throw Error(why); }
function primes(bound) {
  let values=[];
  for(let q=3; q<=bound; q+=2) {
    let composite=false;
    for(let d=2;d*d<=q;d++) if(q%d===0) {composite=true;break;}
    if (!composite) values.push(q);
  }
  return values;
}
function mod(a,p) { return ((a%p)+p)%p; }
function multiply(A,B,p) {
  return [mod(A[0]*B[0]+A[1]*B[2],p),mod(A[0]*B[1]+A[1]*B[3],p),
          mod(A[2]*B[0]+A[3]*B[2],p),mod(A[2]*B[1]+A[3]*B[3],p)];
}
function scalarOrder(a,p) {
  let h=1;
  let x=mod(a,p);
  while (x!==1 && h<=p) { x=mod(x*a,p);h++; }
  need(x===1 && h<=p-1, 'not a unit');
  for (let proper=1;proper<h;proper++) if(h%proper===0) {
    let y=1;for(let j=0;j<proper;j++)y=mod(y*a,p);
    need(y!==1,'order not minimal');
  }
  return h;
}
function returned(v,p) {
  let mat=[1,0,0,1];
  let F=[0,mod(-2,p),1,mod(-2,p)];
  let firstPoint=null,firstKummer=null;
  for(let step=1;step<=4*(p-1);step++) {
    mat=multiply(F,mat,p);
    let x=mod(mat[0]*v[0]+mat[1]*v[1],p);
    let y=mod(mat[2]*v[0]+mat[3]*v[1],p);
    if(firstKummer===null && ((x===v[0]&&y===v[1]) ||
       (x===mod(-v[0],p)&&y===mod(-v[1],p)))) firstKummer=step;
    if(x===v[0]&&y===v[1]) {firstPoint=step;break;}
  }
  need(firstPoint!==null&&firstKummer!==null,'no return '+p);
  return [firstPoint,firstKummer];
}
function main() {
  need(JSON.stringify(returned([3,1],5))===JSON.stringify([1,1]),'split-case control');
  let result=[];
  for (let ell of primes(211)) {
    if(ell%4!==3)continue;
    let a=mod(-4,ell);
    let h=scalarOrder(a,ell);
    let expected=[4*h,4*(h%2===0?h/2:h)];
    for(let v of [[1,0],[0,1],[1,1]]) {
      need(JSON.stringify(returned(v,ell))===JSON.stringify(expected),'degree failure ell='+ell);
    }
    result.push([ell,h,expected[0],expected[1]]);
  }
  need(result.some(x=>JSON.stringify(x)===JSON.stringify([11,10,40,20])),'ell11');
  need(result.some(x=>JSON.stringify(x)===JSON.stringify([3,2,8,4])),'ell3');
  const B=10;
  let prod=1n;
  for(let k=1;k<=B;k++)prod*=(-4n)**BigInt(k)-1n;
  for(let [ell,h] of result) {
    let divides=prod%BigInt(ell)===0n;
    need(divides===(h<=B),'divisor equivalence');
  }
  const output={
    curve:'y^2+y=x^3+x^2 over F2; characteristic polynomial Z^2+2Z+2',
    inert_count:result.length,
    inert_primes_upto:211,
    return_degrees_ell_order_point_x:result,
    sparsity_bound_B:B,
    sparsity_bound_distinct_prime_count:B*(B+2),
    sparsity_observed_ell_upto_211:result.filter(row=>row[1]<=B).length,
    split_eigenline_ell5_point_x:[1,1]
  };
  process.stdout.write(JSON.stringify(output)+'\n');
}
main();
