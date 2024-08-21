use crate::core::environment::java::check_java_version;
use simx_common::entity::flow::Environment;

#[allow(unused_variables)]
pub fn check(name: Vec<Environment>) {
    // println!("{:?}", name);
    check_java_version();
}