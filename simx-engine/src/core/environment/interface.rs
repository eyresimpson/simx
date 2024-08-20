use crate::core::environment::java::check_java_version;
use crate::entity::flow::Environment;

pub fn check(name: Vec<Environment>) {
    // println!("{:?}", name);
    check_java_version();
}