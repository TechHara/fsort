```text
Sort fields within each line

Usage: fsort [OPTIONS] [INPUT] [OUTPUT]

Arguments:
  [INPUT]   Input file
  [OUTPUT]  Output file

Options:
  -d, --delim <DELIM>  Field delimiter character [default: "\t"]
  -w, --white-space    Separate fields by whitespace. Specify output delimiter with -d option
  -f, --fold-case      Fold to upper case when comparing
  -n, --numeric        TODO: Compare according to string numerical value
  -r, --reverse        Reverse the result of comparisons
  -c, --check          Check each line is sorted
  -u, --unique         Print only unique fields per line
  -h, --help           Print help information
  -V, --version        Print version information
```