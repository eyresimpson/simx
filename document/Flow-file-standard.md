# Simx 流文件规范

> 规范版本: 0.0.1 （草稿）
>
> 创建时间：2024年04月25日
>
> 修订时间：2024年10月10日
>
> 此文档描述了流引擎默认的 `flow` 文件的基础规范
>
> 本规范目前为草稿，仅适用于 Simx engine 0.1.2 的初步测试

## 介绍

此规范规定了可运行于Simx引擎中的流文件的基本格式，包括所有可用，可识别的条目以及各个条目的作用。

**注意，本文档不介绍Simx中的处理器，与处理器相关的文档请参阅《Simx Handler Document》或其他第三方的文档**

## 概念

在`Simx`中，流作为文件存储在计算机硬盘上，在引擎初始化或收到用户任务后，引擎会主动寻找指定的流文件并将其加载到内存中执行。

目前 `Simx` 支持两种类型的方式组织流文件，一种是将流文件存放在系统中，由系统进行管理，另一种则允许用户通过项目进行管理。
在一个项目中，用户可以自定义该项目的配置，项目中的配置要高于simx引擎默认的配置（simx配置文件不会被覆盖，以引擎的配置文件为准），以此可以实现一些独特的定制化需求。

## 完整案例

目前Simx引擎（0.1.2）支持Json格式的flow文件

```json
{
  "version": "流文件版本号",
  "name": "流名称",
  "update_date": "流文件更新日期",
  "create_date": "流文件创建日期",
  "developer": "流文件作者",
  "requirements": [
    {
      "key": "engine",
      "val": "Base"
    }
  ],
  "blueprint": {
    "parallel_endpoints": true,
    "parallel_routes": false,
    "maximum_repetition": 30,
    "endpoints": [
      "1"
    ],
    "routes": {
      "节点ID": {
        "name": "处理器名称",
        "handler": "处理器路径",
        "downstream": [],
        "redress_stream": [],
        "attr": {
          "属性1": "属性值",
          "属性2": "属性值"
        }
      }
    }
  }
}
```

### version

流文件的版本号，用于标识流文件版本，便于引擎识别

**必需的属性，不存在会引起流引擎报错**

### name

流文件的名称，如果不存在在执行时会被标识为 undefined

_可选的属性，不存在不会引起流引擎报错_

### update_date

文件更新日期，用于标识文件最近的修改时间，引擎根据此属性判断是否更新内存中的流文件缓存

**必需的属性，不存在会引起流引擎报错**

### create_date

当前文件创建时间

_可选的属性，不存在不会引起流引擎报错_

### developer

开发者名称，可以是个人或公司/部门名称

_可选的属性，不存在不会引起流引擎报错_

### requirements

流运行的环境需求，比如Simx版本、JDK版本、Python版本等

_可选的属性，不存在不会引起流引擎报错_

#### 基础

requirements 属性是一个数组，每个元素都是一个Json对象，如下所示：

```json
    {
  "key": "engine",
  "val": "0.1.2"
}
```

引擎在不安装环境检查扩展插件的情况下，可以识别如下的名称：

| 名称     | 描述       |
|--------|----------|
| base   | 基础环境     |
| engine | 引擎版本     |
| java   | JDK版本    |
| python | Python版本 |
| node   | Node版本   |

### blueprint

Simx 蓝图，用于描述流的运行逻辑，包含流中的所有节点以及节点间的关系，引擎调度器根据此配置执行

**必需的属性，不存在会引起流引擎报错**

#### parallel_endpoints

开始节点并行，即同时执行多任务，默认为 false

_可选的属性，不存在不会引起流引擎报错_

#### parallel_routes

节点会以并行的方式执行，不会等待前一个节点执行完成，但会等待所有节点执行完成后，才会退出流的执行调度，默认为 false

_可选的属性，不存在不会引起流引擎报错_

#### maximum_repetition

最大重复次数

此配置用于防止死循环导致的栈溢出，如果设置为30，则所有路由节点最多被调度30次，如果超出就报错，并截止流的运行，默认为30

_可选的属性，不存在不会引起流引擎报错_

#### endpoints

开始节点列表，引擎会根据此列表开始执行流

- 如果parallel_endpoints为false，引擎会依次执行endpoints中的流
- 如果parallel_endpoints为true，引擎会并发执行endpoints中的流

**必需的属性，不存在会引起流引擎报错**

#### routes

核心路由，所有的节点配置都在此属性中

**必需的属性，不存在会引起流引擎报错**

```json
{
  "routes": {
    "节点ID": {
      "name": "处理器名称",
      "handler": "处理器路径",
      "downstream": [],
      "redress_stream": [],
      "attr": {
        "属性1": "属性值",
        "属性2": "属性值"
      }
    }
  }
}
```

##### 节点ID

当前节点的ID，用于标识节点，节点ID必须唯一（在整个流文件中唯一）

##### name

节点名称，用于标识节点，节点名称允许重复，允许为空，为空时显示 undefined

_可选的属性，不存在不会引起流引擎报错_

##### handler

节点处理器路径

##### downstream

下游节点列表，其中为String字符串，代表下游节点ID，多个下游节点用逗号隔开

> ⚠️ 注意：此内容有改变的计划：
> 允许配置是否执行的表达式，可以淘汰部分路由组件

##### attr

节点属性信息，是一个String类型的键值对

_可选的属性，不存在不会引起流引擎报错_

> ⚠️ 注意：此内容有改变的计划：
> 允许传入任意类型的数据，而非仅允许String类型

##### redress_stream

补偿流id列表，用于标识节点的补偿流，当节点执行失败时，会根据配置，调度器会调度这些补偿流，用于处理节点执行失败的情况

_可选的属性，不存在不会引起流引擎报错_

> ⚠️ 注意：此内容有改变的计划：
> 允许根据错误类型，执行对应的补偿流

##### tags

节点标签信息

_可选的属性，不存在不会引起流引擎报错_

> ⚠️ 注意：此配置项目前参数不是稳定版，后续可能会调整

调度器会根据这些标签对节点进行额外处理，目前支持如下的标签：

- Compute：标识节点为大负荷的计算节点
- IO：标识节点为IO密集型节点
- Command：标识节点为命令执行节点
- Route：标识节点为路由节点
- Data：标识节点与数据库交互
- Debug：标识为调试节点
- Delay：标识为耗时节点
- Priority：标识为优先级节点，会优先给予资源
- Sync：标识为阻塞节点，会强制调度器等待此节点执行完成
- Async：标识为异步节点，调度器不会等待此节点执行完成