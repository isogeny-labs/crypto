"use strict";
// Separate validation using scalar eigenvalues and the Frobenius recurrence.
const assert = (cond,why)=> {if(!cond) throw Error(why)};
const primes = [3,5,7,11,13,17,19,23,31,37,41,43,67,163,257];
function powmod(a,k,m){let z=1; while(k){if(k%2)z=z*a%m;a=a*a%m;k=Math.floor(k/2)}return z}
function order(a,m){let z=1;for(let i=1;i<m;i++){z=z*a%m;if(z==1)return i}throw Error("order missing")}
function pointCount(){let s=0;for(let x=0;x<2;x++)for(let y=0;y<2;y++)if(((y*y+y-x*x*x+x*x+10*x+20)%2+2)%2===0)s++;return s+1}
for(const ell of primes){const f=order(2,ell);assert((ell-1)%f===0,"bad multiplicative order");assert(powmod(2,f,ell)===1,"bad Frobenius determinant");for(let j=1;j<f;j++)assert(powmod(2,j,ell)!==1,"nonminimal f");console.log(`ell=${ell} cyclotomic_global_degree_floor=${ell-1} binary_full_basis_residue_degree_floor=${f}`)}
// Independent full GL2 census, via explicit symplectic form on columns.
for(const ell of [3,5,7]){
 let seen=0;
 for(let a=0;a<ell;a++)for(let b=0;b<ell;b++)for(let c=0;c<ell;c++)for(let d=0;d<ell;d++){
   const z=((a*d-b*c)%ell+ell)%ell;
   if(z===0)continue;
   seen++;
   const u=[a,c],v=[b,d];
   const pairing=((u[0]*v[1]-u[1]*v[0])%ell+ell)%ell;
   assert(pairing===z,"symplectic determinant violation");
 }
 assert(seen===(ell*ell-1)*(ell*ell-ell),"GL2 census");
}
assert(pointCount()===5,"wrong curve count");
// Characteristic polynomial X^2+2X+2 mod 5 has simple roots 1 and 2.
assert((1+2+2)%5===0&&(4+4+2)%5===0&&1!==2,"eigenvalues");
// Trace over extension via the recurrence t_n=-2*t_{n-1}-2*t_{n-2}.
let t0=2,t1=-2;
for(let f=1;f<=4;f++){
 if(f>1){const z=-2*t1-2*t0;t0=t1;t1=z}
 const full=powmod(2,f,5)===1; // eigenvalue 1 fixed for all f, 2 fixed only when 2^f=1.
 const fixed=full?25:5;
 const groupCount=2**f+1-t1;
 assert(groupCount%fixed===0,`inconsistent extension count ${f}`);
 assert(full===(f===4)&&groupCount===(f===4?25:5),`false full basis at ${f}`);
 console.log(`11a1 mod2 f=${f} fixed_5_torsion=${fixed} all_basis=${full}`);
}
assert(order(2,5)===4 && pointCount()===5,"incorrect rational-point implies full-basis mutant");
console.log("PASS: exact finite controls; no runtime or general DLP claim [pcc:unverified]");
