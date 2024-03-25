# Simx Flow Engine

> Eyre Simpson
>
> March 25, 2024
>
> **MIT Opensource**
>

## Introduce

simx is a tool for flow management across systems. In simx, a flow is a flow in which data passes through a series of nodes. The main content of simx is a flow-based operating system. Each node of the flow can be regarded as an actuator. After the flow flows through this node, the data will be handed to the actuator and wait for it to flow the data. Finally to complete the required operation. simx is implemented through rust, and flowes can parse xml, json, and flow files that comply with the specification. 

## Usage

simx supports common distributions of windows, linux, and newer versions of macos (including the ARM architecture), and you can run compiled executables directly in the operating system, note that you need to modify the configuration file first.