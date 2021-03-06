# fail_on_ci

Have you ever written temporary test code, forgot about it and push it to prod?

This crate provides you with a macro which fails to compile on known CI servers. Just wrap your test code in it
and it will only compile it locally.

## How it works
Most CI servers define specific environment variables which can be used to detect if a process is running on
a CI server.

Below is a list of the CI servers that should be detected.

Alternatively you can set the environment variable `FAIL_ON_CI` to `true`. This can be used for CI servers that are not supported.

## Detected CI Servers
- AppVeyor
- AwsCodeBuild
- AzurePipelines
- Bamboo
- BitbucketPipelines
- Buddy
- Codeship
- CircleCI
- Drone
- Github Actions
- Gitlab CI
- Jenkins
- TeamCity
- Travis
- Wercker

## Examples

### Use for expression
```rust
use fail_on_ci::*;

// Struct used for local test
#[derive(FailOnCi)]
struct TestStruct {}

// alias for FailOnCi
#[derive(TempStruct)]
struct TestStruct2 {}

#[temp_function]
fn test_function() -> bool {
    true 
}

fn main() {
    // insert arbitrary code
    fail_on_ci!{
        println!("This doesn't compile on CI!");
    }
    
    // alias for fail_on_ci
    temp_code!{
        println!("This doesn't compile on CI!");
    }
    
    // returns true
    if temp_true!() {
        println!("Hello, world!");
    }
    
    // returns false
    if temp_false!() {
        println!("Hello, world!");
    }
}
```
