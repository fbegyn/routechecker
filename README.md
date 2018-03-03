# routechecker
Small tool to check if an ip lays on a network route. Used to check if some fancy routing is working.

```
USAGE:
    routechecker [OPTIONS] <hop> [destination]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <delay>        Delay between each check in milliseconds [default: 5000]
    -p <port>         Port to trace to [default: 0]

ARGS:
    <hop>            hop to check for in the route
    <destination>    Destination to traceroute [default: 8.8.8.8]
```
