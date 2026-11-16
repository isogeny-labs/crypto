#!/usr/bin/env node
'use strict';
// CT77: independent characteristic-two partial derivatives, Frobenius matrix,
// CRT audit and direct GF8/GF32 point counts with alternate polynomial basis.
const SOURCE=[0,-1,1,-10,-20], FIXTURES=[[3,29],[5,29],[7,37],[11,59],[13,139]];
function check(v,msg){if(!v)throw Error(msg)}
function mod(a,n){return (a%n+n)%n}
function onCurve(a,x,y){const [a1,a2,a3,a4,a6]=a;
 return mod(y*y+a1*x*y+a3*y-x*x*x-a2*x*x-a4*x-a6,2)===0}
function smooth(a){const [a1,a2,a3,a4]=a;
  if(a1===0)return a3===1;
  const x=a3,y=mod(x+a4,2);
  return !onCurve(a,x,y);
}
function countF2(a){let n=1;for(let x=0;x<2;x++)for(let y=0;y<2;y++)if(onCurve(a,x,y))n++;return n}
function discr(a){const [u,v,w,x,y]=a;
  const b2=u*u+4*v,b4=2*x+u*w,b6=w*w+4*y;
  const b8=u*u*y+4*v*y-u*w*x+v*w*w-x*x;
  return -b2*b2*b8-8*b4**3-27*b6*b6+9*b2*b4*b6;
}
function crt(a,p){return a.map((b,i)=>SOURCE[i]+mod(b-SOURCE[i],2)*p)}
function matrixMul(A,B,n){const C=[];
 for(let i=0;i<2;i++)for(let j=0;j<2;j++)
   C.push(mod(A[2*i]*B[j]+A[2*i+1]*B[j+2],n));
 return C;
}
function traceCount(t,f){let prev=2,cur=t;for(let n=1;n<f;n++){let nxt=t*cur-2*prev;prev=cur;cur=nxt}return 2**f+1-cur}
function gate(t,l){const F=[0,mod(-2,l),1,mod(t,l)];let M=[1,0,0,1],some=null,full=null;
 for(let k=1;k<512;k++){
   M=matrixMul(F,M,l);
   const d=mod((M[0]-1)*(M[3]-1)-M[1]*M[2],l);
   let prev=mod(2,l),cur=mod(t,l),q=1;
   for(let n=1;n<=k;n++){
     q=mod(q*2,l);
     if(n>1){let nxt=mod(t*cur-2*prev,l);prev=cur;cur=nxt}
   }
   check(d===mod(q+1-cur,l),'matrix/trace disagreement');
   if(d===0&&some===null)some=k;
   if(M.every((x,i)=>x===[1,0,0,1][i])){full=k;break}
 }
 check(some!==null&&full!==null,'no Frobenius period');return [some,full];
}
function gfMul(a,b,poly,n){let r=0;while(b){if(b&1)r^=a;b>>=1;a<<=1;if(a&(1<<n))a^=poly}return r}
function directCount(a,f){const poly=f===3?13:f===5?41:0;check(poly!==0,'unsupported degree');
 const q=1<<f,[a1,a2,a3,a4,a6]=a;let total=1;
 for(let x=0;x<q;x++){
   let x2=gfMul(x,x,poly,f),x3=gfMul(x2,x,poly,f),rhs=x3^(a2*x2)^(a4*x)^a6;
   for(let y=0;y<q;y++)
     if((gfMul(y,y,poly,f)^(a1*gfMul(x,y,poly,f))^(a3*y))===rhs)total++;
 }
 return total;
}
function main(){const reps={},hist={};let good=0,singular=0;
 for(let a1=0;a1<2;a1++)for(let a2=0;a2<2;a2++)
 for(let a3=0;a3<2;a3++)for(let a4=0;a4<2;a4++)for(let a6=0;a6<2;a6++){
  const a=[a1,a2,a3,a4,a6],smoothHere=smooth(a);
  check(smoothHere===(mod(discr(a),2)===1),'derivative/discriminant mismatch');
  if(!smoothHere){singular++;continue}
  good++;const t=3-countF2(a);
  if(!Object.prototype.hasOwnProperty.call(reps,t))reps[t]=a;
  hist[t]=(hist[t]||0)+1;
  for(let [ell,p] of FIXTURES){const A=crt(a,p);
    check(A.every((x,i)=>mod(x-SOURCE[i],p)===0&&mod(x-a[i],2)===0),'CRT mismatch');
    check(mod(discr(A),p)!==0&&mod(discr(A),2)!==0,'bad CRT reduction');
  }
 }
 check(good===16&&singular===16,'wrong census');
 const traces=Object.keys(hist).map(Number).sort((a,b)=>a-b).map(t=>[t,hist[t]]);
 check(JSON.stringify(traces)===JSON.stringify([[-2,2],[-1,4],[0,4],[1,4],[2,2]]),'trace histogram wrong');
 console.log(JSON.stringify({good,singular,traces,type:'census'}));
 for(let [ell,p] of FIXTURES){const gates=Object.keys(hist).map(t=>{
     const [f,full]=gate(Number(t),ell);return {f,trace:Number(t),full};
   }).sort((a,b)=>a.f-b.f||a.trace-b.trace||a.full-b.full);
   const {f,trace,full}=gates[0],model=reps[trace];
   console.log(JSON.stringify({ell,f,full,model,p,trace,type:'best'}));
   if([7,11,13].includes(ell)){
     const points=directCount(model,f);
     check(points===traceCount(trace,f)&&points%ell===0,'direct extension count mismatch');
     console.log(JSON.stringify({ell,f,points,type:'direct'}));
   }
 }
}
main();
