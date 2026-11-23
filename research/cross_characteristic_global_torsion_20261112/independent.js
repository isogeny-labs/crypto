#!/usr/bin/env node
// Finite-field verifier independent of the Python rational arithmetic path.
'use strict';
const fixed=[null,[5,5],[16,-61],[16,60],[5,-6]];
const mod=(x,p)=>((x%p)+p)%p;
function check(t,msg){if(!t)throw new Error(msg)}
function on(P,p){if(P===null)return true;const [x,y]=P;return mod(y*y+y-x*x*x+x*x+10*x+20,p)===0}
function inv(a,p){a=mod(a,p);for(let j=1;j<p;j++)if(mod(j*a,p)===1)return j;throw new Error('noninvertible')}
function sum(P,Q,p){
 if(P===null)return Q;if(Q===null)return P;
 const [x,y]=P,[u,v]=Q;
 if(x===u&&mod(y+v+1,p)===0)return null;
 let L,V;
 if(x===u&&y===v){
  const d=inv(2*y+1,p);
  L=mod((3*x*x-2*x-10)*d,p);
  V=mod((-x*x*x-10*x-40-y)*d,p);
 }else{
  const d=inv(u-x,p);
  L=mod((v-y)*d,p);
  V=mod((y*u-v*x)*d,p);
 }
 const X=mod(L*L+1-x-u,p),Y=mod(-L*X-V-1,p),R=[X,Y];
 check(on(R,p),'closure');return R;
}
const enc=P=>P===null?'O':P.join(',');
for(const p of [2,3,7,23]){
 const G=fixed.map(P=>P===null?null:P.map(x=>mod(x,p)));
 check(new Set(G.map(enc)).size===5,'collision');
 check(G.every(P=>on(P,p)),'equation');
 for(let i=0;i<5;i++)for(let j=0;j<5;j++)
  check(enc(sum(G[i],G[j],p))===enc(G[(i+j)%5]),'source law');
 const all=[null];
 for(let x=0;x<p;x++)for(let y=0;y<p;y++)if(on([x,y],p))all.push([x,y]);
 if(p===2||p===3)check(all.length===5,'group-size');
 const B=new Map(G.map((Q,n)=>[enc(Q),fixed[n]===null?null:fixed[n].map(x=>mod(x,2))]));
 check(B.size===5,'map size');
 for(let i=0;i<5;i++)for(let j=0;j<5;j++)
  check(enc(B.get(enc(sum(G[i],G[j],p))))===enc(sum(B.get(enc(G[i])),B.get(enc(G[j])),2)),'hom');
 let Q=null;
 for(let n=0;n<5;n++){check(enc(Q)===enc(B.get(enc(G[n]))),'public toy recovery');Q=sum(Q,B.get(enc(G[1])),2);}
 check(Q===null,'order five');
 console.log('JS source p='+p+' to char 2: five lifts, 25 hom pairs, five toy recovery cases');
}
const C=[null,[1,1],[0,1],[0,0],[1,0]],bad=C.slice();
check(enc(sum(C[1],C[1],2))===enc(C[2]),'positive doubling');
[bad[2],bad[3]]=[bad[3],bad[2]];
check(enc(sum(bad[1],bad[1],2))!==enc(bad[2]),'negative control');
console.log('nonhomomorphic swap rejected');
