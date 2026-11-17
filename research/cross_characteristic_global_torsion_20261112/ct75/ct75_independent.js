#!/usr/bin/env node
'use strict';
// Independent finite-difference Newton x-lifts on the torsion x-defect.
// Checks all encoded target points, not just a pre-labelled generator.
const FIXTURES=[[29n,3n,[18n,5n],6],[29n,5n,[5n,5n],6],
 [37n,7n,[9n,4n],4],[59n,11n,[19n,22n],3],[139n,13n,[41n,9n],3]];
function check(ok,why){if(!ok)throw Error(why)}
function mod(a,m){const r=a%m;return r<0n?r+m:r}
function inv(a,m){let r0=m,r1=mod(a,m),t0=0n,t1=1n;
 while(r1!==0n){const q=r0/r1;[r0,r1]=[r1,r0-q*r1];[t0,t1]=[t1,t0-q*t1]}
 check(r0===1n,'nonunit inverse');return mod(t0,m)}
const rhs=x=>x*x*x-x*x-10n*x-20n;
function valid(q,m){const [x,y]=q;return mod(y*y+y-rhs(x),m)===0n}
function neg(q,m){return [q[0],mod(-q[1]-1n,m)]}
function add(a,b,m,p){const [x,y]=a,[z,w]=b,doubled=x===z&&y===w;
 const d=doubled?2n*y+1n:z-x;
 check(mod(d,p)!==0n,'nonunit affine denominator');
 const numer=doubled?3n*x*x-2n*x-10n:w-y;
 const s=mod(numer*inv(d,m),m),X=mod(s*s+1n-x-z,m);
 return [X,mod(s*x-s*X-y-1n,m)]}
function multiply(k,q,m,p){check(k>0n,'positive multiplication only');let ans=null,base=q;
 while(k){if(k&1n)ans=ans===null?base:add(ans,base,m,p);
   k>>=1n;if(k)base=add(base,base,m,p)}return ans}
function equal(a,b){return a[0]===b[0]&&a[1]===b[1]}
function torsion(q,ell,m,p){const h=(ell-1n)/2n;
 return equal(multiply(h,q,m,p),neg(multiply(h+1n,q,m,p),m))}
function liftY(x,y,place,p){const err=y*y+y-rhs(x);
 check(err%place===0n,'old point invalid');
 const yy=y+place*mod(-(err/place)*inv(2n*y+1n,p),p);
 check(valid([x,yy],place*p),'ordinate Hensel failure');return yy}
function defect(x,y,ell,m,p){const q=[x,y],h=(ell-1n)/2n;
 return mod(multiply(h,q,m,p)[0]-multiply(h+1n,q,m,p)[0],m)}
function liftNext(q,ell,place,p){const [x,y]=q,m=place*p;
 const f0=defect(x,liftY(x,y,place,p),ell,m,p);
 const f1=defect(x+place,liftY(x+place,y,place,p),ell,m,p);
 check(f0%place===0n&&f1%place===0n,'previous precision lost');
 const differential=mod((f1-f0)/place,p);
 check(differential!==0n,'torsion x-defect has nonunit derivative');
 const digit=mod(-(f0/place)*inv(differential,p),p);
 const X=x+digit*place,Y=liftY(X,y,place,p);
 check(valid([X,Y],m)&&torsion([X,Y],ell,m,p),'Newton x lift failed');return [X,Y]}
for(const [p,ell,generator,NMAX] of FIXTURES){
 check(valid(generator,p)&&torsion(generator,ell,p,p),'source order');
 const inputs=[];for(let k=1n;k<ell;k++)inputs.push(multiply(k,generator,p,p));
 check(new Set(inputs.map(x=>x.toString())).size===Number(ell-1n),'source duplicates');
 let lifts=inputs,place=p;
 for(let n=1;n<=NMAX;n++){
  if(n>1){lifts=lifts.map(q=>liftNext(q,ell,place,p));place*=p}
  const base=lifts[0];check(valid(base,place)&&torsion(base,ell,place,p),'bad generator');
  for(let j=0;j<lifts.length;j++){
   const q=lifts[j];check(valid(q,place)&&torsion(q,ell,place,p),'bad target');
   check(mod(q[0],p)===inputs[j][0]&&mod(q[1],p)===inputs[j][1],'bad reduction');
   check(equal(multiply(BigInt(j+1),base,place,p),q),'log relation changed');
  }
  let naiveY=generator[1],old=p;
  for(let j=1;j<n;j++){naiveY=liftY(generator[0],naiveY,old,p);old*=p}
  const naive=[generator[0],naiveY];check(valid(naive,place),'naive curve invalid');
  const record={N:n,all_public_nonzero:Number(ell-1n),curve:true,ell:Number(ell),
   modulus:place.toString(),naive_torsion:torsion(naive,ell,place,p),
   point:base.map(x=>x.toString()),second_multiple:lifts[1].map(x=>x.toString()),
   source:generator.map(x=>x.toString()),source_prime:Number(p),
   verified_relations:Number(ell-1n)};
  process.stdout.write(JSON.stringify(record)+'\n');
 }
}
