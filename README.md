# Example API with rust and mongodb

Simple project with one model (Song) and CRUD operations for it
## Installation

1. Clone the repo
2. Put the root password for mongo in `docker-compose.yml` under `MONGO_INITDB_ROOT_PASSWORD` env var
3. Put the DB password in `init-mongo.js` under `pwd` key
4. You can now start the app with `docker-compose up`
