use crate::{traits::LockIgnorePoisoned, ModMetaData};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, MutexGuard},
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SortError {
    #[error("cyclic dependency")]
    CyclicError,
    #[error("missing mod metadata")]
    MissingData,
}

/// Performs a [topological sort](https://en.wikipedia.org/wiki/Topological_sorting) on the given list of mods.
/// Uses the depth-first search algorithm.
///
/// # Errors
/// * [`SortError::CyclicError`] if there is a cyclic dependency
/// * [`SortError::MissingData`] if there is missing data in `mod_metadata` (e.g. missing `package_id`)
#[allow(clippy::implicit_hasher, clippy::missing_panics_doc)]
pub fn sort(
    mods: &[String],
    mod_metadata: &Arc<Mutex<HashMap<String, ModMetaData>>>,
) -> Result<Vec<String>, SortError> {
    let mut sorted = Vec::from(mods);
    sorted.sort();

    let mmd = mod_metadata.lock_ignore_poisoned();
    let deps = build_deps(&sorted, &mmd)?;

    let mut output: Vec<String> = Vec::with_capacity(mods.len());
    let mut unmarked: Vec<&String> = sorted.iter().collect();
    let mut temp_marks: HashSet<&String> = HashSet::new();

    // I would do a `while let` but `visit` would break the iterator
    while !unmarked.is_empty() {
        visit(
            unmarked.first().unwrap(),
            &deps,
            &mut output,
            &mut unmarked,
            &mut temp_marks,
        )?;
    }

    Ok(output)
}

fn visit<'a>(
    node: &'a String,
    deps: &HashMap<&'a String, Vec<&'a String>>,
    output: &mut Vec<String>,
    unmarked: &mut Vec<&String>,
    temp_marks: &mut HashSet<&'a String>,
) -> Result<(), SortError> {
    if !unmarked.iter().any(|&x| x == node) {
        // Already done
        return Ok(());
    }

    let newly_marked = temp_marks.insert(node);
    if !newly_marked {
        // Must contain at least one cycle
        return Err(SortError::CyclicError);
    }

    if let Some(d) = deps.get(node) {
        for d in d {
            visit(d, deps, output, unmarked, temp_marks)?;
        }
    }

    temp_marks.remove(node);
    unmarked.remove(unmarked.iter().position(|&x| x == node).unwrap());
    output.push(node.clone());

    Ok(())
}

fn build_deps<'a, 'b>(
    mods: &'a [String],
    mod_metadata: &'b MutexGuard<HashMap<String, ModMetaData>>,
) -> Result<HashMap<&'a String, Vec<&'b String>>, SortError> {
    #[allow(clippy::needless_pass_by_value)]
    fn ext<'c>(md: &mut Vec<&'c String>, m: Vec<&'c String>) {
        md.extend(m.iter());
    }
    fn ins<'c, 'd>(
        deps: &mut HashMap<&'c String, HashSet<&'d String>>,
        m: &'c String,
        mod_deps: Vec<&'d String>,
    ) {
        if !deps.contains_key(m) {
            deps.insert(m, HashSet::new());
        }
        deps.get_mut(m).unwrap().extend(mod_deps);
    }

    let mut deps: HashMap<&'a String, HashSet<&'b String>> = HashMap::new();

    for m in mods {
        let mut mod_deps = Vec::new();

        if let Some(mmd) = mod_metadata.get(m) {
            if let Some(d) = mmd.modDependencies.as_ref() {
                ext(
                    &mut mod_deps,
                    d.iter().filter_map(|d| d.packageId.as_ref()).collect(),
                );
            }
            if let Some(d) = mmd.loadAfter.as_ref() {
                ext(&mut mod_deps, d.iter().collect());
            }
            if let Some(d) = mmd.forceLoadAfter.as_ref() {
                ext(&mut mod_deps, d.iter().collect());
            }
        } else {
            return Err(SortError::MissingData);
        }

        ins(&mut deps, m, mod_deps);
    }

    // i am O(n^2)'s strongest soldier
    // this should be done in the other loop but lifetimes are being annoying
    let combinations = mods.iter().combinations(2);
    for combination in combinations {
        let (m, n) = (combination[0], combination[1]);
        let mut mod_deps = Vec::new();

        if let Some(mmd) = mod_metadata.get(n) {
            if let Some(d) = mmd.loadBefore.as_ref() {
                if d.contains(m) {
                    mod_deps.push(mmd.packageId.as_ref().ok_or(SortError::MissingData)?);
                }
            }
            if let Some(d) = mmd.forceLoadBefore.as_ref() {
                if d.contains(m) {
                    mod_deps.push(mmd.packageId.as_ref().ok_or(SortError::MissingData)?);
                }
            }
        } else {
            return Err(SortError::MissingData);
        }

        ins(&mut deps, m, mod_deps);
    }

    Ok(deps
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().collect()))
        .collect())
}
