use std::path::Path;

use crate::types::{CommandDef, DoctorCheck};

pub trait Platform: Send + Sync {
    fn name(&self) -> &str;

    fn detect(&self, dir: &Path) -> f32;

    fn commands(&self) -> Vec<CommandDef>;

    fn doctor_checks(&self, dir: &Path) -> Vec<DoctorCheck>;
}
