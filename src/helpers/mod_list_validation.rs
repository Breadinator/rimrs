use std::{
    sync::{
        Arc,
        Mutex,
    },
    collections::{
        HashMap,
        HashSet,
    },
};
use crate::{
    ModMetaData,
    traits::LockIgnorePoisoned,
};

pub struct ModListValidator<'mmd> {
    mod_meta_data: &'mmd Arc<Mutex<HashMap<String, ModMetaData>>>,
}

impl<'mmd> From<&'mmd Arc<Mutex<HashMap<String, ModMetaData>>>> for ModListValidator<'mmd> {
    fn from(mod_meta_data: &'mmd Arc<Mutex<HashMap<String, ModMetaData>>>) -> Self {
        Self { mod_meta_data }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub enum ModListValidationResult {
    Ok,
    Warn {
        warnings: Vec<String>,
    },
    Err {
        warnings: Vec<String>,
        errors: Vec<String>,
    },
}

impl<'mmd> ModListValidator<'mmd> {
    #[must_use]
    pub fn new(mod_meta_data: &'mmd Arc<Mutex<HashMap<String, ModMetaData>>>) -> Self {
        Self::from(mod_meta_data)
    }

    #[must_use]
    pub fn validate(&self, mod_list: &[String]) -> ModListValidationResult {
        let mod_list: Vec<_> = mod_list.iter().map(|pid| pid.to_lowercase()).collect();

        let mut loaded_so_far: HashSet<String> = HashSet::new();
        let mut should_load_after: HashMap<String, &HashSet<String>> = HashMap::new(); // key should be loaded after anything in hashset
        let mut incompatible: HashMap<String, &HashSet<String>> = HashMap::new();

        let mut errors: Vec<String> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();

        let mmd = self.mod_meta_data.lock_ignore_poisoned();
        for package_id in &mod_list {
            if let Some(meta_data) = mmd.get(package_id) {
                if let Some(incompat) = &meta_data.incompatibleWith {
                    incompatible.insert(package_id.to_lowercase(), incompat);
                }
                if let Some(hard_reqs) = &meta_data.modDependencies {
                    for hard_req in hard_reqs.iter().filter_map(|r| r.packageId.as_ref()) {
                        if !loaded_so_far.contains(&hard_req.to_lowercase()) {
                            errors.push(format!("{package_id} requires {hard_req}"));
                        }
                    }
                }
                if let Some(load_before) = &meta_data.loadBefore {
                    for lbf in loaded_so_far.intersection(load_before) {
                        warnings.push(format!("{package_id} should be loaded before {lbf}"));
                    }
                }
                for (a, load_after) in &should_load_after {
                    if load_after.contains(package_id) {
                        warnings.push(format!("{a} should be loaded after {package_id}"));
                    }
                }

                if let Some(load_after) = &meta_data.loadAfter {
                    should_load_after.insert(package_id.to_lowercase(), load_after);
                }
                loaded_so_far.insert(package_id.clone());
            } else {
                warnings.push(format!("Couldn't find metadata for {package_id}"));
            }
        }

        for (a, incompatiblities) in incompatible {
            for b in incompatiblities.intersection(&loaded_so_far) {
                errors.push(format!("{a} is incompatible with {b}"));
            }
        }

        if !errors.is_empty() {
            ModListValidationResult::Err { warnings, errors }
        } else if !warnings.is_empty() {
            ModListValidationResult::Warn { warnings }
        } else {
            ModListValidationResult::Ok
        }
    }
}

impl ModListValidationResult {
    #[must_use]
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }

    #[must_use]
    pub fn is_warn(&self) -> bool {
        matches!(self, Self::Warn { warnings: _ })
    }

    #[must_use]
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err { warnings: _, errors: _ })
    }

    #[must_use]
    pub fn warnings(&self) -> Option<&Vec<String>> {
        match self {
            Self::Ok => None,
            Self::Warn { warnings } | Self::Err { warnings, errors: _ } => Some(warnings),
        }
    }

    #[must_use]
    pub fn errors(&self) -> Option<&Vec<String>> {
        if let Self::Err { warnings: _, errors } = self {
            Some(errors)
        } else {
            None
        }
    }
}

impl From<ModListValidationResult> for Result<(), Vec<String>> {
    fn from(res: ModListValidationResult) -> Self {
        match res {
            ModListValidationResult::Ok => Ok(()),
            ModListValidationResult::Warn { warnings } => Err(warnings),
            ModListValidationResult::Err { warnings, errors } => {
                let mut e = warnings;
                e.extend(errors);
                Err(e)
            },
        }
    }
}

