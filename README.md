<p align="center">
  <img src="https://raw.githubusercontent.com/BraneFramework/braneframework.github.io/refs/heads/main/assets/images/logo.png" alt="logo" width="250"/>
  <h3 align="center">Programmable Orchestration of Applications and Networking</h3>
</p>

----

<span align="center">

  [![Audit](https://github.com/BraneFramework/brane/actions/workflows/audit.yml/badge.svg)](https://github.com/BraneFramework/brane/actions/workflows/audit.yml)
  [![CI](https://github.com/BraneFramework/brane/actions/workflows/ci.yml/badge.svg)](https://github.com/BraneFramework/brane/actions/workflows/ci.yml)
  [![Test coverage](https://codecov.io/gh/BraneFramework/brane/graph/badge.svg?token=U2FHZN5BPT)](https://codecov.io/gh/BraneFramework/brane)
  [![Release](https://img.shields.io/github/release/braneframework/brane.svg)](https://github.com/braneframework/brane/releases/latest)
  [![License: Apache-2.0](https://img.shields.io/github/license/braneframework/brane.svg)](https://github.com/braneframework/brane/blob/main/LICENSE)
  [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.3890928.svg)](https://doi.org/10.5281/zenodo.3890928)

</span>

## Introduction
Regardless of the context and rationale, running distributed applications on geographically dispersed IT resources often comes with various technical and organizational challenges. If not addressed appropriately, these challenges may impede development, and in turn, scientific and business innovation. We have designed and developed Brane to support implementers in addressing these challenges. Brane makes use of containerization to encapsulate functionalities as portable building blocks. Through programmability, application orchestration can be expressed using intuitive domain-specific languages. As a result, end-users with limited or no programming experience are empowered to compose applications by themselves, without having to deal with the underlying technical details.

See the [documentation](https://braneframework.github.io) for more information.


## Installation
For a full how-to on how to install or run the framework, refer to the [manual](https://braneframework.github.io/manual/) and then the section that represents your role best. Typically, if you are installing the framework yourself, consult the [installation chapter for administrators](https://braneframework.github.io/manual/system-admins/installation/introduction.html); otherwise, consult the [installation chapter for Software Engineers](https://braneframework.github.io/manual/software-engineers/installation.html) or [that for Scientists](https://braneframework.github.io/manual/scientists/installation.html). The former is a bit more comprehensive, while the latter is a bit more to-the-point and features more visualisations.


## Usage
Similarly to installing it, for using the framework we refer you to the [wiki](https://braneframework.github.io/manual).
Again, choose the role that suits you best
([System administrators](https://braneframework.github.io/manual/system-admins/introduction.html) if you are managing an instance,
[Policy experts](https://braneframework.github.io/manual/policy-experts/installation.html) if you are writing policies,
[Software engineers](https://braneframework.github.io/manual/software-engineers/introduction.html) if you are writing packages or
[Scientists](https://braneframework.github.io/manual/scientists/introduction.html) if you are writing workflows).
You can also follow the chapters in the order presented in the wiki's sidebar for a full overview of everything in the framework.


## Contributing
If you're interrested in contributing, please read the [code of conduct](.github/CODE_OF_CONDUCT.md) and [contributing](.github/CONTRIBUTING.md) guide.

Bug reports and feature requests can be created in the [issue tracker](https://github.com/braneframework/brane/issues).


### Development
If you are intending to develop on the framework, then you should [setup your machine for framework compilation](https://braneframework.github.io/manual/system-admins/installation/dependencies.html#compilation-dependencies) (install both the dependencies for runtime and compilation).

Then, you can clone this repository (`git clone https://github.com/braneframework/brane.git`) to some folder of choice, and start working on the source code. You can check the [specification](https://braneframework.github.io/specification/) to learn more about the inner workings of the framework, and remember: `make` is your biggest friend when compiling the framework, and `branectl` when running or testing it.

Consult the [code documentation](https://braneframework.github.io/brane/unstable/overview/index.html) for more information about the codebase itself. Note, however, that this is generated for the latest release; to consult it for a non-release, navigate to the root of the repository and run:

```bash
cargo doc --release --no-deps --document-private-items --open
```
## 🧪 Tutorial: CWL Integration in Brane

This tutorial demonstrates how to use the new CWL support in Brane to build and load a CWL-based workflow as a Brane package.

---

### ✅ Prerequisites

Ensure that the following are installed:

- Docker
- Rust (via [rustup](https://rustup.rs))
- Brane and `branectl` built from source
- Brane CLI available via `cargo run --bin brane --`

Optional: [CWL reference tools](https://www.commonwl.org/user_guide/quick-start-guide/) to validate your CWL files locally.

---

### 🛠 Step-by-step Guide

#### 1. Clone and build Brane

```bash
git clone https://github.com/BraneFramework/brane.git
cd brane
cargo build --release
```

#### 2. Run the CWL-to-Brane package generator

You can generate a Brane-compatible package from a CWL file:

```bash
cargo run --bin brane -- cwl tests/packages/hello_cwl/hello_world.cwl
```

✔ This will:

- Parse the `hello_world.cwl` CWL file
- Build a Docker image from the CWL tool definition
- Output a Brane package in `target/generated/hello_world/`

#### 3. Load the CWL package into the Brane instance

```bash
brane package load target/generated/hello_world
```

Once loaded, you can verify it is available:

```bash
brane package list
```

Look for `hello_world` with kind `cwl`.

#### 4. Test the CWL package

To test the package:

```bash
brane package test hello_world
```

This runs the `hello_world` CWL tool inside Brane and validates it in isolation.

---

### 🧪 Example Output

If everything works correctly, you should see:

```
✅ Parsed CWL CommandLineTool
🐳 Docker image built: brane-cwl-hello_world:latest
📦 Brane CWL package available at: target/generated/hello_world
✅ Loaded hello_world into the registry
```

---

### 📂 Example Directory Structure

```
brane/
├── tests/
│   └── packages/
│       └── hello_cwl/
│           ├── hello_world.cwl
│           └── input.json (optional)
├── target/
│   └── generated/
│       └── hello_world/
│           ├── Dockerfile
│           ├── entry.sh
│           └── package.yml
```

---

### 🧩 CWL Subset Support

This integration supports CWL v1.1 `CommandLineTool` definitions with basic `inputs`, `outputs`, and `baseCommand` usage. Support for full workflows and advanced expressions is under development.

---