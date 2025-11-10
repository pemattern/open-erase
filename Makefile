dev:
	docker compose --file docker/docker-compose.dev.yml up --build --remove-orphans

prod:
	docker compose --file docker/docker-compose.yml up --build --remove-orphans --detach

down:
	docker compose down

	
