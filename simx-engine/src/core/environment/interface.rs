use crate::core::environment::java::check_java_version;
use engine_common::entity::flow::Environment;

pub fn check(requirements: Vec<Environment>) -> Result<bool, String> {
    for requirement in requirements {
        if requirement.name == "java" {
            check_java_version();
        }
    }
    check_java_version();
    Ok(true)
}