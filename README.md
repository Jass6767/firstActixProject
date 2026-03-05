### 1. Create SQLite Database
```bash
ni database.db
```

### 2. Install SeaORM CLI
```bash
cargo install sea-orm-cli
```

### 3. Initialize Migration
```bash
sea-orm-cli migrate init
```

### 4. Generate Migration
```bash
sea-orm-cli migrate generate <migration_name>
```

Example:
```bash
sea-orm-cli migrate generate create_users_table
```

### 5. Edit Migration
Modify the generated file inside `migration/src` to define schema changes.

### 6. Run Migration
```bash
sea-orm-cli migrate up
```