- [Preface](#preface)
  - [Ways of working](#ways-of-working)
  - [Cloud-native Applications](#cloud-native-applications)

# Preface

## Ways of working
* Trunk-based development works well to write software that is continuously deployed in a Cloud environment.
* A Gitflow approach works better for a team that sells software that is hosted and run on-premise by their customers.
* If you are working alone, you can just push straight to `main`.

## Cloud-native Applications
* We expect Cloud-native applications to:
  1. Achieve high-availability while running in fault-prone environments
  2. Allow us to continuously release new versions with zero downtime
  3. Handle dynamic workloads (e.g. request volumes)

* High availability means that our application should be able to serve requests with no downtime even if one or more of our machines suddenly starts failing.
  * This forces our application to be *distributed* - there should be multiple instances of it running on multiple machines.
* To handle dynamic workloads, we should be able to measure if our system is under load and throw more compute at the problem by spinning up new instances of the application.
  * This also requires our infrastructure to be elastic to avoid over-provisioning.
  * Running a replicated application influences our approach to data persistence - we will avoid using the local filesystem as our primary storage solution, relying instead on databases for our persistence needs.

# Ch1 - Getting Started
* Tooling should be treated as a first-class concern both when designing and teaching the language itself.
* `rustup` is more than a Rust installer - its main value proposition is *toolchain management*.
  * A toolchain is the combination of a *compilation target* and a *release channel*.

## Installing The Rust Toolchain

### Compilation Targets
* The Rust compiler converts Rust code into machine code therefore you need a different backend of the Rust compiler for each compilation target, i.e. for each platform (e.g. 64-bit Linux or 64-bit OSX) you want to produce a running executable for.
  
### Release Channels
* The Rust compiler itself is a living piece of software: it continuously evolves and improves.
* The Rust project strives for *stability without stagnation*.
* Two other release channels: `beta` & `nightly`.

### What Toolchains Do We Need?
* We will perform any cross-compiling - our production workloads will be running in containers, hence we do not need to cross-compile from our development machine to the target host used in our production environment.

## Project Setup
* `cargo new zero2prod`

## IDEs
* You have two main options for your IDE setup: rust-analyzer and IntelliJ Rust.
* `rust-analyzer` is an implementation of the Language Server Protocol for Rust.
The Language Server Protocol makes it easy to leverage `rust-analyzer` in many different editors.
* IntelliJ Rust provides Rust support to the suite of editors developed by JetBrains.

## Inner Development Loop
* While working on our project, we will be going through the same steps over and over again, aka inner development loop:
 - Make a change
 - Compile the application
 - Run tests
 - Run the application

### Faster Linking
* A sizeable chunk of time is spent in the **linking** phase when are doing incremental builds.
* The default linker does a good job, but there is a faster one by the LLVM project: `lld`.
* See `.cargo/config.toml` to see how to install and add configuration to use `lld`.

### cargo-watch
* We can use `cargo-watch` to reduce the perceived compilation time - i.e. the time you spend looking at your terminal waiting for cargo check or cargo run to complete.
* `cargo watch -x check` monitors your source code to trigger commands like `cargo check` (in this case) every time a file changes.
* We can also do `cargo watch -x check -x test -x run` to chain 3 commands together that `cargo check`, `cargo test`, and `cargo run` each time our code changes.

## Continous Integration
* In trunk-based development we should be able to deploy our `main` branch at any point in time.
* Every member of the team can branch off from `main`, develop a small feature or fix a bug, merge back into `main` and release to our users.
* CI also provides a tighter feedback loop.

### CI Steps

#### Tests
* If our CI pipeline had only 1 step, it should be `cargo test`.

#### Code Coverage
* While using code coverage as a quality check has several drawbacks I do argue that it is a quick way to collect information and spot if some portions of the codebase have been overlooked over time and are indeed poorly tested.
* `cargo install cargo-tarpaulin`
* `cargo tarpaulin --ignore-tests` computes code coverage for your application code, ignoring test functions.

#### Linting
* A **linter** will try to spot unidiomatic code, overly-complex constructs and common mistakes/inefficiencies to avoid convoluted solutions to problems that could be tackled with a much simpler approach.
* The official Rust linter is `clippy`, which can be installed via `rustup component add clippy` and can be run with `cargo clippy` and we can have it fail in the CI `cargo clippy -- -D warnings`.
* From time to time clippy might suggest changes that you do not believe to be either correct or desirable.
  * You can mute a warning using the `#[allow(clippy::lint_name)]` attribute on the affected code block or disable the noisy lint altogether for the whole project with a configuration line in `clippy.toml` or a project-level `#![allow(clippy::lint_name)]` directive.

#### Formatting
* Let machines deal with formatting.
* The official rust formatter is `rustfmt`, which can be installed with `rustup component add rustfmt`, run locally with `cargo fmt`, and can be run in CI `cargo fmt -- --check`.
* You can tune `rustfmt` for a project with a configuration file, `rustfmt.toml`.

#### Security Vulnerabilities
* The Rust Secure Code working group maintains an Advisory Database - an up-to-date collection of reported vulnerabilities for crates published on crates.io.
* `cargo-audit` checks if vulnerabilities have been reported for any of the crates in the dependency tree of your project.
* It can be installed with `cargo install cargo-audit` and ran with `cargo audit`.
* In addition to running `cargo-audit` in the CI on every commit, we will also run it on a daily schedule to stay on top of new vulnerabilities for dependencies of projects that we might not be actively working on at the moment but are still running in our production environment.

### Ready-to-go CI Pipelines
* It is often easier to tweak an existing CI pipeline to fit our needs than to create one from scratch.

# Ch2 - Building An Email Newsletter

## Our Driving Example

### Problem-based Learning
* Choose a problem you want to solve. Let the problem drive the introduction of new concepts and techniques.

## What Should Our Newsletter Do?
* We will try to build an email newsletter service that supports what you need to get off the ground if you are willing to add an email subscription page to your blog.

### Capturing Requirements: User Stories
1. As a blog visitor, I want to subscribe to the newsletter, so that I can receive email updates when new content is published on the blog.
2. As the blog author, I want to send an email to all my subscribers, so that I can notify them when new content is published.

* We will not add features to
1. unsubscribe
2. manage multiple newsletters
3. segment subscribers in multiple audiences
4. track opening and click rates

## Working in Iterations
* Instead of going deep on one story, on details like do we need authentication, do we support HTML and emojis in the emails, we will build enough functionality to satisfy the requirements of all of our stories in our first release.
* Then, we go back and improve: add fault-tolerance and retries for email delivery, add a confirmation email for new subscribers, etc.
* We will work in iterations: each iteration takes a fixed amount of time and gives us a slightly better version of the product, improving the experience of our users.
* We are iterating on product features, not engineering quality: the code produced in each iteration will be tested and properly documented even if it only delivers a tiny, fully functional feature.

### Coming Up
* Getting off the ground will require some initial heavy-lifting: choosing a web framework, setting up the infrastructure for managing database migrations, putting together our application scaffolding as well as our setup for integration testing.

# Ch3 - Sign Up a New Subscriber

* For the first user story, we expect our blog visitors to input their email address in a form embedded on a web page.
* The form will trigger an API call to a backend server that will actually process the information, store it and send back a response.
* This chapter will focus on that backend server - we will implement the `/subscriptions` POST endpoint.

## Our Strategy
* When starting a new project from scratch - there is a fair amount of upfront heavy-lifting we need to do:
  1. choose a web framework and get familiar with it
  2. define our testing strategy
  3. choose a crate to interact with our database
  4. define how we want to manage changes to our database schemas over time (a.k.a. migrations)
  5. actually write some queries
* Before tackling `/subscriptions` we will implement a `/health_check` endpoint.
  * No business logic, but a good opportunity to learn the web framework and understand of all its different moving parts.

## Choosing A Web Framework
