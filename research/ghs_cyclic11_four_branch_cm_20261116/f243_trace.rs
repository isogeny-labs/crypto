// Exhaustive F243 arithmetic screen for the exact cyclic-11 quotient lead.
const Q: usize = 243;

fn digits(mut a: usize) -> [i32; 5] {
    let mut d = [0; 5];
    for x in &mut d { *x = (a % 3) as i32; a /= 3; }
    d
}
fn pack(d: [i32; 5]) -> usize {
    let mut n = 0;
    let mut place = 1;
    for x in d { n += x.rem_euclid(3) as usize * place; place *= 3; }
    n
}
fn add(a: usize, b: usize) -> usize {
    let x = digits(a); let y = digits(b);
    pack(std::array::from_fn(|i| x[i]+y[i]))
}
fn sub(a: usize, b: usize) -> usize {
    let x = digits(a); let y = digits(b);
    pack(std::array::from_fn(|i| x[i]-y[i]))
}
fn mul(a: usize, b: usize) -> usize {
    let x = digits(a); let y = digits(b);
    let mut d = [0_i32; 9];
    for i in 0..5 { for j in 0..5 { d[i+j] += x[i]*y[j]; } }
    // z^5 = z+2 in F3[z]/(z^5+2z+1).
    for k in (5..=8).rev() {
        let c = d[k].rem_euclid(3);
        d[k-4] += c;
        d[k-5] += 2*c;
    }
    pack([d[0],d[1],d[2],d[3],d[4]])
}
fn pow(mut a: usize, mut n: usize) -> usize {
    let mut b = 1;
    while n != 0 {
        if n & 1 == 1 { b = mul(b,a); }
        a = mul(a,a); n >>= 1;
    }
    b
}
fn inv(a: usize) -> usize { assert!(a != 0); pow(a,241) }
fn fifth_trace(s: i64) -> i64 { s.pow(5)-15*s.pow(3)+45*s }

fn main() {
    assert_eq!(add(1,2),0);
    assert_eq!(mul(3,3*3*3*3),add(3,2)); // z^5 = z+2
    for a in 1..Q {
        assert_eq!(pow(a,242),1,"non-field element {a}");
        assert_eq!(mul(a,inv(a)),1,"inverse failed {a}");
    }
    let mut n = 0;
    let mut admissible = 0;
    for x in 0..Q {
        if x == 0 || x == 1 || x == 2 { continue; }
        admissible += 1;
        let f = mul(mul(x,sub(x,1)),inv(add(x,1)));
        assert!(f != 0);
        if pow(f,22) == 1 { n += 1; }
    }
    assert_eq!(admissible,240);
    let c_points = 4+11*n;
    let q_numerator = c_points+244;
    let t_numerator = 240_i64-11*n as i64;
    println!("schema=ghs.cyclic11-f243-trace/v1");
    println!("field=F3[z]/(z^5+2z+1); field_elements={Q}");
    println!("nonzero_field_elements_checked={}",Q-1);
    println!("admissible_x={admissible}; eleventh_power_f_count={n}");
    println!("carrier_C_points={c_points}");
    println!("quotient_Q_points_numerator={q_numerator}; divisible_by_two={}",q_numerator%2==0);
    println!("trace_numerator={t_numerator}; divisible_by_ten={}",t_numerator%10==0);
    if q_numerator%2==0 && t_numerator%10==0 {
        let q_points = q_numerator/2;
        let t = t_numerator/10;
        let descended: Vec<(i64,i64)> = (-3..=3).map(|s|(s,fifth_trace(s))).filter(|(_,v)|*v==t).collect();
        println!("quotient_Q_points={q_points}");
        println!("elliptic_trace_t={t}; ordinary={}; hasse_bound={}; descended_base_traces={descended:?}",t%3!=0,t*t<=4*243);
    }
}
