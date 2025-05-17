# r8s

Welcome to r8s, an automated workflow engine that leverages Rust's performance to provide a scalable and reliable solution for building and executing automation workflows.

## Table of Contents

- [Introduction](#introduction)
- [Project Architecture](#project-architecture)
  - [Core System (Essential)](#core-system-essential)
  - [Backend and API](#backend-and-api)
  - [User Interface (Optional)](#user-interface-optional)
  - [Ecosystem and Integrations](#ecosystem-and-integrations)
- [Development Roadmap](#development-roadmap)
- [How to Contribute](#how-to-contribute)
- [License](#license)

## Introduction

r8s was designed to offer a modern automation workflow solution by combining the reliability of Rust with a modular architecture. It enables the integration of various modules and plugins, as well as providing an optional user interface to simplify the creation and management of workflows.

## Project Architecture

### Core System (Essential)

- **Execution Engine**  
  Responsible for running and efficiently managing workflows.
- **Node Manager**  
  Defines the interaction and data exchange between nodes that comprise the workflows.
- **Workflow Parser**  
  Handles the creation and loading of automation flows.
- **State Storage**  
  Maintains logs and monitors pending executions, ensuring traceability.
- **Execution Queue**  
  Processes tasks asynchronously to enhance system scalability.

### Backend and API

- **HTTP Server**  
  Implemented using frameworks like Actix/Web or Axum to expose REST/gRPC endpoints.
- **Authentication and Access Control**  
  Manages users and permissions, ensuring data security.
- **Plugin Manager**  
  Facilitates the integration and development of additional modules.
- **Database**  
  Uses PostgreSQL or SQLite for persisting workflows and user information.

### User Interface (Optional)

- **Frontend**  
  Can be developed as WebAssembly or as a Single Page Application (SPA) using TypeScript/React to create an intuitive interface for workflow design.
- **WebSocket Connector**  
  Ensures real-time communication between the graphical interface and the backend.

### Ecosystem and Integrations

- **Plugin Library**  
  Allows for the integration of connectors for popular APIs and other external services.
- **CLI for Deployment and Management**  
  Simplifies system administration via the terminal by offering custom commands.
- **Docker and Kubernetes Support**  
  Provides full support for deployment and scalability, enabling integrations with containerization and orchestration environments.

## Development Roadmap

The following detailed roadmap outlines the development phases of r8s, prioritizing from the core system to the integration and expansion of functionalities:

| Phase                                    | Completed Tasks                                                                                                                                                                                                          |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Phase 1 – Core Fundamentals**          | - [x] Define project structure (workspace, crates) <br> - [x] Create the basic execution engine <br> - [ ] Implement primary node types (input, output, basic transformations) <br> - [x] Establish state storage system |
| **Phase 2 – Backend and API**            | - [x] Implement REST API to manage workflows <br> - [ ] Set up user authentication and control <br> - [ ] Add asynchronous execution queue <br> - [x] Implement database persistence                                     |
| **Phase 3 – Interface and Interaction**  | - [ ] Develop a basic web panel <br> - [ ] Implement WebSockets for real-time communication <br> - [ ] Create CLI for management                                                                                         |
| **Phase 4 – Integrations and Expansion** | - [ ] Enable support for external plugins and integrations <br> - [ ] Enhance scalability with Docker/Kubernetes <br> - [ ] Documentation and initial beta tests                                                         |

## How to Contribute

Contributions are always welcome to enrich and expand the project. To collaborate:

- Fork this repository and create a branch for your modifications.
- Follow the established guidelines and coding standards.
- Run tests and, if possible, include additional test cases.
- Submit a pull request for review and integration.

We welcome improvements in both the backend and the user interface. If you prefer focusing on the UI, your collaboration will be essential to offer a richer and more intuitive experience.

## License

This project is licensed under the MIT License. For more details, please refer to the LICENSE file.
