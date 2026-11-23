#!/usr/bin/env node
'use strict';
// CT341 independent-language finite-field oracle. No CT339 imports or shared field functions.
const p=103, nonsquare=3;
const mod=x=>((x%p)+p)%p;
const add=([a,b],[c,d])=>[mod(a+c),mod(b+d)];
const mul=([a,b],[c,d])=>[mod(a*c+nonsquare*b*d),mod(a*d+b*c)];
const key=([a,b])=>a*p+b;
const rootCount=new Uint8Array(p*p);
for(let a=0;a<p;a++)for(let b=0;b<p;b++)rootCount[key(mul([a,b],[a,b]))]++;
let n2=1,zero2=0;
for(let a=0;a<p;a++)for(let b=0;b<p;b++){
  const z=[a,b];let f=[1,0];
  for(let j=1;j<=5;j++)f=mul(f,add([1,0],mul([mod(-j),0],z)));
  const n=rootCount[key(f)];if(n===0)continue;
  if(key(f)===0)zero2++;
  n2+=n;
}
const sqrtBaseCount=new Uint8Array(p);
for(let y=0;y<p;y++)sqrtBaseCount[mod(y*y)]++;
let n1=1;
for(let z=0;z<p;z++){
  let f=1;
  for(let j=1;j<=5;j++)f=mod(f*mod(1-j*z));
  n1+=sqrtBaseCount[f];
}
const inv4=26; // 4*26 = 104 ≡ 1 (mod 103)
let np=1,nm=1;
for(let x=0;x<p;x++){
  let fp=1,fm=1;
  for(const odd of [1,3,5]){
    const r=mod(odd*odd*inv4);
    fp=mod(fp*mod(x-r));fm=mod(fm*mod(1-r*x));
  }
  np+=sqrtBaseCount[fp];nm+=sqrtBaseCount[fm];
}
const q=p,t1=q+1-n1,t2=q*q+1-n2,e2=(t1*t1-t2)/2;
const jac=1-t1+e2-q*t1+q*q;
const result={schema:'ct341-independent-node-zeta/v1',p,n1,n2,e2,jac,elliptic_orders:[np,nm],product_order:np*nm,fq2_squares:rootCount.filter(Boolean).length,fq2_zero_roots:zero2};
if(n1!==88||n2!==10862||jac!==9200||np!==92||nm!==100||jac!==np*nm||result.fq2_squares!==5305)throw Error('independent source oracle disagrees with frozen claims: '+JSON.stringify(result));
process.stdout.write(JSON.stringify(result)+'\n');
