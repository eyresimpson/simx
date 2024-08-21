use crate::core::environment::java::check_java_version;
use engine_common::entity::flow::Environment;

#[allow(unused_variables)]
pub fn check(name: Vec<Environment>) {
    // println!("{:?}", name);
    check_java_version();
}