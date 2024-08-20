# Simx Serve Extension

> **Eyre Simpson**
>
> August 16, 2024
>
> 0.0.1
>
> **MIT License**

本插件可以让simx引擎对外开放http（restful）服务和socket 服务，可以在配置目录下创建serve.toml控制服务行为

同时为系统添加了 handle_http_server 和 handle_socket_server 两个流开始节点，可以监听http和socket请求

### 拥有的handle：

#### handle_http_server:

##### 参数列表

| 参数名       | 描述      | 类型          | 默认值                         |
|-----------|---------|-------------|-----------------------------|
| addr      | 监听地址    | 字符串         | 0.0.0.0                     |
| port      | 监听端口    | 整数（1-65534） | 8080                        |
| functions | 监听方法的集合 | 数组          | [ ]                         |
| security  | 安全配置    | 安全配置对象      | HttpSecurityConfig::Default |

#### handle_socket_server:

##### 参数列表

| 参数名       | 描述      | 类型          | 默认值                           |
|-----------|---------|-------------|-------------------------------|
| addr      | 监听地址    | 字符串         | 0.0.0.0                       |
| port      | 监听端口    | 整数（1-65534） | 9080                          |
| functions | 监听方法的集合 | 数组          | [ ]                           |
| security  | 安全配置    | 安全配置对象      | SocketSecurityConfig::Default |
