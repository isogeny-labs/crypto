'use strict';
// CT93 independently replays source arithmetic and the two binary Frobenius orbits.
// The output certifies torsion-module transport ambiguity, NOT any curve scheme map.
const lp=7, p=37;
const mod=(x,n)=>(x%n+n)%n;
function inverse(a,n){ for(let i=1;i<n;i++) if(mod(a*i,n)===1)return i;throw Error('nonunit'); }
function sourceAdd(P,Q){
  if(P===null)return Q;if(Q===null)return P;
  const [x,y]=P,[u,v]=Q;
  if(x===u && mod(y+v+1,p)===0)return null;
  let slope,intercept;
  if(x===u&&y===v){
    const di=inverse(mod(2*y+1,p),p);
    slope=mod((3*x*x-2*x-10)*di,p);
    intercept=mod((-x*x*x-10*x-40-y)*di,p);
  }else{
    const di=inverse(mod(u-x,p),p);
    slope=mod((v-y)*di,p);
    intercept=mod((y*u-v*x)*di,p);
  }
  const X=mod(slope*slope+1-x-u,p),Y=mod(-slope*X-intercept-1,p);
  if(mod(Y*Y+Y-X*X*X+X*X+10*X+20,p)!==0)throw Error('point off curve');
  return [X,Y];
}
const source=[null];let Q=null;
for(let k=1;k<=7;k++){Q=sourceAdd(Q,[9,4]);if(k<7)source.push(Q);}
if(Q!==null || new Set(source.map(p=>JSON.stringify(p))).size!==7)throw Error('bad source order');
function curveOrder(q){let n=1;for(let x=0;x<q;x++)for(let y=0;y<q;y++)
  if(mod(y*y+y-x*x*x+x*x+10*x+20,q)===0)n++;return n;}
if(curveOrder(37)!==35 || curveOrder(2)!==5)throw Error('curve order');
function matMult(A,C){let R=[[0,0],[0,0]];for(let i=0;i<2;i++)for(let j=0;j<2;j++)
  for(let k=0;k<2;k++)R[i][j]=mod(R[i][j]+A[i][k]*C[k][j],lp);return R;}
function matPow(A,n){let R=[[1,0],[0,1]];for(let i=0;i<n;i++)R=matMult(R,A);return R;}
function act(A,v){return [mod(A[0][0]*v[0]+A[0][1]*v[1],lp),mod(A[1][0]*v[0]+A[1][1]*v[1],lp)];}
function scale(n,v){return v.map(x=>mod(n*x,lp));}
function key(v){return v.join(',');}
const A=[[0,5],[1,5]];
if(JSON.stringify(matPow(A,4))!==JSON.stringify([[3,0],[0,3]]) ||
   JSON.stringify(matPow(A,12))!==JSON.stringify([[6,0],[0,6]]) ||
   JSON.stringify(matPow(A,24))!==JSON.stringify([[1,0],[0,1]]))throw Error('Frobenius identity');
// Choose each new seed by lexicographic traversal of the nonzero target vectors.
let classes=[],covered=new Set();
for(let a=0;a<lp;a++)for(let b=0;b<lp;b++){
  if(a===0&&b===0)continue;
  let seed=[a,b];if(covered.has(key(seed)))continue;
  let orbit=[];
  for(let i=0;i<24;i++){
    let w=act(matPow(A,i),seed);
    if(covered.has(key(w)))throw Error('overlapping Frobenius orbit');
    covered.add(key(w));orbit.push(w);
  }
  if(key(act(matPow(A,24),seed))!==key(seed))throw Error('not orbit');
  classes.push(orbit);
}
if(classes.length!==2||covered.size!==48)throw Error('not full E[7]');
const imageCount=new Map(),lines=new Map();let labeled=0,checks=0;
for(let src=1;src<7;src++){
  for(let w=0;w<2;w++)for(const T of classes[w]){
    labeled++;
    const U=scale(inverse(src,7),T),ukey=key(U);
    imageCount.set(ukey,(imageCount.get(ukey)||0)+1);
    const line=[1,2,3,4,5,6].map(a=>key(scale(a,U))).sort().join('|');
    lines.set(line,true);
    for(let k=0;k<7;k++){
      const got=scale(k,U);
      if(key(got)!==key(act([[k,0],[0,k]],U)))throw Error('scalar map');
      if(key(scale(k+1,U))!==key(scale(k,U).map((x,i)=>mod(x+U[i],7))))throw Error('additivity');
      const srcNext=sourceAdd(source[k],source[1]);
      if(JSON.stringify(srcNext)!==JSON.stringify(source[(k+1)%7]))throw Error('source addition');
      checks++;
    }
  }
}
if(labeled!==288||checks!==2016||imageCount.size!==48||
  ![...imageCount.values()].every(x=>x===6)||lines.size!==8)throw Error('transport cardinality');
let lineVectorSet=new Set();for(const line of lines.keys())for(const v of line.split('|'))lineVectorSet.add(v);
if(lineVectorSet.size!==48||![...imageCount.keys()].every(k=>lineVectorSet.has(k)))throw Error('line coverage');
const defaultT=classes[0][0],shift=matPow(A,4);
let wrongSrc=0,wrongBin=0;
for(let k=1;k<7;k++){
 if(key(scale(k,defaultT))!==key(scale(mod(k*inverse(2,7),7),defaultT)))wrongSrc++;
 if(key(scale(k,defaultT))!==key(act(shift,scale(k,defaultT))))wrongBin++;
}
if(wrongSrc!==6||wrongBin!==6)throw Error('orientation controls');
const data={binary_curve_order:5,binary_orbit_lengths:classes.map(x=>x.length),
 binary_orbit_seeds:classes.map(x=>x[0]),binary_prime_ideals:2,
 distinct_group_maps:imageCount.size,experiment:'ct93-torsion-place-ambiguity',
 fibers_over_each_map:6,frobenius_matrix:A,independently_checked_public_images:checks,
 labeled_triplets:labeled,public_test_cases_per_map:7,
 residue_embeddings_per_binary_prime:24,source_curve_order:35,source_degree_one_places:6,
 source_generator:[9,4],source_public_points:source.map(v=>v===null?['O']:v),
 target_7_torsion_lines:lines.size,wrong_source_orientation_nonzero_failures:wrongSrc,
 wrong_target_frobenius_nonzero_failures:wrongBin};
// JS insertion order deliberately differs from Python internal construction: recursive normalization.
function canon(x){if(Array.isArray(x))return x.map(canon);if(x!==null&&typeof x==='object')
 return Object.fromEntries(Object.keys(x).sort().map(k=>[k,canon(x[k])]));return x;}
console.log(JSON.stringify(canon(data)));
