# DEX Integrity and Bytecode Constraints

This document describes the syntactical and semantical constraints for a file to be a valid `.dex` file.

## Categories

1. [**General Integrity Constraints**](./constraints/integrity.md) - Structural constraints of the DEX file format (G-series).
2. [**Static Bytecode Constraints**](./constraints/static.md) - Constraints on individual bytecode instructions (A-series).
3. [**Structural Bytecode Constraints**](./constraints/structural.md) - Constraints on instruction relationships and control flow (B-series).

## Overview
A `.dex` file is the transport format for Dalvik bytecode. A runtime is required to support only valid `.dex` files. Verification ensures that these constraints are met before execution.
