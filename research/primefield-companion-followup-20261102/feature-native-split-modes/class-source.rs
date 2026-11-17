type C=polynomial::ResultantClass;
type CP=[C;2];
fn cm(a:C,b:C)->C{a.combine(b).expect("internal classes share the source field")}
fn pm(a:CP,b:CP)->CP{[cm(a[0],b[0]),cm(a[1],b[1])]}
fn pp(a:CP,e:u64)->CP{[a[0].power(e),a[1].power(e)]}
fn neg(mut a:CP)->CP{a[0]=a[0].negate();a}
fn nonzero(a:CP)->Result<CP,Error>{if a[0].is_zero()||a[1].is_zero(){Err(Error::Degenerate)}else{Ok(a)}}
