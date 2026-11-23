#!/usr/bin/env node
// CT105 independent BigInt implementation: Newton precision-doubling, not digit Hensel.
'use strict';
function check(ok,msg) {if(!ok)throw Error(msg);}
const P=59n, N=8;
function mod(a,m){const r=a%m;return r<0n?r+m:r;}
function mul(a,b){const c=Array(a.length+b.length-1).fill(0n);
  for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]+=a[i]*b[j];return c;}
function sub(a,b){const c=[];for(let i=0;i<Math.max(a.length,b.length);i++)c.push((a[i]||0n)-(b[i]||0n));return c;}
function pw(a,n){let o=[1n];while(n){if(n&1)o=mul(o,a);a=mul(a,a);n>>=1;}return o;}
const D=[-79n,-40n,-4n,4n], A=[-21n,-237n,-60n,-4n,3n], B=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const F5=sub(mul(pw(D,2),B),pw(A,3));
const F6=mul(A,sub(F5,pw(B,2)));
const F7=sub(mul(F5,pw(A,3)),mul(pw(D,2),pw(B,3)));
const F11=sub(mul(F7,pw(F5,3)),mul(mul(pw(D,2),B),pw(F6,3)));
check(F11.length===61&&F11[60]===11n,'division polynomial length');
const FD=F11.slice(1).map((v,i)=>v*BigInt(i+1));
function ev(f,x,m){let z=0n;for(let i=f.length-1;i>=0;i--)z=mod(z*x+f[i],m);return z;}
function rhs(x){return x*x*x-x*x-10n*x-20n;}
function inv(a,m){let t=0n,nt=1n,r=m,nr=mod(a,m);
  while(nr){let q=r/nr; [t,nt]=[nt,t-q*nt];[r,nr]=[nr,r-q*nr];}
  check(r===1n,'nonunit derivative');return mod(t,m);}
function admit(x,y){return 0n<=x&&x<P&&0n<=y&&y<P&&ev(F11,x,P)===0n&&
  mod(y*y+y-rhs(x),P)===0n&&ev(FD,x,P)!==0n&&mod(2n*y+1n,P)!==0n;}
function lift(x,y){if(!admit(x,y))throw Error('not an admissible nonzero 11-torsion source point');
  let X=x,Y=y,pr=P;
  for(let e=1;e<N;e*=2){let targetExponent=Math.min(e*2,N),m=P**BigInt(targetExponent);
    X=mod(X-ev(F11,X,m)*inv(ev(FD,X,m),m),m);
    Y=mod(Y-mod(Y*Y+Y-rhs(X),m)*inv(2n*Y+1n,m),m);
    check(ev(F11,X,m)===0n&&mod(Y*Y+Y-rhs(X),m)===0n,'bad Newton lift');pr=m;
  }
  check(pr===P**BigInt(N)&&mod(X,P)===x&&mod(Y,P)===y,'orientation or precision');
  return {input:[Number(x),Number(y)],x59adic:String(X),y59adic:String(Y)};
}
let roots=[];for(let x=0n;x<P;x++)if(ev(F11,x,P)===0n)roots.push(x);
check(roots.map(Number).join(',')==='19,20,22,43,46','unexpected rational division roots');
let points=[];for(const x of roots)for(let y=0n;y<P;y++)if(mod(y*y+y-rhs(x),P)===0n)points.push([x,y]);
check(points.length===10&&points.every(([x,y])=>admit(x,y)),'oriented places');
check(!admit(19n,23n)&&!admit(0n,0n),'negative controls');
let count=0;for(let x=0n;x<P;x++)for(let y=0n;y<P;y++)if(mod(y*y+y-rhs(x),P)===0n)count++;
check(count===54,'point count');
let selected=points;
if(process.argv.length===4&&process.argv[2]==='--point'){
  const args=process.argv[3].split(',');check(args.length===2,'point x,y');selected=[[BigInt(args[0]),BigInt(args[1])]];
}else check(process.argv.length===2,'unexpected arguments');
const result={case:'ct105-scalar-blind-local-hensel',count:selected.length,
 fixed_binary_specialization_evaluated:false,global_K_element_reconstructed:false,
 lift_precision_digits:N,modulus:String(P**BigInt(N)),points:selected.map(([x,y])=>lift(x,y)),
 source_scalar_recovered:false};
process.stdout.write(JSON.stringify(result)+'\n');
