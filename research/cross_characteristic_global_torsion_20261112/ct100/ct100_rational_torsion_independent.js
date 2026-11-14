#!/usr/bin/env node
"use strict";
// Independent exact-integer rational 5-torsion and residue group-law oracle.
const test=(b,s)=>{if(!b)throw Error(s)};
const mod=(a,p)=>((a%p)+p)%p;
const same=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function inverse(a,p) {
 for(let i=1;i<p;i++)if(mod(a*i,p)===1)return i;
 throw Error('noninvertible slope');
}
function add(P,Q,p) {
 if(P===null)return Q;
 if(Q===null)return P;
 const [x,y]=P,[u,v]=Q;
 if(x===u&&mod(y+v+1,p)===0)return null;
 let n,d;
 if(x===u&&y===v){n=3*x*x-2*x-10;d=2*y+1;}
 else{n=v-y;d=u-x;}
 const lambda=mod(n*inverse(d,p),p);
 const xx=mod(lambda*lambda+1-x-u,p);
 return [xx,mod(-lambda*(xx-x)-y-1,p)];
}
function integerAdd(P,Q) {
 if(P===null)return Q;
 if(Q===null)return P;
 const [x,y]=P,[u,v]=Q;
 if(x===u&&y+v+1===0)return null;
 let n,d;
 if(x===u&&y===v){n=3*x*x-2*x-10;d=2*y+1;}
 else{n=v-y;d=u-x;}
 test(d!==0&&n%d===0,'rational group law requires noninteger intermediate');
 const lambda=n/d,xx=lambda*lambda+1-x-u;
 return [xx,-lambda*(xx-x)-y-1];
}
function log(G,Q,p) {
 let W=null;
 for(let i=0;i<5;i++){
   if(same(W,Q))return i;
   W=add(W,G,p);
 }
 throw Error('missing five-torsion log');
}
function onCurve(P,p){if(P===null)return true;let[x,y]=P;return mod(y*y+y-x*x*x+x*x+10*x+20,p)===0}
function curveOrder(p){let n=1;for(let x=0;x<p;x++)for(let y=0;y<p;y++)if(onCurve([x,y],p))n++;return n}
test(curveOrder(7)===10&&curveOrder(2)===5,'wrong finite-field curve orders');
const G=[5,-6];let R=null;const global=[null];
for(let k=1;k<5;k++){R=integerAdd(R,G);test(R!==null,'early global zero');global.push(R);}
test(integerAdd(R,G)===null,'not rational 5-torsion');
test(same(global,[null,[5,-6],[16,60],[16,-61],[5,5]]),'global coordinates differ');
function reduce(p){return global.map(P=>P===null?null:P.map(v=>mod(v,p)));}
const src=reduce(7),dst=reduce(2);
test(same(src,[null,[5,1],[2,4],[2,2],[5,5]]),'source wrong');
test(same(dst,[null,[1,0],[0,0],[0,1],[1,1]]),'target wrong');
test(src.every(P=>onCurve(P,7))&&dst.every(P=>onCurve(P,2)),'off-curve point');
const map=new Map(src.map((P,k)=>[JSON.stringify(P),dst[k]]));
test(map.size===5,'noninjective source point map');
for(let i=0;i<5;i++){
 test(log(src[1],src[i],7)===i,'source log differs');
 test(log(dst[1],dst[i],2)===i,'target log differs');
 test(same(map.get(JSON.stringify(src[i])),dst[i]),'online input lift differs');
 for(let j=0;j<5;j++){
   let k=(i+j)%5;
   test(same(integerAdd(global[i],global[j]),global[k]),'rational add');
   test(same(add(src[i],src[j],7),src[k]),'char7 add');
   test(same(add(dst[i],dst[j],2),dst[k]),'char2 add');
 }
}
const out={all_25_pair_operations:true,experiment:'CT100-rational-torsion-positive-control',
 global_points:global,group_order:5,source_curve_order:10,target_curve_order:5,pcc:'unverified',precomputed_full_torsion_table:true,
 scalar_free_input_lookup:true,source_bsgs_cases:4,source_points:src,
 target_bsgs_cases:4,target_points:dst};
const canonical=o=>Array.isArray(o)?'['+o.map(canonical).join(',')+']':
 o!==null&&typeof o==='object'?'{'+Object.keys(o).sort().map(k=>JSON.stringify(k)+':'+canonical(o[k])).join(',')+'}':JSON.stringify(o);
process.stdout.write(canonical(out)+'\n');
