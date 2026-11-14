#!/usr/bin/env node
'use strict';
// CT98 independently certifies integer-polynomial differential identities,
// source/binary ordinates, and target group laws. The multiplication-map
// fixture is an oracle, NOT independent CT97 algebraic-root discovery.
// Scientific status: [pcc:unverified].
const ck=(v,m)=>{if(!v)throw Error(m)};
const trim=p=>{p=p.slice();while(p.length>1&&p.at(-1)===0n)p.pop();return p};
const add=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)+(b[i]||0n)));
const scale=(a,c)=>trim(a.map(v=>v*c));
const sub=(a,b)=>add(a,scale(b,-1n));
function mul(a,b){const r=Array(a.length+b.length-1).fill(0n);
 for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)r[i+j]+=a[i]*b[j];return trim(r)}
function pow(a,n){let v=[1n];for(let i=0;i<n;i++)v=mul(v,a);return v}
const deriv=p=>trim(p.slice(1).map((v,i)=>v*BigInt(i+1)));
const eq=(a,b)=>JSON.stringify(a.map(String))===JSON.stringify(b.map(String));
const X=[0n,1n],D=[-79n,-40n,-4n,4n],P3=[-21n,-237n,-60n,-4n,3n],
 F4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n],M=[-10n,-2n,3n];
const A2=add(pow(M,2),mul([1n,-2n],D));
const A3=sub(mul(X,pow(P3,2)),mul(D,F4));
const specs=[[X,[1n]],[A2,D],[A3,pow(P3,2)]];
function at(a,z,p){let v=0n;for(let i=a.length-1;i>=0;i--)v=(v*z+a[i])%p;return (v+p)%p}
function inv37(z){for(let i=1n;i<37n;i++)if(z*i%37n===1n)return i;throw Error('bad 37 prime')}
function m12(a,b){let w=0;while(b){if(b&1)w^=a;b>>=1;a<<=1;if(a&4096)a^=0x1225;}return w&4095}
function exp12(a,n){let c=1;while(n){if(n&1)c=m12(c,a);a=m12(a,a);n>>=1}return c}
function at12(a){let v=0;for(let i=a.length-1;i>=0;i--)v=m12(v,2)^Number(a[i]&1n);return v}
const ax=(u,v)=>[u[0]^v[0],u[1]^v[1]];
function mx(u,v){let q=m12(u[1],v[1]);return [m12(u[0],v[0])^m12(q,12),m12(u[0],v[1])^m12(u[1],v[0])^q]}
function fx(a,n){let t=[1,0];while(n){if(n&1)t=mx(t,a);a=mx(a,a);n>>=1}return t}
const equal=(a,b)=>a.length===b.length&&a.every((v,i)=>v===b[i]);
function plus(P,Q){if(!P)return Q;if(!Q)return P;let [x,y]=P,[z,w]=Q;let lambda;
 if(equal(x,z)){
  if(equal(ax(y,w),[1,0]))return null;
  ck(equal(y,w),'y equality');lambda=mx(x,x);
 } else lambda=mx(ax(y,w),fx(ax(x,z),2**24-2));
 const X= ax(ax(mx(lambda,lambda),x),ax(z,[1,0]));
 const Y= ax(ax(mx(lambda,ax(x,X)),y),[1,0]);return [X,Y]
}
function onCurve(P){const [x,y]=P;return equal(ax(mx(y,y),y),ax(mx(mx(x,x),x),mx(x,x)))}
const rows=[],original= [[2,0],[0,1]];
ck(onCurve(original),'binary generator');
let multiple=original;
for(let ix=0;ix<specs.length;ix++){
 const [A,B]=specs[ix],B2=pow(B,2);
 const N=sub(mul(deriv(A),B),mul(A,deriv(B)));
 const H=add(sub(sub(scale(pow(A,3),4n),scale(mul(pow(A,2),B),4n)),scale(mul(A,B2),40n)),scale(pow(B,3),-79n));
 // Recover multiplier from the map's leading coefficients, not a public scalar.
 const ratioN=B.at(-1),ratioD=A.at(-1);
 ck(ratioN%ratioD===0n,'leading ratio nonintegral');
 const n2=ratioN/ratioD;
 let m=1n;while(m*m<n2)m++;
 ck(m*m===n2,'leading ratio not square');
 ck(eq(mul(D,pow(N,2)),scale(mul(B,H),m*m)),'differential identity');
 ck(N.every(c=>c%m===0n),'N not divisible by multiplier');
 const V=N.map(c=>c/m), Vminus=sub(V,B2);
 ck(Vminus.every(c=>c%2n===0n),'not 2-integral');
 const U=Vminus.map(c=>c/2n),f=[-20n,-10n,-1n,1n];
 const lhs=add(add(pow(U,2),mul(U,B2)),mul(pow(V,2),f));
 const rhs=mul(B,sub(sub(sub(pow(A,3),mul(pow(A,2),B)),scale(mul(A,B2),10n)),scale(pow(B,3),20n)));
 ck(eq(lhs,rhs),'global curve equation');
 const sd=at(B2,9n,37n),sx=at(A,9n,37n)*inv37(at(B,9n,37n))%37n;
 const sy=(at(U,9n,37n)+4n*at(V,9n,37n))*inv37(sd)%37n;
 const targetDen=at12(B),targetDen2=m12(targetDen,targetDen);
 ck(targetDen!==0,'binary denominator');
 const dinv=exp12(targetDen2,4094), xbin=m12(at12(A),exp12(targetDen,4094));
 const posY=[m12(at12(U),dinv),m12(at12(V),dinv)];
 ck(onCurve([[xbin,0],posY]),'binary point equation');
 ck(equal([[xbin,0],posY][0],multiple[0])&&equal(posY,multiple[1]),'independent group-law control');
 for(let sign of [1,-1]){
  const ys=sign===1?Number(sy):Number((36n-sy+37n)%37n);
  const yt=sign===1?posY:[posY[0]^1,posY[1]];
  rows.push({source:[Number(sx),ys],target_x:xbin,target_y:yt,
      multiplier_from_x_map:Number((BigInt(sign)*m+7n)%7n),multiplier_magnitude:Number(m),
      differential_certificate:true,two_integral_numerator:true});
 }
 multiple=plus(multiple,original);
}
rows.sort((a,b)=>a.source[0]-b.source[0]||a.source[1]-b.source[1]);
const positive=rows.filter(z=>z.multiplier_from_x_map<4);
ck(equal(positive.map(r=>r.target_x),[2,17,1562]),'three binary x roots');
const output={experiment:'CT98-differential-ordinate-certificate',orientation_transport:true,
 scalar_independent_transfer:false,leaked_source_log:true,source_rows:rows};
function stable(v){if(Array.isArray(v))return '['+v.map(stable).join(',')+']';
 if(v&&typeof v==='object')return '{'+Object.keys(v).sort().map(k=>JSON.stringify(k)+':'+stable(v[k])).join(',')+'}';
 return JSON.stringify(v)}
process.stdout.write(stable(output)+'\n');
