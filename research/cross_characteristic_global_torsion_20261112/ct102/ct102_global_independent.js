#!/usr/bin/env node
'use strict';
// CT102 independent BigInt division polynomial, finite-field Rabin proof,
// and exact fraction-free Sylvester determinant certificate.
function check(b,s){if(!b)throw Error(s);}
const trim=a=>{a=a.slice();while(a.length>1&&a.at(-1)===0n)a.pop();return a;};
const sub=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)-(b[i]||0n)));
function mul(a,b){let o=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)o[i+j]+=a[i]*b[j];return trim(o);}
function pow(a,n){let r=[1n];while(n){if(n&1)r=mul(r,a);a=mul(a,a);n>>=1;}return r;}
const D=[-79n,-40n,-4n,4n],f3=[-21n,-237n,-60n,-4n,3n],f4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const d2=pow(D,2),f5=sub(mul(d2,f4),pow(f3,3)),f6=mul(f3,sub(f5,pow(f4,2))),f7=sub(mul(f5,pow(f3,3)),mul(d2,pow(f4,3)));
const f11=sub(mul(f7,pow(f5,3)),mul(mul(d2,f4),pow(f6,3)));
check(f11.length===61&&f11[60]===11n,'psi11 degree/leading');

// Distinct mod13 arithmetic from Python quotient implementation.
const p=13,modnum=z=>Number((z%13n+13n)%13n);
function tr(a){a=a.slice();while(a.length>1&&a.at(-1)===0)a.pop();return a;}
function mpow(a,e){let z=1;while(e){if(e&1)z=z*a%p;a=a*a%p;e>>=1;}return z;}
function pmul(a,b){let o=Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)o[i+j]=(o[i+j]+a[i]*b[j])%p;return tr(o);}
function psub(a,b){return tr(Array.from({length:Math.max(a.length,b.length)},(_,i)=>((a[i]||0)-(b[i]||0)+p)%p));}
function prem(a,b){a=tr(a);b=tr(b);let inv=mpow(b.at(-1),p-2);while(a.length>=b.length&&!(a.length===1&&a[0]===0)){let off=a.length-b.length,coef=a.at(-1)*inv%p;for(let j=0;j<b.length;j++)a[off+j]=(a[off+j]-coef*b[j]%p+p)%p;a=tr(a);}return a;}
function pgcd(a,b){while(!(b.length===1&&b[0]===0))[a,b]=[b,prem(a,b)];let v=mpow(a.at(-1),p-2);return a.map(x=>x*v%p);}
function pexp(a,n,f){let r=[1];while(n){if(n&1)r=prem(pmul(r,a),f);a=prem(pmul(a,a),f);n>>=1;}return r;}
const q=f11.map(modnum),X=[0,1];let v=X;
for(let k=1;k<=60;k++){
 v=pexp(v,13,q);
 if([12,20,30].includes(k))check(JSON.stringify(pgcd(q,psub(v,X)))==='[1]','Rabin gcd '+k);
 if(k===60)check(JSON.stringify(v)==='[0,1]','Rabin final');
}
// Sylvester determinant of degree-60 psi11 and cubic D.
const n=60,m=3,s=n+m,A=[];
for(let i=0;i<m;i++)A.push([...Array(i).fill(0n),...f11.toReversed(),...Array(m-1-i).fill(0n)]);
for(let i=0;i<n;i++)A.push([...Array(i).fill(0n),...D.toReversed(),...Array(n-1-i).fill(0n)]);
check(A.length===s&&A.every(row=>row.length===s),'Sylvester');
let prev=1n,sign=1n;
for(let k=0;k<s-1;k++){
 if(A[k][k]===0n){let j=k+1;while(j<s&&A[j][k]===0n)j++;check(j<s,'singular');[A[k],A[j]]=[A[j],A[k]];sign=-sign;}
 let pivot=A[k][k];
 for(let i=k+1;i<s;i++){
  let left=A[i][k];
  for(let j=k+1;j<s;j++){
   let t=A[i][j]*pivot-left*A[k][j];check(t%prev===0n,'nonexact Bareiss');A[i][j]=t/prev;
  }
  A[i][k]=0n;
 }
 prev=pivot;
}
const resultant=sign*A[s-1][s-1];
const expected=-1617717835776189959423474389729474523259174290922026987531771334356336309518989285550030273833071044614659831525201714316224621866288174560213656800883169001n;
check(resultant===expected&&resultant<0n,'resultant negative norm');
const obj={case:'ct102-global-eleven-point-field',global_lift_of_public_source_computed:false,irreducible_mod_prime:13,minimal_x_degree:60,negative_norm_certifies_nonsquare:true,norm_denominator:1331,oriented_point_field_degree:120,rabin_final_exponent:60,rabin_gcd_exponents:[12,20,30],resultant_f11_D:String(resultant)};
function sorted(o){if(Array.isArray(o))return o.map(sorted);if(o&&typeof o==='object')return Object.fromEntries(Object.keys(o).sort().map(k=>[k,sorted(o[k])]));return o;}
process.stdout.write(JSON.stringify(sorted(obj))+'\n');
