#!/usr/bin/env node
// Independently enumerated smooth genus-2 binary models and F4 point counts.
'use strict';
function check(b,m){if(!b)throw Error(m);}
function degree(n){return 31-Math.clz32(n);}
function multiply(a,b){let s=0;while(b){if(b&1)s^=a;a<<=1;b>>=1;}return s;}
function remainder(a,b){while(a&&degree(a)>=degree(b))a^=b<<(degree(a)-degree(b));return a;}
function gcd(a,b){while(b){let t=remainder(a,b);a=b;b=t;}return a;}
function diff(a){let d=0;for(let i=1;i<=degree(a);i+=2)if(a&(1<<i))d^=1<<(i-1);return d;}
function times4(a,b){return remainder(multiply(a,b),7);}
function eval4(a,x){let v=0;for(let i=degree(a);i>=0;i--)v=times4(v,x)^((a>>i)&1);return v;}
function eval2(a,x){let v=0;for(let i=degree(a);i>=0;i--)v=(v*x)^((a>>i)&1);return v;}
function okay(h,f){
  if(!h)return false;
  const dh=diff(h),df=diff(f);
  if(gcd(h, multiply(multiply(dh,dh),f)^multiply(df,df))!==1)return false;
  const h3=(h>>3)&1;
  if(h3)return true;
  const inf=((f>>6)&1);
  return ((((h>>2)&1)*inf)^((f>>5)&1))===1;
}
function infinities(h,f,q){
  const h3=(h>>3)&1, f6=(f>>6)&1;
  let n=0;
  for(let v=0;v<q;v++)if(((q===2?v*v:times4(v,v))^(q===2?h3*v:times4(h3,v)))===f6)n++;
  return n;
}
function points(h,f,q){
  let n=infinities(h,f,q);
  for(let x=0;x<q;x++)for(let y=0;y<q;y++){
    const H=q===2?eval2(h,x):eval4(h,x), F=q===2?eval2(f,x):eval4(f,x);
    const lhs=(q===2?y*y:times4(y,y))^(q===2?H*y:times4(H,y));
    if(lhs===F)n++;
  }
  return n;
}
let models=0,max=0;
for(let h=1;h<16;h++)for(let f=0;f<128;f++){
  if(degree(f)!==5&&degree(f)!==6)continue;
  if(!okay(h,f))continue;
  models++;
  const n1=points(h,f,2),n2=points(h,f,4);
  const s1=3-n1,s2=5-n2;
  check((s1*s1-s2)%2===0,'integrality');
  const J=5+3*(-s1)+(s1*s1-s2)/2;
  if(J>max)max=J;
}
const h=11,f=34;
check(okay(h,f),'witness smooth');
check(points(h,f,2)===6&&points(h,f,4)===6,'witness point counts');
check(infinities(h,f,2)===2&&infinities(h,f,4)===2,'infinity pair');
check(models===640&&max===19,'bounded census');
console.log(JSON.stringify({max_j:19,n1:6,n2:6,smooth_models:640,witness_f:34,witness_h:11}));
