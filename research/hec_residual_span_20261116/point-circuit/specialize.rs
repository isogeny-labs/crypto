use std::collections::BTreeMap;
mod tensor { include!("../generic-kummer/terms.rs"); }
fn main() {
    println!("form\tcoefficient\tc_power\tr_power\tK_exponents");
    for (j, terms) in tensor::FORMS.iter().enumerate() {
        let mut out=BTreeMap::<([u8;4],u8,u8),i64>::new();
        for &(v,c,k,l) in *terms {
            assert_eq!(k.iter().sum::<u8>(),2);
            assert_eq!(l.iter().sum::<u8>(),2);
            if l[0]!=0 {continue}
            *out.entry((k,c,l[2]+2*l[3])).or_default()+=v;
        }
        out.retain(|_,v|*v!=0);
        eprintln!("form={} surviving_terms={}",j,out.len());
        for ((k,c,r),v) in out {println!("{}\t{}\t{}\t{}\t{:?}",j,v,c,r,k)}
    }
}
