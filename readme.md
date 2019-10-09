# Run in container

To run the container, use

```
docker run -it -v "/home/marv/rs-kicker-api/database:/usr/src/app/database" -p 8000:8000 container:name
```

Volume is an sqlite database, build by running `setup_database.sh`
