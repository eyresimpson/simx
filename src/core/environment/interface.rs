use crate::core::environment::java::check_java_version;
use crate::entity::flow::Env;

pub fn check(name: Vec<Env>) {
    println!("{:?}", name);
    check_java_version();
}