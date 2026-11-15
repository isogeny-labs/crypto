#!/usr/bin/env node
'use strict';
// CT65 independent binary-field orbit enumeration + diagonal Frobenius check.
const Q=1024, POLY=0x409, ell=31;
function ck(b,m){if(!b)throw Error(m);}
function mult(a,b){let z=0;while(b){if(b&1)z^=a;b>>>=1;a<<=1;if(a&Q)a^=POLY;}return z&(Q-1);}
function power(a,n){let v=1;while(n){if(n%2)v=mult(v,a);a=mult(a,a);n=Math.floor(n/2);}return v;}
const omega=Array.from({length:Q-2},(_,i)=>i+2).find(x=>(mult(x,x)^x^1)===0);
const eq=new Map();
for(let y=0;y<Q;y++){const z=mult(y,y)^y;const ar=eq.get(z)||[];ar.push(y);eq.set(z,ar);}
const involutions=new Map(),order4=[];
for(let r=1;r<Q;r++){
 const r2=mult(r,r),r3=mult(r2,r),r4=mult(r2,r2);
 const a=power(r,Q/2),b=r4^mult(omega,r2);
 if((mult(b,b)^a)!==mult(omega,r))continue;
 const cc=eq.get(mult(r4,r)^mult(omega,r3));
 if(!cc)continue;
 const flag=mult(a,r2)^mult(b,r);
 if(flag===0)involutions.set(r,[a,b,cc]);else order4.push(r);
}
ck(involutions.size===5&&order4.length===10,'automorphism census');
const rs=[...involutions.keys()].sort((a,b)=>a-b);
ck(rs.join(',')==='20,534,619,809,832','involutions');
const orbit=[20];let z=power(20,4);
while(z!==20){ck(!orbit.includes(z),'no repeated orbit');orbit.push(z);z=power(z,4);}
ck(orbit.join(',')==='20,832,619,809,534','cycle');
const tuple=[];
for(const [r,[a,b,cs]] of involutions)for(const c of cs)tuple.push([r,a,b,c]);
const visited=new Set(),lengths=[];
for(const t of tuple){
 const k=t.join(',');if(visited.has(k))continue;
 let x=t.slice(),count=0;
 do{ck(count<10,'orbit bound');visited.add(x.join(','));count++;x=x.map(v=>power(v,4));}
 while(x.join(',')!==k);
 lengths.push(count);
}
ck(lengths.length===2&&lengths.every(v=>v===5),'two five-cycles');
let selected=[20,6,120,432];
for(let i=0;i<4;i++)selected=selected.map(v=>power(v,4));
ck(selected.join(',')==='534,794,636,668','quotient conjugacy');
function poly(v){return ((((v+2)*v+4)*v+8)*v+16)%ell;}
const eigen=[1,2,4,8,16].filter(v=>v!==2);
ck(eigen.length===4&&eigen.every(v=>poly(v)===0),'Weil polynomial roots');
const projected=eigen.map(v=>{
 let sum=0,x=1;
 for(let i=0;i<5;i++){sum=(sum+x)%ell;x=x*v%ell;}
 return sum*25%ell;
});
ck(projected.join(',')==='1,0,0,0','trace eigenvalues');
ck(eigen.every(v=>powerMod(v,5,ell)===1),'F^5');
function powerMod(a,n,m){let z=1;while(n){if(n%2)z=z*a%m;n=Math.floor(n/2);a=a*a%m;}return z;}
console.log(JSON.stringify({omega,involutive_r:rs,orbit_r20:orbit,lift_orbits:lengths.length,
 lift_orbit_length:5,ct64_quotients_conjugate:true,frobenius_roots_mod31:eigen,
 frobenius_order_on_31torsion:5,trace_projector_rank:projected.filter(v=>v).length,
 trace_projector_kernel_rank:projected.filter(v=>!v).length,
 trace_projector_idempotent:projected.every(v=>v*v%ell===v)}));
