# Simx 流规范

> 适用于 Flow Version 1.x.x
> 
> 创建时间：2024年04月25日
> 
> 修订时间：2024年04月25日
> 
> 此文档描述了流引擎默认的 `flow` 文件的基础规范，其中包括了系统自带的`handler`的介绍


## 概念：单一流和项目

在`Simx`中，流作为文件存储在计算机硬盘上，在引擎初始化或收到用户任务后，引擎会主动寻找指定的流文件并将其加载到内存中执行。

目前 `Simx` 支持两种类型的方式组织流文件，一种是将流文件存放在系统中，由系统进行管理，另一种则允许用户通过项目进行管理。
在一个项目中，用户可以自定义该项目的配置，项目中的配置要高于simx引擎默认的配置（simx配置文件不会被覆盖，以引擎的配置文件为准），以此可以实现一些独特的定制化需求。