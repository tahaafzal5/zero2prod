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
