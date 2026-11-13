'use strict';
// Separate finite-field implementation; no imported CT80 arithmetic.
const p=37, mod=x=>((x%p)+p)%p;
function inv(x){for(let j=1;j<p;j++)if(mod(x*j)===1)return j;throw Error("inverse");}
function add(P,Q){
 if(P===null)return Q;if(Q===null)return P;
 const [x,y]=P,[u,v]=Q;
 if(x===u&&mod(y+v+1)===0)return null;
 let m,n;
 if(x===u&&y===v){let d=inv(mod(2*y+1));m=mod((3*x*x-2*x-10)*d);n=mod((-x*x*x-10*x-40-y)*d);}
 else{let d=inv(mod(u-x));m=mod((v-y)*d);n=mod((y*u-v*x)*d);}
 const X=mod(m*m+1-x-u),Y=mod(-m*X-n-1);
 if(mod(Y*Y+Y-X*X*X+X*X+10*X+20)!==0)throw Error("curve");
 return [X,Y];
}
let points=[null],cur=null;
for(let i=1;i<=7;i++){cur=add(cur,[9,4]);if(i<7)points.push(cur);}
if(cur!==null||new Set(points.map(x=>JSON.stringify(x))).size!==7)throw Error("order");
let count=0,good=0,fixed=0;
function visit(a,mask){
 if(a.length===6){
  count++;let v=[0,...a],ok=true;
  for(let i=0;i<7;i++)for(let j=0;j<7;j++)if(v[(i+j)%7]!==((v[i]+v[j])%7))ok=false;
  if(ok){good++;if(v[1]===1)fixed++;}return;
 }
 for(let j=1;j<=6;j++)if(!(mask&(1<<j)))visit([...a,j],mask|(1<<j));
}
visit([],0);
let xs=[...new Set(points.slice(1).map(P=>P[0]))].sort((a,b)=>a-b);
if(JSON.stringify(xs)!=='[9,27,28]'||count!==720||good!==6||fixed!==1)throw Error("alignment");
console.log("CT81 PASS: 7 points; x=9,27,28; bijections=720; homomorphisms=6; fixed-generator=1");
