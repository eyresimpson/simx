use crate::core::flow::entity::standardisation::Flow;

// 执行标准化的流
pub fn exec_standardisation_flow(flow: Flow){
    for step in flow.steps {
        
    }
    println!("{}", flow.flow_name.as_str());
}