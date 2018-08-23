# TimeTrack [![Build Status](https://api.travis-ci.org/JoshMcguigan/timetrack.svg?branch=master)](https://travis-ci.org/JoshMcguigan/timetrack)

TimeTrack watches the file system to automatically track how much time you are spending on each of your projects. 

## Setup

TimeTrack requires Rust in order to build/install. If you need to install Rust, follow [these instructions](https://www.rust-lang.org/en-US/install.html). Once you have Rust installed, TimeTrack can be installed by running the command below:

```
cargo install traffic
```

## Use

```bash
    # to start time tracking
    timetrack track
    
    # to see the results
    timetrack
    
    # to clear the tracking history
    timetrack clear
    
    # to view the configuration
    timetrack config
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
