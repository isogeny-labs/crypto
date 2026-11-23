//! Constant-space paging of retained data. The bound counts borrowed elements,
//! not nested bytes or serialization work; encoding needs separate admission.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Error { ZeroLimit, Cursor }
pub struct Page<'a,T> { pub items:&'a [T], pub next:Option<usize>, pub total:usize }
pub fn page<T>(items:&[T],start:usize,limit:usize)->Result<Page<'_,T>,Error>{
    if limit==0{return Err(Error::ZeroLimit)}
    let remaining=items.len().checked_sub(start).ok_or(Error::Cursor)?;
    // Subtraction before addition proves end <= len even for limit=usize::MAX.
    let end=start+remaining.min(limit);
    Ok(Page{items:&items[start..end],next:if end<items.len(){Some(end)}else{None},total:items.len()})
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn preserves_nonclone_data_and_address(){struct Retained(u64);let data=[Retained(7),Retained(9)];let p=page(&data,1,1).unwrap();assert!(std::ptr::eq(&p.items[0],&data[1]));assert_eq!(p.items[0].0,9);}
 #[test] fn reconstructs_without_gaps_or_repeats(){let a=[0,1,2,3,4];let mut start=0;let mut seen=0;loop{let p=page(&a,start,2).unwrap();assert!(p.items.len()<=2);for &x in p.items{assert_eq!(x,seen);seen+=1}match p.next{Some(n)=>start=n,None=>break}}assert_eq!(seen,5);}
 #[test] fn zero_cap_is_explicit_refusal(){assert!(matches!(page(&[1],0,0),Err(Error::ZeroLimit)));}
 #[test] fn past_end_is_not_silent_empty(){assert!(matches!(page(&[1],2,1),Err(Error::Cursor)));}
 #[test] fn huge_limit_does_not_overflow(){let a=[0,1,2];let p=page(&a,1,usize::MAX).unwrap();assert_eq!(p.items,&[1,2]);assert_eq!(p.next,None);}
 #[test] fn end_and_empty_have_terminal_pages(){for a in [&[][..],&[1,2][..]]{let p=page(a,a.len(),1).unwrap();assert!(p.items.is_empty());assert_eq!(p.total,a.len());assert_eq!(p.next,None);}}
 #[test] fn nested_payload_is_borrowed_not_cloned(){let a=[vec![1u8;1024]];let p=page(&a,0,1).unwrap();assert_eq!(p.items[0].as_ptr(),a[0].as_ptr());assert_eq!(p.items[0].len(),1024);}
}
