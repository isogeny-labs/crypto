// Native coefficient check for the exact F3 dihedral quotient candidate.
type Poly = Vec<u8>;

fn norm(mut a: Poly) -> Poly {
    for c in &mut a { *c %= 3; }
    while a.len() > 1 && a.last() == Some(&0) { a.pop(); }
    a
}
fn add(a: &[u8], b: &[u8]) -> Poly {
    let mut c = vec![0; a.len().max(b.len())];
    for (i, x) in a.iter().enumerate() { c[i] += x; }
    for (i, x) in b.iter().enumerate() { c[i] += x; }
    norm(c)
}
fn neg(a: &[u8]) -> Poly {
    norm(a.iter().map(|x| (3 - x % 3) % 3).collect())
}
fn sub(a: &[u8], b: &[u8]) -> Poly { add(a, &neg(b)) }
fn mul(a: &[u8], b: &[u8]) -> Poly {
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i+j] = (c[i+j] + x*y) % 3;
        }
    }
    norm(c)
}
fn main() {
    let x = vec![0,1];
    let one = vec![1];
    let u_num = vec![2,0,1]; // x^2+2
    let u_den = x.clone();
    let lhs_num = vec![1,2,2,1,1]; // f+1/f numerator
    let lhs_den = vec![0,2,0,1];   // x(x^2+2)
    let rhs_num = add(&add(&mul(&u_num,&u_num),&mul(&u_num,&u_den)),&mul(&u_den,&u_den));
    let rhs_den = mul(&u_num,&u_den);
    assert_eq!(mul(&lhs_num,&rhs_den),mul(&rhs_num,&lhs_den));
    assert_eq!(lhs_num,rhs_num);
    assert_eq!(lhs_den,rhs_den);
    let mut d0 = vec![2];
    let mut d1 = x.clone();
    for _ in 2..=11 {
        let next = sub(&mul(&x,&d1),&d0);
        d0 = d1;
        d1 = next;
    }
    let expected = vec![0,1,0,1,0,1,0,2,0,1,0,1];
    assert_eq!(d1,expected);
    assert_eq!(mul(&x,&d1),mul(&x,&expected));
    assert_eq!(add(&add(&mul(&x,&x),&x),&one),vec![1,1,1]);
    println!("schema=ghs.cyclic11-f3-quotient/v1");
    println!("curve=y^11=x(x-1)/(x+1) over F3");
    println!("involution=(x,y)->(2/x,1/y)");
    println!("u=x+2/x; v=y+1/y");
    println!("rational_identity=f+1/f=u+1+1/u: true");
    println!("D11_coefficients_low_to_high={:?}",d1);
    println!("quotient_equation=u*D11(v)=u^2+u+1");
}
