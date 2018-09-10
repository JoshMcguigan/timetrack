# TimeTrack [![Build Status](https://api.travis-ci.org/JoshMcguigan/timetrack.svg?branch=master)](https://travis-ci.org/JoshMcguigan/timetrack)

TimeTrack watches the file system to automatically track how much time you are spending on each of your projects. 

![screenshot](https://user-images.githubusercontent.com/22216761/45271448-f6c61380-b45b-11e8-9fb8-2485fff06a9a.png)

## Setup

### Install

TimeTrack requires Rust in order to build/install. If you need to install Rust, follow [these instructions](https://www.rust-lang.org/en-US/install.html). Once you have Rust installed, TimeTrack can be installed by running the command below:

```
cargo install timetrack
```

### Configure tracking path(s)

By default, TimeTrack is configured to watch your home directory. You will likely want to reconfigure TimeTrack to watch whichever directory you use to store your projects. After installing TimeTrack, run `timetrack config` to find the `User configuration` file. Edit that file to configure TimeTrack to watch the appropriate directory (or directories if you have multiple, note that at this time TimeTrack does not support watching directories which are nested within one another). As an example, the full contents of my TimeTrack configuration file are `track_paths = ["/Users/josh/Projects"]`. After editing the configuration file, run `timetrack config` again to confirm the `Tracking paths` are displayed correctly.

### Automatic startup

The `timetrack track` command starts TimeTrack in tracking mode. This should be running any time you want to track time. While you can manually start/stop this process, it is recommended that you configure your system to start this process automatically on startup. The specific steps to do this will depend on your OS. On OSX you can use `timetrack schedule` after TimeTrack has been installed to configure it to start tracking every time the current use logs in.  

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
    
    # to configure TimeTrack to start tracking every time the current user logs in 
    # currently only supported on OSX
    timetrack schedule
    
    # to disable TimeTrack from starting every time the current user logs in 
    # currently only supported on OSX
    timetrack unschedule
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
