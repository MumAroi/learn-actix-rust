---
runme:
  id: 01HTTCAXVMY1CPQP0TTV2YWS9E
  version: v3
---

```sh {"id":"01HTTCB0NVC8JKMW1TESBPMEAS"}
cargo run
```

```sh {"id":"01HTWGAT37QF5ZJ85S4BXX8V77"}
sea-orm-cli migrate generate create_real_property_table
```

```sh {"id":"01HTWFMKHYH4776NZY5WTP2D1X"}
sea-orm-cli generate entity -o entity/src -u postgres://looknow:332211@localhost:5432/rp_calendar --lib
```