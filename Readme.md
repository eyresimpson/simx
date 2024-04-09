# Simx Flow Engine

> Eyre Simpson
>
> Apr 9, 2024
>
> **MIT Opensource**
>

## Introduce

simx is a streamlined flow engine, mainly used in data processing, user action encapsulation, user action automation and other scenarios. It has no interface itself, is a command line (shell) tool, but the system provides apis, allowing users to encapsulate a layer of UI interface on it, the subsequent system development is completed, may create a new project to achieve this function.

## Usage

simx supports most operating systems, and currently tested environments include Windows 10/11, macOS, linux(ubuntu/centos).

You can directly run the simx executable file under dist, which will not be displayed under normal circumstances and will automatically start background monitoring (by default, port 9898 is monitored).

When the system starts up, the flows and scripts located in the init directory of the same level will automatically run (as the initialization content), and the scripts and processes of the system are stored in the flow and script of the same level directory respectively. It should be noted that,
When the system starts for the first time, it will actively scan the files in this folder and store them in the database for use when the api is called, so as far as possible, do not manually add or delete these contents. Of course, the system also provides manual database refresh operations.
You can synchronize folders and databases before calling the api to avoid data asynchronous.

## Application

### Data conversion (transit)

Data inflow, manipulation, and outflow can be implemented through specific start nodes (data source nodes) and end nodes (data end nodes)

### Automated operation

You can accept external instructions or listen for certain signals (such as time, file or folder changes, restful, sockets, etc.) through the system's preset functions or plug-ins, and perform corresponding processes and actions

### Remote management

Allows real-time control of the environment on the server through scripts and processes