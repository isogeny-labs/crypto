#!/usr/bin/env node
'use strict';
// Independent CT55 finite-field audit: direct matrix Frobenius, direct curve group.
function check(ok,msg) {if(!ok) throw Error(msg)}
function mod(a,p) {return ((a%p)+p)%p}
function inv(a,p) {for(let x=1;x<p;x++)if(mod(a*x,p)===1)return x;throw Error('no inverse')}
function isPrime(p) {if(p<2)return false;for(let d=2;d*d<=p;d++)if(p%d===0)return false;return true}
function shortPoints(p,a,b) {const ps=[];for(let x=0;x<p;x++)for(let y=0;y<p;y++) if(mod(y*y-x*x*x-a*x-b,p)===0)ps.push([x,y]);return ps}
function add(p,a,u,v) {if(u===null)return v;if(v===null)return u;const [x,y]=u,[s,t]=v;
if(x===s&&mod(y+t,p)===0)return null;let slope;
if(x===s&&y===t) slope=mod((3*x*x+a)*inv(mod(2*y,p),p),p);
else slope=mod((t-y)*inv(mod(s-x,p),p),p);
const X=mod(slope*slope-x-s,p),Y=mod(slope*(x-X)-y,p);return [X,Y]
}
function mul(p,a,u,k) {let out=null;for(;k;k>>=1){if(k&1)out=add(p,a,out,u);u=add(p,a,u,u)}return out}
function samePoint(a,b) {return JSON.stringify(a)===JSON.stringify(b)}
function sourceCase(p,ell){for(let a=0;a<p;a++)for(let b=0;b<p;b++){
if(mod(4*a*a*a+27*b*b,p)===0)continue;
const points=shortPoints(p,a,b),order=points.length+1;
if(order%ell!==0)continue;
for(let P of points){if(mul(p,a,P,ell)===null)return {a,b,order,P}}
}throw Error('no source')}
function matmul(A,B,l){return [mod(A[0]*B[0]+A[1]*B[2],l),mod(A[0]*B[1]+A[1]*B[3],l),mod(A[2]*B[0]+A[3]*B[2],l),mod(A[2]*B[1]+A[3]*B[3],l)]}
function matrixGate(ell) {let F=[0,mod(-2,ell),1,0],v=[1,0,0,1],fs=null,ff=null;
for(let k=1;k<=2*ell+5;k++){
v=matmul(v,F,ell);
const det=mod((v[0]-1)*(v[3]-1)-v[1]*v[2],ell);
if(fs===null&&det===0)fs=k;
if(v[0]===1&&v[1]===0&&v[2]===0&&v[3]===1){ff=k;break}
}
check(fs!==null&&ff!==null,'Frobenius gate not found');return {fs,ff}}
function traceAndOrder(ell,f){let t0=2,t1=0;
for(let j=2;j<=f;j++){const next=mod(-2*t0,ell);t0=t1;t1=next}
const trace=f===1?0:t1;
let pow=1;for(let i=0;i<f;i++)pow=mod(pow*2,ell);
return mod(pow+1-trace,ell)
}
function main(){const tests=[[5,3],[7,5],[11,7],[13,3],[17,5],[19,7],[23,11],[29,5],[31,7]];
for(const [p,ell] of tests){const {a,b,order,P}=sourceCase(p,ell);const A=a+(a%2?p:0),B=b+(b%2?p:0);
// E : y^2+p*y=x^3 + A*x+B.
const disc=-64*A*A*A-27*(p*p+4*B)**2;
check(mod(disc,p)!==0 && mod(disc,2)!==0,'bad discriminant');
check(mod(A,p)===a&&mod(B,p)===b&&A%2===0&&B%2===0,'bad CRT');
check(mul(p,a,P,ell)===null,'source order fail');
const gate=matrixGate(ell);
console.log('source',p,'ell',ell,'a,b',a,b,'group',order,'P',P,'lift A,B',A,B,'binary_f',gate.ff);
}
console.log('ell : f_some, f_full, #E(F_2^f_full) divisible by ell^2');
const expected=[[3,1,2],[5,8,8],[7,12,12],[11,5,10],[13,24,24],[17,16,16],[19,9,18],[23,44,44],[31,20,20],[37,72,72],[41,40,40],[43,7,14]];
for(const [ell,fsExpected,ffExpected] of expected){const {fs,ff}=matrixGate(ell);
check(fs===fsExpected&&ff===ffExpected,'expected gate mismatch '+ell);
check(traceAndOrder(ell,fs)===0&&traceAndOrder(ell,ff)===0,'point count gate fails '+ell);
for(let f=1;f<fs;f++)check(traceAndOrder(ell,f)!==0,'nonminimal some point '+ell);
let r=1,t=mod(-2,ell);while(t!==1){t=mod(t*(-2),ell);r++}
check(ff===2*r,'F^2 relation fails');
console.log(ell,fs,ff)
}
// Reversing the CRT parity changes the F2 reduction (negative control).
// For E': y²+y=x³+x over F2, #E'(F2)=5 vs #E(F2)=3.
const num=(A)=>{let total=1;for(let x=0;x<2;x++)for(let y=0;y<2;y++)if(mod(y*y+y-x*x*x-A*x,2)===0)total++;return total};
check(num(0)===3&&num(1)===5,'bad parity mutant');
console.log('PASS: independent CRT, point orders, matrix Frobenius gates; parity mutant rejected');
}
main();
