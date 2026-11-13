#!/usr/bin/env node
'use strict';
// Independently written direct finite-field Weierstrass group law.
const ok=(v,m)=>{if(!v)throw Error(m)};
const mod=(a,q)=>((a%q)+q)%q;
const inverse=(v,q)=>{for(let a=1;a<q;a++)if(mod(a*v,q)===1)return a;throw Error('inverse')};
const same=(P,Q)=>JSON.stringify(P)===JSON.stringify(Q);
function on(P,q,p,A,B){if(P===null)return true;const [x,y]=P;return mod(y*y+p*y-x*x*x-A*x-B,q)===0}
function add(P,Q,q,p,A,B){if(P===null)return Q;if(Q===null)return P;
const [x,y]=P,[u,v]=Q;
if(x===u&&mod(y+v+p,q)===0)return null;
let m;if(x===u&&y===v)m=mod((3*x*x+A)*inverse(mod(2*y+p,q),q),q);
else m=mod((v-y)*inverse(mod(u-x,q),q),q);
const nu=mod(y-m*x,q),X=mod(m*m-x-u,q),Y=mod(-m*X-nu-p,q);
const R=[X,Y];ok(on(R,q,p,A,B),'off curve');return R}
function multiply(P,n,q,p,A,B){let R=null;while(n){if(n&1)R=add(R,P,q,p,A,B);P=add(P,P,q,p,A,B);n=Math.floor(n/2)}return R}
function main(){
for(const [q,p,A,B] of [[5,5,0,6],[2,5,0,6]]){
const g=[null,[0,1],[0,mod(-6,q)]];
for(let i=0;i<3;i++)for(let j=0;j<3;j++)
ok(same(add(g[i],g[j],q,p,A,B),g[(i+j)%3]),'three-torsion table fails');
for(let i=0;i<3;i++){
let R=null,k=0;for(;k<3;k++){if(same(R,g[i]))break;R=add(R,g[1],q,p,A,B)}
ok(k===i,'DLP mismatch')}
}
console.log('positive p5 to p2: rational 3-torsion, all 9 group pairs, all 3 public logs');
const p=7,A=8,B=8;
const S=[0,1];
ok(on(S,7,p,A,B)&&on(S,2,p,A,B),'naive lift not on both fibers');
ok(multiply(S,5,7,p,A,B)===null,'source point not order5');
ok(multiply(S,3,2,p,A,B)===null&&multiply(S,1,2,p,A,B)!==null,'binary point not order3');
ok(same(multiply(S,5,2,p,A,B),multiply(S,2,2,p,A,B)),'noninjectivity mutant not detected');
console.log('negative p7 to p2: naive rational lift has order 5 at source but 3 at target');
console.log('PASS: independent finite reductions; no scalable map');
}
main();
