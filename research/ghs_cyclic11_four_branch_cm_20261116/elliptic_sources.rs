const P:usize=5;
const Q:usize=3125;
fn modp(x:i32)->i32 {x.rem_euclid(5)}
fn digits(mut a:usize)->[i32;5] {
    let mut d=[0;5];for x in &mut d {*x=(a%P) as i32;a/=P;}d
}
fn pack(d:[i32;5])->usize {
    let (mut n,mut k)=(0,1);
    for x in d {n+=modp(x) as usize*k;k*=P;}n
}
fn add(a:usize,b:usize)->usize {
    let x=digits(a);let y=digits(b);pack(std::array::from_fn(|i|x[i]+y[i]))
}
fn mul(a:usize,b:usize)->usize {
    let x=digits(a);let y=digits(b);let mut d=[0_i32;9];
    for i in 0..5 {for j in 0..5 {d[i+j]+=x[i]*y[j];}}
    // z^5+4z+1=0, so z^5=z+4.
    for k in (5..=8).rev() {let c=modp(d[k]);d[k-5]+=4*c;d[k-4]+=c;}
    pack([d[0],d[1],d[2],d[3],d[4]])
}
fn pow(mut a:usize,mut n:usize)->usize {
    let mut b=1;while n>0 {if n&1==1 {b=mul(b,a);}a=mul(a,a);n>>=1;}b
}
fn main() {
    assert_eq!(mul(5,625),add(5,4));
    for a in 1..Q {assert_eq!(pow(a,Q-1),1,"non-field element {a}");}
    let mut squares=vec![0_u8;Q];
    for y in 0..Q {squares[mul(y,y)]+=1;}
    assert_eq!(squares[0],1);
    assert_eq!(squares.iter().map(|x|*x as usize).sum::<usize>(),Q);
    let cubes:Vec<usize>=(0..Q).map(|x|mul(mul(x,x),x)).collect();
    let mut found2=None;
    let mut found24=None;
    let mut scanned=0;
    let mut nonsingular=0;
    'search:for a in 1..=40 {for b in 0..Q {
        scanned+=1;
        let disc=add(mul(4,mul(mul(a,a),a)),mul(2,mul(b,b)));
        if disc==0 {continue;}
        nonsingular+=1;
        let mut points=1;
        for x in 0..Q {
            let rhs=add(add(cubes[x],mul(a,x)),b);
            points+=squares[rhs] as usize;
        }
        let t=(Q+1) as i64-points as i64;
        if t==2&&found2.is_none(){found2=Some((a,b,points,scanned));}
        if t==24&&found24.is_none(){found24=Some((a,b,points,scanned));}
        if found2.is_some()&&found24.is_some(){break 'search;}
    }}
    println!("schema=ghs.f3125-elliptic-sources/v1");
    println!("field=F5[z]/(z^5+4z+1); checked_nonzero={}",Q-1);
    println!("models_scanned={scanned}; nonsingular={nonsingular}");
    println!("trace_2_model_A_B_points_scan_index={found2:?}");
    println!("trace_24_model_A_B_points_scan_index={found24:?}");
}
