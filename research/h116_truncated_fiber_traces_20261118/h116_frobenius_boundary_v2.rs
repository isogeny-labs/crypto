use std::fmt::Write;

const P: u64 = 31;
const A: u64 = 1;
const B: u64 = 19;
const SCALE: u64 = 25;
const N: usize = 18;
const PHI_N: [u64; 38] = [1,7,11,7,15,30,20,14,25,11,2,16,10,4,19,14,0,26,17,6,0,12,12,28,12,5,8,14,26,17,15,7,28,1,11,20,21,3];
const PHI_D: [u64; 37] = [1,7,22,22,27,19,11,13,26,12,25,18,22,6,6,7,18,18,6,29,29,9,3,17,28,17,6,15,24,29,27,4,9,18,28,11,19];
type Poly = Vec<u64>;

#[derive(Default)]
struct Count { add: u64, mul: u64, inv: u64 }
fn add(x:u64,y:u64,c:&mut Count)->u64{c.add+=1;(x+y)%P}
fn sub(x:u64,y:u64,c:&mut Count)->u64{c.add+=1;(x+P-y)%P}
fn mul(x:u64,y:u64,c:&mut Count)->u64{c.mul+=1;x*y%P}
fn pow_mod(mut x:u64,mut n:u64)->u64{let mut o=1;while n>0{if n&1==1{o=o*x%P;}x=x*x%P;n>>=1;}o}
fn inv(x:u64,c:&mut Count)->u64{assert_ne!(x,0);c.inv+=1;pow_mod(x,P-2)}
fn trim(mut x:Poly)->Poly{while x.len()>1&&x.last()==Some(&0){x.pop();}x}
fn pa(x:&[u64],y:&[u64],c:&mut Count)->Poly{trim((0..x.len().max(y.len())).map(|i|add(x.get(i).copied().unwrap_or(0),y.get(i).copied().unwrap_or(0),c)).collect())}
fn ps(x:&[u64],y:&[u64],c:&mut Count)->Poly{trim((0..x.len().max(y.len())).map(|i|sub(x.get(i).copied().unwrap_or(0),y.get(i).copied().unwrap_or(0),c)).collect())}
fn pm(x:&[u64],y:&[u64],c:&mut Count)->Poly{let mut o=vec![0;x.len()+y.len()-1];for(i,&a)in x.iter().enumerate(){for(j,&b)in y.iter().enumerate(){let t=mul(a,b,c);o[i+j]=add(o[i+j],t,c);}}trim(o)}
fn sc(x:&[u64],a:u64,c:&mut Count)->Poly{trim(x.iter().map(|&v|mul(v,a,c)).collect())}
fn pp(x:&[u64],n:usize,c:&mut Count)->Poly{let mut o=vec![1];for _ in 0..n{o=pm(&o,x,c);}o}
fn der(x:&[u64],c:&mut Count)->Poly{if x.len()<2{vec![0]}else{trim(x.iter().enumerate().skip(1).map(|(i,&v)|mul(i as u64%P,v,c)).collect())}}
fn sx(x:&[u64])->Poly{let mut o=vec![0];o.extend_from_slice(x);trim(o)}
fn curve()->Poly{vec![B,A,0,1]}

fn scalar_six(c:&mut Count)->(Poly,Poly){
    let f=curve();let a2=mul(A,A,c);let b2=mul(B,B,c);
    let p3=vec![sub(0,a2,c),mul(12,B,c),mul(6,A,c),0,3];
    let g=vec![sub(sub(0,mul(8,b2,c),c),mul(a2,A,c),c),sub(0,mul(4,mul(A,B,c),c),c),sub(0,mul(5,a2,c),c),mul(20,B,c),mul(5,A,c),0,1];
    let d3=pp(&p3,2,c);
    let fg=pm(&f,&g,c);let fg8=sc(&fg,8,c);
    let n3=ps(&sx(&d3),&fg8,c);
    let n2=pp(&n3,2,c);let n_cubed=pm(&n2,&n3,c);let n4=pp(&n2,2,c);
    let d2=pp(&d3,2,c);let d_cubed=pm(&d2,&d3,c);let d4=pp(&d2,2,c);
    let n2d2=pm(&n2,&d2,c);let term2=sc(&n2d2,mul(2,A,c),c);
    let nd3=pm(&n3,&d_cubed,c);let term3=sc(&nd3,mul(8,B,c),c);
    let term4=sc(&d4,a2,c);
    let numerator=pa(&ps(&ps(&n4,&term2,c),&term3,c),&term4,c);
    let nd2=pm(&n3,&d2,c);let and2=sc(&nd2,A,c);let bd3=sc(&d_cubed,B,c);
    let inside=pa(&pa(&n_cubed,&and2,c),&bd3,c);
    let denominator=sc(&pm(&d3,&inside,c),4,c);
    (numerator,denominator)
}

fn identity(n:&[u64],d:&[u64],at:u64,bt:u64,c:&mut Count)->bool{
    let h=ps(&pm(&der(n,c),d,c),&pm(n,&der(d,c),c),c);
    let lhs=pm(&curve(),&pp(&h,2,c),c);let d2=pp(d,2,c);let d3=pm(&d2,d,c);
    let n3=pp(n,3,c);let nd2=pm(n,&d2,c);let atnd2=sc(&nd2,at,c);let btd3=sc(&d3,bt,c);
    let inner=pa(&pa(&n3,&atnd2,c),&btd3,c);lhs==pm(d,&inner,c)
}
fn quo(n:&[u64],d:&[u64],c:&mut Count)->Vec<u64>{let di=inv(d[0],c);let mut o=vec![0;N];for k in 0..N{let mut v=n.get(k).copied().unwrap_or(0);for i in 1..=k.min(d.len()-1){v=sub(v,mul(d[i],o[k-i],c),c);}o[k]=mul(v,di,c);}o}
fn fd(x:&[u64],y:&[u64])->i64{x.iter().zip(y).position(|(a,b)|a!=b).map_or(-1,|i|i as i64)}
fn csv(x:&[u64])->String{let mut o=String::new();for(i,v)in x.iter().enumerate(){if i>0{o.push(',');}write!(&mut o,"{v}").unwrap();}o}

fn main(){
    let mut c=Count::default();let s2=mul(SCALE,SCALE,&mut c);let s4=mul(s2,s2,&mut c);let s6=mul(s4,s2,&mut c);let at=mul(s4,A,&mut c);let bt=mul(s6,B,&mut c);
    let(sn,sd)=scalar_six(&mut c);assert_eq!((sn.len()-1,sd.len()-1),(36,35));let snn=sc(&sn,s2,&mut c);
    let mut snr=snn.clone();snr.reverse();let mut sdr=sd.clone();sdr.reverse();let scalar=quo(&snr,&sdr,&mut c);let phi=quo(&PHI_N,&PHI_D,&mut c);let first=fd(&phi,&scalar);let delta=sub(phi[first as usize],scalar[first as usize],&mut c);
    let mut pn=PHI_N.to_vec();pn.reverse();let mut pd=PHI_D.to_vec();pd.reverse();let pi=identity(&pn,&pd,at,bt,&mut c);let si=identity(&snn,&sd,at,bt,&mut c);
    let mut unr=sn.clone();unr.reverse();let omitted=fd(&phi,&quo(&unr,&sdr,&mut c));let wat=mul(s2,A,&mut c);let wbt=mul(s4,B,&mut c);let wrong=!identity(&snn,&sd,wat,wbt,&mut c);
    let mut mr=snr.clone();mr[15]=add(mr[15],1,&mut c);let mf=fd(&quo(&mr,&sdr,&mut c),&scalar);mr.reverse();let mi=!identity(&mr,&sd,at,bt,&mut c);
    println!("schema\th116.frobenius-boundary-result/v1");println!("p\tscale\tprecision\tphi_prefix\tscalar_prefix\tfirst_difference\tdelta\tpredicted_first_difference\tpredicted_delta\tphi_identity\tscalar_identity\tomitted_scale_first_difference\twrong_target_rejected\tmutation_first_difference\tmutation_identity_rejected\tadditions\tmultiplications\tinversions");
    println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t15\t10\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",P,SCALE,N,csv(&phi),csv(&scalar),first,delta,pi,si,omitted,wrong,mf,mi,c.add,c.mul,c.inv);
}
