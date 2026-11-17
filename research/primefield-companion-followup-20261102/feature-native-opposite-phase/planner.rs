#[derive(Debug,Clone,Copy)]struct WidthPlan{width:usize,table_words:usize,quotient_products:usize,convolutions:usize,call_bound:u128}
fn width_plan(n:usize,length:usize,max_words:usize)->Result<WidthPlan,Error>{
 if n==0||length==0{return Err(Error::Domain)}let limit=length.min(max_words/n);if limit==0{return Err(Error::Limit)}
 let degree_bound=n.min(length);let coefficients=degree_bound.checked_add(1).ok_or(Error::Limit)?;
 let mut best=None;
 for q in 1..=limit{let projected=length.div_ceil(q);let certificate=coefficients.div_ceil(q);
  let quotient=q.checked_add(projected-1).and_then(|v|v.checked_add(certificate-1)).ok_or(Error::Limit)?;
  let cost=quotient as u128+projected as u128;
  if best.as_ref().map_or(true,|p:&WidthPlan|cost<p.call_bound){best=Some(WidthPlan{width:q,table_words:q.checked_mul(n).ok_or(Error::Limit)?,quotient_products:quotient,convolutions:projected,call_bound:cost});}
 }Ok(best.unwrap())
}
