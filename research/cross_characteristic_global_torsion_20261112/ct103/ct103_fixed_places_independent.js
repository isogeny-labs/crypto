#!/usr/bin/env node
'use strict';
// CT103 independently checks the same global presentation using BigInt quotient
// arithmetic and separately enumerated Frobenius orbits. Not a DLP solver.
const check=(v,msg)=>{if(!v)throw Error('CT103: '+msg);};
const trim=a=>{while(a.length>1&&a[a.length-1]===0n)a.pop();return a;};
const add=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)+(b[i]||0n)));
const sub=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)-(b[i]||0n)));
function mul(a,b){const c=Array(a.length+b.length-1).fill(0n);a.forEach((u,i)=>b.forEach((v,j)=>c[i+j]+=u*v));return trim(c);}
function power(a,n){let b=[1n];while(n){if(n%2)b=mul(b,a);a=mul(a,a);n=Math.floor(n/2);}return b;}
const S=s=>s.map(BigInt);
const D=S([-79,-40,-4,4]),f3=S([-21,-237,-60,-4,3]),f4=S([-5821,-1496,-210,-790,-100,-4,2]);
const d2=power(D,2),f5=sub(mul(d2,f4),power(f3,3));
const f6=mul(f3,sub(f5,power(f4,2)));
const f7=sub(mul(f5,power(f3,3)),mul(d2,power(f4,3)));
const f11=sub(mul(f7,power(f5,3)),mul(mul(d2,f4),power(f6,3)));
check(f11.length===61&&f11[60]===11n,'global f11');
function ev(a,x,p){let z=0n;for(let i=a.length-1;i>=0;i--)z=(z*x+a[i])%p;return (z+p)%p;}
check(ev(f11,19n,59n)===0n,'source f11 root');
check((22n*22n+22n-(19n**3n-19n**2n-10n*19n-20n))%59n===0n,'source point');
check((2n*22n+1n)%59n===45n,'source y derivative');
check(ev(f11.slice(1).map((c,i)=>BigInt(i+1)*c),19n,59n)===17n,'source f derivative');
// Unbounded binary polynomial quotient instead of Python's 20-bit bit loop.
function carry(a,b){let z=0n;while(b){if(b&1n)z^=a;a<<=1n;b>>=1n;}return z;}
function degree(a){return a.toString(2).length-1;}
function rem(a,m){let d=degree(m);while(a>0n&&degree(a)>=d)a^=m<<BigInt(degree(a)-d);return a;}
function gcd(a,b){while(b){[a,b]=[b,rem(a,b)];}return a;}
function sq(a,m){return rem(carry(a,a),m);}
function pow(a,n,m){let z=1n;while(n){if(n&1n)z=rem(carry(z,a),m);a=sq(a,m);n>>=1n;}return z;}
const g=0x130027n,h=0x131203n,j=0x131211n;
const f2=f11.reduce((r,c,i)=>r|((c&1n)<<BigInt(i)),0n);
check(f2===carry(carry(g,h),j),'all global f11 mod2 factors');
for(const factor of [g,h,j]){
 let t=2n;
 for(let i=1;i<=20;i++){t=sq(t,factor);if(i===4||i===10)check(gcd(t^2n,factor)===1n,'degree20 Rabin');}
 check(t===2n,'x^(2^20)=x');
}
const x=2n,r=rem(carry(sq(x,g),x),g)^sq(x,g);
let z=r,tr=0n;for(let k=0;k<20;k++){tr^=z;z=sq(z,g);}
check(tr===1n&&z===r,'oriented degree40 field');
function cycles(A){
 const seen=new Set(),counts=new Map();const tag=v=>v[0]+':'+v[1];
 function step(v){return [(A[0][0]*v[0]+A[0][1]*v[1])%11,(A[1][0]*v[0]+A[1][1]*v[1])%11];}
 for(let a=0;a<11;a++)for(let b=0;b<11;b++){
  if(!a&&!b)continue;
  const v=[a,b];if(seen.has(tag(v)))continue;
  let w=v,n=0;
  while(!seen.has(tag(w))){seen.add(tag(w));n++;w=step(w);}
  check(tag(w)===tag(v),'cycle closure');
  counts.set(n,(counts.get(n)||0)+1);
 }
 check(seen.size===120,'nonzero vectors');
 return [...counts.entries()].sort((a,b)=>a[0]-b[0]);
}
const src=cycles([[0,7],[1,5]]),dst=cycles([[0,9],[1,9]]);
check(JSON.stringify(src)==='[[1,10],[5,22]]','source 59 splitting');
check(JSON.stringify(dst)==='[[40,3]]','target 2 splitting');
const result={
 case:'ct103-fixed-oriented-primes',ell:11,fixed_specialization_ideals:true,
 global_oriented_degree:120,global_x_degree:60,public_point_transfer_evaluated:false,
 source_generator:[19,22],source_modulus:59,source_oriented_frobenius_cycles:src,
 source_point_scalar_recovered:false,source_y_derivative:45,target_dlp_evaluated:false,
 target_modulus:2,target_oriented_frobenius_cycles:dst,target_residue_degree:40,
 target_rhs_trace:Number(tr),target_x_factor:'0x130027'
};
function sorted(v){return Array.isArray(v)?v.map(sorted):v&&typeof v==='object'?Object.fromEntries(Object.keys(v).sort().map(k=>[k,sorted(v[k])])):v;}
process.stdout.write(JSON.stringify(sorted(result))+'\n');
