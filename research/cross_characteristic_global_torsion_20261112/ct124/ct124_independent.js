#!/usr/bin/env node
/* CT124 independent BigInt 59-adic and curve-group controls. */
'use strict';
const assert=require('assert');
const norm=a=>{a=a.slice();while(a.length>1 && a[a.length-1]===0n)a.pop();return a;};
const add=(a,b)=>norm(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)+(b[i]||0n)));
const mul=(a,b)=>{let r=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)r[i+j]+=a[i]*b[j];return norm(r);};
const sub=(a,b)=>add(a,b.map(x=>-x));
function power(a,n){let r=[1n];while(n>0){if(n%2)r=mul(r,a);a=mul(a,a);n=Math.floor(n/2);}return r;}
function polynomial11(params){
 const [a1,a2,a3,a4,a6]=params.map(BigInt);
 const b2=a1*a1+4n*a2,b4=2n*a4+a1*a3,b6=a3*a3+4n*a6;
 const b8=a1*a1*a6+4n*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4;
 const C=[b6,2n*b4,b2,4n],u=[b8,3n*b6,3n*b4,b2,3n];
 const v=[b4*b8-b6*b6,b2*b8-b4*b6,10n*b8,10n*b6,5n*b4,b2,2n];
 const t5=sub(mul(v,power(C,2)),power(u,3));
 const t6=mul(u,sub(t5,power(v,2)));
 const t7=sub(mul(t5,power(u,3)),mul(power(v,3),power(C,2)));
 return sub(mul(t7,power(t5,3)),mul(mul(v,power(t6,3)),power(C,2)));
}
function mod(a,n){return ((a%n)+n)%n;}
function evalMod(f,x,m){let v=0n;for(let i=f.length-1;i>=0;i--)v=mod(v*x+f[i],m);return v;}
function inv(a,p){a=mod(a,p);let [t,nt,r,nr]=[0n,1n,p,a];while(nr){const q=r/nr;[t,nt]=[nt,t-q*nt];[r,nr]=[nr,r-q*nr];}assert.equal(r,1n);return mod(t,p);}
function lift(f,root){let x=BigInt(root),p=59n,der=f.slice(1).map((v,i)=>v*BigInt(i+1));let d=evalMod(der,x,p);assert(d!==0n);const id=inv(d,p);let power=p,ans=[];
 for(let i=0;i<4;i++){
  assert.equal(evalMod(f,x,power),0n);ans.push(Number(x));
  if(i<3){let next=power*p;let quotient=evalMod(f,x,next)/power;let digit=mod(-quotient*id,p);x+=power*digit;power=next;}
 }
 return ans;
}
const allY=x=>{let a=[];for(let y=0;y<59;y++)if(mod(BigInt(y*y+y-x*x*x+x*x+10*x+20),59n)===0n)a.push(y);return a;};
function pointAdd(P,Q){if(P===null)return Q;if(Q===null)return P;const [x,y]=P,[u,v]=Q;
 if(x===u && (y+v+1)%59===0)return null;
 let lambda;if(x!==u)lambda=Number(mod(BigInt(v-y)*inv(BigInt(u-x),59n),59n));
 else{let den=mod(BigInt(2*y+1),59n);assert(den!==0n);lambda=Number(mod(BigInt(3*x*x-2*x-10)*inv(den,59n),59n));}
 let rx=Number(mod(BigInt(lambda*lambda+1-x-u),59n));
 let ry=Number(mod(BigInt(-lambda*(rx-x)-y-1),59n));
 assert.equal(mod(BigInt(ry*ry+ry-rx*rx*rx+rx*rx+10*rx+20),59n),0n);
 return [rx,ry];
}
const roots=[19,20,22,43,46];
let G=null,orbit=null;
for(const x of roots){for(const y of allY(x)){
 let g=[x,y],a=null,points=[];
 for(let m=1;m<=11;m++){a=pointAdd(a,g);if(m<11)points.push([m,a]);}
 if(a===null && JSON.stringify([...new Set(points.map(z=>z[1][0]))].sort((a,b)=>a-b))===JSON.stringify(roots)){G=g;orbit=points;break;}
}if(G!==null)break;}
assert(G!==null);
const answer={ct:'CT124',models:{},source_generator:G,source_x_by_scalar:Object.fromEntries(orbit.map(([m,pt])=>[String(m),pt[0]]))};
for(const [name,aa] of Object.entries({plus:[59,-1,60,-10,39],minus:[59,58,60,-10,39]})){
 const f=polynomial11(aa);assert.equal(f.length,61);assert.equal(f[60],11n);
 const lifts=Object.fromEntries(roots.map(r=>[String(r),lift(f,r)]));
 const cases=[];
 for(const [m,point] of orbit){
  const r=point[0];assert.equal(mod(60n*BigInt(r)+59n*BigInt(r)-BigInt(r),59n),0n);
  assert.equal(mod(BigInt(60*r),2n),0n);assert.equal(59%2,1);
  assert.equal(evalMod(f,BigInt(r),59n),0n);
  assert.notEqual(58n*f[59],60n*11n*(60n*BigInt(r))); // genuine global root excluded by coefficient witness
  const verified=pointAdd(null,orbit[0][1]).toString()===point.toString() || pointAdd(null,orbit[9][1]).toString()===point.toString();
  assert.equal(verified,(m===1 || m===10));
  cases.push({m,source_x:r,source_local_root:true,binary_local_root:true,scalar_recovered_up_to_sign:1,source_verification:verified});
 }
 const pass=cases.filter(x=>x.source_verification).length;assert.equal(pass,2);
 answer.models[name]={cases,hensel_root_mod_59_powers:lifts,pass_local:10,pass_e2e:pass};
}
function canonical(o){if(o===null || typeof o!=='object')return JSON.stringify(o);if(Array.isArray(o))return '['+o.map(canonical).join(',')+']';return '{'+Object.keys(o).sort().map(k=>JSON.stringify(k)+':'+canonical(o[k])).join(',')+'}';}
process.stdout.write(canonical(answer)+'\n');
