
## Requirements
 - cli accepting multiple watch dirs
 - changes to be monitored
   - file creation
   - file removal
 - on event "relevant" information should be printed to stdout
 - works on linux, yet portable
 - "future components" should be able to monitor changes without modification

## Assumptions
 - To be printed information is intended for human audience
 - The "future components" requirement means that the functionality behind the cli should be be exposed as a library so that it may be included and re-used in other rust applications.

## Implementation plan

1. set up cli basics
2. create integration test
3. core implementation
4. finishing touches

