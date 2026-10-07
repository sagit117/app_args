### App Args

#### Получение аргументов приложения

```rust
let mut port_arg = Arg::short('p', true, "short flag arg")
    .map_err(|e| format!("Ошибка создания параметра port: {}", e)).unwrap();
let mut config_arg = Arg::short_long("config", false, "short long arg")
    .map_err(|e| format!("Ошибка создания параметра config: {}", e)).unwrap();
let mut pos1 = Arg::position("position arg1");
let mut pos2 = Arg::position("position arg2");

/// основной метод парсинга, после которого, переменные получат значения (Option<&String>)
app_args::parse(&mut [
    &mut port_arg,
    &mut config_arg,
    &mut pos1,
    &mut pos2
])
    .map_err(|e| format!("Ошибка парсинга данных: {}", e))
    .unwrap();

println!("-p {:?}", port_arg.value());
println!("--config {:?}", config_arg.value());
println!("pos1 {:?}", pos1.value());
println!("pos2 {:?}", pos2.value());
```