'use strict';
// Independent BigInt CT83: modular ambiguity, binary counterexample and norm bound.
const p=37n, d=24, n=5;
function times(a,b){let out=Array(a.length+b.length-1).fill(0n);a.forEach((u,i)=>b.forEach((v,j)=>{out[i+j]+=u*v;}));return out;}
function power(a,k){let out=[1n];for(let j=0;j<k;j++)out=times(out,a);return out;}
function minus(a,b){return Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)-(b[i]??0n));}
function horner(a,x){return a.reduceRight((y,c)=>y*x+c,0n);}
const D=[-79n,-40n,-4n,4n], P3=[-21n,-237n,-60n,-4n,3n], F4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const F=minus(times(minus(times(power(D,2),F4),power(P3,3)),power(P3,3)),times(power(D,2),power(F4,3)));
function assert(ok,msg){if(!ok)throw Error(msg);}
assert(F.length===25&&F[24]===7n,'division polynomial');
const abs=x=>x<0n?-x:x,C=F.reduce((a,b)=>a>abs(b)?a:abs(b),0n);
assert(C===2207441511783053n,'coefficient max');
let F9=horner(F,9n);assert(F9%p===0n && F9%(p*p)!==0n,'mod 37 root order');
const G=power([-9n,1n],n),H=G.reduce((a,b)=>a>abs(b)?a:abs(b),0n);
assert(G.length===6&&H===59049n,'height');
for(let k=1;k<=n;k++)assert(horner(G,9n+37n*987654321n)%(p**BigInt(k))===0n,'37-adic collision');
// Binary polynomial reduction via carry-less arithmetic in 12-bit polynomial basis.
const irreducible=(1<<12)|(1<<9)|(1<<5)|(1<<2)|1;
function fieldMul(a,b){let out=0;while(b){if(b&1)out^=a;b>>>=1;a<<=1;if(a&4096)a^=irreducible;}return out;}
function fieldPow(a,k){let out=1;while(k){if(k&1)out=fieldMul(out,a);a=fieldMul(a,a);k>>>=1;}return out;}
const binary=fieldPow(2^1,n);
assert(binary===51 && binary!==0,'binary discriminant');
let boxes=(2n*H+1n)**BigInt(d),necessary=0;
while(p**BigInt(necessary)<boxes)necessary++;
assert(p**BigInt(necessary-1)<boxes,'pigeonhole');
const bound=(BigInt(d+1)*C)**BigInt(d-1)*(2n*BigInt(d)*H)**BigInt(d);
let sufficient=0;while(p**BigInt(sufficient)<=bound)sufficient++;
assert(sufficient>=necessary && necessary>n,'resultant bound');
console.log(JSON.stringify({
  binary_witness:binary,coefficient_height:Number(H),degree:d,
  experiment:'ct83-fixed-prime-precision-obstruction',
  f9_valuation:1,information_required_digits:necessary,
  local_collision_digits:n,local_modulus:Number(p**BigInt(n)),p:Number(p),
  poly_height:Number(C),resultant_sufficient_digits:sufficient,
}));
