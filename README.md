# solid-spork
Microservice written in Rust for Machine Learning

This is a skeleton that can be used to build a microservice in Rust.
I'm particularly interested in using Polars for Data Science.

I have a Dockerfile and docker-compose.yml to get anyone started!

Some lingo for me (because everyone is different):
- Domain: pure functions and immutable objects
- Services: functions that interact with the database and insights
- Insights: monadic code that performs ML

More to come!

# Structure

### api/

This is where the HTTP API lives. Add routes in the handlers.rs file, and
hook them up in the router.rs file. DTOs are also defined here.

### common/

Common code shared between all the various modules. Separated out by whether
they incure side effects or not.

### database/

This is where the database code lives. Nothing interesting here.

### domain/

Domain is where all the object live for the application, and all the pure functions
for the various models.


### insights/

This is where the magic happens! This is the pure function code
for each of the insights (ML models). Each folder contains the
pure function code for a single insight.


### services/

This is where the business logic lives. These are monadic elements that
interact with the database and insights. Everything here should be returning
a Result type. Errors should be propagated upwards using the error.rs file.