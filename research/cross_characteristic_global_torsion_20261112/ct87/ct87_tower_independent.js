'use strict';
// CT87 independent BigInt verification of explicit L/F relative cubic.
const check=x=>{if(!x)throw Error('CT87 certificate failed')};
function shrink(a){while(a.length>1&&a[a.length-1]===0n)a.pop();return a;}
function sum(a,b,s=1n){return shrink(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)+s*(b[i]??0n)));}
function mul(a,b){const c=Array(a.length+b.length-1).fill(0n);a.forEach((v,i)=>b.forEach((w,j)=>{c[i+j]+=v*w}));return shrink(c);}
function power(a,e){let x=[1n];while(e){if(e%2)x=mul(x,a);a=mul(a,a);e=Math.floor(e/2);}return x;}
function pseudo(a,b){let z=[...a];while(z.length>=b.length&&z.some(v=>v!==0n)){let shift=z.length-b.length,leading=z.at(-1);z=sum(z.map(c=>c*b.at(-1)),Array(shift).fill(0n).concat(b.map(c=>c*leading)),-1n);}return z;}
const x=[0n,1n],d=[-79n,-40n,-4n,4n],p3=[-21n,-237n,-60n,-4n,3n],p4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const p5=sum(mul(power(d,2),p4),power(p3,3),-1n),p7=sum(mul(p5,power(p3,3)),mul(power(d,2),power(p4,3)),-1n);
check(p7.length===25&&p7[24]===7n);
const a2=sum(power([-10n,-2n,3n],2),mul([1n,-2n],d)),b3=power(p3,2),a3=sum(mul(x,b3),mul(d,p4),-1n),B=mul(d,b3);
const N=sum(sum(mul(x,B),mul(a2,b3)),mul(a3,d)), E2=sum(sum(mul(mul(x,a2),b3),mul(mul(x,a3),d)),mul(a2,a3)),E3=mul(mul(x,a2),a3);
const q2=23624165350186n,C2=[51746818064829n,-330291288560641n,-95029425174429n,-1711075375204n,-1596513316848n,-44914399152n,-455319459n,54044109n];
const q3=10252887761980724n,C3=[-201756977283948362n,563360952055619683n,149521192983511524n,3495575179577110n,2638006865239263n,74263835359077n,711827220769n,-88442189137n];
function hom(coeff){let r=[0n];for(let i=0;i<8;i++)r=sum(r,mul([coeff[i]],mul(power(N,i),power(B,7-i))));return r;}
const c2=hom(C2),c3=hom(C3);
check(JSON.stringify(pseudo(sum(mul([q2],mul(E2,power(B,6))),c2,-1n),p7),(_,v)=>typeof v==='bigint'?String(v):v)==='["0"]');
check(pseudo(sum(mul([q3],mul(E3,power(B,6))),c3,-1n),p7).every(v=>v===0n));
const cubic=sum(sum(mul([q2*q3],mul(power(B,7),power(x,3))),mul([q2*q3],mul(N,mul(power(B,6),power(x,2)))),-1n),mul([q3],mul(c2,x)));
check(pseudo(sum(cubic,mul([q2],c3),-1n),p7).every(v=>v===0n));
console.log('CT87 PASS: relative_cubic_degree=3 base_degree=8 galois_x=3 denominator_e2=23624165350186 denominator_e3=10252887761980724');
