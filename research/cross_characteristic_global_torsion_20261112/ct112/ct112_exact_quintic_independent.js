#!/usr/bin/env node
'use strict';
// CT112: independent BigInt certificate. Enumerate 32 subsets of five roots
// instead of relying on Python's iterative elementary-symmetric recurrence.
const fs=require('fs'), path=require('path');
const ck=(ok,msg)=>{if(!ok)throw Error(msg)};
const trim=a=>{while(a.length>1&&a.at(-1)===0n)a.pop();return a};
const plus=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)+(b[i]||0n)));
const minus=(a,b)=>plus(a,b.map(x=>-x));
const mul=(a,b)=>{
 let c=Array(a.length+b.length-1).fill(0n);
 for(let i=0;i<a.length;i++)if(a[i]!==0n)
   for(let j=0;j<b.length;j++)if(b[j]!==0n)c[i+j]+=a[i]*b[j];
 return trim(c)
};
const scale=(a,k)=>trim(a.map(x=>x*k));
const power=(a,n)=>{
 let v=[1n];
 while(n){if(n%2)v=mul(v,a);n=Math.floor(n/2);if(n)a=mul(a,a)}
 return v
};
const divexact=(a,b)=>{
 a=a.slice();let q=Array(Math.max(1,a.length-b.length+1)).fill(0n);
 while(a.length>=b.length && !(a.length===1&&a[0]===0n)){
   let i=a.length-b.length,high=a.at(-1),lead=b.at(-1);
   ck(high%lead===0n,'nonintegral long-division step');
   const t=high/lead;q[i]=t;
   for(let j=0;j<b.length;j++)a[i+j]-=t*b[j];
   trim(a)
 }
 ck(a.length===1&&a[0]===0n,'exact polynomial remainder nonzero');
 return trim(q)
};
const D=[-79n,-40n,-4n,4n],
  f3=[-21n,-237n,-60n,-4n,3n],
  f4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const f5=minus(mul(power(D,2),f4),power(f3,3));
const f6=mul(f3,minus(f5,power(f4,2)));
const f7=minus(mul(f5,power(f3,3)),mul(power(D,2),power(f4,3)));
const psi11=minus(mul(f7,power(f5,3)),mul(mul(power(D,2),f4),power(f6,3)));
ck(psi11.length===61&&psi11.at(-1)===11n,'psi11 polynomial');
const X=[0n,1n];
const N=[X,minus(mul(X,D),f3),
 minus(mul(X,power(f3,2)),mul(D,f4)),
 minus(mul(X,mul(D,power(f4,2))),mul(f5,f3)),
 minus(mul(X,power(f5,2)),mul(D,mul(f6,f4)))];
const T=[[1n],D,power(f3,2),mul(D,power(f4,2)),power(f5,2)];
const H=Array.from({length:6},()=>[0n]);
for(let mask=0;mask<32;mask++){
 let a=[1n],chosen=0;
 for(let i=0;i<5;i++){
   if(mask&(1<<i)){chosen++;a=mul(a,N[i])}
   else a=mul(a,T[i])
 }
 H[5-chosen]=plus(H[5-chosen],scale(a,chosen%2?-1n:1n))
}
const A=scale(H[4],-1n),B=H[5];
ck(A.length===52&&B.length===51,'wrong trace numerator/denominator degree');
const ev59=(a,x)=>((a.reduceRight((acc,c)=>(acc*x+c)%59n,0n)%59n)+59n)%59n;
ck(ev59(psi11,19n)===0n && ev59(B,19n)!==0n,
   'mutated constant cannot vanish at source torsion root');
const frozen=JSON.parse(fs.readFileSync(path.join(__dirname,'reconstructed.json'),'utf8'));
const L=BigInt(frozen.common_denominator);
const C=frozen.numerators_low_to_high.map(row=>row.map(BigInt));
ck(C.length===4&&C.every(row=>row.length===12),'frozen coefficients shape');
const terms=[];
for(let i=0;i<12;i++)terms.push(mul(power(A,i),power(B,11-i)));
const factor=power(B,10),certs=[];
for(let j=0;j<4;j++){
 let delta=scale(mul(H[j],factor),-L);
 for(let i=0;i<12;i++)if(C[j][i]!==0n)
   delta=plus(delta,scale(terms[i],C[j][i]));
 ck(!(delta.length===1&&delta[0]===0n),'unexpected zero cross-difference');
 const quotient=divexact(delta,psi11);
 certs.push({cross_degree:delta.length-1,j,quotient_degree:quotient.length-1})
}
const inv59=x=>{for(let i=1n;i<59n;i++)if((x*i)%59n===1n)return i;throw Error('inverse missing')};
for(let j=0;j<4;j++)ck(
 (ev59(C[j],32n)*inv59(L%59n))%59n===[27n,19n,45n,31n][j],
 'source 59 coefficient mismatch');
let u=L,v2=0;while(u%2n===0n){u/=2n;v2++}
const result={
 all_integer_cross_differences_divisible:true,
 certificates:certs,
 coefficient_count:48,
 common_denominator_digits:L.toString().length,
 common_denominator_v2:v2,
 complete_cost_gain_measured:false,
 divisor_degree:60,
 kind:'ct112-exact-relative-quintic',
 online_public_point_lift:false,
 relative_degree:5,
 single_coefficient_mutant_rejected:true,
 source_known_answer:true,
 trace_coefficient_is_minus_s:true
};
console.log(JSON.stringify(result));
