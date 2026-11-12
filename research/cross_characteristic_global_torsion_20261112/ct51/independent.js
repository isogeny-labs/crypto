#!/usr/bin/env node
'use strict';
// Independent CT51 verifier: direct F2[U]/(U^8+U^6+U^5+U^4+U^3+U+1).
// Python instead uses F16[Y]/(Y^2+Y+X^3+X^2).
const must=(ok,msg)=>{if(!ok)throw Error(msg);};
const POLY=0x17b, X=0x13, Y=2;
function mul(a,b){
  let z=0;
  for(let i=0;i<8;i++){
    if(b&1)z^=a;
    b>>>=1;a<<=1;
    if(a&256)a^=POLY;
  }
  return z;
}
function exp(a,n){
  let z=1;
  for(;n;n>>>=1,a=mul(a,a))if(n&1)z=mul(z,a);
  return z;
}
const norm=a=>(a%29+29)%29;
const eq=(a,b)=>a===null?b===null:b!==null&&a[0]===b[0]&&a[1]===b[1];
function on(p,char2){
  if(p===null)return true;
  const [x,y]=p;
  if(char2){
    const xx=mul(x,x);
    return (mul(y,y)^y)===(mul(xx,x)^xx);
  }
  return norm(y*y+y-x*x*x+x*x+10*x+20)===0;
}
const neg=(p,two)=>p===null?null:[p[0],two?p[1]^1:norm(-p[1]-1)];
function inverse29(z){
  for(let i=1;i<29;i++)if(z*i%29===1)return i;
  throw Error('bad inverse 29');
}
function add(p,q,two){
  if(p===null)return q;
  if(q===null)return p;
  must(on(p,two)&&on(q,two),'off curve input');
  if(eq(p,neg(q,two)))return null;
  const [x,y]=p,[u,v]=q;
  let l,nu;
  if(two){
    if(eq(p,q)){
      l=mul(x,x);
      nu=mul(mul(x,x),x)^y;
    }else{
      const z=exp(x^u,254);
      l=mul(y^v,z); nu=mul(mul(y,u)^mul(v,x),z);
    }
    const rx=mul(l,l)^1^x^u;
    const ry=mul(l,rx)^nu^1;
    const ret=[rx,ry];must(on(ret,true),'off F256 curve');return ret;
  }
  if(eq(p,q)){
    const z=inverse29(norm(2*y+1));
    l=norm((3*x*x-2*x-10)*z);
    nu=norm((-x*x*x-10*x-40-y)*z);
  }else{
    const z=inverse29(norm(u-x));
    l=norm((v-y)*z);nu=norm((y*u-v*x)*z);
  }
  const rx=norm(l*l+1-x-u);
  const ry=norm(-l*rx-nu-1);
  const ret=[rx,ry];must(on(ret,false),'off F29 curve');return ret;
}
function scalar(p,n,two){
  let z=null;
  for(;n;n>>>=1,p=add(p,p,two))if(n&1)z=add(z,p,two);
  return z;
}
function psi3(z){
  return mul(mul(mul(z,z),z),z)^z^1;
}
function run(){
  must(psi3(X)===0,'X quartic');
  must((mul(Y,Y)^Y)===(mul(mul(X,X),X)^mul(X,X)),'Y quadratic');
  must(X===(1^Y^exp(Y,4)),'embedding X=1+Y+Y^4');
  must(exp(Y,16)!==Y&&exp(Y,256)===Y,'degree 8');
  for(let z=1;z<256;z++)must(mul(z,exp(z,254))===1,'inverse');
  const x=18, a=[18,5], b=[X,Y];
  must(norm(3*x**4-4*x**3-60*x*x-237*x-21)===0,'psi3 mod29');
  must(on(a,false)&&on(b,true),'initial points');
  must(eq(scalar(a,3,false),null)&&eq(scalar(b,3,true),null),'order3');
  const source=[null,a,scalar(a,2,false)];
  const target=[null,b,scalar(b,2,true)];
  must(eq(source[2],[18,23])&&eq(target[2],[X,Y^1]),'negative');
  for(let i=0;i<3;i++){
    for(let j=0;j<3;j++){
      must(eq(add(source[i],source[j],false),source[(i+j)%3]),'source homomorphism');
      must(eq(add(target[i],target[j],true),target[(i+j)%3]),'target homomorphism');
    }
    const n=[0,1,2].find(k=>eq(target[i],scalar(target[1],k,true)));
    must(n===i,'wrong DLP');
  }
  const xs=Array.from({length:256},(_,i)=>i).filter(psi3);
  const torsion=[];
  for(const xx of xs)for(let yy=0;yy<256;yy++)if(on([xx,yy],true))torsion.push([xx,yy]);
  must(xs.length===4&&torsion.length===8,'geometric 3 torsion count');
  for(const p of torsion){
    must(eq(scalar(p,3,true),null),'3-torsion point');
    must(exp(p[0],16)===p[0]&&exp(p[1],16)!==p[1],'extension exact degree');
  }
  const order29=1+Array.from({length:29},(_,x)=>x).reduce(
    (t,x)=>t+Array.from({length:29},(_,y)=>y).filter(y=>on([x,y],false)).length,0);
  const order256=1+Array.from({length:256},(_,x)=>x).reduce(
    (t,x)=>t+Array.from({length:256},(_,y)=>y).filter(y=>on([x,y],true)).length,0);
  must(order29===30&&order256===225,'group cardinalities');
  must(!eq(target[1],target[2]),'wrong generator orientation');
  const raw=[18%2,5%2];
  must(!on(raw,true)||!eq(scalar(raw,3,true),null),'naive coordinates wrong');
  console.log('CT51 JS PASS: F29->F256; 9 group pairs, 3 logs, 8 torsion, orders 30/225');
}
run();
