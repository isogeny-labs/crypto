function ct337() {
  "use strict";
  const p=2147483647n;
  const m=x=>((x%p)+p)%p;
  const power=(a,e)=>{let x=m(a),r=1n;for(let n=e;n>0n;n>>=1n){if(n&1n)r=m(r*x);x=m(x*x);}return r;};
  const inv=a=>{if(m(a)===0n)throw Error("noninvertible");return power(a,p-2n);};
  const sqrt=a=>{a=m(a);if(!a)return 0n;if(power(a,(p-1n)/2n)!==1n)return null;const s=power(a,(p+1n)/4n);if(m(s*s)!==a)throw Error("sqrt");return s;};
  const assert=(ok,label)=>{if(!ok)throw Error("CT337 failure: "+label);};
  const polyMul=(a,b)=>{const r=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)r[i+j]=m(r[i+j]+a[i]*b[j]);return r;};
  const sub=(a,b)=>Array.from({length:Math.max(a.length,b.length)},(_,i)=>m((a[i]||0n)-(b[i]||0n)));
  const rem2=(A,r1,r0)=>{const v=A.slice();for(let i=v.length-1;i>1;i--){const k=m(v[i]);v[i-1]=m(v[i-1]-k*r1);v[i-2]=m(v[i-2]-k*r0);}return [m(v[0]||0n),m(v[1]||0n)];};
  const isZero=r=>r.every(x=>x===0n);
  const h=m(7n*inv(2n)),a=inv(4n),b=m(9n*a),c=m(25n*a);
  const F=Array.from({length:7},(_,i)=>0n);
  let fpoly=[1n];
  for(let k=1n;k<=6n;k++)fpoly=polyMul(fpoly,[-k,1n]);
  const plus=[m(-a*b*c),m(a*b+a*c+b*c),m(-a-b-c),1n];
  const minus=[1n,m(-a-b-c),m(a*b+a*c+b*c),m(-a*b*c)];
  const f=x=>{let r=1n;for(let j=1n;j<=6n;j++)r=m(r*(x-j));return r;};
  const plusR=u=>m((u-a)*(u-b)*(u-c));
  const minusR=X=>m((1n-a*X)*(1n-b*X)*(1n-c*X));
  const ecAdd=(P,Q,coeff)=>{
    if(P===null)return Q;
    if(Q===null)return P;
    const [x,y]=P,[z,w]=Q, [d3,d2,d1,d0]=coeff;
    if(x===z&&m(y+w)===0n)return null;
    let s;
    if(x===z){assert(y!==0n,"double y=0");s=m((3n*d3*x*x+2n*d2*x+d1)*inv(2n*y));}
    else s=m((w-y)*inv(z-x));
    const X=m((s*s-d2)*inv(d3)-x-z),Y=m(s*(x-X)-y);
    assert(m(Y*Y-d3*X**3n-d2*X*X-d1*X-d0)===0n,"EC chord");
    return [X,Y];
  };
  const eq=(P,Q)=>P===null?Q===null:Q!==null&&P[0]===Q[0]&&P[1]===Q[1];
  const neg=P=>P===null?null:[P[0],m(-P[1])];
  const image=(x,y)=>{
    assert(m(y*y)===f(x),"source curve");
    const t=m(x-h),u=m(t*t),Ep=[u,y],Em=u===0n?null:[inv(u),m(t*y*inv(u*u))];
    assert(m(Ep[1]*Ep[1])===plusR(Ep[0]),"Eplus equation");
    if(Em!==null)assert(m(Em[1]*Em[1])===minusR(Em[0]),"Eminus cubic equation");
    return [Ep,Em];
  };
  const nums=v=>v.map(Number);
  let rational=[];
  for(let x=1n;x<130n && rational.length<36;x++){
    const y=sqrt(f(x));if(y!==null)rational.push([x,y]);
  }
  assert(rational.length===36,"sample not full");
  let fiberChecks=0;
  for(const [x,y] of rational){
    const [pp,pm]=image(x,y),[sp,sm]=image(m(7n-x),y),[rp,rm]=image(m(7n-x),m(-y));
    assert(eq(pp,sp)&&eq(rm,pm),"involution fiber images");
    assert(eq(ecAdd(pp,sp,[...plus].reverse()),ecAdd(pp,pp,[...plus].reverse())),"plus diagonal");
    assert(eq(ecAdd(pm,sm,[...minus].reverse()),null),"minus cross zero");
    assert(eq(ecAdd(pp,rp,[...plus].reverse()),null),"plus cross zero");
    assert(eq(ecAdd(pm,rm,[...minus].reverse()),ecAdd(pm,pm,[...minus].reverse())),"minus diagonal");
    fiberChecks++;
  }
  const mapMumford=(s,z,A,v0,mutate=0)=>{
    const B=m(2n*h-s),C=m(h*h-h*s+z), D=m(A*h+v0),S=m(B*B-2n*C);
    if(B===0n||C===0n)throw Error("generic chart singular B*C");
    const Rplus=[m(C*C),m(-S),1n];
    const Vplus=[m(D-A*C*inv(B)),m(-A*inv(B))];
    const Rminus=[inv(C*C),m(-S*inv(C*C)),1n];
    const Vminus=[m(D*inv(B*C)),m(A-D*inv(B)-D*S*inv(B*C))];
    if(mutate===1)Vplus[0]=m(Vplus[0]+1n);
    if(mutate===2)Vminus[1]=m(Vminus[1]+1n);
    assert(isZero(rem2(sub(polyMul([v0,A],[v0,A]),fpoly),m(-s),z)),"source Mumford");
    assert(isZero(rem2(sub(polyMul(Vplus,Vplus),plus),Rplus[1],Rplus[0])),"plus Mumford image");
    assert(isZero(rem2(sub(polyMul(Vminus,Vminus),minus),Rminus[1],Rminus[0])),"minus Mumford image");
    return {Rplus:nums(Rplus),Vplus:nums(Vplus),Rminus:nums(Rminus),Vminus:nums(Vminus)};
  };
  let splitPairs=0, splitOne=null, exceptionsB=0, exceptionsC=0;
  for(let i=0;i<rational.length;i++)for(let j=i+1;j<rational.length;j++){
    const [x,y]=rational[i],[z,w]=rational[j];
    const s=m(x+z),q=m(x*z),A=m((y-w)*inv(x-z)),v0=m(y-A*x);
    const B=m(2n*h-s),C=m(h*h-h*s+q);
    if(B===0n){exceptionsB++;continue;}
    if(C===0n){exceptionsC++;continue;}
    const img=mapMumford(s,q,A,v0);
    if(splitOne===null)splitOne={support:[Number(x),Number(z)],initial:{U:[Number(q),Number(m(-s)),1],V:nums([v0,A])},image:img};
    splitPairs++;
  }
  const fF2=d=>{
    const mul=(x,y)=>[m(x[0]*y[0]+d*x[1]*y[1]),m(x[0]*y[1]+x[1]*y[0])];
    let z=[1n,0n];for(let k=1n;k<=6n;k++)z=mul(z,[m(-k),1n]);
    return {z,mul};
  };
  let nonsplit=[];
  for(let d=2n;d<140n&&nonsplit.length<20;d++){
    if(sqrt(d)!==null)continue;
    const {z,mul}=fF2(d),[aa,bb]=z;
    const norm=m(aa*aa-d*bb*bb),r=sqrt(norm);
    if(r===null)continue;
    let C=sqrt(m((aa+r)*inv(2n)));
    if(C===null)C=sqrt(m((aa-r)*inv(2n)));
    if(C===null||C===0n)continue;
    const E=m(bb*inv(2n*C));
    assert(mul([C,E],[C,E]).every((x,i)=>x===z[i]),"nonsplit extension square root");
    const images=mapMumford(0n,m(-d),E,C);
    nonsplit.push({d:Number(d),ordinate:[Number(C),Number(E)],images});
  }
  assert(nonsplit.length===20,"nonsplit count");
  // The three prescribed rational 2-torsion candidates: each W_i+W_{7-i} minus both source infinities.
  // The two source infinity images on Eminus are (0,+1),(0,-1), summing to O.
  const infinitiesMinus=[[0n,1n],[0n,m(-1n)]];
  assert(eq(ecAdd(...infinitiesMinus,[...minus].reverse()),null),"infinity cancellation");
  let kernels=[];
  for(const k of [1n,2n,3n]){
    const [ep,em]=image(k,0n),[ep2,em2]=image(7n-k,0n);
    assert(eq(ecAdd(ep,ep2,[...plus].reverse()),null),"2-torsion plus kernel");
    assert(eq(ecAdd(em,em2,[...minus].reverse()),null),"2-torsion minus kernel");
    kernels.push([Number(k),Number(7n-k)]);
  }
  let mutants=[];
  const shouldReject=(name,action)=>{
    let rejected=false;try{action();}catch(_e){rejected=true;}
    assert(rejected,"ineffective mutation "+name);mutants.push(name);
  };
  const [x1,y1]=rational[6],[x2,y2]=rational[7];
  const ss=m(x1+x2),zz=m(x1*x2),AA=m((y1-y2)*inv(x1-x2)),v0=m(y1-AA*x1);
  shouldReject("plus ordinate intercept",()=>mapMumford(ss,zz,AA,v0,1));
  shouldReject("minus ordinate slope",()=>mapMumford(ss,zz,AA,v0,2));
  shouldReject("wrong source point y",()=>image(x1,m(y1+1n)));
  shouldReject("cross-sign involution confusion",()=>{
    const [pp,pm]=image(x1,y1),[rp,rm]=image(m(7n-x1),m(-y1));
    assert(eq(ecAdd(pm,rm,[...minus].reverse()),null),"bad cross relation");
  });
  const result={protocol:"CT337_PROTOCOL.md",source_model:"C: y^2=product(x-1..6); p=2^31-1",
    field:Number(p),cubic_plus:nums(plus),cubic_minus:nums(minus),
    rational_points:rational.map(nums),fiber_checks:fiberChecks,split_generic_pairs:splitPairs,
    exceptional_B_zero_pairs:exceptionsB,exceptional_C_zero_pairs:exceptionsC,
    split_example:splitOne,nonsplit_supports:nonsplit,kernel_pairs:kernels,
    geometic_t_zero_branched_p:power(f(h),(p-1n)/2n).toString(),
    mutants_rejected:mutants,
    interpretation:"finite effective divisor support and elliptic-fiber checks only; no native Cantor, no complete ECDLP benchmark, unsigned"};
  assert(splitPairs===627,"split count regression");
  assert(exceptionsB===3,"B=0 three paired branch controls");
  return result;
}

if (typeof require !== 'undefined' && typeof module !== 'undefined' && require.main === module) {
  process.stdout.write(JSON.stringify(ct337(), null, 2) + '\n');
}
