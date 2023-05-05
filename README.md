# Axon Framework in Rust

`axon-rs` is 100% inspired from the amazing [Axon Framework](https://www.axoniq.io/axoniq-products). This project
does not try to be functional replacement in Rust. Please see project goals below for further clarifications.

**In short, this is a toy project to learn how Axon works under the hood by building it in Rust.**

## Description

To quote from the OG:

> `axon-rs` is a framework for building evolutionary, event-driven microservice systems based on the principles of
> Domain-Driven Design (DDD), Command-Query Responsibility Separation (CQRS), and Event Sourcing.

## Project Goals

- Learn internals of how Axon Framework functions
- Build a **light-weight** working version in Rust
- This project does not aim to build a 1:1 implementation of Axon Framework
- Rust semantics and best practices are top priority
- Single crate contains framework components
- Server, cloud or server application are out of scope

## Pre-Requisites

Just Rust tools. `rustup` will give everything you need: https://www.rust-lang.org/tools/install

## Documentation

Create html files for documentation and view it. This command will open a web browser to generated html files:

`cargo doc --no-deps --open`

## !! WORK IN PROGRESS !!

**Do not use this project for any production work.**