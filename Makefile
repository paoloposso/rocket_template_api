mongo-start:
	docker-compose up -d mongodb

mongo-destroy:
	docker-compose down -v

mongo-recreate: mongo-destroy mongo-start
