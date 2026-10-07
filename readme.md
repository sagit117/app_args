### App Args

#### Получение аргументов приложения

#### вариант 1
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

#### вариант 2
```rust
let matches = app_args::builder::new()
    .arg(Arg::short('p', true, "short flag arg").unwrap())
    .arg(Arg::short_long("config",false, "short long arg").unwrap())
    .arg(Arg::position("position arg1"))
    .arg(Arg::position("position arg2"))
    .parse()
    .unwrap();

println!("-p {:?}", matches.get_flag("-p"));
println!("--config {:?}", matches.get_value("--config"));
println!("--config {:?}", matches.get_value("config"));
println!("pos1 {:?}", matches.get_position_at(1));
println!("pos2 {:?}", matches.get_position_at(2));
```
