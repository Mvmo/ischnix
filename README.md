# ISCHNIX

> This is a experimental project, where i'll try to implement a dsl which invokes rust library code

```
script_engine.register_module("test")
    .add_instance("db", Instance::default().add_fn("connect", || self.database.connect())
```

```
import { move_mouse_left } from core;

mode == normal {
    key <h> -> move_mouse_left();
}

```
