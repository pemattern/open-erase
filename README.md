# Migrations

```bash
sqlx migrate add -r <name_of_change>
```
# SQL Queries

```bash
psql -h localhost -p 5432 -U postgres -d postgres
```

then

```sql
SELECT * FROM users;
```

# Test
```bash
make dev
```
