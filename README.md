<div align="center">
  <h1>🛡️ SecureLang</h1>
  <p><strong>An enterprise-grade, security-focused programming language and ecosystem.</strong></p>

  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](#)
  [![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)](#)
  [![React](https://img.shields.io/badge/react-%2320232a.svg?style=for-the-badge&logo=react&logoColor=%2361DAFB)](#)
  [![Node.js](https://img.shields.io/badge/node.js-6DA55F?style=for-the-badge&logo=node.js&logoColor=white)](#)
</div>

---

## 📖 Overview

**SecureLang** is a modern programming language designed from the ground up for enterprise applications where security, memory safety, and performance are paramount. 

This repository contains the entire SecureLang ecosystem, including the core compiler engine, web-based playground, backend services, and a dedicated VS Code extension.

## 🏗️ Architecture & Ecosystem

The project is structured as a monorepo containing several interconnected systems:

- **`securelang-engine/` (Rust)**: The core compiler and runtime. Features a custom lexer, parser, bytecode generator, optimizer, and virtual machine.
- **`backend/` (Node.js, Express, TypeScript)**: API services handling authentication, user management, and code execution endpoints, powered by PostgreSQL & Prisma.
- **`frontend/` (React, Vite, TailwindCSS)**: A beautiful, responsive web application for interacting with SecureLang, writing code, and viewing execution results.
- **`vscode-extension/` (TypeScript)**: A Visual Studio Code extension providing syntax highlighting and language server integration.
- **`docs/`**: Comprehensive architecture and language guidelines.

## 🚀 Getting Started

Follow these steps to set up the full ecosystem locally.

### Prerequisites
- [Docker](https://www.docker.com/) & Docker Compose (for PostgreSQL)
- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://rustup.rs/) (Stable toolchain)

### 1. Database Setup
Start the PostgreSQL database using Docker Compose:
```bash
docker-compose up -d
```

### 2. Backend Services
Navigate to the backend and set up the environment:
```bash
cd backend
npm install
npx prisma generate
npm run dev
```

### 3. Web Frontend
In a new terminal, spin up the React application:
```bash
cd frontend
npm install
npm run dev
```

### 4. Compiler Engine (Rust)
To build and test the core compiler:
```bash
cd securelang-engine
cargo build
cargo test
```
*(To run the CLI: `cargo run -p cli`)*

### 5. VS Code Extension
To test the syntax highlighting extension in VS Code:
```bash
cd vscode-extension
npm install
```
Then, open the `vscode-extension` folder in VS Code and press `F5` to start a new debugging window with the extension loaded.

## 🛡️ Language Features (WIP)
- Memory-safe execution environment
- Strict static typing with inference
- First-class concurrency primitives
- Built-in bounds checking and overflow protection

## 📜 License
This project is licensed under the standard open-source license. See the [LICENSE](./LICENSE) file for details.
