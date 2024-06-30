- [Preface](#preface)
  - [Ways of working](#ways-of-working)
  - [Cloud-native Applications](#cloud-native-applications)
- [Ch1 - Getting Started](#ch1---getting-started)
  - [Installing The Rust Toolchain](#installing-the-rust-toolchain)
    - [Compilation Targets](#compilation-targets)
    - [Release Channels](#release-channels)
    - [What Toolchains Do We Need?](#what-toolchains-do-we-need)
  - [Project Setup](#project-setup)
  - [IDEs](#ides)
  - [Inner Development Loop](#inner-development-loop)
    - [Faster Linking](#faster-linking)
    - [cargo-watch](#cargo-watch)
  - [Continous Integration](#continous-integration)
    - [CI Steps](#ci-steps)
      - [Tests](#tests)
      - [Code Coverage](#code-coverage)
      - [Linting](#linting)
      - [Formatting](#formatting)
      - [Security Vulnerabilities](#security-vulnerabilities)
    - [Ready-to-go CI Pipelines](#ready-to-go-ci-pipelines)
- [Ch2 - Building An Email Newsletter](#ch2---building-an-email-newsletter)
  - [Our Driving Example](#our-driving-example)
    - [Problem-based Learning](#problem-based-learning)
  - [What Should Our Newsletter Do?](#what-should-our-newsletter-do)
    - [Capturing Requirements: User Stories](#capturing-requirements-user-stories)
  - [Working in Iterations](#working-in-iterations)
    - [Coming Up](#coming-up)
- [Ch3 - Sign Up a New Subscriber](#ch3---sign-up-a-new-subscriber)
  - [Our Strategy](#our-strategy)
  - [Choosing A Web Framework](#choosing-a-web-framework)
  - [Our First Endpoint: A Basic Health Check](#our-first-endpoint-a-basic-health-check)
    - [Anatomy Of An actix-web Application](#anatomy-of-an-actix-web-application)
      - [Server - `HttpServer`](#server---httpserver)
      - [Application - `App`](#application---app)
      - [Endpoint - `Route`](#endpoint---route)
      - [Runtime - `tokio`](#runtime---tokio)
    - [Implementing The Health Check Handler](#implementing-the-health-check-handler)
  - [Our First Integration Test](#our-first-integration-test)
    - [How Do You Test An Endpoint?](#how-do-you-test-an-endpoint)
    - [Where Should I Put My Tests?](#where-should-i-put-my-tests)
    - [Changing Our Project Structure For Easier Testing](#changing-our-project-structure-for-easier-testing)
  - [Implementing Our First Integration Test](#implementing-our-first-integration-test)
    - [Polishing](#polishing)
      - [Choosing A Random Port](#choosing-a-random-port)
  - [Refocus](#refocus)
  - [Working With HTML Forms](#working-with-html-forms)
    - [Refining Our Requirements](#refining-our-requirements)
    - [Capturing Our Requirements As Tests](#capturing-our-requirements-as-tests)
    - [Parsing Form Data From A POST Request](#parsing-form-data-from-a-post-request)
      - [Extractors](#extractors)
      - [`Form` And `FormRequest`](#form-and-formrequest)
  - [Storing Data: Databases](#storing-data-databases)
    - [Choosing A Database](#choosing-a-database)
    - [Choosing A Database Crate](#choosing-a-database-crate)
      - [Compile-time safety](#compile-time-safety)
      - [Query Interface](#query-interface)
      - [Async Support](#async-support)
      - [Our Pick: `sqlx`](#our-pick-sqlx)
    - [Integration Testing With Side-effects](#integration-testing-with-side-effects)
    - [Database Setup](#database-setup)
      - [Docker](#docker)
      - [Database Migrations](#database-migrations)
        - [sqlx-cli](#sqlx-cli)
        - [Database Creation](#database-creation)
        - [Adding A Migration](#adding-a-migration)
    - [Writing Our First Query](#writing-our-first-query)
      - [Sqlx Feature Flags](#sqlx-feature-flags)
      - [Configuration Management](#configuration-management)
        - [Reading A Configuration File](#reading-a-configuration-file)
      - [Connecting To Postgres](#connecting-to-postgres)
      - [Our Test Assertion](#our-test-assertion)
      - [Updating Our CI Pipeline](#updating-our-ci-pipeline)
  - [Persisting A New Subscriber](#persisting-a-new-subscriber)
    - [Application State In `actix-web`](#application-state-in-actix-web)
    - [`actix-web` Workers](#actix-web-workers)
    - [The `Data` Extractor](#the-data-extractor)
    - [The `INSERT` Query](#the-insert-query)
    - [Test Isolation](#test-isolation)
- [Ch4 - Telemetry](#ch4---telemetry)
  - [Unknown Unknowns](#unknown-unknowns)
  - [Observability](#observability)
  - [Logging](#logging)
    - [The `log` crate](#the-log-crate)
    - [`actix-web`'s `Logger` Middleware](#actix-webs-logger-middleware)
    - [The Facade Pattern](#the-facade-pattern)
  - [Instrumenting POST /subscriptions](#instrumenting-post-subscriptions)
    - [Interactions With External Systems](#interactions-with-external-systems)
    - [Think Like A User](#think-like-a-user)
    - [Logs Must Be Easy to Correlate](#logs-must-be-easy-to-correlate)
  - [Structured Logging](#structured-logging)
    - [The `tracing` crate](#the-tracing-crate)
    - [Migration From `log` to `tracing`](#migration-from-log-to-tracing)
    - [`tracing`'s Span](#tracings-span)
    - [Instrumenting Futures](#instrumenting-futures)
    - [`tracing`'s Subscriber](#tracings-subscriber)
    - [`tracing-subscriber`](#tracing-subscriber)
    - [`tracing-bunyan-formatter`](#tracing-bunyan-formatter)
    - [`tracing-log`](#tracing-log)
    - [Removing Unused Dependencies](#removing-unused-dependencies)
    - [Logs For Integration Tests](#logs-for-integration-tests)
    - [Cleaning Up Instrumentation Code - tracing::instrument](#cleaning-up-instrumentation-code---tracinginstrument)
    - [Protect Your Secrets - `secrecy`](#protect-your-secrets---secrecy)
    - [Request Id](#request-id)
    - [Leveraging The `tracing` Ecosystem](#leveraging-the-tracing-ecosystem)
- [Ch 5 - Going Live](#ch-5---going-live)
  - [We Must Talk About Deployments](#we-must-talk-about-deployments)
  - [Choosing Our Tools](#choosing-our-tools)
    - [Virtualization: Docker](#virtualization-docker)
    - [Hosting: DigitalOcean](#hosting-digitalocean)
  - [A Dockerfile For Our Application](#a-dockerfile-for-our-application)
    - [Dockerfiles](#dockerfiles)
    - [Build Context](#build-context)
    - [Sqlx Offline Mode](#sqlx-offline-mode)
    - [Running An Image](#running-an-image)
    - [Networking](#networking)
    - [Hierarchical Configuration](#hierarchical-configuration)
    - [Optimizing Our Docker Image](#optimizing-our-docker-image)
      - [Docker Image Size](#docker-image-size)
      - [Caching For Rust Docker Builds](#caching-for-rust-docker-builds)
  - [Deploy To DigitalOcean](#deploy-to-digitalocean)
    - [Setup](#setup)
    - [App Specification](#app-specification)
    - [How To Inject Secrets Using Environment Variables](#how-to-inject-secrets-using-environment-variables)
    - [Connecting To Digital Ocean’s Postgres Instance](#connecting-to-digital-oceans-postgres-instance)
    - [Environment Variables In The App Spec](#environment-variables-in-the-app-spec)
    - [One Last Push](#one-last-push)
- [Ch 6 - Reject Invalid Subscribers #1](#ch-6---reject-invalid-subscribers-1)
  - [Requirements](#requirements)
    - [Domain Constraints](#domain-constraints)
    - [Security Constraints](#security-constraints)
  - [First Implementation](#first-implementation)
  - [Validation Is A Leaky Cauldron](#validation-is-a-leaky-cauldron)
  - [Type-Driven Development](#type-driven-development)
  - [Ownership Meets Invariants](#ownership-meets-invariants)
    - [AsRef](#asref)
  - [Error As Values - Result](#error-as-values---result)
    - [Converting `parse` To Return `Result`](#converting-parse-to-return-result)
  - [Insightful Assertion Errors: `claims`](#insightful-assertion-errors-claims)
  - [Unit Tests](#unit-tests)
  - [Handling A `Result`](#handling-a-result)
    - [The `?` Operator](#the--operator)
    - [400 Bad Request](#400-bad-request)
  - [The Email Format](#the-email-format)
  - [The `SubscriberEmail` Type](#the-subscriberemail-type)
    - [Breaking The Domain Sub-Module](#breaking-the-domain-sub-module)
    - [Skeleton Of A New Type](#skeleton-of-a-new-type)
  - [Property-based Testing](#property-based-testing)
    - [How To Generate Random Test Data With `fake`](#how-to-generate-random-test-data-with-fake)
    - [Getting Started with `quickcheck`](#getting-started-with-quickcheck)
    - [Implementing the `Arbitrary` Trait](#implementing-the-arbitrary-trait)
  - [Payload Validation](#payload-validation)
    - [Refactoring With `TryForm`](#refactoring-with-tryform)
- [Ch 7 - Reject Invalid Subscribers #2](#ch-7---reject-invalid-subscribers-2)
  - [Confirmation Emails](#confirmation-emails)
    - [Subscriber Consent](#subscriber-consent)
    - [The Confirmation User Journey](#the-confirmation-user-journey)
    - [The Implementation Strategy](#the-implementation-strategy)
  - [`EmailClient`, Our Email Delivery Component](#emailclient-our-email-delivery-component)
    - [How To Send An Email](#how-to-send-an-email)
      - [Choosing An Email API](#choosing-an-email-api)
      - [The Email Client Interface](#the-email-client-interface)
    - [How To Write A REST Client Using reqwest](#how-to-write-a-rest-client-using-reqwest)
      - [`reqwest::Client`](#reqwestclient)
      - [Connection Pooling](#connection-pooling)
      - [How To Reuse The Same reqwest::Client In `actix-web`](#how-to-reuse-the-same-reqwestclient-in-actix-web)
      - [Configuring Our EmailClient](#configuring-our-emailclient)
    - [How To Test A REST Client](#how-to-test-a-rest-client)
      - [`wiremock::MockServer`](#wiremockmockserver)
      - [`wiremock::Mock`](#wiremockmock)
      - [The Intent Of A Test Should Be Clear](#the-intent-of-a-test-should-be-clear)
      - [Mock expectations](#mock-expectations)
    - [First Sketch Of `EmailClient::send_email`](#first-sketch-of-emailclientsend_email)
      - [`reqwest::Client::post`](#reqwestclientpost)
      - [JSON body](#json-body)
    - [Tightening Our Happy Path Test](#tightening-our-happy-path-test)
        - [Headers, Path And Method](#headers-path-and-method)
        - [Body](#body)
      - [Refactoring: Avoid Unnecessary Memory Allocations](#refactoring-avoid-unnecessary-memory-allocations)
    - [Dealing With Failures](#dealing-with-failures)
      - [Error Status Codes](#error-status-codes)
      - [Timeouts](#timeouts)
      - [Refactoring: Test Helpers](#refactoring-test-helpers)
      - [Refactoring: Fail fast](#refactoring-fail-fast)
  - [Skeleton And Principles For A Maintainable Test Suite](#skeleton-and-principles-for-a-maintainable-test-suite)
    - [Why Do We Write Tests?](#why-do-we-write-tests)
    - [Why Don't We Write Tests?](#why-dont-we-write-tests)
    - [Test Code Is Still Code](#test-code-is-still-code)
    - [Our Test Suite](#our-test-suite)
    - [Test Discovery](#test-discovery)
    - [One Test File, One Crate](#one-test-file-one-crate)
    - [Sharing Test Helpers](#sharing-test-helpers)
    - [Sharing Startup Logic](#sharing-startup-logic)
      - [Extracting Our Startup Code](#extracting-our-startup-code)
    - [Build An API Client](#build-an-api-client)
  - [Refocus](#refocus-1)
  - [Zero Downtime Deployments](#zero-downtime-deployments)
    - [Reliability](#reliability)
    - [Deployment Strategies](#deployment-strategies)
      - [Naive Deployment](#naive-deployment)
      - [Load Balancers](#load-balancers)
        - [Horizontal Scaling](#horizontal-scaling)
        - [Health Checks](#health-checks)
      - [Rolling Update Deployments](#rolling-update-deployments)
      - [Digital Ocean App Platform](#digital-ocean-app-platform)
  - [Database Migrations](#database-migrations-1)
    - [Deployments And Migrations](#deployments-and-migrations)
    - [Multi-step Migrations](#multi-step-migrations)
    - [A New Mandatory Column](#a-new-mandatory-column)
      - [Step 1: Add As Optional](#step-1-add-as-optional)
      - [Step 2: Start Using The New Column](#step-2-start-using-the-new-column)
      - [Step 3: Backfill And Mark As `NOT NULL`](#step-3-backfill-and-mark-as-not-null)
    - [A New Table](#a-new-table)
  - [Sending A Confirmation Email](#sending-a-confirmation-email)
    - [A Static Email](#a-static-email)
      - [Red Test](#red-test)
      - [Green Test](#green-test)
    - [A Static Confirmation Link](#a-static-confirmation-link)
      - [Red Test](#red-test-1)
      - [Green Test](#green-test-1)

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
* We will use `actix-web`

## Our First Endpoint: A Basic Health Check
* We would need to add `actix-web` and `tokio` to our list of dependencies with `cargo add <package-name>@<version>`

### Anatomy Of An actix-web Application

#### Server - `HttpServer`
* `HttpServer` is the backbone supporting our application and takes care of *transport level* concerns like:
  1. where should the application be listening for incoming requests? A TCP socket (e.g. `127.0.0.1:8000`)? A Unix domain socket?
  2. what is the maximum number of concurrent connections that we should allow? How many new connections per unit of time?
  3. should we enable transport layer security (TLS)?
  4. etc.

#### Application - `App`
* For things like what does `HttpServer` do when it has established a new connection with a client of our API and we need to start handling their requests, `App` comes into play.
* `App` is where all your application logic lives: routing, middlewares, request handlers, etc.

#### Endpoint - `Route`
* The `route` method lets us add endpoints to our app.
* `route` takes two parameters:
  1. `path`, a string, possibly templated (e.g. "`/{name}`") to accommodate dynamic path segments
  2. `route`, an instance of the `Route` struct.
* In `.route("/", web::get().to(greet))`
  * "/" will match all requests without any segment following the base path
  * `web::get()` passes the request to the handler if and only if its HTTP method is `GET`.
* The handler is `greet`: an asynchronous function that takes an `HttpRequest` as input and returns something that implements something that implements the `Responder` trait.

#### Runtime - `tokio`
* We want `main` to be `async` since `HttpServer::run` is asynchronous, but our binary's entry point can't be asynchronous.
* Our binary's entry point can't be an asynchronous function because:
  * Asynchronous programming in Rust is built on top of the `Future` trait: a future stands for a value that may not be there *yet*.
  * All futures expose a `poll` method which has to be called to allow the future to make progress and eventually resolve to its final value.
* This explains why main cannot be an asynchronous function: who is in charge to call `poll` on it?
* There is no special configuration syntax that tells the Rust compiler that one of your dependencies is an asynchronous runtime.
  * You are therefore expected to launch your asynchronous runtime at the top of your main function and then use it to drive your futures to completion.
* `tokio::main` is a procedural macro that takes in the entire `main` function and outputs a stream of new symbols.
  * We can use `cargo expand` (that relies on the `nightly` compiler) to demystify `#[tokio:main]`.
  * Our `main` function is expanded to:
    ```Rust
      fn main() -> Result<(), std::io::Error> {
        let body = async {
          HttpServer::new(|| {
                  App::new()
                      .route("/", web::get().to(greet))
                      .route("/{name}", web::get().to(greet))
              })
              .bind("127.0.0.1:8000")?
              .run()
              .await
        };

        #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
        {
          return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
        }
      }
    ```
  * The expanded code is synchronous, which explains why it compiles without issues.
    * We are starting tokio’s async runtime and we are using it to drive the future returned by `HttpServer::run` to completion.
    * In other words, the job of `#[tokio::main]` is to give us the illusion of being able to define an asynchronous `main` while, under the hood, it just takes our `main` asynchronous body and writes the necessary boilerplate to make it run on top of tokio’s runtime.

### Implementing The Health Check Handler
* This should be similar to how we implemented the `/` endpoint.

## Our First Integration Test
* As our application gets bigger, it gets more and more expensive to manually test all our endpoints, so we would like to automate this.

### How Do You Test An Endpoint?
* The endpoints we expose in our API define the *contract* between us and our clients: a shared agreement about the inputs and the outputs of the system, its *interface*.
* The contract might evolve over time and we can roughly picture two scenarios: - *backwards-compatible* changes (e.g. adding a new endpoint); - *breaking changes* (e.g. removing an endpoint or dropping a field from the schema of its output).
* We can verify the `health_check` endpoint works with *black box testing*: we verify the behavior of a system by examining its output given a set of inputs without having access to the details of its internal implementation.
  * Just checking that our function `health_check` returns `Ok` is not good enough since that hasn't tested the `/health_check` as the path or the `GET` request.
  * While `actix-web` provides conveniences to interact with an `App` without skipping routing logic, using this approach would force us to rewrite our entire test suite if we migrate to another web framework.
  * So, we will use an HTTP client `reqwest` to launch our application at the beginning of each test and interact with it.

### Where Should I Put My Tests?
* Our options in Rust are:
  1. next to your code in an *embedded test module*
     * An embedded test module is part of your project, just hidden behind a configuration conditional check, `#[cfg(test)]`.
     * It has privileged access to the code living next to it: it can interact with structs, methods, fields and functions that have not been marked as public and would normally not be available to a user of our code if they were to import it as a dependency of their own project.
     * Embedded test modules are useful for *iceberg projects*, i.e. the exposed surface is very limited (e.g. a couple of public functions), but the underlying machinery is much larger and fairly complicated (e.g. tens of routines).
       * It might not be straight-forward to exercise all the edge cases via the exposed functions - you can then leverage embedded test modules to write unit tests for private sub-components to increase confidence in the correctness of the whole project.
  2. in an external `tests` folder
  3. as part of our public documentation (*doc tests*)
     * Tests in the external `tests` folder and `doc` tests have exactly the same level of access to your code that you would get if you were to add your crate as a dependency in another project.
     * They are therefore used mostly for integration testing, i.e. testing your code by calling it in the same exact way a user would.
* We are going to use the `tests` folder for our API integration tests - it is more clearly separated and it is easier to manage test helpers as sub-modules of an external test binary.

### Changing Our Project Structure For Easier Testing
* To be able to share our code so we can test it, we need to convert it from a binary into a library & a binary where the binary will just be entrypoint with a very slim `main` function.
* We can do this by
  * specifying `[lib]` and `[[bin]]` in our Cargo.toml
  * moving code from "src/main.rs" to "src/lib.rs" renaming `main` to `run` to avoid conflicts.
    * We can now remove the `#[tokio::main]` from the `run` function since it is no longer the binary entry point and marking it `pub`
    * Our "src/main.rs" will have an `async main` function that calls `run` and is marked by `#[tokio::main]`. 

## Implementing Our First Integration Test
* We can now add the tests in "tests/health_check.rs"
* We can add the `reqwest` dependency to `[dev-dependencies]`.
* In our test, `spawn_app` is the only piece that will depend on our application code.
* Everything else is entirely decoupled from the underlying implementation details - if tomorrow we decide to ditch Rust and rewrite our application in Ruby on Rails we can still use the same test suite to check for regressions in our new stack as long as `spawn_app` gets replaced with the appropriate trigger (e.g. a bash command to launch the Rails app).
* In our `spawn_app` function when we call and `await` `HttpServer::run`, it returns an instance of `Server`
  * when we call `.await` it starts listening on the address we specified indefinitely: it will handle in-coming requests as they arrive, but it will never shutdown or “complete” on its own.
  * This implies that `spawn_app` never returns and our test logic never gets executed.
  * To fix this, we need to launch our app in the background.
* `tokio::spawn`’s documentation says when a `tokio` runtime is shut
down all tasks spawned on it are dropped.
`tokio::test` spins up a new runtime at the beginning of each test case and they shut down at the end of each test case.

### Polishing

#### Choosing A Random Port
* If we try to run two or more tests in parallel only one of them will manage to bind port `8000`, all others will fail, so tests should run in the background on a random available port.
* We can use `TcpListener` to bind the port on our own and then hand it to `HttpServer` using listen.
* We will use port `0` for the tests, port `0` is special-cased at the OS level: trying to bind port `0` will trigger an OS scan for an available port which will then be bound to the application.

## Refocus
* We expect our blog visitors to input their email address in a form embedded on a web page.
* The form will trigger a `POST /subscriptions` call to our backend API that will actually process the information, store it and send back a response.
* To implement the user story for the blog visitor, we have the following steps to do:
  1. how to read data collected in a HTML form in `actix-web` (i.e. how do I parse the request body of a `POST`?)
  2. what libraries are available to work with a PostgreSQL database in Rust
  3. how to setup and manage migrations for our database
  4. how to get our hands on a database connection in our API request handlers
  5. how to test for side-effects (a.k.a. stored data) in our integration tests
  6. how to avoid weird interactions between tests when working with a database
  
## Working With HTML Forms

### Refining Our Requirements
* We want an email address and a name for all new subscribers.
* Given that the data is collected via a HTML form, it will be passed to our backend API in the body of a `POST` request, where the body is going to be encoded as `application/x-www-form-urlencoded`
  * The keys and values are encoded in key-value tuples separated by `&`, with a `=` between the key and the value. Non-alphanumeric characters in both keys and values are percent encoded.
  * E.g. If the name is Taha Afzal and the email is tahaafzal5@hotmail.com, the `POST` request body would be `name=Taha%20Afzal&email=tahaafzal5%40hotmail.com`
  * Spaces are replaced by `%20` while `@` becomes `%40`.
* If we get both a valid name and email, we should return `200 OK` otherwise `400 BAD REQUEST`.

### Capturing Our Requirements As Tests
* `subscribe_returns_a_400_when_data_is_missing` is an example of *table-driven test* also known as *parametrised test*.
  * Instead of duplicating test logic several times we can simply run the same assertion against a collection of known invalid bodies that we expect to fail in the same way.
  * With "roll-you-own" parametrised tests: as soon as one test case fails, the execution stops and we do not know the outcome for the following test cases.

### Parsing Form Data From A POST Request
* We can add a new `/subscriptions` route.

#### Extractors
* `actix-web`'s extractors are used to tell the framework to extract certain pieces of information from an incoming request.
  * `Path` to get dynamic path segments from a request’s path
  * `Query` for query parameters
  * `Json` to parse a JSON-encoded request body
  * etc
* For our use case, we can use the `Form` extractor. [See](https://actix.rs/docs/extractors/#url-encoded-forms).

#### `Form` And `FormRequest`
* `Form` is nothing more than a wrapper: it is generic over a type `T` which is then used to populate `Form`'s only field.
* `from_request` in `FormRequest` takes as inputs the head of the incoming HTTP request and the bytes of its payload.
  * It then returns `Self`, if the extraction succeeds, or an error type that can be converted into `actix_web::Error`.
  * All arguments in the signature of a route handler must implement the `FormRequest` trait: `actix-web` will invoke `from_request` for each argument and, if the extraction succeeds for all of them, it will then run the actual handler function.
  * If one of the extractions fails, the corresponding error is returned to the caller and the handler is never invoked.
* In `Form`’s `FromRequest` implementation, all the heavy-lifting seems to be happening inside that `UrlEncoded` struct:
  * It transparently handles compressed and uncompressed payloads
  * it deals with the fact that the request body arrives a chunk at a time as a stream of bytes
  * etc
* The key passage, after all those things have been taken care of, is: `serde_urlencoded::from_bytes::<T>(&body).map_err(|_| UrlencodedError::Parse)`
  * `serde_urlencoded` provides (de)serialization support for the `application/x-www-form-urlencoded` data format.
  * `from_bytes` takes as input a contiguous slice of bytes and it deserializes an instance of type `T` from it according to the rules of the URL-encoded format: the keys and values are encoded in key-value tuples separated by `&`, with a `=` between the key and the value; non-alphanumeric characters in both keys and values are percent encoded.
* `#[derive(Serialize)]` and `#[derive(Deserialize)]` procedural macros, bundled with `serde` behind the derive feature flag, will parse the definition of your type and automatically generate for you the right `Serialize/Deserialize` implementation.
  * Generically: once your type implements `Serialize`, you are then free to use any concrete implementation of Serializer to actually perform the serialization step - i.e. you can serialize your type to any format for which there is an available `Serializer` implementation on crates.io. Same is true for `Deserialize` and `Deserializer`.
  * Efficiently: thanks to *monomorphization*, we don't pay any runtime cost for using generics.
  * Conveniently: the presence of `#[derive(Serialize)]` and `#[derive(Deserialize)]`.

* Our `subscribe` handler looks like this:
  * ```Rust
      #[derive(Deserialize)]
      struct FormData {
          name: String,
          email: String,
      }

      async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
        HttpResponse::Ok().finish()
      }
    ```
* All this is going on under the hood:
  * before calling `subscribe`, `actix-web` invokes the `from_request` method for all subscribe’s input arguments: in our case, `Form::from_request`
  * `Form::from_request` tries to deserialize the body into `FormData` according to the rules of URL- encoding leveraging `serde_urlencoded` and the `Deserialize` implementation of `FormData`, automatically generated for us by `#[derive(serde::Deserialize)]`
  * if `Form::from_request` fails, a `400 BAD REQUEST` is returned to the caller. If it succeeds, `subscribe` is invoked and we return a `200 OK`.

## Storing Data: Databases
* Our application needs to be **distributed** - there should be multiple instances of it running on multiple machines in order to survive hardware failures.
* This has consequences when it comes to data persistence: we cannot rely on the filesystem of our host as a storage layer for incoming data.
* Anything that we save on disk would only be available to one of the many replicas of our application. Furthermore, it would probably disappear if the underlying host crashed.
* This explains why Cloud-native applications are usually **stateless**: their persistence needs are delegated to specialized external systems - databases.

### Choosing A Database
* There are so many options: NoSQL (MongoDB), key-value stores (AWS DynamoDB), graph databases (Neo4j), databases that use RAM for storage (Redis), etc.
* Relational databases are good as jack-of-all trades and are often good choices when building the first version of your application.
* This is why we will use `PostgreSQL`: a battle-tested piece of technology, widely supported across all cloud providers if you need a managed offering, open-source, exhaustive documentation, easy to run locally and in CI via Docker, well-supported within the Rust ecosystem.

### Choosing A Database Crate
* `tokio-postgres`, `sqlx`, and `diesel` are popular choices as of August 2020.
* To pick one we should look at:
  * compile-time safety
  * SQL-first vs DSL for query building
  * async vs sync interface

#### Compile-time safety
* Errors like a typo in the name of a column or a table mentioned in our query; performing operations that are rejected by the database engine (e.g.summing a string and a number) or joining two tables on the wrong column); and expecting to have a certain field in the returned data that is actually not there; will be caught at **runtime** in `tokio-postgres`.
* `diesel` and `sqlx` try to speed up the feedback cycle by detecting at **compile-time** most of these mistakes.
  * `diesel` leverages its CLI to generate a representation of the database schema as Rust code, which is then used to check assumptions on all of your queries.
  * `sqlx` uses procedural macros to connect to a database at compile-time and check if the provided query is valid.

#### Query Interface
* Both `tokio-postgres` and `sqlx` expect you to use SQL directly to write your queries.
* `diesel` provides its own query builder: queries are represented as Rust types and you add filters, perform joins and similar operations by calling methods on them. This is often referred to with the name of **Domain Specific Language (DSL)**.

#### Async Support
* Your database is not sitting next to your application on the same physical machine host: to run queries you have to perform network calls.
* An asynchronous database driver will not reduce how long it takes to process a single query, but it will enable your application to leverage all CPU cores to perform other meaningful work (e.g. serve another HTTP request) while waiting for the database to return results.
* Both `sqlx` and `tokio-postgres` provide an asynchronous interface, while `diesel` is synchronous.
  
#### Our Pick: `sqlx`
* We will use `sqlx`: its asynchronous support simplifies the integration with `actix-web` without forcing us to compromise on compile-time guarantees.
* It also limits the API surface that we have to cover and become proficient with thanks to its usage of raw SQL for queries.

### Integration Testing With Side-effects
* In `subscribe_returns_a_200_for_valid_form_data`, just asserting that the response is `200 OK` is not enough, we need to confirm that our side-effect (data storage) has taken place.
* We have 2 options:
  1. leverage another endpoint of our public API to inspect the application state
  2. query directly the database in our test case
* Option 1 should be our go-to when possible so that our tests remain oblivious to the implementation details of the API (e.g. the underlying database technology and its schema) and are less likely to be disrupted by future refactoring.
  * Unfortunately we do not have any public endpoint on our API that allows us to verify if a subscriber exists.
  * We could add a `GET /subscriptions` endpoint to fetch the list of existing subscribers, but we would then have to worry about securing it: we do not want to have the names and emails of our subscribers exposed on the public internet without any form of authentication.
* So, we will go for Option 2 for now.

### Database Setup
* To run queries in our test suite we need:
  1. a running Postgres instance
  2. a table to store our subscribers data
* We will use the same database both for our tests and the production environment instead of "in-memory test database" to avoid issues due to differences between the in-memory stub and the real database engine.

#### Docker
* To run Postgres we will use Docker - before launching our test suite we will launch a new Docker container using Postgres’ official Docker image.
* We have a `scrips/init_db.sh` to launch Postgres with custom settings in a Docker container.

#### Database Migrations
* To store our subscribers details, we need to create our first table.
* To add a new table to our database, we need to change its schema - commonly known as *database migration*

##### sqlx-cli
* `sqlx` provides a cli to manage database migrations
* `cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres`

##### Database Creation
* The first command we will usually want to run is `sqlx database create`, but because we launched our Postgres Docker instance with environment variables, our Postgres Docker instance already comes with a default database named `newsletter`.
* We would have to run this command in the CI and in our production environment, however.

##### Adding A Migration
* Running `sqlx migrate add create_subscriptions_table` creates a `migration` directory in our project to store all our migrations.
* We add the SQL code for our first migration to create the `subscriptions` table in the file in that directory named `{timestamp}_create_subscriptions_table.sql`.
* Database constraints are useful as a last line of defence from application bugs but they come at a cost - the database has to ensure all checks pass before writing new data into the table.
* Therefore constraints impact our write-throughput.
* We can then migrations by `sqlx migrate run`.
* Looking at the database using a Postgres GUI, we can see a `subscriptions` table alongside `_sqlx_migrations table` where `sqlx` keeps track of what migrations have been run against our database - it should contain a single row now for our first migration.
  * We can use `psql` to connect to the PostgreSQL server from outside the Docker container if you have set up port forwarding like so: `psql -h localhost -p 5432 -U postgres`.
  * We can connect to a specific database with `\c newsletter` and run SQL to look at the contents of tables.
  
### Writing Our First Query

#### Sqlx Feature Flags
* We have a migrated database up and running and need to add `sqlx-cli` as a dependency to our application's Cargo.toml so we can talk to it.

#### Configuration Management
* The simplest entry-point to connect to a Postgres database is `PgConnection`.
* `PgConnection` implements the `Connection` trait which provides us with a `connect` method: it takes as input a connection string and returns us, asynchronously, a `Result<PostgresConnection. sqlx::Error>`.
* Where do we get a connection string?
  * We could hard-code one in our application and then use it for our tests as well.
  * Or we could choose to introduce immediately some basic mechanism of configuration management.
* We will use a configuration management.

##### Reading A Configuration File
* To manage configuration with `config` we must represent our application settings as a Rust type that implements `serde`’s `Deserialize` trait.
* We can do this through a `Settings` struct to configure
  * `port` where `actix-web` is listening for incoming requests
  * database connection parameters
* We will read the configuration from "configuration.yaml"

#### Connecting To Postgres
* We can use a convenience function `connection_string` to get the connection string from `DatabaseSettings` to pass to `PgConnection::connect`.

#### Our Test Assertion
* We can write sql queries in `sqlx::query!`.
* The `query!` macro returns an anonymous record type: a struct definition is generated at compile-time after having verified that the query is valid, with a member for each column on the result .
* `sqlx` reaches out to Postgres at compile-time to check that queries are well-formed.
* Just like `sqlx-cli` commands, it relies on the `DATABASE_URL` environment variable to know where to find the database, so we will add it to a `.env` file in our project.
* It feels a bit dirty to have the database connection parameters in two places (`.env` and `configuration.yaml`), but it is not a major problem:
  * `configuration.yaml` can be used to alter the runtime behavior of the application after it has been compiled, while `.env` is only relevant for our development process, build and test steps.

#### Updating Our CI Pipeline
* Now, tests, linting, and coverage in the CI also needs a up-and-running database, so we need to update those jobs in the CI with that.

## Persisting A New Subscriber
* Just as we wrote a `SELECT` query to inspect what subscriptions had been persisted to the database in our test, we now need to write an `INSERT` query to actually store the details of a new subscriber when we receive a valid `POST /subscriptions` request.
* To execute a query within `subscribe` we need to get our hands on a database connection.

### Application State In `actix-web`
* Our application has been entirely stateless: our handlers work solely with the data from the incoming request.
* `actix-web` gives us the possibility to attach to the application other pieces of data that are not related to the lifecycle of a single incoming request - the so-called *application state* and we can do that using the `app_data` method.
* `HttpServer` expects `PgConnection` to be cloneable, which unfortunately is not the case.

### `actix-web` Workers
* `HttpServer::new` does not take `App` as argument - it wants a closure that returns an `App` struct.
* This is to support `actix-web`’s runtime model:` actix-web` will spin up a worker process for each available core on your machine.
* Each worker runs its own copy of the application built by `HttpServer` calling the very same closure that `HttpServer::new` takes as argument.
* That is why `connection` has to be cloneable - we need to have one for every copy of `App`.
* `PgConnection` does not implement `Clone` because it sits on top of a non-cloneable system resource, a TCP connection with Postgres.
* But we can use another `actix-web` extractor, `web::Data`, to wrap our connection in an **A**tomic **R**eference **C**ounted pointed, an Arc: each instance of the application, instead of getting a raw copy of a `PgConnection`, will get a pointer to one.
  * `Arc<T>` is always cloneable: cloning an Arc increments the number of active references and hands over a new copy of the memory address of the wrapped value.

### The `Data` Extractor
* We can now use `Arc<PgConnection>` in our request handler, `subscribe`, using the `web::Data` extractor.
* But what is it extracting a `PgConnection` from?
  * `actix-web` uses a type-map to represent its application state: a `HashMap` that stores arbitrary data (using the `Any` type) against their unique type identifier (obtained via `TypeId::of`).
  * `web::Data`, when a new request comes in, computes the `TypeId` of the type you specified in the signature (in our case `PgConnection`) and checks if there is a record corresponding to it in the type-map.
  * If there is one, it casts the retrieved `Any` value to the type you specified and passes it to your handler.
  * It is a technique referred to as dependency injection.

### The `INSERT` Query
* We can bind dynamic data to our `INSERT` query using `$1` to refer to the first argument passed to the `query!` after the query itself, and so on.
* `execute` wants an argument that implements `sqlx`'s `Executor` trait, but only `&mut PgConnection` implements it, not `&PgConnection`.
* Why?
  * `sqlx` has an asynchronous interface, but it does not allow you to run multiple queries concurrently over the same database connection.
  * Requiring a mutable reference allows them to enforce this guarantee in their API, since we cannot have more than one active mutable references to the same value at the same time.
* `web::Data` will never give us mutable access to the application state.
  * We could leverage interior mutability - e.g. putting our `PgConnection` behind a lock (e.g. a `Mutex`) would allow us to synchronize access to the underlying TCP socket and get a mutable reference to the wrapped connection once the lock has been acquired, but this way we could only run one query at a time.
* A shared reference to `PgPool` implements `sqlx`'s `Executor` trait as well and is a pool of connections to a Postgres database.
* But how does it bypass the concurrency issue discussed above for `PgConnection`?
  * There is still interior mutability at play, but of a different kind: when you run a query against a `&PgPool`, `sqlx` will borrow a `PgConnection` from the pool and use it to execute the query; if no connection is available, it will create a new one or wait until one frees up.
  * This increases the number of concurrent queries that our application can run and it also improves its resiliency: a single slow query will not impact the performance of *all* incoming requests by creating contention on the connection lock.
* We will update the relevant functions in the app and the tests.

### Test Isolation
* Running the tests more than once causes the `subscribe_returns_a_200_for_valid_form_data` test to fail: "Failed to execute query: error returned from database: duplicate key value violates unique constraint "subscriptions_email_key""
  * Since our database is a global variable, running the test again to insert the same email address in the database causes the `INSERT` query to fail since `email_address` is supposed to be unique.
* We can ensure test isolation in 2 ways:
  1. wrap the whole test in a SQL transaction and rollback at the end of it
  2. spin up a brand-new logical database for each integration test
* The first is clever and rolling back a SQL transaction takes less time than spinning up a new logical database.
  * It works quite well when writing unit tests for your queries but it is tricky to pull off in an integration test like ours: our application will borrow a `PgConnection` from a `PgPool` and we have no way to “capture” that connection in a SQL transaction context.
* The second option: potentially slower, yet much easier to implement. Before each test run, we want to:
  * create a new logical database with a unique name
  * run database migrations on it
* We will go with option 2 and the best way to do this would be in `spawn_app` before launching our `actix-web` test application.

# Ch4 - Telemetry
  
## Unknown Unknowns
* A test suite is not proof of the correctness of our application.
* We would have to explore significantly different approaches to prove that something is correct.
* A few blind spots we can encounter are:
  * what happens if we lose connection to the database? Does `sqlx::PgPool` try to automatically recover or will all database interactions fail from that point onwards until we restart the application?
  * what happens if an attacker tries to pass malicious payloads in the body of the `POST/subscriptions` request (i.e. extremely large payloads, SQL injection, etc.)
* Unknown unknowns might emerge when:
  * the system is pushed outside of its usual operating conditions (e.g.an unusual spike of traffic)
  * multiple components experience failures at the same time (e.g.a SQL transaction is left hanging while the database is going through a master-replica failover
  * a change is introduced that moves the system equilibrium (e.g. tuning a retry policy)
  * no changes have been introduced for a long time (e.g. applications have not been restarted for weeks and you start to see memory leaks);
* These scenarios might be hard/impossible to reproduce outside of a live environment.

## Observability
* One of the things we can rely on to understand and debug an unknown unknown is **telemetry data**: information about our running applications that is collected automatically and can be later inspected to answer questions about the state of the system at a certain point in time.
  > Observability is about being able to ask arbitrary questions about your environment without having to know ahead of time what you wanted to ask.

## Logging
* A **log record** is usually a bunch of text data, with a line break to separate the current record from the next one.

### The `log` crate
* The go-to crate for logging in Rust is `log`.
  * `log` provides five macros: `trace`, `debug`, `info`, `warn` and `error`.
* *trace* is the lowest level: they are are often extremely verbose and have a low signal-to-noise ratio (e.g. emit a trace-level log record every time a TCP packet is received by a web server).
* *Error* is the highest level and us used to report serious failures that might have user impact (e.g. we failed to handle an incoming request or a query to the database timed out).
* We can use `log`’s macros to *instrument* our codebase.

### `actix-web`'s `Logger` Middleware
* `actix_web` provides a `Logger` middleware. It emits a log record for every incoming request.

### The Facade Pattern
* The `log` crate leverages the facade pattern to handle questions like what the application should do with the log records?
  * Append them to a file?
  * Print them to the terminal?
  * Send them to a remote system over HTTP (e.g. ElasticSearch)?
* `log` gives you the tools you need to emit log records, but it does not *prescribe* how those log records should be processed. 
* At the beginning of your main function you can call the `set_logger` function and pass an implementation of the `Log` trait: every time a log record is emitted `Log::log` will be called on the `logger` you provided, therefore making it possible to perform whatever form of processing of log records you deem necessary.
* If you do not call `set_logger`, then all log records will simply be discarded.
  * `init` calls `set_logger`.
* We will use `env_logger` crate, which prints log records to the terminal in the following format: `[<timestamp> <level> <module path>] <log message>`.
* It looks at the `RUST_LOG` environment variable to determine what logs should be printed and what logs should be filtered out.
  * `RUST_LOG=debug cargo run`will surface all logs at debug-level or higher emitted by our application or the crates we are using.
  * `RUST_LOG=zero2prod`, instead, would filter out all records emitted by our dependencies.

## Instrumenting POST /subscriptions
* We will add the `log` crate as a dependency.

### Interactions With External Systems
* A tried-and-tested rule of thumb is: any interaction with external systems over the network should be closely monitored.
* We might experience networking issues, the database might be unavailable, queries might get slower over time as the subscribers table gets longer, etc.

### Think Like A User
* We should capture what we are trying to do so we can look up the user's information in the logs to debug issues. We add a `log::info!` to our `subscribe` function.

### Logs Must Be Easy to Correlate
* If we are handling multiple requests concurrently, our current logging might not be able to correspond to a user's actions as there might be multiple users interacting with our server at the same time.
* We add a `request_id` to the logs and can see them using `curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions`.
* We can then grab the `request_id` for Tom and search for it, but `request_id` is created in our `subscribe` handler, therefore `actix_web`’s Logger middleware is completely unaware of it.
  * That means that we will not know what status code our application has returned to the user when they tried to subscribe to our newsletter.
  
## Structured Logging
* To ensure that `request_id` is included in all log records we would have to:
  * rewrite all upstream components in the request processing pipeline (e.g. `actix-web`’s Logger);
  * change the signature of all downstream functions we are calling from the `subscribe` handler; if they are emitting a log statement, they need to include the `request_id`, which therefore needs to be passed down as an argument.
* What about log records emitted by the crates we are importing into our project? Should we rewrite those as well? It is clear that this approach cannot scale because `log` is the wrong abstraction.

### The `tracing` crate
> `tracing` expands upon logging-style diagnostics by allowing libraries and applications to record structured events with additional information about temporality and causality — unlike a log message, a span in tracing has a beginning and end time, may be entered and exited by the flow of execution, and may exist within a nested tree of similar spans.

### Migration From `log` to `tracing`
* `tracing`'s `log` feature flag  ensures that every time an event or a span are created using `tracing`’s macros a corresponding `log` event is emitted, allowing `log`’s loggers to pick up on it (`env_logger`, in our case).
  
### `tracing`'s Span
* We can now start to leverage `tracing`’s `Span` to better capture the structure of our program. We want to create a span that represents the whole HTTP request.
* We use the `info_span!` macro to create a new span and attach some values to its context: `request_id`, `form.email` and `form.name`.
* We are not using string interpolation anymore: `tracing` allows us to associate structured information to our spans as a collection of key-value pairs.
  * We can explicitly name them (e.g. `subscriber_email` for `form.email`) or implicitly use the variable name as key (e.g. the isolated `%request_id` is equivalent to `request_id = request_id`).
  * We prefixed all of them with `%`: we are telling tracing to use their `Display` implementation for logging purposes.
* `info_span` returns the newly created span, but we have to explicitly step into it using the `.enter()` method to activate it.
* `.enter()` returns an instance of `Entered`, a *guard*: as long the guard variable is not dropped all downstream spans and log events will be registered as children of the entered span.
* This is a typical Rust pattern, often referred to as Resource Acquisition Is Initialization (**RAII**): the compiler keeps track of the lifetime of all variables and when they go out of scope it inserts a call to their destructor, `Drop::drop`.
* We can closely follow the lifetime of our span using the emitted logs:
  * "Adding a new subscriber" is logged when the span is created
  * We enter the span `->`
  * We execute the INSERT query
  * We exit the span `<-`
  * We finally close the span `--`
* You can enter (and exit) a span multiple times.
* Closing a span, instead, is final: it happens when the span itself is dropped.
* This comes pretty handy when you have a unit of work that can be paused and then resumed e.g. an asynchronous task.

### Instrumenting Futures
* Looking at our database query as an example, the executor might have to poll its future more than once to drive it to completion - while that future is idle, we are going to make progress on other futures.
* This can cause issues: how do we make sure we don’t mix their respective spans?
  * The best way would be to closely mimic the future’s lifecycle: we should enter into the span associated to our future every time it is polled by the executor and exit every time it gets parked.
* That’s where `Instrument` comes into the picture.
  * It is an extension trait for futures.
  * `Instrument::instrument` enters the span we pass as argument every time self, the future, is polled; it exits the span every time the future is parked.
* Now, if we launch the application with `RUST_LOG=TRACE cargo run` and try a `POST /subscriptions` request, we will see how many times the query future has been polled by the executor before completing.

### `tracing`'s Subscriber
* But the above only prints the `request_id` on the very first log where we attach it explicitly to the span context.
  * This is because we are still using `env_logger` to process everything.
  * `env_logger`'s logger implements `log`'s `Log` trait and it knows nothing about `tracing`'s `Span`.
* The `tracing` crate follows the same facade pattern used by `log` - you can freely use its macros to instrument your code, but applications are in charge to spell out how that span telemetry data should be processed.
* `Subscriber` is the tracing counterpart of `log`’s `Log`: an implementation of the `Subscriber` trait exposes a variety of methods to manage every stage of the lifecycle of a `Span` - creation, enter/exit, closure, etc.

### `tracing-subscriber`
* `tracing` does not provide any subscriber out of the box.
* We need to look into `tracing-subscriber`, another crate maintained in-tree by the `tracing` project, to find a few basic subscribers to get off the ground, which we can do by adding it as a dependency.
* `tracing-subscriber` does much more than providing us with a few handy subscribers.
* It introduces another key trait into the picture, `Layer`.
  * `Layer` makes it possible to build a processing pipeline for spans data: we are not forced to provide an all-encompassing subscriber that does everything we want; we can instead combine multiple smaller layers to obtain the processing pipeline we need.
  * This substantially reduces duplication across in `tracing` ecosystem: people are focused on adding new capabilities by churning out new layers rather than trying to build the best-possible-batteries-included subscriber.
* The cornerstone of the layering approach is `Registry`, which implements the `Subscriber` trait and takes care of all the difficult stuff:
  > `Registry` does not actually record traces itself: instead, it collects and stores span data that is exposed to any layer wrapping it [...]. The `Registry` is responsible for storing span metadata, recording relationships between spans, and tracking which spans are active and which are closed.
* Downstream layers can piggyback on `Registry`’s functionality and focus on their purpose: filtering what spans should be processed, formatting span data, shipping span data to remote systems, etc.

### `tracing-bunyan-formatter`
* Everything we attached to the original context has been propagated to all its sub-spans.
* `tracing-bunyan-formatter` also provides duration out-of-the-box: every time a span is closed a JSON message is printed to the console with an `elapsed_millisecond` property attached to it.
* The JSON format is extremely friendly when it comes to searching: an engine like ElasticSearch can easily ingest all these records, infer a schema and index the `request_id`, `name` and `email` fields. It unlocks the full power of a querying engine to sift through our logs!
* This is exponentially better than we had before: to perform complex searches we would have had to use custom-built regexes, therefore limiting considerably the range of questions that we could easily ask to our logs.

### `tracing-log`
* `tracing`’s `log` feature flag ensures that a log record is emitted every time a tracing event happens, allowing `log`’s loggers to pick them up.
* This enables us to get `actix-web`'s log records as well.

### Removing Unused Dependencies
* `cargo install cargo-udeps` scans your Cargo.toml file and checks if all the crates listed under `[dependencies]` have actually been used in the project.
* It needs to run with the nightly compiler: `cargo +nightly udeps`.

### Logs For Integration Tests
* As a rule of thumb, everything we use in our application should be reflected in our integration tests.
* Structured logging, in particular, can significantly speed up our debugging when an integration test fails:
  * we might not have to attach a debugger, more often than not the logs can tell us where something went wrong.
  * It is also a good benchmark: if you cannot debug it from logs, imagine how difficult would it be to debug in production.
* We will initialize the `subscriber` in the same way for our tests, but `init_subscriber` should only be called once. Calling it in `spawn_app` would call it for each test, so we will use the `once_cell` crate dependency to rectify it.
* To make sure tests don't print the logs each time to the console, we need something similar to `cargo test`'s `--nocapture` option that opts us in to look at `println` statements.
  * We will use `Sink`.

### Cleaning Up Instrumentation Code - tracing::instrument
* Logging has added some noise to our `subscribe` function.
* Extracting each sub-task in its own function is a common way to structure routines to improve readability and make it easier to write tests; therefore we will often want to attach a span to a function declaration.
* `tracing` caters for this specific use-case with its `tracing::instrument` procedural macro.
* `#[tracing::instrument]` creates a span at the beginning of the function invocation and automatically attaches all arguments passed to the function to the context of the span - in our case, `form` and `connection_pool`.
* Often function arguments won’t be displayable on log records (e.g. `connection_pool`) or we’d like to specify more explicitly what should/how they should be captured (e.g. naming each field of `form`) - we can explicitly tell tracing to ignore them using the `skip` directive.
* `name` can be used to specify the message associated to the function span - if omitted, it defaults to the function name.
* We can also enrich the span’s context using the `fields` directive.
  * It uses the same syntax as the `info_span!` macro.
* The result is quite nice: all instrumentation concerns are visually separated by execution concerns - the first are dealt with in a procedural macro that “decorates” the function declaration, while the function body focuses on the actual business logic.

### Protect Your Secrets - `secrecy`
* `#[tracing::instrument]` automatically attaches all arguments passed to the function to the context of the span - you have to **opt-out** of logging function inputs (via `skip`) rather than **opt-in**.
* Opt-out is a dangerous default since each time we add a new function parameter, we need to make sure that we update the `skip`.
* We can use a wrapper type `secrecy::Secret` to explicitly mark which fields are considered to be sensitive.
* We can wrap our database password in a secret so it outputs `Secret([REDACTED String])` to mask its `Debug` implementation `println!("{:?}, db_password)`.
* `Secret` does not implement `Display` so we need to manually expose the secret with the `expose_secret()` method.

### Request Id
* We need to ensure that all logs for a particular request, in particular the record with the returned status code, are enriched with a `request_id` property.
* To avoid touching `actix_web::Logger`, we can add another middleware, `RequestIdMiddleware` that:
  1. generates a unique request identifier
  2. creates a new span with the request identifier attached as context
  3. wraps the rest of the middleware chain in the newly created span
* Since our `subscribe` function generates a new `request_id` for the span, we have to remove it so we don't get 2 `request_id`s in our log messages for the same request.

### Leveraging The `tracing` Ecosystem
* `tracing` is a foundational crate in the Rust ecosystem.
* While `log` is the minimum common denominator, `tracing` is the modern backbone of the whole diagnostics and instrumentation ecosystem.
* It can do more things like:
  1. `tracing-actix-web` is OpenTelemetry-compatible. If you plug-in `tracing-opentelemetry` you can ship spans to an OpenTelemetry-compatible service (e.g. Jaeger or Honeycomb.io) for further analysis.
  2. `tracing-error` enriches our error types with a `SpanTrace` to ease troubleshooting.

# Ch 5 - Going Live
* We will package our Rust application as a Docker container to deploy it on DigitalOcean.

## We Must Talk About Deployments
* It is difficult to talk about database schema migrations, domain validation and API evolution without taking into account your deployment process.

## Choosing Our Tools

### Virtualization: Docker
* The fundamental idea behind virtualization technology: what if, instead of shipping code to production, you could ship a self-contained environment that included your application?
* It is not enough to copy the source code to our production servers. Our software is likely to make assumptions on the capabilities exposed by the underlying operating system (e.g. a native Windows application will not run on Linux), on the availability of other software on the same machine (e.g. a certain version of the Python interpreter) or on its configuration (e.g. do I have root permissions?).
* Even if we started with two identical environments we would, over time, run into troubles as versions drift and subtle inconsistencies come up.
* The easiest way to ensure that our software runs correctly is to tightly control the environment it is being executed into.

### Hosting: DigitalOcean
* The options are AWS, Google Cloud, Azure, Digital Ocean, Clever Cloud, Heroku, Qovery...
* For the intersection of a good developer experience and minimal unnecessary complexity, we will use Digital Ocean.

## A Dockerfile For Our Application
* Our first task is to write a Docker file to build and execute our application as a Docker container.
 
### Dockerfiles
* A Dockerfile is a recipe for your application environment.
* They are organized in layers: you start from a base image (usually an OS enriched with a programming language toolchain) and execute a series of commands (`COPY`, `RUN`, etc.), one after the other, to build the environment you need.
* We added our recipe in the `Dockerfile` that we can build using `docker build --tag zero2prod --file Dockerfile .`
* We added `--tag` to our build command so that we can refer to that image in other Docker commands: like to run it `docker run zero2prod`.

### Build Context
* What does the `.` at the end of the last command stand for?
* `docker build` generates an image starting from a recipe (the Dockerfile) and a *build context*.
* The only point of contact between the image and your local machine are commands like `COPY` and `ADD`: the build context determines what files on your host machine are visible inside the Docker container to `COPY` etc.
* Using `.` tells Docker to use the current directory as the build context for this image; `COPY . app` will therefore copy all files from the current directory (including the source code) into the app directory of the Docker image.
* Using `.` as build context implies, for example, that Docker will not allow `COPY` to see files from the parent directory or from arbitrary paths on your machine into the image.
* We can use a different path or even a URL as build context depending on our needs.

### Sqlx Offline Mode
* The above build command won't work:
  * `sqlx` calls into our database at compile-time to ensure that all queries can be successfully executed considering the schemas of our tables.
  * When running `cargo build` inside our Docker image, though, `sqlx` fails to establish a connection with the database that the `DATABASE_URL` environment variable in the `.env` file points to.
* We could allow our image to talk to the database running on our local machine at build time using the `--network` flag,  but because how Docker networking is implemented on different OSs it would compromise how reproducible our builds are.
* A better option is to use `sqlx` **offline mode**.
* `sqlx prepare` performs the same work that is usually done when `cargo build` is invoked but it saves the outcome of those queries into a directory (`.sqlx`) which can later be detected by `sqlx` itself and used to skip the queries altogether and perform an offline build.
* We check the query data in `.sqlx` into version control.
* We can then set the `SQLX_OFFLINE` environment variable to `true` in our Dockerfile to force `sqlx` to look at the saved metadata instead of trying to query a live database.
* To ensure that the queries in `.sqlx` do not go out of sync (e.g. when the schema of our database changes or when we add new queries), we can use the `--check` flag in our CI pipeline to ensure that it stays up-to-date.

### Running An Image
* `docker run zero2prod` triggers the execution of the command we specified in our `ENTRYPOINT` statement.
* Running our image will fail:
  * ```
      thread 'main' panicked at
        'Failed to connect to Postgres:
        Io(Os {
          code: 99,
          kind: AddrNotAvailable,
          message: "Cannot assign requested address"
      })'
    ```
* We can relax our requirements by using `connect_lazy` instead of `connect` so it will only try to establish a connection when the pool is used for the first time.
* With this change, we can run our image but making a request to our health check endpoint, for example, fails: `curl: (7) Failed to connect to 127.0.0.1 port 8000 after 1 ms: Couldn't connect to server`.

### Networking
* The above error is because, by default, Docker images don't expose their ports to the underlying host machine. We need to do it explicitly with the `-p` flag.
* `docker run -p 8000:8000 zero2prod`
* We not get this error: `curl: (56) Recv failure: Connection reset by peer`
* Looking at our `main.rs`, we are using `127.0.0.1` as our host in address - we are instructing our application to only accept connections coming from the same machine.
* A GET request to `/health_check` from the host machine, which is not seen as local by our Docker image, therefore triggering the error.
* We need to use `0.0.0.0` as host to instruct our application to accept connections from any network interface, not just the local one.
* But using `0.0.0.0` significantly increases the “audience” of our application, with some security implications.
* The best way is to make the host portion of our address configurable - keep using `127.0.0.1` for our local development and set it to `0.0.0.0` in our Docker images.

### Hierarchical Configuration
* We add `ApplicationSettings` struct to our `Settings` struct and make adjustments to the configuration.yaml to read the `host` from.
* To use a different value for different environments, we need to make our configuration hierarchical.
* We will make adjustments to our `get_configuration` function to have a more refined approach. We will have:
  1. A base configuration file, for values that are shared across our local and production environment (e.g. database name);
  2. A collection of environment-specific configuration files, specifying values for fields that require customization on a per-environment basis (e.g. `host`)
  3. An environment variable, `APP_ENVIRONMENT`, to determine the running environment (e.g. `production` or `local`).
* We will store all these configurations in a top-level configurations directory.

### Optimizing Our Docker Image
* There are two optimisations we can make to our Dockerfile to make our life easier going forward:
  1. smaller image size for faster usage
  2. Docker layer caching for faster builds

#### Docker Image Size
* We can reduce the size of the Docker build context by excluding files that are not needed to build our image.
* These files will be added to `.dockerignore` and are not sent by Docker as part of the build context to the image, which means they will not be in scope for `COPY` instructions.
* The next optimization is due to one of Rust's strengths.
  * Rust's binaries are statically linked - we don't need to keep the source code or intermediate compilation artifacts around to run the binary since it is self-contained.
  * So, we can use Docker's multi-stage builds feature to split our build into 2 stages:
    1. a `builder` stage to generate a compiled binary
    2. a `runtime` stage to run the binary
* The `builder` stage doesn't contribute to the size - it is an intermediate step and is discraded at the end of the build. The only piece of the `builder` stage that is found in the final artifact is what we explicitly copy over.
* `runtime` is our final image.
* This brings our image from 5.5 GB to 1.75 GB, but we can go even smaller by using a smaller image for rust with `-slim`: down to 1.09 GB
* We can go even smaller by shaving off the whole Rust toolchain and machinery (`rustc`, `cargo`, etc.) since none of that is needed to run our binary.
* We can use the bare operating system as the base image (`debian:bookwork-slim`) for our runtime stage. Down to 119 MB!

#### Caching For Rust Docker Builds
* Each `RUN`, `COPY` and `ADD` instruction in a Dockerfile creates a layer: a diff between the previous state (the layer above) and the current state after having executed the specified command.
* Layers are cached: if the starting point of an operation has not changed (e.g. the base image) and the command itself has not changed (e.g. the checksum of the files copied by `COPY`) Docker does not perform any computation and just retrieves a copy of the result from the local cache.
* Docker layer caching is fast and can be leveraged to massively speed up Docker builds.
  * The trick is optimising the order of operations in your Dockerfile: anything that refers to files that are changing often (e.g. source code) should appear as late as possible, therefore maximizing the likelihood of the previous step being unchanged and allowing Docker to retrieve the result from the cache.
  * The expensive step is usually compilation.
  * Most programming languages follow the same playbook: you `COPY` a lock-file of some kind first, build your dependencies, `COPY` over the rest of your source code and then build your project.
  * This guarantees that most of the work is cached as long as your dependency tree does not change between one build and the next.
* `cargo`, unfortunately, does not provide a mechanism to build your project dependencies starting from its `Cargo.lock` file, but we can rely on `cargo-chef` to expand cargo’s default capability.
* We are using three stages: the first computes the recipe file, the second caches our dependencies and then builds our binary, the third is our runtime environment.
  * As long as our dependencies do not change the `recipe.json` file will stay the same, therefore the outcome of `cargo chef cook --release --recipe-path recipe.json` will be cached, massively speeding up our builds.
* We are taking advantage of how Docker layer caching interacts with multi-stage builds: the `COPY . .` statement in the planner stage will invalidate the cache for the planner container, but it will not invalidate the cache for the builder container as long as the checksum of the `recipe.json` returned by `cargo chef prepare` does not change.
* You can think of each stage as its own Docker image with its own caching - they only interact with each other when using the `COPY --from` statement.

## Deploy To DigitalOcean

### Setup
* We setup our accoung and install `dotcl`.

### App Specification
* Digital Ocean’s App Platform uses a declarative configuration file called App Spec to let us specify what our application deployment should look like.
* We will put the App Spec in the repo's root `spec.yaml` and create the app for the first time with `doctl apps create --spec spec.yaml`.
* The `/POST` endpoint will still fail since we don't have a live database backing up our application in our production environment.
* We will add one in the `spec.yaml` file and update our app: `doctl apps update <APP-ID> --spec=spec.yaml`
  
### How To Inject Secrets Using Environment Variables
* The connection string will contain values that we do not want to commit to version control - e.g. the username and the password of our database root user.
* The best option is to use environment variables as a way to inject secrets at runtime into the application environment.
* DigitalOcean’s apps, can refer to the `DATABASE_URL` environment variable to get the database connection string at runtime.
* We need to upgrade our `get_configuration` function to fulfill our new requirements.
* The changes to add in settings from environment variables (with a prefix of APP and `__` as separator) allow us to customize any value in our `Settings` struct using environment variables, overriding what is specified in our configuration files.
* This makes it possible to inject values that are too dynamic, not known prior, or too sensitive to be stored in version control.
* We can also change the behavior of our application faster without a full re-build if we want to tune the database port, for example.
* But, environment variables are strings for the `config` crate and it will fail to pick up integers if using the standard deserialization routine from `serde`. So we need to specify a custom deserialization function via a new dependency `serde-aux`.
* We then need to add ` #[serde(deserialize_with = "deserialize_number_from_string")]` to the `u16`s in `ApplicationSettings` and `DatabaseSettings`.

### Connecting To Digital Ocean’s Postgres Instance
* Looking at the connection string of our database on DigitalOcean's dashboard, we see that it isn't using SSL mode.
* While not relevant for local development, we should have transport-level encryption for our client/database communication in production.
* We will add a `require_ssl` field in `DatabaseSettings` and update our configuration files to default it to `false` for local and to `true` for production.

### Environment Variables In The App Spec
* We need to amend our `spec.yaml` manifest to inject the environment variables we need.
* The scope is set to `RUN_TIME` to distinguish between environment variables needed during our Docker build process and those needed when the Docker image is launched.
* We are populating the values of the environment variables by interpolating what is exposed by the Digital Ocean’s platform (e.g. `${newsletter.PORT}`) 

### One Last Push
* We apply the new spec by `doctl apps update YOUR-APP-ID --spec=spec.yaml`
* We also need to migrate the database `DATABASE_URL=DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run` after turning off "Trusted Sources" in the database on Digital Oceans.
  * The `DIGITAL-OCEAN-DB-CONNECTION-STRING` is in the following format: `postgres://<username>:<password>@<host>:<port>/<database_name>` and can be retreived from Digital Ocean's database settings -> Connection Details.
* We can now make `/POST` requests:
  * ```
    curl --request POST \
      --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' \
      https://zero2prod-mnodh.ondigitalocean.app/subscriptions \
      --verbose
    ```

# Ch 6 - Reject Invalid Subscribers #1
* Our input validation for `/POST` is limited: we just ensure that both the `name` and the `email` fields are provided, even if they are empty.

## Requirements
### Domain Constraints
* For `name`, we will just require it to be non-empty.

### Security Constraints
* Forms and user inputs are a primary attack target - if they are not properly sanitized, they might allow an attacker to mess with our database (SQL injection), execute code on our servers, crash our service, etc.
* We are building an email newsletter, which leads us to focus on:
  1. denial of service - e.g. trying to take our service down to prevent other people from signing up. A common threat for basically any online service
  2. data theft -e.g. steal a huge list of email addresses
  3. phishing -e.g. use our service to send what looks like a legitimate email to a victim to trick them into clicking on some links or perform other actions
* We will be:
  * Enforcing a maximum length. We are using `TEXT` as type for our email in Postgres, which is unbounded until disk storage starts to run out. 256 characters should be enough for the greatest majority of our users' name - if not, we will ask them to enter a nickname
  * Reject names containing troublesome characters. `/()"<>\{}` as they are not common in names. Forbidding them raises the complexity bar for SQL injection and phishing attempts.

## First Implementation
* We could add a function `is_valid_name()` that we call before we call `insert_subscirber`.
* That function would trim any whitespace in the name, ensure it isn't too long, isn't non-empty, and doesn't contain forbidden characters, but such an implementation would be a false sense of security.

## Validation Is A Leaky Cauldron
* Just by looking at the type `FormData`, `insert_subscriber` cannot assume that `form.name` will be non-empty. We would have to shift from a *local* (`insert_subscriber` function) approach to a *global* approach (the entire codebase) to ensure something ensured the name is vaild.
* Every other function that uses `form.name` would need to do the same validation (and it could be missed during refactoring etc) and such implementations could result in input checks in multiple places -- also bad. This approach does not scale.
* We need is a parsing function - a routine that accepts unstructured input and, if a set of conditions holds, returns us a more structured output, an output that structurally guarantees that the invariants we care about hold from that point onwards. We can do this using **types**!

## Type-Driven Development
* We will add a new type `SubscriberName` (a tuple struct with a single unnamed field of type `String`) in a new module `Domain` to achieve what we want.
* Since the function `parse` is the only way to construct a `SubscriberName`, we have ensured that `SubsriberName` will never violate our constraints.
* Type-driven development is a powerful approach to encode the constraints of a domain we are trying to model inside the type system, leaning on the compiler to make sure they are enforced.

## Ownership Meets Invariants
* Since `SubsriberName` itself doesn't expose the internal `String`, and we don't want to, we will add a `inner_ref` method to give a shared reference to the inner string.

### AsRef
* While our `inner_ref` method gets the job done, Rust’s standard library exposes a trait that is designed exactly for this type of usage - `AsRef`.
* So, we will replace `inner_ref` with `as_ref`.

## Error As Values - Result
* Rust’s panics are not equivalent to exceptions in languages such as Python, C# or Java.
* Although Rust provides a few utilities to catch (some) panics, it is **not** the recommended approach.
* `Result` is used as the return type for fallible operations: if the operation succeeds, `Ok(T)` is returned; if it fails, you get `Err(E)`.
  
### Converting `parse` To Return `Result`
* We will now return a `Result<SubscriberName, String>` instead of just `SubscriberName`.

## Insightful Assertion Errors: `claims`
* We will be using the `claims` crate to get more informative error messages when a test fails when using assertions and then we can use `claims::assert_ok!()`.

## Unit Tests
* We will add some tests to the `domain` module.
* `claims` needs our type to implement the `Debug` trait to provide those nice error messages. So we will add a `#[derive(Debug)]` attribute on top of `SubscriberName`

## Handling A `Result`
* Instead of panicing when an invalid name is passed in, we want to return a "400 Bad Request".

### The `?` Operator
* `?` is syntactic sugar to reduce the amount of visual noise when you are working with fallible functions and you want to “bubble up” failures.

### 400 Bad Request
* We will return a 400 Bad Request when from `subscribe` if a bad name is used.

## The Email Format
* We will use the `validator` crate to validate emails.

## The `SubscriberEmail` Type
* We will follow the same strategy we used for `name` validation - encode our invariant (“this string represents a valid email”) in a new `SubscriberEmail` type.

### Breaking The Domain Sub-Module
* We will break down the `domain` module into `new_subscriber`, `subscriber_email`, and `subcriber_name` for better organization.
  
### Skeleton Of A New Type
* `SubscriberEmail` will also be a a tuple struct with a single unnamed field of type `String`.
* We will also implement `AsRef` and let the `validator` do all the validation for the email to be accurate.

## Property-based Testing
* We could use another approach to test our parsing logic: instead of verifying that a certain set of inputs is correctly parsed, we could build a random generator that produces valid values and check that our parser does not reject them.
* This approach is called property-based testing.
* Property-based testing significantly increases the range of inputs that we are validating, and therefore our confidence in the correctness of our code, but it does not prove that our parser is correct.

### How To Generate Random Test Data With `fake`
* `fake` provides generation logic for both primitive data types (integers, floats, strings) and higher-level objects (IP addresses, country codes, email, etc).
* But we would have to run our test suite mulitple times to make sure we catch every edge case, or we could have a `for` loop to test.

### Getting Started with `quickcheck`
* There are test crates available for property-based testing, 2 of them are: `quickcheck` and `proptest`, but we will use `quickcheck`.
* `quickcheck` calls our test function in a loop with a configurable number of iterations (100 by default): on every iteration, it generates a new test case and checks the value that function returned.
* If our function fails, it tries to shrink the generated input to the smallest possible failing example to help us debug what went wrong.
* Unfortunately, if we ask for a `String` type as input we are going to get all sorts of garbage which will fail validation. How to get around this?

### Implementing the `Arbitrary` Trait
* How does `quickcheck` know what to generate to test?
* Everything is built on top of `quickcheck`'s `Arbitrary` trait that has 2 methods:
  1. `arbitrary`: given a source of randomness (`g`) it returns an instance of the type
  2. `shrink`: returns a sequence of progressively "smaller" instances of the type to help `quickcheck` find the smallest possible failure case.

* We need to create our own type, `ValidEmailFixture`, and implement `Arbitrary` for it.
* Looking at `Arbitrary`’s trait definition, `shrink` is optional: there is a default implementation (using `empty_shrinker`) which results in `quickcheck` outputting the first failure encountered, without trying to make it any smaller or nicer. So, we only need to provide an implementation of `Arbitrary::arbitrary` for our `ValidEmailFixture`.
* In `Arbitrary::arbitrary`, we get `g` as an input, an argument of type `G`.
* `G` is constraint by a trait bound, `G: quickcheck::Gen` therefore it must implement the `Gen` trait in `quickcheck`.
* And anything that implements `Gen` must also implement `RngCore` trait from the `rand-core` crate.
* We can add a `dbg!(&valid_email.0)` and run tests like `cargo t valid_emails -- --nocapture` to see all the valid emails generated.

## Payload Validation
* We can now make the changes needed in our application to use `SubsriberEmail` and all integration tests would pass.

### Refactoring With `TryForm`
* We can extract the logic to parse `name` and `email` for `NewSubscriber` into a function to get a better separation of concerns.
* We will implement `TryForm` on `NewSubscriber` to explicitly show our intent that we are convert a `FormData` into `NewSubscriber`.
* When we implement `try_form`, we automatically get the corresponding `try_into` for free that we can call.

# Ch 7 - Reject Invalid Subscribers #2

## Confirmation Emails
* Now that our remails are syntactically correct, we need to make sure they exist and reachable.

### Subscriber Consent
* We will confirm emails by sending confirmation emails and this will also tell us about the subsriber's explicit consent before we send our first newsletter.

### The Confirmation User Journey
* The user will receive an email with a confirmation link.
* Once they click on it, we will send a 200 OK to the browser.
* From that point onwards, they will receive all newsletter issues in their inbox.

### The Implementation Strategy
* There is a lot to do here, so we will split the work in three conceptual chunks:
  1. write a module to send an email
  2. adapt the logic of our existing `POST /subscriptions` request handler to match the new specification
  3. write a `GET /subscriptions/confirm` request handler from scratch

## `EmailClient`, Our Email Delivery Component

### How To Send An Email
* SMTP (Simple Mail Transfer Protocol) does for emails what HTTP does for web pages: it is an application-level protocol that ensures that different implementations of email servers and clients can understand each other and exchange messages.
* Since building our own private email server would take a long time, we will be writing a REST client for our email API provider.

#### Choosing An Email API
* We will be using Postmark.

#### The Email Client Interface
* There are two approaches when it comes to a new piece of functionality:
  1. bottom-up, starting from the implementation details and working your way up, or
  2. top-down, by designing the interface first and then figuring out how the implementation is going to work.
* We will go for the second route.

* Our `EmailClient` will be using the same sender email address to send all the emails.
* We would need the receiver's email address, the subject line, and the body of the email.
* We will be sending both HTML and a plain text version of the email content to be safe.
* Some email clients are not able to render HTML and some users explicitly disable HTML emails.
* Our `send_email` function would be asynchronous since we will be talking to a remote server.

### How To Write A REST Client Using reqwest 
* To talk to a REST API, we need an HTTP client and we will choose `reqwest` because:
  1. It has been extensively battle-tested (~8.5 million downloads)
  2. It offers a primarily asynchronous interface, with the option to enable a synchronous one via the `blocking` feature flag
  3. It relies on `tokio` as its asynchronous executor something we are already using due to `actix-web`
  4. It does not depend on any system library if you choose to use `rustls` to back the TLS implementation (`rustls-tls` feature flag instead of `default-tls`), making it extremely portable
* We are also using `reqwest` already but we will move it to a runtime dependency from a development dependency.

#### `reqwest::Client`
* `reqwest::Client` exposes all the methods we need to perform requests against a REST API.

#### Connection Pooling
* Before executing an HTTP request against an API hosted on a remote server we need to establish a connection.
* Connecting is an expensive operation, especially if using HTTPS: creating a brand-new connection every time we want to makek a request can impact the performance of our application and might lead to *socket exhaustion* under load.
* To avoiding re-establishing a connection from scratch, most HTTP clients offer connection pooling: after the first request to a remote server has been completed, they will keep the connection open (for a certain amount of time) and re-use it if we need to send another request to the same server.
* `reqwest` does the same: every time a `Client` instance is created `reqwest` initialises a connection pool under the hood.
* To use this connection pool we need to reuse the same `Client` across multiple requests.
* **Note**: `Client::clone` does not create a new connection pool - we just clone a pointer to the underlying pool.

#### How To Reuse The Same reqwest::Client In `actix-web`
* To re-use the same HTTP client across multiple requests in `actix-web` we need to store a copy of it in the application context - we will then be able to retrieve a reference to `Client` in our request handlers using an extractor (e.g. `actix_web::web::Data`) similar to how we did it for the `HttpServer`.
* We have 2 options:
  1. derive the `Clone` trait for `EmailClient` build an instance of it once and then pass a clone to `app_data` every time we need to build an `App`:
  2. wrap `EmailClient` in `actix_web::web::Data` (an `Arc` pointer) and pass a pointer to `app_data` every time we need to build an App - like we are doing with `PgPool`.
* If `EmailClient` were just a wrapper around a `Client` instance, the first option would be preferable - we avoid wrapping the connection pool twice with `Arc`.
* But, `EmailClient` has two data fields attached (`base_url` and `sender`).
* The first implementation allocates new memory to hold a copy of that data every time an `App` instance is created, while the second shares it among all `App` instances. That’s why we will be using the second strategy.

#### Configuring Our EmailClient
* We will add the `base_url` and `sender_email` to our yaml configuration files and ensure we read them to initialize `EmailClient`.

### How To Test A REST Client
* The main purpose of `EmailClient::send_email` is to perform an HTTP call: how do we know if it happened? How do we check that the body and the headers were populated as we expected?
* We need to intercept that HTTP request so we will spin up a mock server using `wiremock`.

#### `wiremock::MockServer`
* `MockServer::start` asks the operating system for a random available port and spins up the server on a background thread, ready to listen for incoming requests.
* We can then use the `uri` method to get the `base_url` for our `EmailClient`.

#### `wiremock::Mock`
* Out of the box, `wiremock::MockServer` returns "404 Not Found" to all incoming requests.
* We can instruct the mock server to behave differently by mounting a `Mock`.
* When `wiremock::MockServer` receives a request, it iterates over all the mounted mocks to check if the request matches their conditions.
* The matching conditions for a mock are specified using `Mock::given`.
  * Specifying `any()` there would match all incoming requests.
* When an incoming request matches the conditions of a mounted mock, `wiremock::MockServer` returns a response following what was specified in `respond_with`.
* We then mount this `Mock` on our `mock_server`.

#### The Intent Of A Test Should Be Clear
* Using random data generated by `fake` conveys a specific message: do not pay attention to these inputs, their values do not influence the outcome of the test, that’s why they are random.

#### Mock expectations
* Expectations are verified when `MockServer` goes out of scope - at the end of our test function.
* Before shutting down, MockServer will iterate over all the mounted mocks and check if their expectations have been verified.
* If the verification step fails, it will trigger a panic (and fail the test).
* `expect(1)` tells the mock server that it shoudl recieve exactly one request that matches this request.

### First Sketch Of `EmailClient::send_email`
* To send an email, we need:
  1. a `POST` request to the `/email` endpoint
  2. a JSON body with fields (PascalCased) that map closely to the arguments of `send_email`
  3. an authorization header, `X-Postmark-Server-Token` from Postmark.

#### `reqwest::Client::post`

#### JSON body
* `reqwest`'s JSON feature does the work to set the `request_body` as the JSON body and also sets the `Content-Type` header to `application/json`.

### Tightening Our Happy Path Test

##### Headers, Path And Method
* We can chain `and` to continue adding expectations for `Mock`, like the `header`, `path`, `method`, etc.
* We will also replace `any()` with the actual header we want to test for

##### Body
* It would be enough to check that the body is valid JSON and it contains the set of field names shown in Postmark’s example.
* Since there isn't an out-of-the-box matcher that suits our needs - we will implement our own: `SendEmailBodyMatcher` where we get the incoming request as input and we need to return a `boolean` value as output whether the mock matched or not.
* To deserialize the request body as JSON - we will add `serde-json` to the list of our development dependencies.
* We need to add `#[serde(rename_all = "PascalCase")]` to our `SendEmailRequest` struct so that we can meet the casing requirement.

#### Refactoring: Avoid Unnecessary Memory Allocations
* For each field of `SendEmailRequest`, we are making copies of String. We can use a string slice (`&str`) to reference existing data.
* A string slice is a just pointer to a memory buffer owned by somebody else.
* To store a reference in a struct we need to add a lifetime parameter: it keeps track of how long those references are valid for - it’s the compiler’s job to make sure that references do not stay around longer than the memory buffer they point to.

### Dealing With Failures
* Two scenarios are:
  1. non-success status codes (e.g. 4xx, 5xx, etc.)
  2. slow responses

#### Error Status Codes
* `Reqwest`'s `send` returns an `Ok` as long as it gets a valid response from the server - no matter the status code.
* So, we need to use `error_for_status()` in `send_email()` to turn a response into an error if the server returned an error so that our `send_email_fails_if_the_server_returns_500` test passes.

#### Timeouts
* We can instruct our mock server to wait a configurable amount of time before sending a response back using `set_delay()`.
* We are not hanging up on the server, so the connection is busy: every time we need to send an email we will have to open a new connection.
  * If the server does not recover fast enough, and we do not close any of the open connections, we might end up with accumulating several "hanging" requests socket exhaustion/performance degradation.
* As a rule of thumb: every time you are performing an IO operation, always set a timeout! But what the timeout should be is an art:
  * too low and you might overwhelm the server with retried requests
  * too high and you risk again to see degradation on the client side
* `reqwest` gives us two options & we will choose the first one.
  1. we can either add a default timeout on the `Client` itself, which applies to all outgoing requests
  2. we can specify a per-request timeout

#### Refactoring: Test Helpers
* We add functions to re-use duplicated code in our tests.

#### Refactoring: Fail fast
* The timeout for our HTTP client is currently hard-coded to 10 seconds, which means running that test takes 10 seconds -- a long time.
* We will make the timeout configurable to keep our test suite responsive.

## Skeleton And Principles For A Maintainable Test Suite

### Why Do We Write Tests?
* Tests mitigate risks, catch bugs in CI, and acr as documentation.

### Why Don't We Write Tests?
* Good tests build technical leverage, but writing tests takes time. 

### Test Code Is Still Code
* As the project evolves, there is more friction to add tests - it gets progressively more cumbersome to write new tests.

### Our Test Suite
* All our integration tests live within a single file, tests/health_check.rs.

### Test Discovery
* Our main goal in this refactoring is discoverability:
  * given an application endpoint it should be easy to find the corresponding integration tests within the tests folder
  * when writing a test, it should be easy to find the relevant test helper functions

### One Test File, One Crate
* The `tests` folder is somewhat special - cargo knows to look into it searching for integration tests.
* Each file within the `tests` folder gets compiled as its own crate and is its own executable.
* If we look in `target/debug/deps`, we will see 1 `health_check-*.d` file -- which is our integration tests as confirmed if we run it like `./target/debug/deps/health_check-*`

### Sharing Test Helpers
* If each integration test file is its own executable, how do we share test helpers functions?
* We have 2 options:
  1. define a stand-alone module - e.g. `tests/helpers/mod.rs` to add common functions and then refer to `helpers` in your test files.
  2. take full advantage of that each file under tests is its own executable - we can create sub-modules scoped to a single test executable.
* The first approach will lead to "function is never used warnings"
  * The issue is that `helpers` is bundled as a sub-module, it is not invoked as a third-party crate: `cargo` compiles each test executable in isolation and warns us if, for a specific test file, one or more public functions in helpers have never been invoked. This is bound to happen as your test suite grows - not all test files will use all your helper methods.
* With the 2nd approach, we can add each test file separately with in `tests/api/` with `main.rs` and `helpers.rs` files.
* Each executable is compiled in parallel, the linking phase is instead entirely sequential. So, bundling all your test cases in a single executable like us reduces the time spent compiling your test suite in CI.

### Sharing Startup Logic
* `spawn_app` in tests and our application's `main` looks very similar & whenever we change something in `main`, we need to make the same change in `spawn_app` too.
* This duplication also means that our application's `main` is never tested.

#### Extracting Our Startup Code
* We moved some code from `main` to `build()` in startup.rs
* In `spawn_app`, we have the following phases:
  1. Execute test-specific setup (i.e. initialize a `tracing` subscriber)
  2. Randomize the configuration to ensure tests do not interfere with each other (i.e. a different logical database for each test case)
  3. Initialize external resources (e.g. create and migrate the database)
  4. Build the application
  5. Launch the application as a background task and return a set of resources to interact with it
* We will add an `Application` struct and re-arrange things around to do the same but re-use code.

### Build An API Client
* All of our integration tests are black-box: we launch our application at the beginning of each test and interact with it using an HTTP client (i.e. `reqwest`).
* As we write tests, we necessarily end up implementing a client for our API.
* It gives us a prime opportunity to see what it feels like to interact with the API as a user.
* We can add a function to `TestApp` to share some of the duplicated code in tests/api/subscriptions.rs.

## Refocus
* Every time a user wants to subscribe, they fire a `POST /subscriptions` request
* Our request handler will:
  1. add their details to our database in the `subscriptions` table, with `status` equal to `pending_confirmation`
  2. generate a unique `subscription_token`
  3. store `subscription_token` in our database against their `id` in a `subscription_tokens` table;
  4. send an email to the new subscriber containing a link structured as `https://<api-domain>/subscriptions/confirm?token=<subscription_token>`
  5. return a 200 OK.

* Once they click on the link, a browser tab will open to send a `GET /subscriptions/confirm` endpoint request.
* Our request handler will:
  1. retrieve `subscription_token` from the query parameters
  2. retrieve the subscriber `id` associated with `subscription_token` from the `subscription_tokens` table
  3. update the subscriber `status` from `pending_confirmation` to `active` in the `subscriptions` table
  4. return a 200 OK.

* There are a few other possible designs (e.g. use a JWT instead of a unique token) and we have a few corner cases to handle (e.g. what happens if they click on the link twice or if they try to subscribe twice?).
* We need to find an implementation route that can be rolled out with **zero downtime**.

## Zero Downtime Deployments

### Reliability
* There is no silver bullet to build a highly available solution: it requires work from the application layer all the way down to the infrastructure layer.
* One thing is certain, though: if you want to operate a highly available service, you should master **zero downtime deployments** - users should be able to use the service before, during and after the rollout of a new version of the application to production.
* This is even more important if you are practising continuous deployment: you cannot release multiple times a day if every release triggers a small outage.

### Deployment Strategies

#### Naive Deployment
* THe "naive” approach looks like:
  * Version A of our service is running in production and we want to roll out version B
    * We switch off all instances of version A running the cluster
    * We spin up new instances of our application running version B
    * We start serving traffic using version B
  * There is a non-zero amount of time where there is no application running in the cluster able to serve user traffic - we are experiencing downtime.

#### Load Balancers
* We have multiple copies of our application running behind a **load balancer**.
  ![Load Balancer](/assets/images/load_balancer.png)
* Even if we have 1 replica of our application running, there is a load balancer between users and the application. Deployments are still performed using a rolling update strategy.
* Each replica of our application is registered with the load balancer as a **backend**.
* Every time somebody sends a request to our API, they hit our load balancer which is then in charge of choosing one of the available backends to fulfill the incoming request.
* Load balancers usually support adding (and removing) backends **dynamically**.
* This enables a few interesting patterns described below.

##### Horizontal Scaling
* We add more capacity when experiencing a spike in traffic by spinning up more replicas of our application.
* It helps spread the load until the work expected of a single instance becomes manageable.

##### Health Checks
* We can ask the load balancer to keep an eye on the **health** of the registered backends.
* Health checking can be:
  1. Passive: the load balancer looks at the distribution of status codes/latency for each backend to determine if they are healthy or not.
  2. Active: the load balancer is configured to send a health check request to each backend on a schedule. If a backend fails to respond with a success code for a set time period, it is marked as unhealthy and removed.
* This is a critical capability to achieve **self-healing** in a cloud-native environment: the platform can detect if an application is not behaving as expected and automatically remove it from the list of available backends to mitigate or nullify the impact on users.

#### Rolling Update Deployments
* We can leverage our load balancer to perform zero downtime deployments.
* Rolling updates:
  * Assume we have three replicas of version A of our application registered as backends for our load balancer.
  * We want to deploy version B.
  * We start by spinning up one replica of version B of our application.
  * When the application is ready to serve traffic (i.e. a few health check requests have succeeded) we register it as a backend with our load balancer.
  * If all is well, we switch off one of the replicas running version A.
  * We continue this until all instances of version A have been replaced by version B.

#### Digital Ocean App Platform
* Digital Ocean boasts zero downtime deployment out-of-the-box, but without details on how it is achieved, however experiments have shown they are using rolling updates.

## Database Migrations
* To ensure high availability in a fault-prone environment, cloud-native applications are **stateless** - they delegate all persistence concerns to external systems (i.e. databases).
* That’s why load balancing works: all backends are talking to the same database to query and manipulate the same **state**.
* The database can be thought of as a single gigantic global variable. Continuously accessed and mutated by all replicas of our application.

### Deployments And Migrations
* During a rolling update deployment, the old and the new version of the application are both serving live traffic, side by side.
* So, the old and the new version of the application are using the **same database at the same time**.
* To avoid downtime, we need a database schema that is understood by both versions.
* This is not an issue for most of our deployments, but it is a serious constraint when we need to evolve the schema.
* To move forward with the implementation strategy for confirmation emails, we need to evolve our database schema as:
  1. add a new table `subscriptions_tokens`
  2. add a new mandatory column, `status`, to the existing `subscription` table.

* Possible scenarios showing we cannot possibly deploy confirmation emails all at once without incurring downtime:
  * We could first migrate the database and then deploy the new version.
    * This implies that the current version is running against the migrated database for some time: our current implementation of `POST /subscriptions` does not know about `status` and it tries to insert new rows into  `subscriptions` without populating it.
    * Given that `status` is a madatory field, all inserts would fail - we would not be able to accept new subscribers until the new version of the application is deployed.
  * We could first deploy the new version and then migrate the database.
    * We get the opposite scenario.
    * When `POST /subscriptions` is called, it tries to insert a row into `subscriptions` with a `status` field that does not exist - all inserts fail and we cannot accept new subscribers until the database is migrated.

### Multi-step Migrations
* A big bang release won’t work - we need to get there in multiple, smaller steps. We'll keep the application code stable and migrate the database.

### A New Mandatory Column
* We look ath the `status` column

#### Step 1: Add As Optional
1. We generate and run a new migration script to add `status` as an optional field to our table.
2. We test the migration and our tests on our local database to confirm all is good before running the migration on our production database.

#### Step 2: Start Using The New Column
1. We can start using `status`, every time a new subscriber is inserted, we set `status` to `confirmed` by changing our `INSERT` query in our application.
2. Again, we run the tests and then deploy the new version of the application to production.

#### Step 3: Backfill And Mark As `NOT NULL`
* The latest version of the application ensures that status is populated for all new subscribers.
* To mark `status` as `NOT NULL` we just need to backfill the value for historical records: we’ll then be free to alter the column.
1. We generate a new migration sciprt to mark `status` as `NOT NULL` and backfill `status` for historical entries with `confirmed`.
2. Migrate our local database, run tests, and deploy to production database.

### A New Table
* This is easier.
* We add the new table in a migration while the application keeps ignoring it.
* We can then deploy a new version of the application that uses it to enable confirmation emails.
1. The migration script will add a table with the necessary columns.
2. We migrate our local database, run tests, and deploy to production.

**Note: We add a migration script with `sqlx migrate <script-name>`**

## Sending A Confirmation Email

### A Static Email
* We will test that `POST /subscriptions` is sending out an email without focusing on the body of the email for now.

#### Red Test
* We need to spin up a mock server to stand in for Postmark’s API and intercept outgoing requests.
* We add a `subscribe_sends_a_confirmation_email_for_valid_data` test.

#### Green Test
* We add the necessary changes to make the test green.
  * This includes adding a mock for `subscribe_returns_a_200_for_valid_form_data` which would now fail since it tries to send an email.

### A Static Confirmation Link
* We will scan the body of the email to retrieve a confirmation link.

#### Red Test
* For now, we will just test that there is something that is a link.
* We can use `received_requests` on `MockServer` to intercept all the requests (as a vector) intercepted by the server if the request recording is enabled (the default).
* To extract links out of the email body, we will use `linkify` and make sure the 1 link exists in the html body and 1 in the text body and that both links are identical.

#### Green Test
* We add a dummy confirmation link to the email to pass the test.
