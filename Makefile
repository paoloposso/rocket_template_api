mongo-start:
	docker-compose up -d mongodb

mongo-destroy: 
	docker-compose stop mongodb

run-local: mongo-start
	MONGO_DB_URI=mongodb://localhost:27017 MONGO_DB_NAME=dev cargo run

test:
	docker run --name mongodb_test -p 27018:27017 -d mongo:latest mongod --storageEngine ephemeralForTest
	cargo test -- --test-threads=1
	docker stop mongodb_test && docker rm mongodb_test
