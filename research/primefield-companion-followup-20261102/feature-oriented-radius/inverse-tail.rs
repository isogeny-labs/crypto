/// Caller supplies the verified inverse of the full nonzero prefix product.
/// prefix[0]=1 and prefix[i+1]=prefix[i]*original_values[i].
fn finish_batch_inverses(values:&mut[u64],prefix:&[u64],mut inverse:u64,r:u64)->Result<(),Error>{
 if prefix.len()!=values.len().saturating_add(1)||prefix.first()!=Some(&1){return Err(Error::Domain)}
 if values.is_empty(){return Ok(())}
 for i in(1..values.len()).rev(){let value=values[i];values[i]=mul(inverse,prefix[i],r);inverse=mul(inverse,value,r);}
 values[0]=inverse;Ok(())
}
