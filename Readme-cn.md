# Simx 流引擎

> Eyre Simpson
>
> 2024年03月25日
>
> **遵循 MIT 开源协议**
>

## 介绍

simx是一个精简的流引擎，主要应用于数据处理、用户操作封装、用户操作自动化等场景。它本身没有界面，是一个命令行（shell）工具，但系统对外提供了api，
允许用户在其上封装一层UI界面，后续系统开发完成后，可能会创建新的项目以实现这个功能。

## 使用

simx支持大多数操作系统，目前测试的环境包括windows 10/11, macos, linux(ubuntu/centos)。

可以直接运行dist下的simx可执行文件即可，正常情况下不会显示，会自动开启后台监听（默认对9898端口进行监听）

系统启动时会自动运行位于同级目录下init目录下的流和脚本（作为初始化内容），系统的脚本和流程分别存放在同级目录下的flow和script，需要注意的是，
系统会在初次启动时，主动扫描该文件夹下的文件并存入数据库，以供api调用时使用，因此尽可能不要手动添加或删除这些内容，当然系统也提供了手动刷新数据库的操作，
可以在调用api时事先同步一下文件夹和数据库，以避免数据不同步的情况。