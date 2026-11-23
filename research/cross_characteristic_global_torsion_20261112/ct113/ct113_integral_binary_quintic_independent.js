#!/usr/bin/env node
'use strict';
// CT113 independent BigInt h-adic arithmetic, F16 irreducibility,
// and fixed CT103 residue evaluation. No scalar lookup.
const fs=require('node:fs');
const path=require('node:path');
const data=JSON.parse(fs.readFileSync(path.join(__dirname,'../ct112/reconstructed.json'),'utf8'));
const check=(condition,msg)=>{if(!condition)throw Error(msg)};
const L=BigInt(data.common_denominator);
check(L%4n===0n&&(L/4n)%2n===1n,'v2(L) != 2');
const C=data.numerators_low_to_high.map(a=>a.map(BigInt));
check(C.length===4&&C.every(a=>a.length===12),'coefficient count');
const trim=a=>{while(a.length>1&&a[a.length-1]===0n)a.pop();return a};
const h=[1n,1n,0n,0n,1n];
function divH(a){
 a=trim(a.slice());const q=Array(Math.max(1,a.length-4)).fill(0n);
 while(a.length>=5&&a.some(z=>z!==0n)){
   const shift=a.length-5,lead=a[a.length-1];
   q[shift]=lead;
   for(let j=0;j<5;j++)a[shift+j]-=lead*h[j];
   trim(a);
 }
 return [trim(q),a];
}
const expanded=C.map(a=>{
 let out=[];
 while(a.some(x=>x!==0n)){
   let [q,r]=divH(a);out.push(r);a=q;
 }
 return out;
});
const powers=[4n,2n,1n];
const parity=expanded.map(rows=>{
 check(rows.length===3&&rows.every(r=>r.length<=4),'wrong field basis');
 return rows.map((row,k)=>{
   check(row.every(z=>z%powers[k]===0n),'not integral at 2');
   return row.map(z=>Number((z/powers[k])&1n));
 });
});
const mutant=C[0].slice();mutant[0]+=1n;
let mr=divH(mutant)[1];
check(mr.some(z=>z%4n!==0n),'mutant passed');

function mul16(a,b){
 let z=0;while(b){if(b&1)z^=a;b>>=1;a<<=1;if(a&16)a^=0x13;}return z;
}
function pow16(a,n){
 let z=1;while(n){if(n&1)z=mul16(z,a);a=mul16(a,a);n>>=1;}return z;
}
function inv16(a){check(a!==0,'zero inverse');let z=pow16(a,14);check(mul16(a,z)===1,'wrong F16 inverse');return z;}
function horner(a,x){let r=0;for(let i=a.length-1;i>=0;i--)r=mul16(r,x)^a[i];return r;}
function polytrim(a){a=a.slice();while(a.length>1&&a.at(-1)===0)a.pop();return a;}
function plus(a,b){let o=Array(Math.max(a.length,b.length)).fill(0);a.forEach((x,i)=>o[i]^=x);b.forEach((x,i)=>o[i]^=x);return polytrim(o);}
function multiply(a,b){let o=Array(a.length+b.length-1).fill(0);a.forEach((x,i)=>b.forEach((y,j)=>{o[i+j]^=mul16(x,y)}));return polytrim(o);}
function remainder(a,b){
 a=a.slice();while(a.length>=b.length&&!(a.length===1&&a[0]===0)){
   let z=mul16(a.at(-1),inv16(b.at(-1))),d=a.length-b.length;
   b.forEach((v,i)=>{a[d+i]^=mul16(v,z)});
   a=polytrim(a);
 }
 return a;
}
function gcd(a,b){while(!(b.length===1&&b[0]===0)){let r=remainder(a,b);a=b;b=r;}const iz=inv16(a.at(-1));return a.map(x=>mul16(x,iz));}
function powmod(a,n,m){let v=[1];while(n>0){if(n%2)v=remainder(multiply(v,a),m);a=remainder(multiply(a,a),m);n=Math.floor(n/2);}return v;}
const equals=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const s=2,w=mul16(s,s)^s,w2=mul16(w,w);
check(w===6&&w2===7,'not cubic unit roots');
const vals=[1,w,w2].map(u=>{
 let c=parity.map(levels=>{
   let z=0,v=1;
   for(const p of levels){z^=mul16(horner(p,s),v);v=mul16(v,u);}
   return z;
 });
 let P=[...c,s,1],Z=[0,1];
 check(equals(gcd(P,plus(powmod(Z,16,P),Z)),[1]),'has degree-one factor');
 check(equals(gcd(P,plus(powmod(Z,256,P),Z)),[1]),'has degree-two factor');
 check(equals(powmod(Z,16**5,P),Z),'not degree-five irreducible');
 return P;
});
check(equals(vals,[[9,6,3,1,2,1],[4,10,15,7,2,1],[15,15,13,6,2,1]]),'binary specializations changed');

// Distinct source: verify the integral reduction on the CT103 binary x-root
// independently from CT112's large-integer exact division certificate.
const g=0x130027;
function gf20mul(a,b){let r=0;while(b){if(b&1)r^=a;b>>>=1;a<<=1;if(a&(1<<20))a^=g;}return r;}
function gf20pow(a,n){let z=1;while(n){if(n&1)z=gf20mul(z,a);a=gf20mul(a,a);n=Math.floor(n/2);}return z;}
function gf20eval(poly,t){let z=0;for(let k=poly.length-1;k>=0;k--)z=gf20mul(z,t)^(Number(poly[k]&1n));return z;}
const ipolytrim=a=>{while(a.length>1&&a.at(-1)===0n)a.pop();return a};
function imul(a,b){let z=Array(a.length+b.length-1).fill(0n);a.forEach((x,i)=>b.forEach((y,j)=>z[i+j]+=x*y));return ipolytrim(z);}
function isub(a,b){return ipolytrim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)-(b[i]??0n)));}
function ipow(a,n){let z=[1n];while(n){if(n%2)z=imul(z,a);a=imul(a,a);n=Math.floor(n/2);}return z;}
const D=[-79n,-40n,-4n,4n],f3=[-21n,-237n,-60n,-4n,3n],f4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const f5=isub(imul(ipow(D,2),f4),ipow(f3,3));
const f6=imul(f3,isub(f5,ipow(f4,2)));
const X=[0n,1n];
const ns=[X,isub(imul(X,D),f3),isub(imul(X,ipow(f3,2)),imul(D,f4)),isub(imul(X,imul(D,ipow(f4,2))),imul(f5,f3)),isub(imul(X,ipow(f5,2)),imul(D,imul(f6,f4)))];
const ds=[[1n],D,ipow(f3,2),imul(D,ipow(f4,2)),ipow(f5,2)];
const t=2,xvals=ns.map((n,i)=>{let den=gf20eval(ds[i],t);check(den!==0,'binary pole');let inv=gf20pow(den,1048574);check(gf20mul(den,inv)===1,'bad gf20 inverse');return gf20mul(gf20eval(n,t),inv);});
let sym=[1],S=0;
for(const root of xvals){S^=root;const next=Array(sym.length+1).fill(0);sym.forEach((v,i)=>{next[i]^=gf20mul(v,root);next[i+1]^=v});sym=next;}
check(sym[4]===S&&(gf20pow(S,4)^S^1)===0,'wrong target residue line field');
const predicted=parity.map(levels=>levels.reduce((z,p)=>z^p.reduceRight((a,v)=>gf20mul(a,S)^v,0),0));
check(equals(sym.slice(0,4),predicted),'fixed target residue mismatch');
console.log(JSON.stringify({
 all_three_irreducible_over_f16:true,
 binary_primes:['1','w','w2'],
 binary_quintics:vals,
 complete_dlp_cost_measured:false,
 exact_integral_h_adic_certificate:true,
 fixed_ct103_residue_checked:true,
 global_orientation_selected:false,
 kind:'ct113-two-integral-relative-quintic',
 mutated_coefficient_rejected:true
}));
