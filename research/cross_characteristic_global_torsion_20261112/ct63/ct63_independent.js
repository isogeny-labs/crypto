#!/usr/bin/env node
'use strict';
/* CT63: independent direct Artin-Schreier image tables, not Python's trace count. */
const fields=[[1,7],[2,19],[3,67],[4,285],[5,1033]];
function mul(a,b,k,f){
  let r=0,mask=(1<<k)-1;
  while(b){if(b&1)r^=a;b>>=1;a<<=1;if(a&(1<<k))a^=f;}
  return r&mask;
}
function pow(a,n,k,f){
  let r=1;
  while(n){if(n&1)r=mul(r,a,k,f);a=mul(a,a,k,f);n>>=1;}
  return r;
}
function count(n,poly){
  const k=2*n,q=1<<k;
  const squares=Array.from({length:q},(_,a)=>mul(a,a,k,poly));
  const as=Array(q).fill(0);
  for(let y=0;y<q;y++)as[squares[y]^y]++;
  const roots=[];
  for(let a=0;a<q;a++)if((squares[a]^a^1)===0)roots.push(a);
  if(roots.length!==2)throw Error('invalid omega embedding '+k);
  const omega=roots[0];
  if(omega===0||omega===1||pow(omega,4,k,poly)!==omega)
    throw Error('invalid F4 subfield');
  let C=1,E=1;
  for(let x=0;x<q;x++){
    const x2=squares[x],x3=mul(x2,x,k,poly),x5=mul(x2,x3,k,poly);
    C+=as[x5^mul(omega,x3,k,poly)];
    E+=as[x3^omega];
  }
  const z=2**n;
  if(C!==4**n+1+(n%5===0?-4*z:z))throw Error('curve mismatch '+n);
  if(E!==(1-z)**2)throw Error('elliptic mismatch '+n);
  const J=n%5===0?(1-z)**4:1+z+z**2+z**3+z**4;
  return {n,curve_points:C,elliptic_points:E,jacobian_points:J};
}
const entries=fields.map(([n,poly])=>count(n,poly));
if(entries[0].jacobian_points!==31||entries[4].jacobian_points!==31**4)
  throw Error('wrong Jacobian count');
console.log(JSON.stringify({model:'C: y^2+y=x^5+omega*x^3; E: y^2+y=x^3+omega',
  frobenius:'X^4+2X^3+4X^2+8X+16=16 Phi5(X/2)',
  minimal_geometric_split_degree:5,entries}));
