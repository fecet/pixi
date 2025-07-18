use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

// Thread-local storage for prefix overrides
thread_local! {
    static PREFIX_OVERRIDES: RefCell<HashMap<String, PathBuf>> = RefCell::new(HashMap::new());
}

/// RAII guard that manages prefix overrides
///
/// This guard allows temporarily overriding the prefix path for specific environments.
/// When the guard is dropped, the overrides are automatically removed.
pub struct PrefixOverrideGuard {
    env_names: Vec<String>,
}

impl PrefixOverrideGuard {
    /// Create a new override guard for a single environment
    ///
    /// # Arguments
    /// * `env_name` - The name of the environment to override
    /// * `custom_prefix` - The custom prefix path to use
    pub fn new(env_name: String, custom_prefix: PathBuf) -> Self {
        PREFIX_OVERRIDES.with(|overrides| {
            overrides
                .borrow_mut()
                .insert(env_name.clone(), custom_prefix);
        });

        Self {
            env_names: vec![env_name],
        }
    }

    /// Create a new override guard for multiple environments
    ///
    /// # Arguments
    /// * `overrides` - A map of environment names to custom prefix paths
    #[allow(dead_code)]
    pub fn new_multiple(overrides: HashMap<String, PathBuf>) -> Self {
        let env_names: Vec<String> = overrides.keys().cloned().collect();

        PREFIX_OVERRIDES.with(|prefix_overrides| {
            prefix_overrides.borrow_mut().extend(overrides);
        });

        Self { env_names }
    }
}

impl Drop for PrefixOverrideGuard {
    fn drop(&mut self) {
        PREFIX_OVERRIDES.with(|overrides| {
            let mut map = overrides.borrow_mut();
            for env_name in &self.env_names {
                map.remove(env_name);
            }
        });
    }
}

/// Check if there's a prefix override for the given environment
///
/// # Arguments
/// * `env_name` - The name of the environment to check
///
/// # Returns
/// * `Some(PathBuf)` if there's an override for this environment
/// * `None` if no override is set
pub fn get_prefix_override(env_name: &str) -> Option<PathBuf> {
    PREFIX_OVERRIDES.with(|overrides| overrides.borrow().get(env_name).cloned())
}
