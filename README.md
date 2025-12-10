# solid-spork
Microservice written in Rust for Machine Learning

This is a skeleton that can be used to build a microservice in Rust.
I'm particularly interested in using Polars for Data Science.

I have a Dockerfile and docker-compose.yml to get anyone started!

More to come!

# Structure

## lib

This is where shared functionality lives. It is separated here to be
much more abstract than the general codebase. Things like generalized
structures to hold a machine learning model, or a common way to export data.

## src

This is where all the business code lives.

### api/

This is where the HTTP API lives. Add routes in the handlers.rs file, and
hook them up in the router.rs file.

### database/

This is where the database code lives. Nothing interesting here.

### domain/

Domain is where all the object live for the application.

### dtos/

This is where the data transfer objects live. These objects should be converted
ASAP to domain objects. Strictly for IO purposes.

### insights/

This is where the magic happens! This is the pure function code
for each of the insights (ML models). Each folder contains the
pure function code for a single insight.

Shared functionality is contained outside /src and instead in
/lib.

### services/

This is where the business logic lives. These are monadic elements that
interact with the database and insights. Everything here should be returning
a Result type. Errors should be propagated upwards using the error.rs file.