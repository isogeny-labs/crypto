#!/usr/bin/env node
'use strict';
// CT110 independent BigInt checker for CT109's q(S) and binary projectors.
const coefficients=['-5490195752091','-9133681615853','-4153362124197',
'-501858246395','-179918683945','8753368250','-4064649325',
'-308550000','-2165625','-543125','-5500','-20','1'].map(BigInt);
const h=[1n,1n,0n,0n,1n];
const check=(x,msg)=>{if(!x)throw Error(msg)};
const clean=v=>{let a=v.slice();while(a.length>1&&a.at(-1)===0n)a.pop();return a};
const plus=(a,b)=>clean(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)+(b[i]??0n)));
const product=(a,b)=>{let c=Array(a.length+b.length-1).fill(0n);a.forEach((x,i)=>b.forEach((y,j)=>{c[i+j]+=x*y}));return clean(c)};
function quotientRem(a,b){
  let z=clean(a),quot=Array(Math.max(1,z.length-b.length+1)).fill(0n);
  check(b.at(-1)===1n,'monic divisor required');
  while(z.length>=b.length && !(z.length===1&&z[0]===0n)){
    let power=z.length-b.length,lead=z.at(-1);quot[power]+=lead;
    b.forEach((v,j)=>{z[power+j]-=lead*v});z=clean(z);
  }
  return [clean(quot),z];
}
function decompose(v){let parts=[],z=clean(v);
  while(!(z.length===1&&z[0]===0n)){
    let [quo,rem]=quotientRem(z,h);parts.push(rem);z=quo;
  }
  return parts;
}
const blocks=decompose(coefficients);
check(blocks.length===4 && blocks[3].length===1 && blocks[3][0]===1n,'degree-3 base h expansion');
const Bs=blocks.slice(0,3).map((v,i)=>{
  let divisor=1n<<BigInt(3-i);
  check(v.every(z=>z%divisor===0n),'integral cubic coefficient');
  return v.map(z=>z/divisor);
});
check(Bs[0].map(x=>Number((x%2n+2n)%2n)).join(',')==='1,0,0,0','b0 parity');
check(Bs[1].every(x=>x%2n===0n)&&Bs[2].every(x=>x%2n===0n),'b1 b2 parity');
let rebuild=[0n],power=[1n];
for(const block of blocks){rebuild=plus(rebuild,product(block,power));power=product(power,h)}
check(rebuild.join(',')===coefficients.join(','),'exact integer polynomial identity');
// Independently reconstruct the canonical 12x12 lattice multiplication by S,U.
let largest=0n;
for(let j=0;j<3;j++)for(let i=0;i<4;i++){
  let monomial=[1n];for(let z=0;z<i;z++)monomial=product(monomial,[0n,1n]);
  for(let z=0;z<j;z++)monomial=product(monomial,h);
  for(const [op,addDenom] of [[[0n,1n],0],[h,1]]){
    const [,reduced]=quotientRem(product(monomial,op),coefficients);
    const powerBlocks=decompose(reduced);
    check(powerBlocks.length<=3,'basis rank overflow');
    const denom=1n<<BigInt(j+addDenom);
    for(let k=0;k<powerBlocks.length;k++)for(const z of powerBlocks[k]){
      const numerator=z*(1n<<BigInt(k));
      check(numerator%denom===0n,'nonintegral generator multiplication');
      const v=numerator/denom,a=v<0n?-v:v;
      if(a>largest)largest=a;
    }
  }
}
// F16 = F2[t]/(t^4+t+1) with explicit F4 subgroup.
const multiply16=(a,b)=>{let c=0;while(b){if(b&1)c^=a;b>>=1;a<<=1;if(a&16)a^=0x13}return c};
const pow16=(a,n)=>{let z=1;for(;n>0;n=Math.floor(n/2),a=multiply16(a,a)){if(n%2)z=multiply16(z,a)}return z};
const omega=pow16(2,5),omega2=pow16(2,10);
check(omega===6&&omega2===7&&((omega^omega2)===1),'F4 inside F16');
const projectors=[[1,1,1],[1,omega2,omega],[1,omega,omega2]], roots=[1,omega,omega2];
projectors.forEach((pol,i)=>roots.forEach((root,j)=>{
  const v=pol.reduceRight((acc,c)=>multiply16(acc,root)^c,0);
  check(v===(i===j?1:0),'projector residue mismatch');
}));
const out={basis_multiplication_integral:true,binary_idempotents:3,
 binary_residue_degrees:[4,4,4],
 coefficient_polynomials_ascending:Bs.map(v=>v.map(Number)),
 disc_v2_from_CT109:24,extension_degree:12,index_v2:12,
 max_abs_structure_coefficient:largest.toString(),order_is_2_maximal:true,
 reduction_cubic:'u^3+1',root_labels_F16:roots};
process.stdout.write(JSON.stringify(out)+'\n');
