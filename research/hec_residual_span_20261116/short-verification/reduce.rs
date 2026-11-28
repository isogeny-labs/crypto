fn norm(a:&[i128])->i128{a.iter().map(|x|x*x).sum()}
fn nearest(a:i128,b:i128)->i128{if a>=0{(a+b/2)/b}else{-((-a+b/2)/b)}}
pub fn short(h:u64,r:&[i128])->(Vec<i128>,Vec<i128>,usize){
 let n=r.len();let mut basis=vec![];for j in 0..n{basis.push((0..n).map(|k|r[(k+n-j)%n]).collect::<Vec<_>>());}basis.push(vec![1;n]);
 let mut a=vec![0i128;n];a[0]=h as i128;let mut witness=vec![0;n+1];let mut moves=0;
 loop{let mut best=None;let old=norm(&a);for(j,b)in basis.iter().enumerate(){let nb=norm(b);if nb==0{continue;}let dot=a.iter().zip(b).map(|(x,y)|x*y).sum();let q=nearest(dot,nb);let decrease=2*q*dot-q*q*nb;if decrease>0&&best.as_ref().map_or(true,|&(_,_,v)|decrease>v){best=Some((j,q,decrease));}}
 let Some((j,q,_))=best else{break};assert!(moves<10000);for k in 0..n{a[k]-=q*basis[j][k];}witness[j]+=q;moves+=1;assert!(norm(&a)<old);}
 for k in 0..n{assert_eq!((if k==0{h as i128}else{0})-a[k],basis.iter().zip(&witness).map(|(b,w)|b[k]*w).sum::<i128>());}
 (a,witness,moves)
}
