
## Requirements

The requirements as provided and the assumptions I have made.

### Provided
 - cli accepting multiple watch dirs
 - changes to be monitored
   - file creation
   - file removal
 - on event "relevant" information should be printed to stdout
 - works on linux, yet portable
 - "future components" should be able to monitor changes without modification

### Assumptions
 - To be printed information is intended for human audience
 - The "future components" requirement means that the functionality behind the cli should be be exposed as a library so that it may be included and re-used in other rust applications.

## Implementation plan

 - [x] set up cli basics
 - [x] create basic integration tests
 - [ ] core implementation
 - [ ] finishing touches

## Bonus

 - [x] [run tests and lints in CI](https://github.com/mickvangelderen/challenge_watch_cli/actions)
 - [x] gracefully exits on program interrupt signal

## Lessons
 - Github actions work great!
 - Writing cross-platform command line tests for programs that never finish is a bit messy:
   - writing to stdin and reading from stdout and stderr requires threading or use of something like [mio](https://github.com/tokio-rs/mio).
   - I'm not exactly sure how to ensure the program has spawned and I hooked into stdout, there's a `thread::sleep` now.
 - I remembered `notify` from when I used it to implement hot reloading of GLSL shaders in [my 3D renderer](https://github.com/mickvangelderen/clustered-light-shading) written in Rust using not much other than OpenGL, an imaging and math library. It's good to know the time I spent learning then is helping me today.

