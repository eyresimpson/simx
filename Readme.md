# Simx 流引擎

> Eyre Simpson（Noah Jones）
>
> 初步开发：2021年02月13日
>
> 最后更新：2024年10月19日
>
> **使用 MIT 开源协议**

## 💡 介绍

simx是一个精简的流引擎，主要应用于数据处理、用户操作封装、用户操作自动化等场景。它本身没有界面，是一个命令行（shell）工具，可以通过安装工具将其作为一个系统服务进行安装，
或注册为系统中flow文件的默认执行器；用户通过simx-ui项目实现流设计的工作，本项目仅为引擎部分。

在simx中，流（Flow）的概念就是一系列操作（Handle）的集合，操作可以是系统预制的一些操作，或者一个shell/powershell/cmd/python脚本，也可以是一个jar/dll/so中的方法，
总之，流本身就是由操作组成的业务逻辑，simx的设计思路是所有非必须的功能尽可能拆分出去，作为独立的插件，用户需要使用可以将其引入到系统中，主项目中仅保留最小可运行部分，其余均作为插件形式注入

## 🌟 使用

simx支持大多数操作系统，目前测试的环境包括windows 10/11, macos, linux(ubuntu/centos)。

可以直接运行dist下的simx可执行文件即可，正常情况下不会显示，会自动开启进程监听，如果需要开放restful接口，需要增加serve引擎扩展

系统启动时会自动运行位于script下init目录下的流和脚本（可在配置文件中配置），注意，**初始化的流可以不是cron任务，但cron的任务一定需要初始化执行
**

系统的脚本和流程分别存放在同级目录下的flow和script，需要注意的是，
系统会在初次启动时，主动扫描该文件夹下的文件并存入数据库，以供api调用时使用，因此尽可能不要手动添加或删除这些内容，当然系统也提供了手动刷新数据库的操作，
可以在调用api时事先同步一下文件夹和数据库，以避免数据不同步的情况。

建议将程序部署在读取/写入速度较快的硬盘中，一般高并发场景不建议使用机械硬盘，如果没有持久化的场景需求，可以在配置中开启纯内存模式，可以提升非常大的运行效率

### 源码说明

大部分情况下，模块都含有`interface.rs`文件，一般情况下对该模块的所有操作都应通过`interface.rs`进行，而非直接调用深层次的方法

## 应用

### 数据转换（中转）

可以通过特定的开始节点（数据源节点）和结束节点（数据终端节点），实现数据的流入、操作和流出，类似于Nifi的数据转换功能

### 自动化操作

可以通过系统预置的功能或插件接受外部的指令或监听某些信号（如时间、文件或文件夹变动、restful、socket等），并执行相应的流程和动作，类似于RPA的功能

### 定时任务

程序可以根据CRON规则，实现定时任务

### 远程管理

允许通过脚本和流程，实现服务器上环境的实时控制，类似于Jenkins的部分功能
