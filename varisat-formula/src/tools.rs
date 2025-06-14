use crate::lit::{Lit, Var};
use itertools::Itertools;

use rayon::prelude::*;

#[profiling::function]
pub fn no_two_true(vars: &[Var]) -> Vec<Vec<Lit>> {
    vars.iter()
        .map(|&var| var.negative())
        .tuple_combinations()
        .map(|(a, b)| vec![a, b])
        .collect_vec()
}

#[profiling::function]
pub fn all_true(vars: &[Var]) -> Vec<Lit> {
    vars.iter().map(|&var| var.positive()).collect()
}

#[profiling::function]
pub fn exactly_one(vars: &[Var]) -> Vec<Vec<Lit>> {
    no_two_true(vars)
        .into_iter()
        .chain(vec![all_true(vars)])
        .collect()
}

pub fn no_two_true_rayon(vars: &[Var]) -> Vec<Vec<Lit>> {
    let len = vars.len();

    // Each i < j pair is generated from vars[i], vars[j]
    (0..len)
        .into_par_iter() // Parallel over i
        .flat_map_iter(|i| {
            let a = vars[i].negative();
            (i + 1..len).map(move |j| vec![a, vars[j].negative()])
        })
        .collect()
}

pub fn all_true_rayon(vars: &[Var]) -> Vec<Lit> {
    vars.par_iter().map(|&var| var.positive()).collect()
}

pub fn exactly_one_rayon(vars: &[Var]) -> Vec<Vec<Lit>> {
    let mut result = no_two_true_rayon(vars);
    result.push(all_true_rayon(vars));
    result
}

pub fn exactly_one_iter(vars: &[Var]) -> impl Iterator<Item = Vec<Lit>> + '_ {
    vars.iter()
        .map(|&var| var.negative())
        .tuple_combinations()
        .map(|(a, b)| vec![a, b])
        .chain(std::iter::once(all_true(vars)))
}

pub fn all_true_simd_hint(vars: &[Var]) -> Vec<Lit> {
    let mut result = Vec::with_capacity(vars.len());
    let ptr = vars.as_ptr();
    let len = vars.len();
    for i in 0..len {
        unsafe {
            // May help with branch prediction / cache locality
            result.push((*ptr.add(i)).positive());
        }
    }
    result
}
