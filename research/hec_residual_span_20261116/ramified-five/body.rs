fn image(a:&Mat,v:&[u64],p:u64)->Vec<u64>{(0..4).map(|i|(0..4).map(|j|fm(red(a[i][j],p),v[j],p)).sum::<u64>()%p).collect()}
fn contains(w:&[Vec<u64>],mut v:Vec<u64>)->bool{for row in w{let j=row.iter().position(|&x|x!=0).unwrap();let t=v[j];for k in 0..4{v[k]=(v[k]+5-fm(t,row[k],5))%5;}}v.iter().all(|&x|x==0)}
fn planes()->Vec<Vec<Vec<u64>>>{let mut out=vec![];for a in 0..4{for b in a+1..4{let slots:Vec<_>=(0..2).flat_map(|r|{let pivot=if r==0{a}else{b};(pivot+1..4).filter(move |&k|k!=b).map(move|k|(r,k))}).collect();
 for mut code in 0..5usize.pow(slots.len()as u32){let mut w=vec![vec![0;4];2];w[0][a]=1;w[1][b]=1;for &(r,k)in &slots{w[r][k]=(code%5)as u64;code/=5;}out.push(w);}}}out}
fn transpose(a:&Mat)->Mat{(0..4).map(|i|(0..4).map(|j|a[j][i]).collect()).collect()}
fn equiv(a:&Mat,b:&Mat)->bool{a.iter().flatten().zip(b.iter().flatten()).all(|(&x,&y)|red(x-y,5)==0)}
fn pairing(a:&[u64],b:&[u64],e:&Mat)->u64{(0..4).flat_map(|i|(0..4).map(move|j|(i,j))).map(|(i,j)|fm(fm(a[i],red(e[i][j],5),5),b[j],5)).sum::<u64>()%5}
fn main(){counted::disable();let curve=Curve::new(Fp::new(601),Poly::from_coeffs(&[7,0,0,0,0,1]));let(_,_,a)=fast::prepare(&curve);let a:Vec<i128>=a.iter().map(|&x|x as i128).collect();let pi=matrix(&a);let mut n=pi.clone();for i in 0..4{n[i][i]+=1;}
 assert_eq!(characteristic(&pi),vec![1,79,2731,47479,361201]);assert_eq!(rank(&krylov(&pi),5),2);assert_eq!(rank(&n,5),1);assert!(mm(&n,&n).iter().flatten().all(|&x|red(x,5)==0));
 let sigma=matrix(&[0,1,0,0,0]);let ps=planes();assert_eq!(ps.len(),806);let stable:Vec<_>=ps.iter().filter(|w|w.iter().all(|v|contains(w,image(&pi,v,5)))).collect();
 println!("p\tc\tl\tform_id\tstable_planes\tstable_lagrangians\tcontain_image");let mut forms=0;
 for mut code in 0..15625{let id=code;let mut e=vec![vec![0;4];4];for i in 0..4{for j in i+1..4{let x=code%5;code/=5;e[i][j]=x;e[j][i]=-x;}}if rank(&e,5)!=4{continue}
 if !equiv(&mm(&mm(&transpose(&sigma),&e),&sigma),&e){continue}
 assert!(equiv(&mm(&mm(&transpose(&pi),&e),&pi),&e));
 let ls:Vec<_>=stable.iter().filter(|w|pairing(&w[0],&w[1],&e)==0).collect();assert_eq!(ls.len(),6);
 for w in &ls{for j in 0..4{let v=(0..4).map(|i|red(n[i][j],5)).collect();assert!(contains(w,v));}}
 let all=ps.iter().filter(|w|pairing(&w[0],&w[1],&e)==0).count();assert_eq!(all,156);forms+=1;
 println!("601\t7\t5\t{id}\t{}\t{}\ttrue",stable.len(),ls.len());}
 assert!(forms>0);eprintln!("original_model=true rank_pi_plus_one=1 nilpotent_square_zero=true grassmann_planes=806 invariant_nondegenerate_forms={forms} lagrangians_per_form=156 stable_lagrangians_per_form=6 independent_verification=false");}
