'use strict';
// Independent projective/nonzero orbit algorithm, no GL2 matrix enumeration.
function need(x, why) { if (!x) throw Error(why); }
function action(m, v, p) {return [(m[0]*v[0]+m[1]*v[1])%p,(m[2]*v[0]+m[3]*v[1])%p];}
function order(a,p) {let x=1;for(let i=1;i<p;i++){x=x*a%p;if(x===1)return i;}throw Error('order');}
function primitive(p) {for(let a=2;a<p;a++)if(order(a,p)===p-1)return a;throw Error('primitive root');}
function canon(v,p) {
  let x=v[0],y=v[1]; need(x!==0||y!==0,'zero projective vector');
  if(x===0)return 'infty';
  for(let i=1;i<p;i++)if(x*i%p===1)return String(y*i%p);
  throw Error('no inverse');
}
function orbit(p, projective) {
  const s=primitive(p), gens=[[0,p-1,1,0],[1,1,0,1],[s,0,0,1]];
  const encode=v=>projective?canon(v,p):v.join(':');
  const seen=new Set([encode([1,0])]), work=[[1,0]];
  for(let i=0;i<work.length;i++)for(const m of gens){let v=action(m,work[i],p),key=encode(v);if(!seen.has(key)){seen.add(key);work.push(v);}}
  return seen.size;
}
function fixed(m,v,p){let w=action(m,v,p);return (v[0]*w[1]-v[1]*w[0])%p===0;}
function run(){
  for(const p of [3,5,7,11]){
    const lineOrbit=orbit(p,true),vectorOrbit=orbit(p,false);
    need(lineOrbit===p+1,'projective orbit mismatch');
    need(vectorOrbit===p*p-1,'oriented vector orbit mismatch');
    const frob=[1,0,0,2], swap=[0,p-1,1,0];
    const reps=Array.from({length:p},(_,t)=>[1,t]).concat([[0,1]]);
    need(reps.some(v=>fixed(frob,v,p)),'no locally fixed line');
    need(!reps.some(v=>fixed(frob,v,p)&&fixed(swap,v,p)),'unexpected common line');
    const expectedBorel=p*(p-1)**2;
    const expectedPointStab=p*(p-1);
    const group=(p*p-1)*(p*p-p);
    need(group/expectedBorel===lineOrbit && group/expectedPointStab===vectorOrbit,
       'independent orbit-stabilizer disagreement');
    console.log(`ell=${p} GL2=${group} oriented_stabilizer=${expectedPointStab} line_stabilizer=${expectedBorel} oriented_orbit=${vectorOrbit} line_orbit=${lineOrbit} local_not_global=PASS`);
  }
  console.log('PASS: independent BFS-orbit group audit; no proof of Mazur claimed');
}
run();
