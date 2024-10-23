// 服务是Simx的一个概念，与处理器类似，但生命周期长于处理器，如果在项目中加载，那生命周期一般等于项目的生命周期（用户也可以通过处理器中断服务）
// 通常用于提供长时间的功能，比如http监听、Mysql 连接池等
// 注意，Simx引擎不提供任何服务，因此所有的服务都是外部的

use engine_common::entity::services::Service;

// 加载服务
pub fn load_service(service: Service) {}