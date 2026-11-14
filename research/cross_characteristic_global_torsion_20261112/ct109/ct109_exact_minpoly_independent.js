#!/usr/bin/env node
'use strict';
// CT109 independent BigInt polynomial verification, no external dependencies.
// Homogeneous expansion is termwise, separate from Python's Horner proof.
function trim(v){let a=v.slice();while(a.length>1&&a[a.length-1]===0n)a.pop();return a;}
function add(a,b){const z=Array(Math.max(a.length,b.length)).fill(0n);for(let i=0;i<a.length;i++)z[i]+=a[i];for(let i=0;i<b.length;i++)z[i]+=b[i];return trim(z);}
function sub(a,b){return add(a,b.map(v=>-v));}
function mul(a,b){let out=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)if(a[i])for(let j=0;j<b.length;j++)out[i+j]+=a[i]*b[j];return trim(out);}
function pow(a,n){let r=[1n];while(n){if(n%2)r=mul(r,a);a=mul(a,a);n=Math.floor(n/2);}return r;}
function qi(n){return n.map(x=>BigInt(x));}
function ck(x,m){if(!x)throw Error(m);}
function eq(a,b){return a.length===b.length&&a.every((v,i)=>v===b[i]);}
function divExact(A,B){
 let a=trim(A),b=trim(B);
 ck(!(b.length===1&&b[0]===0n),'zero divisor');
 let out=Array(Math.max(1,a.length-b.length+1)).fill(0n);
 while(a.length>=b.length && !(a.length===1&&a[0]===0n)){
  const k=a.length-b.length,v=a[a.length-1],w=b[b.length-1];
  ck(v%w===0n,'nonintegral quotient');
  const c=v/w;out[k]=c;const next=a.slice();
  for(let j=0;j<b.length;j++)next[j+k]-=c*b[j];
  a=trim(next);
 }
 ck(a.length===1&&a[0]===0n,'nonzero remainder');
 return trim(out);
}
const D=qi([-79,-40,-4,4]);
const f3=qi([-21,-237,-60,-4,3]);
const f4=qi([-5821,-1496,-210,-790,-100,-4,2]);
const f5=sub(mul(pow(D,2),f4),pow(f3,3));
const f6=mul(f3,sub(f5,pow(f4,2)));
const f7=sub(mul(f5,pow(f3,3)),mul(pow(D,2),pow(f4,3)));
const f11=sub(mul(f7,pow(f5,3)),mul(mul(pow(D,2),f4),pow(f6,3)));
ck(f11.length===61&&f11[60]===11n,'11-division polynomial');
const X=[0n,1n];
const N=[X,sub(mul(X,D),f3),sub(mul(X,pow(f3,2)),mul(D,f4)),
 sub(mul(X,mul(D,pow(f4,2))),mul(f5,f3)),
 sub(mul(X,pow(f5,2)),mul(D,mul(f6,f4)))];
const V=[[1n],D,pow(f3,2),mul(D,pow(f4,2)),pow(f5,2)];
let B=[1n];for(const v of V)B=mul(B,v);
let A=[0n];for(let j=0;j<5;j++)A=add(A,mul(N[j],divExact(B,V[j])));
ck(A.length===52&&B.length===51,'rational invariant degrees');
const C=qi(['-5490195752091','-9133681615853','-4153362124197','-501858246395',
'-179918683945','8753368250','-4064649325','-308550000',
'-2165625','-543125','-5500','-20','1']);
ck(C.length===13&&C[12]===1n,'candidate polynomial');
const Ap=[[1n]],Bp=[[1n]];
for(let k=1;k<=12;k++){Ap.push(mul(Ap[k-1],A));Bp.push(mul(Bp[k-1],B));}
let H=[0n];
for(let k=0;k<=12;k++){
 const term=mul(Ap[k],Bp[12-k]).map(v=>v*C[k]);
 H=add(H,term);
}
ck(H.length===613,'homogenized polynomial degree');
let Q=divExact(H,f11);ck(Q.length===553,'exact integer quotient degree');
let bits=H.reduce((a,v)=>Math.max(a,(v<0n?-v:v).toString(2).length),0);
ck(bits===1359,'homogeneous coefficient bound');
function bareiss(A0){
 const a=A0.map(row=>row.slice()),size=a.length;let prev=1n,sign=1n;
 for(let k=0;k<size-1;k++){
  if(a[k][k]===0n){
   let r=k+1;while(r<size&&a[r][k]===0n)r++;
   ck(r<size,'zero determinant');
   const old=a[k];a[k]=a[r];a[r]=old;sign=-sign;
  }
  const pivot=a[k][k];
  for(let i=k+1;i<size;i++){
   for(let j=k+1;j<size;j++){
    const t=a[i][j]*pivot-a[i][k]*a[k][j];
    ck(t%prev===0n,'fraction-free determinant');
    a[i][j]=t/prev;
   }
   a[i][k]=0n;
  }
  prev=pivot;
 }
 return sign*a[size-1][size-1];
}
function discriminant(P){
 const n=P.length-1,L=2*n-1,rows=[];
 for(let i=0;i<n-1;i++){
  let row=Array(L).fill(0n);
  for(let j=0;j<=n;j++)row[i+j]=P[n-j];
  rows.push(row);
 }
 for(let i=0;i<n;i++){
  let row=Array(L).fill(0n);
  for(let j=0;j<n;j++)row[i+j]=BigInt(n-j)*P[n-j];
  rows.push(row);
 }
 return bareiss(rows);
}
const disc=discriminant(C);
let temp=disc,v2=0;
while(temp%2n===0n){v2++;temp/=2n;}
ck(v2===24&&disc<0n&&(-disc).toString().length===195,'discriminant');
const p=59n,mod=x=>(x%p+p)%p;
for(const x of [23n,32n]){
 let acc=0n;
 for(let i=C.length-1;i>=0;i--)acc=mod(acc*x+C[i]);
 ck(acc===0n,'source root');
}
const h=[1n,1n,0n,0n,1n];
ck(eq(C.map(x=>x&1n),mul(mul(h,h),h).map(x=>x&1n)),'binary reduction');
const primes=[13,17,19,23,29,31,37,41,43,47,59];
const M=primes.reduce((a,b)=>a*BigInt(b),1n);
ck(M===15704977131048049n,'CRT modulus');
const obj={
 binary_reduction_h_cubed:true,
 case:'ct109-characteristic-zero-line-minpoly',
 coefficients_low_to_high:C.map(Number),
 crt_modulus:M.toString(),
 crt_primes:primes,
 degree:12,
 discriminant_v2:v2,
 end_to_end_advantage_measured:false,
 global_public_lift_constructed:false,
 homogenized_degree:H.length-1,
 homogenized_max_coefficient_bits:bits,
 independent_held_out_primes:[61,79,101],
 index_at_2_valuation:v2/2,
 integer_identity_certificate:true,
 integer_quotient_degree:Q.length-1,
 source_59_roots:[23,32]
};
const sorted=Object.fromEntries(Object.keys(obj).sort().map(k=>[k,obj[k]]));
process.stdout.write(JSON.stringify(sorted)+'\n');
