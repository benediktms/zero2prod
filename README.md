# Setup

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
