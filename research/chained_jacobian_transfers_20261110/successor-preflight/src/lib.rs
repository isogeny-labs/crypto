//! Fixed-field correctness preflight for the next nondual seven-isogenies.
//! All arithmetic uses existing isogeny_algos engines; no benchmark runner.
#[cfg(test)]
mod full_class_group_replay;
#[cfg(test)]
mod geometric_isotypy_inputs;
#[cfg(test)]
mod kernel_replay;
#[cfg(test)]
mod legacy_jacobian;
#[cfg(test)]
mod odd_descent;
#[cfg(test)]
mod quartic_cover_replay;
#[cfg(test)]
mod quartic_t1_replay;
#[cfg(test)]
#[path = "seven-reference.rs"]
mod seven;
#[cfg(test)]
mod successor;
