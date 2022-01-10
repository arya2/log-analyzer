# Log Analyzer

Takes a path to a newline-delimited JSON log file and outputs the number of objects of each type that are in the log along with their cumulative byte size.

```
USAGE:
    ./log-analyzer [OPTIONS] <PATH_IN>

ARGS:
    <PATH_IN>    Path to input file

OPTIONS:
    -h, --help   Print help information
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
│   B  ┆          2          ┆        77       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│   A  ┆          1          ┆        27       │
└──────┴─────────────────────┴─────────────────┘
```

### Opportunities

- Parallelism across CPU cores, where one thread is used to locate newline delimiters, allocate memory, and schedule parsing jobs, while the others load-balance the parsing of the JSON objects between them with work-stealing.
- Parallelism within CPU cores where slices of bytes are operated on at a time instead of individually, e.g. pikkr uses AVX-2, which can perform up to 16 operations at a time, for extracting known-key fields from JSON objects, and nom_locate could be used to locate the newline delimiters.
