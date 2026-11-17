#!/usr/bin/env node
'use strict';
// Independent CT119 arithmetic: exhaustive degree-one/two factor exclusion
// over F16, BigInt polynomial-basis F_(2^20), and source polynomial product.
const q = [
  [9,6,3,1,2,1],
  [4,10,15,7,2,1],
  [15,15,13,6,2,1]
];
const srcRoots = [19,20,43,46,22];
const binaryRoots = [2,17,4956,65536,4697];
function check(test, message) { if (!test) throw Error(message); }
function gf16mul(x, y) {
  let a=x,b=y,r=0;
  while(b){
    if(b&1)r^=a;
    b>>=1; a<<=1;
    if(a&16)a^=0x13;
  }
  return r;
}
function eval16(poly, x) {
  let result=0;
  for(let i=poly.length-1;i>=0;i--) result=gf16mul(result,x)^poly[i];
  return result;
}
function quadraticDivides(poly, a, b) {
  const v=poly.slice();
  for(let i=v.length-1;i>=2;i--) {
    const t=v[i];
    v[i]=0;
    v[i-1]^=gf16mul(t,a);
    v[i-2]^=gf16mul(t,b);
  }
  return v[0]===0 && v[1]===0;
}
function degFiveIrreducible(poly){
  check(poly.length===6 && poly[5]===1,'nonmonic degree five');
  for(let x=0;x<16;x++) if(eval16(poly,x)===0) return false;
  for(let a=0;a<16;a++)for(let b=0;b<16;b++)
    if(quadraticDivides(poly,a,b))return false;
  return true;
}
const irreducible = q.map(degFiveIrreducible);
check(irreducible.every(Boolean), 'binary fibre not degree five');
check(!degFiveIrreducible([0,...q[0].slice(1)]), 'mutation not rejected');
let sc=[1];
for(const root of srcRoots){
  const next=Array(sc.length+1).fill(0);
  for(let i=0;i<sc.length;i++){
    next[i]=(next[i] - sc[i]*root) % 59;
    if(next[i]<0)next[i]+=59;
    next[i+1]=(next[i+1]+sc[i])%59;
  }
  sc=next;
}
check(sc.join(',')==='27,19,45,31,27,1' && new Set(srcRoots).size===5, 'source fibre');
const pol=BigInt(0x130027), top=1n<<20n;
function mul20(x,y){
  let a=BigInt(x),b=BigInt(y),r=0n;
  for(;b!==0n;b>>=1n){
    if(b&1n)r^=a;
    a<<=1n;
    if(a&top)a^=pol;
  }
  return r;
}
function pow20(x,e){
  let base=BigInt(x),n=BigInt(e),v=1n;
  for(;n;n>>=1n){
    if(n&1n)v=mul20(v,base);
    base=mul20(base,base);
  }
  return v;
}
const sr=binaryRoots.reduce((acc,v)=>acc^v,0);
check((pow20(sr,4n)^BigInt(sr)^1n)===0n, 's polynomial');
check(pow20(sr,16n)===BigInt(sr), 's subfield membership');
function embed16(c){
  let v=0n,sPower=1n;
  for(let bit=0;bit<4;bit++){
    if(c & (1<<bit)) v^=sPower;
    sPower=mul20(sPower,BigInt(sr));
  }
  return Number(v);
}
let result=[1n];
for(const rr of binaryRoots){
  let out=Array(result.length+1).fill(0n);
  for(let i=0;i<result.length;i++){
    out[i]^=mul20(result[i],BigInt(rr));
    out[i+1]^=result[i];
  }
  result=out;
}
const binaryCoeffs=result.map(Number);
check(binaryCoeffs.join(',')===q[0].map(embed16).join(','), 'target product mismatch');
let orbit=[];
for(let i=0;i<5;i++)orbit.push(Number(pow20(2n,16n**BigInt(i))));
check(new Set(orbit).size===5 && orbit.every(v=>binaryRoots.includes(v)), 'wrong orbit');
check(pow20(2n,16n**5n)===2n, 'wrong orbit period');
const output={
  binary_F16_polynomials:q,
  binary_quintic_irreducible:irreducible,
  fixed_binary_F16_generator_mask:sr,
  fixed_binary_orbit_masks:orbit,
  fixed_binary_product_coeff_masks:binaryCoeffs,
  mutation_rejected:true,
  scope:'invariant-base-field root exclusion only; not a cost bound or DLP transfer',
  source_modulus:59,
  source_quintic_coeff:sc,
  source_root_count:srcRoots.length
};
process.stdout.write(JSON.stringify(output)+'\n');
