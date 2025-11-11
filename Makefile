dev:
	docker compose --file docker/docker-compose.dev.yml up --build --remove-orphans

dev-down:
	docker compose --file docker/docker-compose.dev.yml down

prod:
	docker compose --file docker/docker-compose.yml up --build --remove-orphans --detach

prod-down:
	docker compose --file docker/docker-compose.yml down
	
