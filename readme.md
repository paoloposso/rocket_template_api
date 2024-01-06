# Rust Rocket Template API

## Running the application

### Environment variables
The application uses environment variables to connect to the database.
So you can create a `.env` file on the root of the project with the following content:
```
MONGO_DB_URI=mongodb://localhost:27017
MONGO_DB_NAME=dev
```
MONGO_DB_NAME can actually be any name you want.
As for the URI, this is the one that will be used to connect to the container database, defined on the `docker-compose.yml` file.

### Running the application locally
On terminal, run `make mongo-start`. This will create a mongodb container.
After that, run `cargo run`. This will start the application.

This can also be done on a single command: `make run-local`. This one will start mongodb and the application at once.

### Debugging
If you want to debug the application, make sure you start the database with `make mongo-start` prior to starting debugging.

## Run tests
`user/routes.rs`` contains the tests for the endpoints.
These are end to end tests, then it will access a database.
This is a volatile mongodb database running on a container that will be created and destroyed for each test.

On terminal, run `make test`. This will run all the tests.

When running the tests, the `make`` command already sets the environment variables for the tests to run so you don't have to worry about it.

## Next Steps
- Create JWT auth
- Update user password
