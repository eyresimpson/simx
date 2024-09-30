use crate::core::environment::java::check_java_version;
use engine_common::entity::flow::Environment;

pub fn check(requirements: Vec<Environment>) -> Result<(), String> {
    for requirement in requirements {
        match requirement.name.as_str() {
            // Java 运行时，此处指JRE，同JRE
            "java" => { check_java_version(); }
            "jre" => {}
            "jdk" => {}
            "net" => {}
            "python" => {}
            "engine" => {}
            "os" => {}
            "nodejs" => {}
            "npm" => {}
            "pip" => {}
            "lib" => {}
            "ext" => {}
            "plugin" => {}
            _ => {}
        }
    }
    check_java_version();
    Ok(())
}

