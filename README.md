# Event-Management Freiwillige Feuerwehr Fischbach [Rust]
- Implementation of the [ffischbach-events](https://github.com/simonpltzw/ffischbach-events/tree/dev/webapi) webapi in rust
- Uses the [ntex](https://github.com/ntex-rs/ntex) framework with a postgresql database as it reached the highest overall score in the latest [TechEmpower Benchmark](https://www.techempower.com/benchmarks/#hw=ph&test=composite&section=data-r22)

## Challenges
- Working OpenId connect to keep Azure Authentication
- RSA encryption of user inputs
- Replacement of AutoMapper functionality in the aspnet core counterpart
