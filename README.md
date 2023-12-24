# TravelAgencyRUST
Agência de viagens utilizando Rust e React

![screenshot](assets/banner.png)

## Steps to run the project
- cargo sqlx prepare (para gerar a pasta com os metadados sql, se desejar suba no controle de versão)
  >  cargo sqlx prepare

  >  query data written to .sqlx in the current directory; please check this into version control

- docker compose up
- follow the link  http://localhost:8080/api/itineraries

## Steps to run the project without Docker
- Run the SQL script at  /database/create_postgres.sql
- Set DATABASE_URL in the .env file
- cargo run
- follow the link  http://localhost:8080/api/itineraries
