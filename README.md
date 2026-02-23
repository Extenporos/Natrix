# Natrix

Natrix is an experimental hybrid environment management system designed to combine structured metadata (like Conda), lightweight runtime reuse (like venv), and declarative configuration (inspired by Docker), while maintaining its own extensible architecture.

> Status: 0.0.1-alpha  
> Platform: Linux (Tested on Linux Mint)  
> Language (Alpha): Python 3.13.5  

---

## Philosophy

Natrix is **not** a wrapper.  
It is an environment engine with:

- Declarative configuration
- Structured metadata
- Backend abstraction (pip for Alpha)
- Future isolation extensibility
- Exportable reproducible environments (.natenv)

---

## Core Concepts

### Environment

An environment in Natrix is:

- Directory-based
- Metadata-driven
- Runtime-referenced (Python in Alpha)
- Reproducible via lockfile

Default location:


~/.natrix/envs/<environment_name>/


Structure:


metadata.json
natrix.lock
Natrixfile
runtime/
bin/
lib/


---

## Natrixfile (Declarative Syntax)

Example:

```c
environment "example" {

    runtime {
        type = "python";
        version = "3.13.5";
    }

    isolation {
        level = "light";
    }

    dependencies {
        requests;
        numpy == "2.0";
    }

}```

The Natrixfile is parsed and compiled into structured metadata before execution.

Alpha Features (0.0.1)

Environment creation

Metadata generation

Dependency installation (pip backend)

Lockfile generation

Declarative environment builds

Manual activation (venv-style)

Export to .natenv (reproducible archive format)

Backend System

Alpha backend:

pip (internal usage)

Runtime reuse (system Python / venv style)

Future backend:

Native dependency resolver

Rust-powered components

Export Format (.natenv)

.natenv is a reproducible archive format containing:

manifest.json

metadata.json

natrix.lock

Natrixfile

checksums

It does NOT package the runtime.
It ensures reproducibility, not portability.

Roadmap
Alpha

Stable CLI

Stable metadata model

Natrixfile parser

Lockfile integrity

Beta

Plugin system

Isolation levels expansion

Partial Rust migration

Future

Strong isolation (namespaces)

Native dependency engine

Performance-critical components in Rust

Design Goals

Deterministic environments

Clean architecture

Backend abstraction

Zero hidden state

Fully declarative builds

Warning

This project is experimental and under active development.
Internal APIs may change.
