## Running the application
On terminal, run `make mongo-start`. This will create a mongodb container.
After that, run `cargo run`. This will start the application.
This can also be done on a single command: `make run`.

## Run tests
`user/routes.rs`` contains the tests for the endpoints.
These are end to end tests, then it will access a database.
This is a volatile mongodb database running on a container that will be created and destroyed for each test.

On terminal, run `make test`. This will run all the tests.

## Next Steps

- Hash password when creating User
- Read params from env
