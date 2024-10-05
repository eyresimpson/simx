Simx 扩展开发手册

## 概述

Simx 是一个基于Rust语言开发的流引擎，主要用于自动化、RPA和后端低代码。

Simx 引擎本身秉承最小化原则，非必须的功能都属于可选部分，因此在默认情况下，Simx 引擎仅附带最常用的处理器，剩余的部分都作为特殊业务扩展。

引擎扩展分为两种：

- 引擎扩展（Engine Extension）
- 处理器扩展（Handler Extension）

### 引擎扩展

Simx 通过扩展的方式，允许用户对流引擎的各个方面进行扩展，因此具有很高的灵活性，默认情况下引擎并不附带Restful API，但可以通过
Restful 插件，开启系统的API功能

### 处理器扩展

处理器扩展则主要用于实现流中的自定义节点，秉承开放性的原则，流处理器扩展的开发模式主要有两种，即适应性扩展和规范化扩展。

#### 适应性扩展

适应性扩展指用户可以安装一个针对特定语言、业务库的解析调用扩展，系统根据此扩展，对第三方的库进行调用和管理，第三方库可以不修改其源代码，直接被Simx
引擎调用其中的方法。

当引擎调度器调度到某个节点，发现该节点的handler不是simx后，会尝试寻找名称匹配handler首位的插件，然后将handler路径、当前节点的参数、当前流数据传递给适应扩展的插件的analysis方法，由该方法决定流接下来的调用方法和方式，并等待该方法返回的流数据。

#### 规范化扩展

规范化扩展是指用户通过Rust、Java、Python、C#、C/C++ 编程语言，按照一定的规范，将自定义的处理器编写为插件，通过插件的方式，被Simx引擎调用，从而实现自定义的处理器。

当引擎调度器调度到某个节点，发现该节点的handler不是simx后，会尝试寻找名称匹配handler首位的插件，然后将handler路径、当前节点的参数、当前流数据传递给插件的interface方法，并等待该方法返回流数据。

### 扩展结构

一个完整的Simx 处理器扩展，需要包含以下的结构：

- 引擎描述文件（`plugin.json`）
- 前台描述文件（`plugin.json`）
- 插件实现文件（`plugin.dll`/`plugin.so`/`plugin.py`/`plugin.jar`）

#### 引擎描述文件

引擎描述文件为一个名为plugin的json文件或者yaml文件，其内容如下（Json示例）：

```json
{
  "name": "插件名称",
  "version": "插件版本",
  "engine": "兼容的引擎版本范围",
  "description": "插件描述",
  "license": "开源许可，非开源设置为none",
  "author": "插件作者",
  "keywords": [
    "关键字"
  ],
  "libs": [
    "依赖库1路径",
    "依赖库2路径"
  ],
  "dependencies": [
    "依赖的其他插件"
  ],
  "init_lib": "入口库所在路径，默认main",
  "init_func": "入口方法名称，没有需要设置为none，默认init",
  "destroy_func": "销毁方法名称，没有需要设置为none，默认destroy"
}
```

#### 前台描述文件

前台描述文件和后台描述文件非常类似，仅新加了一个functions字段，用于向设计器描述插件中暴露的函数，其内容如下（Json示例）：

```json
{
  "name": "插件名称",
  "version": "插件版本",
  "engine": "兼容的引擎版本范围",
  "description": "插件描述",
  "license": "开源许可，非开源设置为none",
  "author": "插件作者",
  "keywords": [
    "关键字"
  ],
  "libs": [
    "依赖库1路径",
    "依赖库2路径"
  ],
  "functions": [
    {
      "handler": "处理器路径",
      "name": "函数名称",
      "description": "函数描述",
      "params": [],
      "return": "返回值类型"
    }
  ],
  "dependencies": [
    "依赖的其他插件"
  ],
  "init_lib": "入口库所在路径，默认main",
  "init_func": "入口方法名称，没有需要设置为none，默认init",
  "destroy_func": "销毁方法名称，没有需要设置为none，默认destroy"
}
```

#### 插件实现文件

插件实现文件就是实际的库，目前支持以下格式：

- Rust lib（dll/so）
- Python（Py）

目前计划支持的格式：

- Java（jar）
- C#（dll）
- C/C++（dll/so）

## 起步

> 本案例以实现一个新的规范化扩展为例，介绍如何通过Rust实现一个处理器，以实现一个简单的加法计算为例。
>
> Windows 10环境，需要安装并正确配置Git和Rust环境。

### 项目创建

在任意文件夹中，创建一个文件夹，此处为`test`，并进入该文件夹，执行以下命令：

```cmd
git clone https://github.com/eyresimpson/simx-extension-template.git
```

### 修改项目

通过Visual Studio Code打开项目，修改`src/main.rs`文件