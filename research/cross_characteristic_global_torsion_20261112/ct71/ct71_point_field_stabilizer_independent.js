#!/usr/bin/env node
/* CT71 independent: generator-conjugation + subgroup fixed-vector check.
 * The Python verifier instead conjugates EVERY element of H. */
'use strict';
const primes=[3,5,7,11];
// Independent exact polynomial substitution, CT69 Y -> -Y-1.
const H=[-101306819n,89655n,267096n,349907n,162543n,-14817n,-4813n,108n,27n];
function polyEval(coeff,x){let a=0n;for(let j=coeff.length-1;j>=0;j--)a=a*x+coeff[j];return a;}
function polyMul(A,B){let R=Array(A.length+B.length-1).fill(0n);for(let i=0;i<A.length;i++)for(let j=0;j<B.length;j++)R[i+j]+=A[i]*B[j];return R;}
let P=[1n], sum=Array(H.length).fill(0n);
for(let i=0;i<H.length;i++){for(let j=0;j<P.length;j++)sum[j]+=H[i]*P[j]; P=polyMul(P,[-1n,-1n]);}
if(sum.some((x,i)=>x!==H[i])||polyEval(H,5n)%29n!==0n||polyEval(H,23n)%29n!==0n)throw Error('CT69 global negation fails');
const mod=(x,p)=>((x%p)+p)%p;
function mm(a,b,p){return [mod(a[0]*b[0]+a[1]*b[2],p),mod(a[0]*b[1]+a[1]*b[3],p),mod(a[2]*b[0]+a[3]*b[2],p),mod(a[2]*b[1]+a[3]*b[3],p)];}
function minv(a,p){let det=mod(a[0]*a[3]-a[1]*a[2],p);if(!det) throw Error('singular');let z=1;while(mod(det*z,p)!==1) z++;return [mod(a[3]*z,p),mod(-a[1]*z,p),mod(-a[2]*z,p),mod(a[0]*z,p)];}
function inH(a,p){return mod(a[0],p)===1 && mod(a[2],p)===0 && mod(a[3],p)!==0;}
function action(a,x,y,p){return [mod(a[0]*x+a[1]*y,p),mod(a[2]*x+a[3]*y,p)];}
function primitive(p){for(let d=2;d<p;d++){let s=new Set(),x=1;for(let i=0;i<p-1;i++){s.add(x);x=mod(x*d,p);}if(s.size===p-1)return d;}throw Error('no primitive');}
for(const ell of primes){
 const U=[1,1,0,1], D=[1,0,0,primitive(ell)], G=[];
 for(let a=0;a<ell;a++)for(let b=0;b<ell;b++)for(let c=0;c<ell;c++)for(let d=0;d<ell;d++)
  if(mod(a*d-b*c,ell)!==0)G.push([a,b,c,d]);
 const Hsize=ell*(ell-1), fixed=[];
 for(let x=0;x<ell;x++)for(let y=0;y<ell;y++){
   let ux=action(U,x,y,ell),dx=action(D,x,y,ell);
   if(ux[0]===x&&ux[1]===y&&dx[0]===x&&dx[1]===y)fixed.push([x,y]);
 }
 const normal=[];
 for(const g of G){const gi=minv(g,ell);
  if(inH(mm(mm(g,U,ell),gi,ell),ell)&&inH(mm(mm(g,D,ell),gi,ell),ell))normal.push(g);
 }
 const targetOrbit=new Set(G.map(g=>action(g,1,0,ell).join(',')));
 const images=[...new Set(normal.map(g=>action(g,1,0,ell)[0]))].sort((a,b)=>a-b);
 if(G.length!==(ell*ell-1)*(ell*ell-ell) || fixed.length!==ell ||
    fixed.some(v=>v[1]!==0)||normal.length!==ell*(ell-1)*(ell-1)||
    targetOrbit.size!==ell*ell-1||images.length!==ell-1||images[0]!==1)throw Error('bad ell='+ell);
 console.log(JSON.stringify({ell:ell,gl2_size:G.length,stabilizer_size:Hsize,
  point_field_degree:targetOrbit.size,K_rational_ell_torsion_size:fixed.length,
  K_rational_nonzero_points:fixed.length-1,normalizer_size:normal.length,
  K_field_automorphisms:normal.length/Hsize,automorphism_images:images}));
}
console.log('ct69_negation_verified=true');
console.log('ct71_verified=true');
