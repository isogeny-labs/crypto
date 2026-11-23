// CT336 executed finite-field symbolic and public-point source (Node 22+).
// Deliberately bielliptic CT334 positive control; no full DLP speedup/PCC.
const study=function study(){
 const P=2147483647n;
 const md=x=>((x%P)+P)%P, inv2=(P+1n)/2n;
 const pmul=(a,b)=>{let z=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)z[i+j]=md(z[i+j]+a[i]*b[j]);return z;};
 const product=arr=>arr.reduce((a,b)=>pmul(a,b),[1n]);
 const eq=(a,b)=>a.length===b.length&&a.every((v,i)=>v===b[i]);
 const evalpoly=(a,x)=>{let r=0n;for(let i=a.length-1;i>=0;i--)r=md(r*x+a[i]);return r;};
 const pow=(x,k)=>{let r=1n;x=md(x);while(k){if(k&1n)r=md(r*x);x=md(x*x);k>>=1n;}return r;};
 const numeric=a=>a.map(Number),s=md(7n*inv2),t2=[0n,0n,1n],rootJ=[1n,3n,5n];
 const a=rootJ.map(j=>md(j*j*inv2*inv2));
 const inputShift=ss=>product([1n,2n,3n,4n,5n,6n].map(r=>[md(ss-r),1n]));
 const shifted=inputShift(s);
 const factorU=a.map(q=>[md(-q),1n]),plus=product(factorU),minus=pmul([0n,1n],plus);
 const substitute=u=>u.reduce((z,c,i)=>{let mon=[1n];for(let j=0;j<i;j++)mon=pmul(mon,t2);while(z.length<mon.length)z.push(0n);for(let k=0;k<mon.length;k++)z[k]=md(z[k]+c*mon[k]);return z;},[]);
 const expected=substitute(plus),outMinus=substitute(minus);
 const mut={wrong_half_shift:!eq(inputShift(md(s+1n)),expected),wrong_branch_square:!eq(substitute(product([[md(-(a[0]+1n)),1n],factorU[1],factorU[2]])),shifted),missing_t_factor:!eq(shifted,outMinus)};
 const sqfree=a.every((v,i)=>v!==0n&&a.slice(i+1).every(w=>w!==v));
 if(!eq(shifted,expected)||!eq(outMinus,pmul(t2,shifted))||!sqfree||Object.values(mut).some(x=>!x))throw Error("quotient identity/control failure");
 const fX=product([1n,2n,3n,4n,5n,6n].map(r=>[md(-r),1n]));
 if(!eq(fX,[720n,2147481883n,1624n,2147482912n,175n,2147483626n,1n]))throw Error("CT334 frozen polynomial mismatch");
 const points=[];
 for(let x=7n;x<150n&&points.length<5;x++){
   let fy=evalpoly(fX,x),y=pow(fy,(P+1n)/4n);
   if(fy!==0n&&md(y*y)===fy){
     y=y<P-y?y:P-y;const t=md(x-s),u=md(t*t),w=md(t*y);
     if(md(y*y)!==evalpoly(plus,u)||md(w*w)!==evalpoly(minus,u))throw Error("sample map fails");
     const minusY=md(P-y), minusW=md(t*minusY);
     if(md(minusW*minusW)!==evalpoly(minus,u))throw Error("negative ordinate map fails");
     points.push({x:Number(x),y:Number(y),t:Number(t),plus:{u:Number(u),v:Number(y)},minus:{u:Number(u),w:Number(w)},negated_minus_w:Number(minusW)});
   }
 }
 if(points.length!==5)throw Error("too few rational point controls");
 return{
  p:Number(P),input_ct334_json_blob:"01be355bf299c01419ac3a43a53adc4f8f7d61ea",
  p_mod4:Number(P%4n),inv2:Number(inv2),shift_7_over_2:Number(s),u_roots:numeric(a),
  source_f:numeric(fX),source_in_t:numeric(shifted),plus_cubic:numeric(plus),minus_quartic:numeric(minus),
  identities:{source_even_in_t:shifted.every((v,i)=>i%2===0||v===0n),source_equals_plus_t2:eq(shifted,expected),
              t2_source_equals_minus_t2:eq(outMinus,pmul(t2,shifted)),distinct_four_quartic_roots:sqfree,
              differentials_independent:2n!==0n},
  mutant_rejections:mut,positive_affine_points:points,
  morphisms:"(x,y)->(u=(x-7/2)^2,v=y) and (x,y)->(u=(x-7/2)^2,w=(x-7/2)y)",
  differentials:"pi_plus^*(du/v)=2(x-7/2)dx/y; pi_minus^*(du/w)=2dx/y",
  scope:"explicit degree-two curve maps into genus-one quotients and independent cotangent basis; deliberately decomposable positive control, no ECDLP speedup",
  not_checked:"No native Cantor/Jacobian map image, full 695/701 norm reuse, optimized rho/BSGS/IC cost, or signed PCC"
 };
};
process.stdout.write(JSON.stringify(study(),null,2)+"\n");
