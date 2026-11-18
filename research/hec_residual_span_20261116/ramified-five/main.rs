#[allow(dead_code)]#[path="../canonical-root/root.rs"]mod root;
#[path="../canonical-root/fast.rs"]mod fast;
use hec_native_ic::{counted,fp::Fp,jac::Curve,poly::Poly};
type Mat=Vec<Vec<i128>>;
fn add(a:i128,b:i128)->i128{a.checked_add(b).unwrap()}
fn mul(a:i128,b:i128)->i128{a.checked_mul(b).unwrap()}
fn identity(n:usize)->Mat{(0..n).map(|i|(0..n).map(|j|(i==j)as i128).collect()).collect()}
fn mm(a:&Mat,b:&Mat)->Mat{let n=a.len();(0..n).map(|i|(0..n).map(|j|(0..n).fold(0,|s,k|add(s,mul(a[i][k],b[k][j])))).collect()).collect()}
fn matrix(a:&[i128])->Mat{let d=a.len()-1;let mut m=vec![vec![0;d];d];for col in 0..d{let mut v=vec![0;2*d];for(j,&x)in a.iter().enumerate(){v[j+col]=add(v[j+col],x);}for k in(d..2*d).rev(){let x=v[k];v[k]=0;for j in 0..d{v[k-d+j]=add(v[k-d+j],x.checked_neg().unwrap());}}for i in 0..d{m[i][col]=v[i];}}m}
fn characteristic(m:&Mat)->Vec<i128>{let n=m.len();let mut b=identity(n);let mut c=vec![1];for k in 1..=n{b=mm(m,&b);let trace=(0..n).fold(0,|s,i|add(s,b[i][i]));assert_eq!(trace%k as i128,0);let a=(trace/(k as i128)).checked_neg().unwrap();c.push(a);for i in 0..n{b[i][i]=add(b[i][i],a);}}assert!(b.iter().flatten().all(|x|*x==0));c}
fn det(mut a:Mat)->i128{let n=a.len();let(mut denominator,mut sign)=(1,1);for k in 0..n-1{let Some(pivot)=(k..n).find(|&i|a[i][k]!=0)else{return 0};if pivot!=k{a.swap(k,pivot);sign=-sign;}let p=a[k][k];for i in k+1..n{for j in k+1..n{let v=mul(a[i][j],p).checked_sub(mul(a[i][k],a[k][j])).unwrap();assert_eq!(v%denominator,0);a[i][j]=v/denominator;}a[i][k]=0;}denominator=p;}mul(sign,a[n-1][n-1])}
fn krylov(m:&Mat)->Mat{let n=m.len();let mut k=vec![vec![0;n];n];let mut v=vec![0;n];v[0]=1;for j in 0..n{for i in 0..n{k[i][j]=v[i];}v=(0..n).map(|i|(0..n).fold(0,|s,t|add(s,mul(m[i][t],v[t])))).collect();}k}
fn red(x:i128,p:u64)->u64{x.rem_euclid(p as i128)as u64}
fn fm(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn fp(mut a:u64,mut n:u64,p:u64)->u64{let mut r=1;while n>0{if n&1==1{r=fm(r,a,p);}a=fm(a,a,p);n>>=1;}r}
fn rank(m:&Mat,p:u64)->usize{let n=m.len();let mut a:Vec<Vec<_>>=m.iter().map(|r|r.iter().map(|&x|red(x,p)).collect()).collect();let mut r=0;for j in 0..n{let Some(i)=(r..n).find(|&i|a[i][j]!=0)else{continue};a.swap(r,i);let iv=fp(a[r][j],p-2,p);for k in j..n{a[r][k]=fm(a[r][k],iv,p);}for i in r+1..n{let q=a[i][j];for k in j..n{a[i][k]=(a[i][k]+p-fm(q,a[r][k],p))%p;}}r+=1;if r==n{break}}r}
fn trim(mut a:Vec<u64>)->Vec<u64>{while a.last()==Some(&0){a.pop();}a}
fn sub(a:&[u64],b:&[u64],p:u64)->Vec<u64>{trim((0..a.len().max(b.len())).map(|j|(a.get(j).copied().unwrap_or(0)+p-b.get(j).copied().unwrap_or(0))%p).collect())}
fn rem(a:&[u64],b:&[u64],p:u64)->Vec<u64>{assert!(!b.is_empty());let mut a=trim(a.to_vec());let iv=fp(*b.last().unwrap(),p-2,p);while a.len()>=b.len(){let k=a.len()-b.len();let q=fm(*a.last().unwrap(),iv,p);for j in 0..b.len(){a[k+j]=(a[k+j]+p-fm(q,b[j],p))%p;}a=trim(a);}a}
fn gcd(mut a:Vec<u64>,mut b:Vec<u64>,p:u64)->Vec<u64>{while !b.is_empty(){let v=rem(&a,&b,p);a=b;b=v;}let iv=fp(*a.last().unwrap(),p-2,p);a.iter().map(|&x|fm(x,iv,p)).collect()}
fn pm(a:&[u64],b:&[u64],m:&[u64],p:u64)->Vec<u64>{if a.is_empty()||b.is_empty(){return vec![]}let mut c=vec![0;a.len()+b.len()-1];for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){c[i+j]=(c[i+j]+fm(x,y,p))%p;}}rem(&c,m,p)}
fn pp(a:&[u64],mut n:u64,m:&[u64],p:u64)->Vec<u64>{let mut r=vec![1];let mut a=a.to_vec();while n>0{if n&1==1{r=pm(&r,&a,m,p);}a=pm(&a,&a,m,p);n>>=1;}r}
fn ugcd(mut a:usize,mut b:usize)->usize{while b>0{let r=a%b;a=b;b=r;}a}
fn factors(f:&[u64],p:u64)->Vec<usize>{let d=f.len()-1;let mut counts=vec![0;d+1];let x=vec![0,1];let mut v=x.clone();for k in 1..=d{v=pp(&v,p,f,p);let n=gcd(f.to_vec(),sub(&v,&x,p),p).len()-1;let old=(1..k).filter(|&j|k%j==0).map(|j|j*counts[j]).sum::<usize>();assert!(n>=old&&(n-old)%k==0);counts[k]=(n-old)/k;}(1..=d).flat_map(|k|std::iter::repeat_n(k,counts[k])).collect()}
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
