# fail_on_ci

Have you ever had the problem that you put in test code, than forget about it and push it to prod?

This crate provides you with a macro which fails to compile on known CI servers. Just wrap your test code in it
and it will only compile it locally.

## How it works
Most CI servers define specific environment variables which can be used to detect if a process is running on
a CI server.

Below is a list of the CI servers that should be detected.

Alternatively you can set the environment variable `FAIL_ON_CI` to `true`. This can be used for CI servers that are not supported.

## Detected CI Servers
AppVeyor
AwsCodeBuild
AzurePipelines
Bamboo
BitbucketPipelines
Buddy
Codeship
CircleCI
Drone
Github Actions
Gitlab CI
Jenkins
TeamCity
Travis
Wercker

## Example

```rust
use fail_on_ci::fail_on_ci;

fn main() -> {
    fail_on_ci!{
        println!("This doesn't compile on CI!");
    }
}
```
