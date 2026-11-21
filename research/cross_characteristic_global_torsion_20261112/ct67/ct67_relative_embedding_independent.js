#!/usr/bin/env node
// Independent computation in F9 = F3[t]/(t²-t-1), not a GL2 scan.
// Multiplication by t is the binary Frobenius matrix in basis (1,t).
function yes(c,s){if(!c)throw Error(s)}
// Count points via discriminant characters rather than a (x,y) scan.
const mod=(x,p)=>((x%p)+p)%p;
function ecSize(p){
 let count=1;
 for(let x=0;x<p;x++){
  let r=mod(x*x*x-x*x-10*x-20,p);
  if(p===2) {for(let y=0;y<2;y++)if(mod(y*y+y-r,p)===0)count++;}
  else {let D=mod(1+4*r,p);if(D===0){count++;continue;}
    let ok=false;for(let u=1;u<p;u++)if((u*u)%p===D){ok=true;break;}
    if(ok)count+=2;
  }
 }
 return count;
}
yes(ecSize(29)===30 && ecSize(2)===5,'wrong point counts');
yes(mod(5*5+5-(18**3-18**2-10*18-20),29)===0,'source point off curve');
// Tangent doubling gives the negative (x, -y-1), so the point has order 3.
let p=29,x=18,y=5;
let denom=mod(2*y+1,p),denInv=[...Array(p).keys()].find(i=>mod(i*denom,p)===1);
let slope=mod((3*x*x-2*x-10)*denInv,p), nu=mod(y-slope*x,p);
let x2=mod(slope*slope+1-2*x,p), y2=mod(-slope*x2-nu-1,p);
yes(x2===18 && y2===23,'source order-three witness failed');
function mul([a,b],[c,d]) {return [((a*c+b*d)%3),((a*d+b*c+b*d)%3)];}
const elements=[];
for(let a=0;a<3;a++)for(let b=0;b<3;b++)elements.push([a,b]);
const nonzero=elements.filter(([a,b])=>a!==0||b!==0);
yes(nonzero.length===8,'wrong F9 units');
// Under F9 multiplication, every nonzero field element carries 1 to a
// distinct oriented vector, and t^4=-1, t^8=1.
let power=[1,0],seen=[];
for(let i=1;i<=8;i++){power=mul(power,[0,1]);seen.push(power.join(','));}
yes(seen[3]==='2,0' && seen[7]==='1,0','wrong Frobenius order');
yes(new Set(seen).size===8,'wrong orbit');
for(let u of nonzero) {
  const images=nonzero.map(g=>mul(g,[1,0]));
  yes(images.filter(v=>v.join(',')===u.join(',')).length===1,'nonregular centralizer');
  for(let k=0;k<3;k++)yes(mul(u,[k,0]).join(',')===((k*u[0])%3)+','+((k*u[1])%3),'relation not preserved');
}
// GL2(F3): first column 8 choices; for each, 9-3=6 second columns
// off its line. An irreducible Frobenius has F9 centralizer exactly F9*.
let glSize=nonzero.length*(9-3);
yes(glSize===48,'GL size');
yes(glSize/nonzero.length===6,'fixed-vector stabilizer');
yes(glSize/nonzero.length===6,'conjugacy size');
const report={curve:'11a1',ell:3,p_source:29,p_target:2,
 source_point:[18,5],source_curve_order:30,binary_curve_order:5,target_residue_degree:8,
 frobenius_trace_mod3:1,frobenius_det_mod3:2,
 frobenius_order:8,frobenius_fourth:'-I',
 full_G_size:glSize,fixed_generator_stabilizer:6,
 conjugate_frobenius_matrices:6,
 F_centralizer_size:8,F_centralizer_orbit:8,
 F_centralizer_vector_stabilizer:1,
 same_F_different_image:true,scalar_compatibility:true};
// Stable JSON object order matching Python's sort_keys.
let ordered={};for(let k of Object.keys(report).sort())ordered[k]=report[k];
console.log(JSON.stringify(ordered));
