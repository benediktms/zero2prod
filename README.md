# Setup Development Environment

You need to have Postges and Redis running locally. If you do not have a local instance, you can run the scripts in `/scripts` to spin up clusters.

You also need to create a `.env` file and store the Postgres connection string in a variable called `DATABASE_URL`.
Without this Lazy instances from `OnceCell` can potentially error out:

```log
Lazy instance has previously been poisoned
```

Any new queries that get added will also have to be supplied to be documented for sqlx's offline mode:

```bash
cargo sqlx prepare
```

# Setup Production Environment

Login with

```
doctl auth init
```

Create the app with

```
doctl apps create --spec spec.yaml
```

migrate DB

```
DATABASE_URL=DB-CONNECTION-STRING sqlx migrate run
```
