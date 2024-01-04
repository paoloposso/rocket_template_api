mongo-start:
	docker-compose up -d mongodb

mongo-destroy: 
	docker-compose stop mongodb

test: 
	docker-compose up -d mongodb_test
	cargo test -- --test-threads=1
	docker-compose stop mongodb_test

run: mongo-start
	cargo run
