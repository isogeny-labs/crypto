//! Fixed-field correctness preflight for the next nondual seven-isogenies.
//! All arithmetic uses existing isogeny_algos engines; no benchmark runner.
#[cfg(test)]
mod kernel_replay;
#[cfg(test)]
mod odd_descent;
#[cfg(test)]
#[path = "seven-reference.rs"]
mod seven;
#[cfg(test)]
mod successor;
