#!/usr/bin/env node
'use strict';
// CT111 independently derives 2-adic fixed prime labels using mod4 polynomial
// arithmetic and Fermat inverses over F2^20. No external dependencies.
const ck=(ok,msg)=>{if(!ok)throw Error(msg)};
const trim=a=>{let b=a.slice();while(b.length>1&&b.at(-1)===0)b.pop();return b};
const plus=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>((a[i]??0)+(b[i]??0))%4));
const minus=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>((a[i]??0)-(b[i]??0)+4)%4));
const times=(a,b)=>{let c=Array(a.length+b.length-1).fill(0);a.forEach((x,i)=>b.forEach((y,j)=>{c[i+j]=(c[i+j]+x*y)%4}));return trim(c)};
const power=(a,n)=>{let v=[1];while(n){if(n&1)v=times(v,a);a=times(a,a);n=Math.floor(n/2)}return v};
const D=[-79,-40,-4,4].map(z=>(z%4+4)%4);
const f3=[-21,-237,-60,-4,3].map(z=>(z%4+4)%4);
const f4=[-5821,-1496,-210,-790,-100,-4,2].map(z=>(z%4+4)%4);
const f5=minus(times(power(D,2),f4),power(f3,3));
const f6=times(f3,minus(f5,power(f4,2)));
const f7=minus(times(f5,power(f3,3)),times(power(D,2),power(f4,3)));
const f11=minus(times(f7,power(f5,3)),times(times(power(D,2),f4),power(f6,3)));
ck(f11.length===61&&f11.at(-1)===3,'bad division polynomial');
const X=[0,1];
const nums=[X,minus(times(X,D),f3),
    minus(times(X,power(f3,2)),times(D,f4)),
    minus(times(X,times(D,power(f4,2))),times(f5,f3)),
    minus(times(X,power(f5,2)),times(D,times(f6,f4)))];
const dens=[[1],D,power(f3,2),times(D,power(f4,2)),power(f5,2)];

function ring(g){
 const n=20,coeff=Array.from({length:n},(_,i)=>(g>>>i)&1);
 const red=raw=>{let a=raw.map(z=>(z%4+4)%4);while(a.length<n)a.push(0);
   for(let k=a.length-1;k>=n;k--){let c=a[k];
     if(c)for(let j=0;j<n;j++)if(coeff[j])a[k-n+j]=(a[k-n+j]-c+4)%4;
   }return a.slice(0,n);
 };
 const constant=z=>red([z]);
 const add=(a,b)=>a.map((v,i)=>(v+b[i])%4);
 const subtract=(a,b)=>a.map((v,i)=>(v-b[i]+4)%4);
 const mul=(a,b)=>{let c=Array(2*n-1).fill(0);
   for(let i=0;i<n;i++)if(a[i])for(let j=0;j<n;j++)c[i+j]=(c[i+j]+a[i]*b[j])%4;
   return red(c);
 };
 const pow=(a,k)=>{let v=constant(1);while(k){if(k&1)v=mul(v,a);a=mul(a,a);k=Math.floor(k/2)}return v};
 const evaluate=(pol,a)=>{let v=constant(0);for(let i=pol.length-1;i>=0;i--)v=add(mul(v,a),constant(pol[i]));return v};
 const bits=a=>a.reduce((v,x,i)=>v|((x&1)<<i),0);
 const embed=bit=>red(Array.from({length:n},(_,i)=>(bit>>>i)&1));
 const binaryMul=(a,b)=>{let c=0;while(b){if(b&1)c^=a;b>>>=1;a<<=1;if(a&(1<<n))a^=g;}return c};
 const binaryPow=(a,k)=>{let v=1;while(k){if(k&1)v=binaryMul(v,a);a=binaryMul(a,a);k=Math.floor(k/2)}return v};
 const inv=a=>{let z=bits(a);ck(z!==0,'zero denominator');
   let seed=embed(binaryPow(z,2**n-2));
   let answer=mul(seed,subtract(constant(2),mul(a,seed)));
   ck(mul(a,answer).join(',')===constant(1).join(','),'bad mod4 inverse');
   return answer;
 };
 return {n,red,constant,add,subtract,mul,pow,evaluate,bits,embed,binaryMul,binaryPow,inv};
}
const output=[];
for(const g of [0x130027,0x131203,0x131211]){
 const R=ring(g),t=R.red([0,1]);
 const ft=R.evaluate(f11,t);
 ck(ft.every(v=>v%2===0),'not a binary division polynomial root');
 const derivative=f11.slice(1).map((x,i)=>x*(i+1)%4);
 const dt=R.bits(R.evaluate(derivative,t));ck(dt!==0,'not a simple root');
 const correction=R.binaryMul(R.bits(ft.map(z=>z/2)),R.binaryPow(dt,2**R.n-2));
 const alpha=R.add(t,R.red(Array.from({length:R.n},(_,i)=>2*((correction>>>i)&1))));
 ck(R.evaluate(f11,alpha).every(x=>x===0),'lifted root is not valid modulo4');
 let s=R.constant(0);
 for(let i=0;i<5;i++)s=R.add(s,R.mul(R.evaluate(nums[i],alpha),R.inv(R.evaluate(dens[i],alpha))));
 ck(R.evaluate([1,3,3,1,3,2,3,0,3,3,0,0,1],s).every(x=>x===0),'CT109 q(S) modulo4');
 const hs=R.add(R.add(R.pow(s,4),s),R.constant(1));
 ck(hs.every(x=>x%2===0),'h(S) not divisible by two');
 const U=R.bits(hs.map(x=>x/2)),s0=R.bits(s),
       w=R.binaryMul(s0,s0)^s0,w2=R.binaryMul(w,w);
 const label=U===1?'1':U===w?'w':U===w2?'w2':'INVALID';
 ck(label!=='INVALID','wrong CT110 prime label');
 output.push({alpha_correction:'0x'+correction.toString(16),
   g:'0x'+g.toString(16),label,s:'0x'+s0.toString(16),
   u:'0x'+U.toString(16),w:'0x'+w.toString(16),w2:'0x'+w2.toString(16)});
}
ck(output.map(x=>x.label).join(',')==='1,w,w2','binary prime ordering discrepancy');
console.log(JSON.stringify({binary_prime_labels:output,case:'ct111-fixed-binary-prime-label',
 division_polynomial_degree:60,fixed_ct103_prime_u_label:'1',
 modulus:4,source_scalar_used:false}));
