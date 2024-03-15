# HTTP to SQL

## About

(!) Not use it production environment it's very
dangereous!!!

it's simple service for use this service for make http query
with postman to DB

## How to use

- run container

```bash
docker run -it --name httpsql \
        --env DB_USER={Your db user}   \
        --env DB_PASSWORD={Your db password}   \
        --env DB_HOST={Host to db with out http}   \
        --env DB_PORT={Your db port}   \
        --env DB_NAME={Name of DB}   \
        -p 4545:4545\
        --detach \
        t1mon1106/http_to_sql
```

- make http query from postman

![Alt postman query example](images/http_to_sql.png)
