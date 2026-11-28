'use strict';
/* CT79 independently authored digit-Hensel and target arithmetic; bounded toy. [pcc:unverified] */
const H=[-101306819n,89655n,267096n,349907n,162543n,-14817n,-4813n,108n,27n];
const A=[15208005n,352381n,332454n,-39746n,-19603n,324n,108n];
const D=4992581n,p=29n, n=p**6n;
const ck=(test,message)=>{if(!test)throw new Error(message)};
const mod=(a,m)=>((a%m)+m)%m;
const poly=(C,v,m)=>{let z=0n;for(let i=C.length-1;i>=0;--i)z=mod(z*v+C[i],m);return z};
function invert(a,m){let x=0n,y=1n,xx=m,yy=mod(a,m);while(yy){let q=xx/yy;[xx,yy]=[yy,xx-q*yy];[x,y]=[y,x-q*y]}ck(xx===1n,'nonunit');return mod(x,m)}
function rootlift(a){let v=BigInt(a),base=p;ck(poly(H,v,p)===0n,'initial H root');let tests=0;
 for(let k=2;k<=6;k++){let next=base*p,hits=[];for(let d=0n;d<29n;d++){let w=v+base*d;tests++;if(poly(H,w,next)===0n)hits.push(w)}ck(hits.length===1,'ambiguous digit');v=hits[0];base=next}
 return {v,tests};}
function xcoord(y,m){return mod(-poly(A,y,m)*invert(D,m),m)}
function codepoint(y,m){return [xcoord(y,m),mod(y,m)]}
const reps=[[0n,1n],[-1n,-1n]];
// Recognition reads only encoded x/y, not a scalar/label. Every candidate is tested.
function recognize(z){if(z===null)return [null,0];let hits=[],tests=0;for(let r of reps){tests++;let y=mod(r[0]+5n*r[1],p),q=codepoint(y,p);if(q[0]===BigInt(z[0])&&q[1]===BigInt(z[1]))hits.push(r)}ck(hits.length===1,'bad encoded input');return [hits[0],tests]}
function gfmult(a,b){let poly=0;for(let i=0;i<8;i++)if((b>>i)&1)poly^=a<<i;for(let d=14;d>=8;d--)if((poly>>d)&1)poly^=0x17b<<(d-8);return poly;}
function gfpow(a,b){let z=1;while(b){if(b&1)z=gfmult(z,a);a=gfmult(a,a);b>>=1;}return z}
function gfpoly(C,z){let a=0;for(let i=C.length-1;i>=0;i--)a=gfmult(a,z)^Number(C[i]&1n);return a}
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function binaryAdd(P,Q){if(!P)return Q;if(!Q)return P;let [x,y]=P,[z,t]=Q;
 if(x===z&&(y^t)===1)return null;
 let lambda,nu;
 if(!eq(P,Q)){let inv=gfpow(z^x,254);lambda=gfmult(t^y,inv);nu=gfmult(gfmult(y,z)^gfmult(t,x),inv)}
 else{lambda=gfmult(x,x);nu=gfmult(gfmult(x,x),x)^y}
 let w=gfmult(lambda,lambda)^1^x^z,v=gfmult(lambda,w)^nu^1;
 ck((gfmult(v,v)^v)===(gfmult(gfmult(w,w),w)^gfmult(w,w)),'bad target curve');return [w,v];}
const f29=v=>Number(mod(BigInt(v),p));
function sourceAdd(A,B){if(!A)return B;if(!B)return A;let [x,y]=A,[z,t]=B;
 if(x===z&&f29(y+t+1)===0)return null;
 let l,nu;
 const inv=a=>Number(invert(BigInt(f29(a)),p));
 if(!eq(A,B)){l=f29((t-y)*inv(z-x));nu=f29((y*z-t*x)*inv(z-x))}
 else{let d=inv(2*y+1);l=f29((3*x*x-2*x-10)*d);nu=f29((-x*x*x-10*x-40-y)*d)}
 let xx=f29(l*l+1-x-z),yy=f29(-l*xx-nu-1);
 ck(f29(yy*yy+yy-xx**3+xx*xx+10*xx+20)===0,'bad source curve');return [xx,yy];}
function solve(P,Q,add){let cur=null;for(let i=0;i<3;i++){if(eq(cur,Q))return [i,i];cur=add(cur,P)}throw new Error('not in subgroup')}
function run(){
 let r5=rootlift(5),r23=rootlift(23);ck(r5.v===425642517n,'unexpected 29-adic root');
 ck(r23.v===mod(-r5.v-1n,n),'opposite root');
 ck(eq(codepoint(r5.v,n).map(Number),[25104680,425642517]),'CT74 disagreement');
 ck(eq(codepoint(r23.v,n).map(Number),[25104680,169180803]),'inverse disagreement');
 const beta=2, x=gfpoly(A,beta);ck(x===19,'global X binary mismatch');
 ck(gfpoly(H,beta)===0,'wrong residue polynomial');
 let all=[null,[18,5],[18,23]],srcBase=[18,5],targetBase=[x,beta];
 ck(sourceAdd(srcBase,sourceAdd(srcBase,srcBase))===null,'bad source order');
 ck(binaryAdd(targetBase,binaryAdd(targetBase,targetBase))===null,'bad binary order');
 let rows=[],checks=0;
 for(let q of all){let [rep,nt]=recognize(q);checks+=nt;let local=null,target=null;
  if(rep){let y=mod(rep[0]+rep[1]*r5.v,n),z=codepoint(y,n);local=z.map(Number);
   ck(eq(local.map(v=>v%29),q)&&local[0]===25104680,'source prime drift');
   let ty=(Number(rep[0]&1n))^gfmult(Number(rep[1]&1n),beta);target=[x,ty];}
  let [s,ss]=solve(srcBase,q,sourceAdd),[t,ts]=solve(targetBase,target,binaryAdd);
  ck(s===t,'wrong transported discrete log');
  rows.push({input:q,local_y:local?local[1]:null,target,source_dlog:s,binary_dlog:t,source_scan_additions:ss,binary_scan_additions:ts,candidate_checks:nt});
 }
 ck(eq(codepoint(r23.v,p).map(Number),[18,23]),'wrong-place setup');
 ck(!eq(rows[2].target,targetBase)&&solve(targetBase,targetBase,binaryAdd)[0]===1,'falsifier failed');
 for(let bad of [[5,5],[18,6],[0,0]]){let rejected=false;try{recognize(bad)}catch(e){rejected=true}ck(rejected,'accepted bad input')}
 let result={experiment:'ct79-global-recognition',field_degree:8,source_prime:29,target_field:256,precision:6,source_root5:Number(r5.v),alternate_root23:Number(r23.v),shared_source_place:5,target_beta_byte:beta,candidate_checks_total:checks,rows,wrong_source_place_log:1,log_lookup_used:false,candidate_enumeration_used:true,global_field_precomputed:true,scalable:false};
 function sorted(v){if(Array.isArray(v))return v.map(sorted);if(v&&typeof v==='object'){let w={};for(let k of Object.keys(v).sort())w[k]=sorted(v[k]);return w}return v}
 console.log(JSON.stringify(sorted(result)));
}
run();
