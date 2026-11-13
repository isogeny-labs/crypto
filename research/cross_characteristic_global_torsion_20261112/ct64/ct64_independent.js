#!/usr/bin/env node
'use strict';
// Independent characteristic-two quotient verifier, based on direct point enumeration.
// No Mumford or genus-two arithmetic is imported from Python.
const K=10, Q=1024, RED=0x409, L=31;
function multiply(x,y){
  let t=0;
  for(;y;y>>>=1){if(y&1)t^=x;x<<=1;if(x&Q)x^=RED;}
  return t&(Q-1);
}
function powering(x,e){let z=1;for(;e;e>>>=1){if(e&1)z=multiply(z,x);x=multiply(x,x);}return z;}
function division(x,y){if(!y)throw Error('divide by zero');return multiply(x,powering(y,Q-2));}
function check(b,s){if(!b)throw Error(s);}
function cube(x){return multiply(x,multiply(x,x));}
function fifth(x){return multiply(multiply(x,x),cube(x));}
const omega=Array.from({length:Q},(_,i)=>i).find(x=>x>1 && (multiply(x,x)^x^1)===0);
check(omega===236,'omega representation');
const rhsSolutions=new Map();
for(let y=0;y<Q;y++){const f=multiply(y,y)^y;if(!rhsSolutions.has(f))rhsSolutions.set(f,[]);rhsSolutions.get(f).push(y);}
function lift(r){
  const r2=multiply(r,r),r3=multiply(r2,r),r4=multiply(r2,r2);
  const a=powering(r,Q/2),b=r4^multiply(omega,r2);
  if((multiply(b,b)^a)!==multiply(omega,r))return null;
  const constant=multiply(r4,r)^multiply(omega,r3),pre=rhsSolutions.get(constant);
  if(!pre)return null;
  const c=pre[0];
  return {r,a,b,c,comm:multiply(a,r2)^multiply(b,r)};
}
const translations=[];for(let r=1;r<Q;r++){const t=lift(r);if(t)translations.push(t);}
const involutions=translations.filter(x=>x.comm===0);
check(translations.length===15 && involutions.length===5,'automorphism census');
check(translations.filter(x=>x.comm===1).length===10,'order-four controls');
function quotient(t){
 check(t.comm===0,'non involutive lift');
 const r=t.r, A=division(t.a,r), C=division(t.c^multiply(A,cube(r)),r);
 const alpha=division(1,r),beta=r,gamma=division(C,r);
 check(multiply(alpha,r)===1,'leading coefficient');
 check(multiply(alpha,cube(r))===(omega^A),'cubic coefficient');
 check((multiply(beta,multiply(r,r))^gamma)===multiply(C,C),'quadratic coefficient');
 check(multiply(gamma,r)===C,'linear coefficient');
 return {...t,A,C,coef:[alpha,beta,gamma]};
}
const maps=involutions.map(quotient);
check(JSON.stringify(maps.slice(0,2).map(t=>[t.r,t.a,t.b,t.c]))===JSON.stringify([[20,6,120,432],[534,794,636,668]]),'two selected translations');
function onE(P,co){if(P===null)return true;const [x,y]=P,[a,b,g]=co;return (multiply(y,y)^y)===(multiply(a,cube(x))^multiply(b,multiply(x,x))^multiply(g,x));}
function curveMap(P,m){
 const [x,y]=P,xx=multiply(x,x)^multiply(m.r,x),yy=y^multiply(m.A,cube(x))^multiply(m.C,x);
 const R=[xx,yy];check(onE(R,m.coef),'image off E');return R;
}
function key(P){return P===null?'O':(P[0]+':'+P[1]);}
function sumE(P,R,co){
 if(P===null)return R;if(R===null)return P;
 const [x,y]=P,[z,w]=R,[a,b,g]=co;
 let m;
 if(x===z){if(y!==w)return null;m=multiply(a,multiply(x,x))^g;}
 else m=division(y^w,x^z);
 const xx=x^z^division(b^multiply(m,m),a),yy=multiply(m,x^xx)^y^1;
 const S=[xx,yy];check(onE(S,co),'bad E sum');return S;
}
function ellipticCoordinateTable(co){
 const pts=[];
 for(let x=0;x<Q;x++){
  const f=multiply(co[0],cube(x))^multiply(co[1],multiply(x,x))^multiply(co[2],x);
  for(const y of rhsSolutions.get(f)||[])pts.push([x,y]);
 }
 check(pts.length===960,'E order not 961');
 const P=pts[0],small=new Set();let walk=null;
 for(let i=0;i<L;i++){small.add(key(walk));walk=sumE(walk,P,co);}
 check(walk===null&&small.size===31,'elliptic P order');
 const S=pts.find(x=>!small.has(key(x)));check(S!==undefined,'no independent torsion point');
 const tab=new Map();let row=null;
 for(let j=0;j<L;j++){
  let col=row;
  for(let i=0;i<L;i++){const k=key(col);check(!tab.has(k),'duplicate elliptic coordinate');tab.set(k,[i,j]);col=sumE(col,P,co);}
  row=sumE(row,S,co);
 }
 check(tab.size===961,'elliptic not rank two');return tab;
}
function mod(x){return ((x%L)+L)%L;}
function rank(rows){
 const a=rows.map(x=>x.slice());let r=0;
 for(let j=0;j<4&&r<a.length;j++){
  let p=r;while(p<a.length&&a[p][j]===0)p++;
  if(p===a.length)continue;
  [a[r],a[p]]=[a[p],a[r]];
  const inverse=modInverse(a[r][j]);
  for(let c=j;c<4;c++)a[r][c]=mod(a[r][c]*inverse);
  for(let t=r+1;t<a.length;t++){
    const scale=a[t][j];for(let c=j;c<4;c++)a[t][c]=mod(a[t][c]-scale*a[r][c]);
  }
  r++;
 }
 return r;
}
function modInverse(x){for(let i=1;i<L;i++)if((i*x)%L===1)return i;throw Error('no inverse mod31');}
const source=[];
for(let x=0;x<Q;x++){
 const f=fifth(x)^multiply(omega,cube(x));
 for(const y of rhsSolutions.get(f)||[])source.push([x,y]);
}
check(source.length===896,'wrong C point count');
const encoded=maps.map(m=>({model:m, coords:ellipticCoordinateTable(m.coef)}));
for(const pt of source)for(const e of encoded)curveMap(pt,e.model);
const pairs=[];
for(let i=0;i<5;i++)for(let j=i+1;j<5;j++){
 const a=encoded[i],b=encoded[j];const rows=[];
 for(const P of source){
   rows.push([...a.coords.get(key(curveMap(P,a.model))),...b.coords.get(key(curveMap(P,b.model)))]);
   if(rank(rows)===4)break;
 }
 const rr=rank(rows);check(rr===4,'product quotient misses 31 torsion');
 pairs.push([a.model.r,b.model.r,rr]);
}
const chosen=source.filter(P=>(P[0]===0&&P[1]===0)||(P[0]===2&&P[1]===738)||(P[0]===4&&P[1]===684)||(P[0]===6&&P[1]===920));
const witness=chosen.map(P=>[P[0],P[1],...encoded.slice(0,2).flatMap(e=>e.coords.get(key(curveMap(P,e.model))))]);
check(chosen.length===4&&rank(witness.map(row=>row.slice(2)))===4,'fixed witness rank');
check(maps.every(m=>onE(curveMap([0,0],m),m.coef)),'generator image');
console.log(JSON.stringify({omega,nonzero_translation_lifts:translations.length,genuine_involutions:involutions.length,order_four_lifts:10,curve_affine_points:source.length,selected_quotients:maps.slice(0,2).map(t=>t.coef),elliptic_points:961,all_pair_ranks:pairs,fixed_rank4_witness:witness},null,0));
