# Log Analyzer

Counts the number of objects of each type that are in the log, and the total byte size of all messages for each type in a log of JSON objects

```
USAGE:
    ./log-analyzer [OPTIONS] <PATH_IN>

ARGS:
    <PATH_IN>    Path to input file

OPTIONS:
    -h, --help             Print help information
```

### Example Input File:

```
  {"type":"B","foo":"bar","items":["one","two"]}
  {"type": "A","foo": 4.0 }
  {"type": "B","bar": "abcd"}
```

### Example Output:

```
┌──────┬─────────────────────┬─────────────────┐
│ Type ┆ Number of Instances ┆ Total Byte Size │
╞══════╪═════════════════════╪═════════════════╡
│   B  ┆          3          ┆       114       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│   C  ┆          3          ┆       114       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│   A  ┆          3          ┆       114       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│   D  ┆          3          ┆       114       │
└──────┴─────────────────────┴─────────────────┘
```

### Notes 

- Async io with tokio::fs::File performed significantly worse than std
- Map-Fold-Reduce pattern performed slightly better than what's currently in benches/my_benchmark but still took more than twice as long as the single-thread blocking-io implementation