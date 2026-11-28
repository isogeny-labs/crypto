'use strict';
// Independently build four centralizer algebras instead of scanning GL2.
// The joint commutant is the intersection of split diagonal and nonsplit torus.
function assert(x){if(!x) throw new Error('CT68 independent check failed')}
function mul(A,B,p){return [
 (A[0]*B[0]+A[1]*B[2])%p,(A[0]*B[1]+A[1]*B[3])%p,
 (A[2]*B[0]+A[3]*B[2])%p,(A[2]*B[1]+A[3]*B[3])%p]}
function det(A,p){return ((A[0]*A[3]-A[1]*A[2])%p+p)%p}
function commute(A,F,p){return JSON.stringify(mul(A,F,p))===JSON.stringify(mul(F,A,p))}
function orbit(group,v,p){return new Set(group.map(a=>JSON.stringify([(a[0]*v[0]+a[1]*v[1])%p,(a[2]*v[0]+a[3]*v[1])%p]))).size}
function groupCase(p){
 let sq=new Set(Array.from({length:p},(_,i)=>i*i%p)),n=Array.from({length:p-2},(_,i)=>i+2).find(d=>!sq.has(d));
 assert(n!==undefined);
 let Fs=[1,0,0,2],Fi=[0,n,1,0],Fj=[1,1,0,1],Fz=[1,0,0,1];
 let scalar=[],split=[],jordan=[],irreducible=[],joint=[];
 for(let a=0;a<p;a++)for(let b=0;b<p;b++){
  if(a){jordan.push([a,b,0,a]);if(b===0)joint.push([a,0,0,a])}
  let x=[a,(n*b)%p,b,a];if(det(x,p))irreducible.push(x);
 }
 // Separate full group enumeration for the scalar case.
 for(let a=0;a<p;a++)for(let b=0;b<p;b++)for(let c=0;c<p;c++)for(let d=0;d<p;d++){
   let x=[a,b,c,d];if(det(x,p))scalar.push(x);
 }
 // Construct the diagonal split torus explicitly.
 for(let a=1;a<p;a++)for(let d=1;d<p;d++)split.push([a,0,0,d]);
 assert(scalar.length===(p*p-1)*(p*p-p));
 assert(split.every(a=>commute(a,Fs,p))&&jordan.every(a=>commute(a,Fj,p))&&irreducible.every(a=>commute(a,Fi,p)));
 assert(joint.every(a=>commute(a,Fs,p)&&commute(a,Fi,p)));
 // Require every constructed family to be the entire centralizer, by checking
 // each full-group matrix against the corresponding algebraic parametrization.
 for(let [F,C] of [[Fs,split],[Fi,irreducible],[Fj,jordan],[Fz,scalar]]){
   const keys=new Set(C.map(x=>x.join(',')));
   assert(scalar.filter(x=>commute(x,F,p)).length===C.length);
   assert(C.every(x=>keys.has(x.join(','))));
 }
 assert(scalar.filter(x=>commute(x,Fs,p)&&commute(x,Fi,p)).length===joint.length);
 const out={ell:p,
  scalar:{centralizer:scalar.length,orbit_eigen:orbit(scalar,[1,0],p)},
  split:{centralizer:split.length,orbit_eigen:orbit(split,[1,0],p),orbit_generic:orbit(split,[1,1],p)},
  jordan:{centralizer:jordan.length,orbit_eigen:orbit(jordan,[1,0],p),orbit_generic:orbit(jordan,[0,1],p)},
  irreducible:{centralizer:irreducible.length,orbit_eigen:orbit(irreducible,[1,0],p)},
  joint_split_irreducible:{centralizer:joint.length,orbit_line:orbit(joint,[1,0],p),fixed_oriented_point:joint.filter(a=>a[0]===1).length}
 };
 if(p===3){
   const Fct=[0,1,1,1], jointCT=scalar.filter(x=>commute(x,Fs,p)&&commute(x,Fct,p));
   const targetCT=scalar.filter(x=>commute(x,Fct,p));
   out.joint_11a1_ell3_matrix_model={target_centralizer:targetCT.length,
      joint_centralizer:jointCT.length,joint_line_orbit:orbit(jointCT,[1,0],p)};
   assert(targetCT.length===8&&jointCT.length===2&&out.joint_11a1_ell3_matrix_model.joint_line_orbit===2);
 }
 assert(out.irreducible.centralizer===p*p-1&&out.joint_split_irreducible.centralizer===p-1);
 return out;
}
function ordered(v){if(Array.isArray(v))return v.map(ordered);if(v!==null&&typeof v==='object'){
 const r={};for(const k of Object.keys(v).sort())r[k]=ordered(v[k]);return r}
 return v;}
const result={classification:'fixed_frobenius_centralizer',field:'F_ell',
 cases:[3,5,7,11].map(groupCase),scope:'abstract_GL2_relabelings_only_no_global_input_lift'};
process.stdout.write(JSON.stringify(ordered(result))+'\n');
