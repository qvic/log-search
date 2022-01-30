# Log search

Efficiently search for a line that starts with specific date using binary search.
Written in Rust.

**NOTE:** Each line in the file must start with date string, in provided format.
Also, the file must be linearly ordered by date.

### Usage

```
log-search {file_path} {date_format} {delimiter} {target_date}
```

For example,
```shell
log-search "log.txt" "%Y-%m-%d %H:%M:%S" " - " "2022-01-01 02:53:40"
```
will output pointed line:
```
log.txt:
  ...
  2022-01-01 02:53:22 - something
> 2022-01-01 02:53:40 - something
  2022-01-01 02:53:57 - something
  ...
```


### Build from source
```shell
cargo build --release
./target/release/log-search "log.txt" "%Y-%m-%d %H:%M:%S" " - " "2022-01-01 02:53:40"
```