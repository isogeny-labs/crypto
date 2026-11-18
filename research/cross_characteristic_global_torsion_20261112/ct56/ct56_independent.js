#!/usr/bin/env node
// CT56: independent BigInt exact integer gates (no floating point).
'use strict';
function check(p, label) { if (!p) throw new Error(label); }
function sqrtFloor(v) {
  if (v < 0n) throw Error('negative sqrt');
  if (v < 2n) return v;
  let hi = 1n << BigInt(Math.ceil(v.toString(2).length / 2));
  while (true) {
    const x = (hi + v / hi) >> 1n;
    if (x >= hi) return hi;
    hi = x;
  }
}
function basis(f) {
  check(f >= 1, 'f');
  const q = 2n ** BigInt(f);
  return q + 1n + sqrtFloor(4n*q);
}
function minGenus(n, f) {
  let prod = 1n, g = 0;
  while (prod < n) {prod *= basis(f); g++;}
  return g;
}
function minExtension(n, g) {
  let f = 1;
  while (basis(f) ** BigInt(g) < n) f++;
  return f;
}
function powMod(a, b, m) {
  let z=1n;
  while(b) {if(b%2n)z=(z*a)%m; a=(a*a)%m; b=b/2n;}
  return z;
}
function full(ell, f) {
  return ell**2n <= basis(f) && powMod(2n, BigInt(f), ell) === 1n;
}
function multiplicativeOrder(ell) {
  let r=2n % ell;
  for(let j=1;j<=Number(ell);j++) {
    if(r===1n)return j;
    r=(r*2n)%ell;
  }
  throw Error('not prime ell');
}
function countF2(a) {
  let total=1;
  for(let x=0;x<2;x++) for(let y=0;y<2;y++) {
    if(((y*y+y-x*x*x-a*x)%2+2)%2===0)total++;
  }
  return total;
}
check(countF2(0)===3 && countF2(1)===5,'curve controls');
check([1,2,3,4].every((f,i)=>basis(f)===[5n,9n,14n,25n][i]), 'basis');
for(const [bits,n] of [[127,(2n**127n)-1n], [255,(2n**255n)-19n]]) {
  const gs=[1,2,4,8].map(f=>minGenus(n,f));
  const fs=[1,2,3,4].map(g=>minExtension(n,g));
  for (const [i,g] of gs.entries()) {
    check(basis([1,2,4,8][i])**BigInt(g) >=n, 'g lower');
    check(g===0 || basis([1,2,4,8][i])**BigInt(g-1)<n,'g minimal');
  }
  for (const [i,f] of fs.entries()) {
    const g=i+1;
    check(basis(f)**BigInt(g)>=n, 'f lower');
    check(f===1||basis(f-1)**BigInt(g)<n, 'f minimal');
  }
  console.log('bits',bits,'minimum g for f=1,2,4,8',gs,
              'minimum f for g=1,2,3,4',fs);
}
check(!full(3n,1)&&full(3n,2),'full 3');
check(!full(5n,2)&&full(5n,4),'full 5');
// E0:F^2=[-2], hence F^4=[4] !== identity on 5-torsion,
// although the universal F16 full-level necessary gate admits it.
check(powMod(-2n+5n,2n,5n)===4n, 'necessary not sufficient');
for(const ell of [3,5,7,11,13,17,19,23,31,43]) {
  const l=BigInt(ell); let f=1;
  while (!full(l,f)) f++;
  check(f % multiplicativeOrder(l) ===0, 'order condition');
  check(Array.from({length:f-1},(_,j)=>j+1).every(j=>!full(l,j)),'minimality');
  console.log('ell',ell,'f_min_full_necessary',f,'ord_ell(2)',multiplicativeOrder(l));
}
console.log('PASS: exact Serre/Weil pairing gates; no existence or runtime claim');
