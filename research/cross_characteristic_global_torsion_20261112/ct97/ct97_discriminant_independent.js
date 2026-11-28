#!/usr/bin/env node
"use strict";
// CT97 independent BigInt check of global discriminant-square resolvent and
// recovered rational-function specialization. It does NOT reproduce SymPy
// nullspace discovery, nor does it certify y-orientation. [pcc:unverified]
const ck=(ok,msg)=>{if(!ok)throw Error(msg)};
const Q2=23624165350186n,Q3=10252887761980724n;
const g=[-2102915n,3828478n,-6423347n,-1696464n,-39690n,
         -29862n,-840n,-8n,1n];
const c2=[51746818064829n,-330291288560641n,-95029425174429n,
          -1711075375204n,-1596513316848n,-44914399152n,
          -455319459n,54044109n];
const c3=[-201756977283948362n,563360952055619683n,
          149521192983511524n,3495575179577110n,2638006865239263n,
          74263835359077n,711827220769n,-88442189137n];
const cd=[1330115088493357156n,656532317695454585n,
          17598156120098064n,12706441996691436n,225325011610699n,
          -309225601391n,-443542601663n,4007355953n];
const trim=v=>{while(v.length>1&&v[v.length-1]===0n)v.pop();return v};
function add(a,b){let r=Array(Math.max(a.length,b.length)).fill(0n);
 for(let i=0;i<r.length;i++)r[i]=(a[i]||0n)+(b[i]||0n);return trim(r)}
const scale=(a,c)=>trim(a.map(x=>x*c));
const neg=(a)=>scale(a,-1n);
const sub=(a,b)=>add(a,neg(b));
function mul(a,b){let r=Array(a.length+b.length-1).fill(0n);
 for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)r[i+j]+=a[i]*b[j];
 return trim(r)}
function pow(a,n){let v=[1n];while(n-->0)v=mul(v,a);return v}
function rem(a){a=trim(a.slice());
 while(a.length>=g.length&&!(a.length===1&&a[0]===0n)){
  const coeff=a[a.length-1],shift=a.length-g.length;
  for(let i=0;i<g.length;i++)a[i+shift]-=coeff*g[i];trim(a);
 }return a}
const S=[0n,1n],den=Q2**3n*Q3**2n;
// Delta = s^2 e2^2 -4e2^3 -4s^3 e3 -27e3^2 +18s e2 e3.
// Verify Cd(s)^2/Q3^2 = Delta in the exact degree-eight subgroup field.
const lhs=scale(pow(cd,2),den/(Q3**2n));
let rhs=scale(mul(pow(S,2),pow(c2,2)),den/(Q2**2n));
rhs=sub(rhs,scale(pow(c2,3),4n*den/(Q2**3n)));
rhs=sub(rhs,scale(mul(pow(S,3),c3),4n*den/Q3));
rhs=sub(rhs,scale(pow(c3,2),27n*den/(Q3**2n)));
rhs=add(rhs,scale(mul(mul(S,c2),c3),18n*den/(Q2*Q3)));
ck(rem(sub(lhs,rhs)).length===1&&rem(sub(lhs,rhs))[0]===0n,
   "resolvent is not a square in the subgroup field");

const D=[-79n,-40n,-4n,4n],P3=[-21n,-237n,-60n,-4n,3n],
      F4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const M=[-10n,-2n,3n];
const A2=add(pow(M,2),mul([1n,-2n],D)), B2=D;
const A3=sub(mul([0n,1n],pow(P3,2)),mul(D,F4)),B3=pow(P3,2);
function at(a,x,m){let v=0n;for(let i=a.length-1;i>=0;i--)v=(v*x+a[i])%m;
 return (v+m)%m}
function invmod(a,m){
 for(let v=1n;v<m;v++)if(a*v%m===1n)return v;
 throw Error("zero denominator")}
function src(A,B){
 return Number(at(A,9n,37n)*invmod(at(B,9n,37n),37n)%37n)}
function m12(a,b){
 let out=0;
 while(b){if(b&1)out^=a;a<<=1;if(a&4096)a^=0x1225;b>>=1}
 return out}
function p12(a,n){let r=1;while(n){if(n&1)r=m12(r,a);a=m12(a,a);n>>=1}return r}
function eval12(a){let v=0;for(let i=a.length-1;i>=0;i--)
 v=m12(v,2)^Number(a[i]&1n);return v}
function bin(A,B){const b=eval12(B);ck(b!==0,"binary denominator");
 return m12(eval12(A),p12(b,4094))}
const rows=[
 {public_x:src([0n,1n],[1n]),target_binary_x:bin([0n,1n],[1n]),
  numerator_degree:1,denominator_degree:0,global_identity:true},
 {public_x:src(A2,B2),target_binary_x:bin(A2,B2),
  numerator_degree:A2.length-1,denominator_degree:B2.length-1,global_identity:true},
 {public_x:src(A3,B3),target_binary_x:bin(A3,B3),
  numerator_degree:A3.length-1,denominator_degree:B3.length-1,global_identity:true}
];
ck(JSON.stringify(rows.map(r=>r.public_x))==="[9,27,28]","source roots");
ck(JSON.stringify(rows.map(r=>r.target_binary_x))==="[2,17,1562]","target roots");
ck(!rows.some(r=>r.public_x===10),"negative source control");
const output={experiment:"CT97-cubic-discriminant-scalar-blind-x",
 exact_three_global_roots:true,nonmember_rejected:true,
 oriented_y_reconstruction:false,source_targets:rows};
function stable(o){if(Array.isArray(o))return "["+o.map(stable).join(",")+"]";
 if(o!==null&&typeof o==="object")return "{"+Object.keys(o).sort().map(
 k=>JSON.stringify(k)+":"+stable(o[k])).join(",")+"}";
 return JSON.stringify(o)}
process.stdout.write(stable(output)+"\n");
