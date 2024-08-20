# Simx Flow Engine

> Eyre Simpson (Noah Jones)
>
> Initial Development: February 13, 2021
>
> Last Updated: August 20, 2024
>
> **Licensed under the MIT Open Source License**

## 💡 Introduction

Simx is a streamlined Flow engine, with its name derived from the initials of the author's pet. The original name was
Glue. It is primarily used in scenarios such as data processing, user operation encapsulation, and automation. It has no
interface and operates as a command-line (shell) tool. You can install it as a system service using installation tools
or register it as the default executor for flow files in the system. The flow design work is done through the simx-ui
project; this project is only the engine part.

In Simx, the concept of a "Flow" is a collection of operations (Handles). These operations can be predefined system
actions, shell/powershell/cmd/python scripts, or methods from a jar/dll/so file. Essentially, a flow is the business
logic composed of operations. Simx is designed to split all non-essential functions into independent plugins, which can
be imported into the system as needed. The main project only retains the minimal runnable parts, with everything else
injected as plugins.

## 🌟 Usage

Simx supports most operating systems. The environments tested include Windows 10/11, macOS, and Linux (Ubuntu/CentOS).

You can directly run the Simx executable file in the `dist` directory. By default, it runs in the background, listening
on port 9898.

At system startup, flows and scripts located in the `script/init` directory will automatically run (this can be
configured in the configuration file). Note that initialization flows do not need to be cron jobs, but cron jobs must be
initialized.

The system's scripts and flows are stored in the `flow` and `script` directories at the same level. The system will scan
these directories and store the files in the database on the first startup for API usage. Therefore, it is recommended
not to manually add or delete these contents. However, the system provides a manual refresh operation to synchronize the
files and database when calling the API to avoid data inconsistencies.

It is recommended to deploy the program on a fast read/write hard drive. For high concurrency scenarios, mechanical hard
drives are not recommended. If there is no need for persistence, you can enable the pure memory mode in the
configuration, which significantly improves running efficiency.

## Module Description

- `conf`: Module for handling configuration files
  - `toml`: TOML configuration file parsing module
  - `yaml`: YAML configuration file parsing module (planned)
  - `json`: JSON configuration file parsing module (planned)
- `core`: Core module responsible for running and controlling flows
  - `common`: Common module including logging and other tools
  - `engine`: Engine module, the core system module
  - `env`: Environment check and control module
  - `event`: Event module for event listening and dispatch
  - `extension`: Extension module for adding extra features
  - `flow`: Flow module responsible for the creation, running, and control of flows and flow nodes
  - `runtime`: Runtime module for managing system configuration data
  - `script`: Script module responsible for running and controlling scripts
- `db`: Database module for database connection and operations
- `entity`: Data structures used within the system
- `net`: Network module for network request handling
  - `http`: RESTful-friendly wrapper
  - `socket`: Socket communication module (TCP)
- `test`: Functional and unit testing
- `tools`: Tools module containing general utilities

### Source Code Instructions

In most cases, modules contain an `interface.rs` file. Generally, all operations of that module should be conducted
through `interface.rs` rather than directly invoking deeper methods.

## Future Plans

- Complete plugin modularization
- Separate the listening module (HTTP listening)
- Separate the script execution module (as an independent function)
- Improve the simx-core part
- Improve the simx-ui part
- Enhance basic built-in operations

## Built-in Operations (Simx)

| **Submodule**             | **Operations**                                               |
|---------------------------|--------------------------------------------------------------|
| Basic Files (files.plain) | File Read (Binary) (reader)                                  |
|                           | File Write (Binary) (writer)                                 |
|                           | File Delete (del)                                            |
|                           | File Move (mv)                                               |
|                           | File Rename (rename)                                         |
|                           | File Copy (cp)                                               |
|                           | File Compress (compress)                                     |
|                           | File Uncompress (uncompress)                                 |
|                           | File Info (info)                                             |
| Script System (script)    | -                                                            |
| Basic OS (os)             | Operating System Info (info)                                 |
|                           | Screen Info (display_info)                                   |
|                           | Hardware Info (hardware_info)                                |
|                           | Execute Command (exec)                                       |
| Basic Network (net)       | Connectivity Test (ping)                                     |
|                           | RESTful Request (get, post)                                  |
|                           | HTTP Request (request)                                       |
|                           | Socket (TCP) Request (tcp)                                   |
| Basic Math (math)         | Basic Operations (add, sub, mul, div, mod, pow)              |
|                           | Logical Operations (eq, neq, gt, lt, gte, lte, and, or, not) |
|                           | Aggregate Operations (max, min, avg, sum, count)             |
| Engine Control (engine)   | Dynamic Load (active)                                        |
|                           | Dynamic Unload (deactivate)                                  |

## Simx Series Projects

| **Project Name** | **Status**     | **Description**                                                                                    |
|------------------|----------------|----------------------------------------------------------------------------------------------------|
| simx-engine      | Open-source    | Responsible for running and managing flows and related plugins                                     |
| simx-core        | Open-source    | Extends built-in system features; recommended to include this plugin                               |
| simx-ui          | In development | UI for flow design, developed in Flutter, for graphically designing flows and managing simx-engine |
| simx-math        | In development | Math extension module, enhancing mathematical processing capabilities                              |
| simx-net         | In development | Network extension module, adding network communication capabilities                                |
| simx-db          | In development | Database extension module, supporting MySQL, SQL Server, Oracle, and other databases               |
| simx-file        | In development | File extension module, handling common file formats like JSON, XML, YAML, TOML                     |
| simx-img         | In development | Image processing module, capable of compression, cropping, rotation, etc. (To be improved)         |

## Applications

### Data Transformation (Relay)

You can implement data flow, manipulation, and output through specific start nodes (data source nodes) and end nodes (
data terminal nodes).

### Automation

The system can receive external commands or monitor specific signals (e.g., time, file or folder changes, RESTful,
socket) through built-in functions or plugins and execute corresponding flows and actions.

### Scheduled Tasks

The program can implement scheduled tasks based on CRON rules.

### Remote Management

Allows real-time control of the server environment through scripts and flows.