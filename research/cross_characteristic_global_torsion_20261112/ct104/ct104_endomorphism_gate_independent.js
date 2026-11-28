#!/usr/bin/env node
'use strict';
// Independent CT104 BigInt polynomial / Number finite-field check.
// No arbitrary-Q transfer or benchmark.
const fail = (reason) => { throw new Error(reason); };
const chk = (a, b, reason) => { if (JSON.stringify(a) !== JSON.stringify(b)) fail(reason); };
const strip = (a) => { while (a.length > 1 && a[a.length-1] === 0n) a.pop(); return a; };
const padd = (a,b) => strip(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)+(b[i]??0n)));
const pneg = a => a.map(x=>-x);
const psub = (a,b) => padd(a,pneg(b));
const pmul = (a,b) => {const c=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]+=a[i]*b[j];return strip(c);};
const ppow = (a,n) => {let v=[1n];while(n){if(n&1)v=pmul(v,a);a=pmul(a,a);n>>=1;}return v;};
const bmod = (x,p) => ((x%p)+p)%p;
const mod = (x,p) => Number(bmod(x,BigInt(p)));
const inv = (x,p) => {const original=x;let z=1,n=p-2;while(n){if(n&1)z=z*x%p;x=x*x%p;n>>=1;}if(z*original % p !== 1)fail('wrong inverse');return z;};
const rem = (a,b,p) => {a=strip(a.map(x=>BigInt(mod(x,p))));b=strip(b.map(x=>BigInt(mod(x,p))));if(b.length===1&&b[0]===0n)fail('zero divisor');while(a.length>=b.length&&!(a.length===1&&a[0]===0n)){
 const shift=a.length-b.length, k=mod(a[a.length-1],p)*inv(mod(b[b.length-1],p),p)%p;
 for(let i=0;i<b.length;i++)a[shift+i]=BigInt((mod(a[shift+i],p)-k*mod(b[i],p)+p*p)%p);
 strip(a);
}return a;};
const gcddeg = (a,b,p) => {a=strip(a.map(x=>BigInt(mod(x,p))));b=strip(b.map(x=>BigInt(mod(x,p))));while(!(b.length===1&&b[0]===0n)){let r=rem(a,b,p);a=b;b=r;}return a.length-1;};
const evaluate = (f,x,p) => {let z=0;for(let i=f.length-1;i>=0;i--)z=(z*x+mod(f[i],p))%p;return z;};

// Compute invariants from general Weierstrass model, not from stored j.
const [a1,a2,a3,a4,a6]=[0n,-1n,1n,-10n,-20n];
const b2=a1*a1+4n*a2, b4=a1*a3+2n*a4, b6=a3*a3+4n*a6;
const b8=a1*a1*a6+4n*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4;
const c4=b2*b2-24n*b4;
const delta=-b2*b2*b8-8n*b4*b4*b4-27n*b6*b6+9n*b2*b4*b6;
chk([b2,b4,b6,b8,c4,delta].map(String),['-4','-20','-79','-21','496','-161051'],'Weierstrass invariant discrepancy');
const jn=c4*c4*c4;
const egcd = (a,b)=>{while(b){const t=a%b;a=b;b=t;}return a;};
if(egcd(jn,-delta)!==1n||jn!==122023936n)fail('nonintegral j failure');

const D=[b6,2n*b4,b2,4n];
const P3=[b8,3n*b6,3n*b4,b2,3n];
const F4=[b4*b8-b6*b6,b2*b8-b4*b6,10n*b8,10n*b6,5n*b4,b2,2n];
chk(F4.map(String),["-5821","-1496","-210","-790","-100","-4","2"],"fourth division polynomial");
const F5=psub(pmul(ppow(D,2),F4),ppow(P3,3));
const F6=pmul(P3,psub(F5,ppow(F4,2)));
const denominator={1:[1n],2:D,3:ppow(P3,2),4:pmul(D,ppow(F4,2)),5:ppow(F5,2)};
const numerator={1:[0n,1n],2:psub([0n,...D],P3),3:psub([0n,...denominator[3]],pmul(D,F4)),
    4:psub([0n,...denominator[4]],pmul(F5,P3)),5:psub([0n,...denominator[5]],pmul(D,pmul(F6,F4)))};
let degrees=[];
for(let n=1;n<=5;n++){
 degrees.push([n,numerator[n].length-1,denominator[n].length-1]);
 if(numerator[n][numerator[n].length-1]!==1n||denominator[n][denominator[n].length-1]!==BigInt(n*n))fail('degree / leading coefficient');
 for(const q of [101,103])if(gcddeg(numerator[n],denominator[n],q)!==0)fail('rational map not coprime mod '+q);
}
chk(degrees,[[1,1,0],[2,4,3],[3,9,8],[4,16,15],[5,25,24]],'scalar map degrees');
let crossMax=0;
for(let m=1;m<=5;m++)for(let n=m+1;n<=5;n++){
 const diff=psub(pmul(numerator[m],denominator[n]),pmul(numerator[n],denominator[m]));
 if(diff.length===1&&diff[0]===0n)fail('colliding scalar rational maps');
 crossMax=Math.max(crossMax,diff.length-1);
}
if(crossMax!==40||crossMax>=60)fail('degree 60 noncollision');

// Different group-law implementation: residues, Fermat inversion.
const p=59;
const adjust=x=>(x%p+p)%p;
const onCurve=Q=>Q===null||adjust(Q[1]*Q[1]+Q[1]-Q[0]*Q[0]*Q[0]+Q[0]*Q[0]+10*Q[0]+20)===0;
const pointPlus = (P,Q) => {
 if(P===null)return Q;if(Q===null)return P;
 const [x,y]=P,[xx,yy]=Q;
 if(x===xx&&adjust(y+yy+1)===0)return null;
 const top=(x===xx)?3*x*x-2*x-10:yy-y;
 const bot=(x===xx)?2*y+1:xx-x;
 const slope=adjust(adjust(top)*inv(adjust(bot),p));
 const x3=adjust(slope*slope+1-x-xx),y3=adjust(-slope*(x3-x)-y-1);
 if(!onCurve([x3,y3]))fail('point off curve');
 return [x3,y3];
};
const P=[19,22];if(!onCurve(P))fail('source P');
const mult=[null];for(let i=0;i<11;i++)mult.push(pointPlus(mult[mult.length-1],P));
if(mult.length!==12||mult[11]!==null||new Set(mult.slice(0,11).map(x=>JSON.stringify(x))).size!==11)fail('source order');
for(let i=0;i<11;i++)for(let j=0;j<11;j++)chk(pointPlus(mult[i],mult[j]),mult[(i+j)%11],'source table');
const roots=[];for(let n=1;n<=5;n++){
 const x=mult[n][0],z=evaluate(numerator[n],19,p)*inv(evaluate(denominator[n],19,p),p)%p;
 if(z!==x)fail('source specialization division map');roots.push(x);
}
chk(roots,[19,20,43,46,22],'source x classes');
const out={
 "case":"ct104-noncm-endomorphism-gate","complete_cost_measured":false,"curve":"11a1","curve_discriminant":Number(delta),"fixed_target_binary_degree":40,
 "global_oriented_degree":120,"global_x_degree":60,"j_den":161051,"j_integral":false,"j_num":-122023936,"max_cross_difference_degree":crossMax,
 "online_global_lift_evaluated":false,"scalar_map_degrees":degrees,"scalar_maps_coprime_mod_primes":[101,103],"source_group_addition_pairs":121,
 "source_modulus":59,"source_subgroup_order":11,"source_x_multiples_1_to_5":roots
};
process.stdout.write(JSON.stringify(Object.fromEntries(Object.entries(out).sort(([a],[b])=>a.localeCompare(b))))+'\n');
