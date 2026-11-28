#!/usr/bin/env node
'use strict';
// CT62 independent finite field/smoothness/zeta census. JS uses direct (x,y)
// point enumeration, and distinct smoothness checks for F2/F4 models.
function field(poly,deg){
 const n=1<<deg, mul=Array.from({length:n},()=>Array(n).fill(0));
 for(let a=0;a<n;a++)for(let b=0;b<n;b++){
  let aa=a,bb=b,z=0;
  while(bb){if(bb&1)z^=aa;bb>>=1;aa<<=1;if(aa&n)aa^=poly;}
  mul[a][b]=z;
 }
 return mul;
}
const m4=field(7,2),m16=field(19,4);
const omega=Array.from({length:14},(_,i)=>i+2).find(x=>(m16[x][x]^x^1)===0);
if(omega===undefined)throw Error('no F4 embedding');
const embed=[0,1,omega,omega^1];
for(let a=0;a<4;a++)for(let b=0;b<4;b++)
 if(m16[embed[a]][embed[b]]!==embed[m4[a][b]])throw Error('not embedding');
function px(x,n,m){let v=[1];for(let i=1;i<=6;i++)v.push(m[v[i-1]][x]);return v;}
function evalPoly(coeff,x,m){const p=px(x,m.length,m);let z=0;for(let j=0;j<coeff.length;j++)z^=m[coeff[j]][p[j]];return z;}
function pointCount(f,h,m){
 const n=m.length;
 let ct=0;
 if(h[3]!==0){for(let y=0;y<n;y++)if((m[y][y]^m[h[3]][y])===f[6])ct++;}
 else ct=1;
 for(let x=0;x<n;x++){
  const rhs=evalPoly(f,x,m), hv=evalPoly(h,x,m);
  for(let y=0;y<n;y++)if((m[y][y]^m[hv][y])===rhs)ct++;
 }
 return ct;
}
function pmul(a,b){let z=0;for(let j=0;j<=31;j++)if((b>>>j)&1)z^=a<<j;return z;}
function prem(a,b){const deg=x=>31-Math.clz32(x);while(a&&deg(a)>=deg(b))a^=b<<(deg(a)-deg(b));return a;}
function pgcd(a,b){while(b){[a,b]=[b,prem(a,b)];}return a;}
function smoothF2(f,h){
 const H=h.reduce((v,x,i)=>v|(x<<i),0);
 if(!H)return false;
 const F=f.reduce((v,x,i)=>v|(x<<i),0);
 const hp=h[1]|(h[3]<<2),fp=f[1]|(f[3]<<2)|(f[5]<<4);
 return pgcd(H,pmul(fp,fp)^pmul(F,pmul(hp,hp)))===1;
}
// For h degree <=2 over F4, all roots of h are in F16; thus the singularity
// condition can be tested by directly evaluating both polynomials at those roots.
function smoothF4(f,h){
 if(h.every(x=>x===0))return false;
 const ff=f.map(x=>embed[x]),hh=h.map(x=>embed[x]);
 for(let x=0;x<16;x++){
  if(evalPoly(hh,x,m16)!==0)continue;
  const xp=m16[x][x],x4=m16[xp][xp];
  const fp=ff[1]^m16[ff[3]][xp]^m16[ff[5]][x4];
  const hp=hh[1]^m16[hh[3]][xp];
  if((m16[fp][fp]^m16[m16[hp][hp]][evalPoly(ff,x,m16)])===0)return false;
 }
 return true;
}
function* tup(n,k){const total=n**k;for(let i=0;i<total;i++){
 let t=i,res=[];for(let j=0;j<k;j++){res.push(t%n);t=Math.floor(t/n);}yield res;
}}
function factorPrimes(n){let out=[];for(let p=2;p*p<=n;p++)if(n%p===0){out.push(p);while(n%p===0)n/=p;}if(n>1)out.push(n);return out;}
function sweep(q){
 const orders={},primes={},infs={'one_infinity':0,'two_rational_infinities':0,'nonsplit_infinities':0};
 const m=q===2?field(3,1):m4;let total=0,smooth=0;
 const hs3=q===2?Array.from({length:q},(_,i)=>i):[0];
 for(let h3 of hs3)for(let r of tup(q,3)){
  const h=[...r,h3];
  const F6=h3===0?[0]:[0, q===2?1:m4[m4[h3][h3]][2]];
  for(let f6 of F6)for(let cs of tup(q,h3===0?5:6)){
   const f=h3===0?[...cs,1,0]:[...cs,f6];total++;
   if(!(q===2?smoothF2(f,h):smoothF4(f,h)))continue;
   smooth++;
   infs[h3===0?'one_infinity':(f6===0?'two_rational_infinities':'nonsplit_infinities')]++;
   const n1=pointCount(f,h,m),n2=pointCount(f.map(x=>q===2?x:embed[x]),h.map(x=>q===2?x:embed[x]),q===2?m4:m16);
   const J=(n1*n1+n2)/2-q;
   if(!Number.isInteger(J))throw Error('nonintegral J');
   orders[J]=(orders[J]||0)+1;
   for(let ell of factorPrimes(J))primes[ell]=(primes[ell]||0)+1;
  }
 }
 return {q,models_total:total,smooth,infinity_types:infs,orders,prime_divisors:primes,
  distinct_orders:Object.keys(orders).length,min_J:Math.min(...Object.keys(orders).map(Number)),max_J:Math.max(...Object.keys(orders).map(Number))};
}
const results=[sweep(2),sweep(4)];
console.log(JSON.stringify({test:'ct62_binary_genus2_complete_model_census_independent',results}));
