# Setup

## Dependencies
**sqlx-cli**
```
cargo install sqlx-cli
```
**Postgres**
```
# Download Postgres
docker pull postgres

# Start Postgres server
docker run --name postgresql -e POSTGRES_USER={{DB_USER}} -e POSTGRES_PASSWORD={{DB_PASSWORD}} -p 5432:5432 -v /data:/var/lib/postgresql/data -d --restart unless-stopped postgres
```

## Migrations
Make sure `DATABASE_URL` is set in the environment
```
# .env

...
DATABASE_URL=postgres://{{DB_USER}}:{{DB_PASSWORD}}@{{DB_HOST}}/movie_memo
```
**Create/Drop database**
```
sqlx database create
sqlx database drop
```

**Run migrations:**
```
sqlx migrate run
```

for more info check the **sqlx-cli documentation** [here](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli).

## Requests
In order for movie memo to make requests to TMDB you will need to get an API key which you can get following [these instructions](https://developers.themoviedb.org/3/getting-started/introduction). Once you have your api key
you will need to add them to a .env file in the root folder of the project:

```
# .env

...
API_KEY_V3 = {{YOUR TMDB API KEY GOES HERE}}
```

In order to use test requests in the `requests` folder, you will need to install the [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension for VSCode. Once installed 
you'll need to add your api key with the following configuration to your `.vscode/settings.json` file:
```
// .vscode/settings.json
{
    ...
    "rest-client.environmentVariables": {
        "$shared": {
            "API_KEY_V3": "{{YOUR TMDB API KEY GOES HERE}}",
            "HOST": "http://localhost:8000" // when running locally
        }
    }
    ...
}
```

## Run the Project
Setup dependencies
Clone the repo, run migrations and the start the server with  
```
cargo run
```