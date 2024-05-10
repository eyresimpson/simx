use crate::core::env::java::check_java_version;
use crate::core::flow::entity::standardisation::Env;

pub fn check(name: Vec<Env>) {
    println!("{:?}", name);
    check_java_version();
}