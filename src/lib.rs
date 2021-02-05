extern crate proc_macro;
use proc_macro::TokenStream;

static CHECK_EXISTENCE: &[&str] = &[
    "CODEBUILD_CI",            // AwsCodeBuild
    "BUILD_DEFINITIONVERSION", // AzurePipelines
    "bamboo_buildKey",         // Bamboo
    "BITBUCKET_COMMIT",        // BitbucketPipelines
    "BUDDY",                   // Buddy
    "CIRCLECI",                // CircleCI
    "GITHUB_ACTIONS",          // Github Actions
    "GITLAB_CI",               // Gitlab CI
    "JENKINS_URL",             // Jenkins
    "TEAMCITY_VERSION",        // TeamCity
    "TRAVIS",                  // Travis
];

static CHECK_VALUE: &[(&str, &str)] = &[
    ("APPVEYOR", "True"),    // AppVeyor
    ("CI_NAME", "codeship"), // Codeship
    ("CI", "drone"),         // Drone
    ("WERCKER", "true"),     // Wercker
    ("FAIL_ON_CI", "true"),     // Wercker
];

fn is_ci() -> bool {
    CHECK_EXISTENCE.iter().map(std::env::var).flatten().count() > 0
        || CHECK_VALUE
            .iter()
            .filter(|(name, value)| &std::env::var(name).unwrap_or_default() == value)
            .count()
            > 0
}

fn panic_on_ci() {
    if is_ci() {
        panic!("Code is not allowed to compile on CI Servers!");
    }
}

/// Macro to for arbitrary code used for a temporary test
#[proc_macro]
pub fn fail_on_ci(item: TokenStream) -> TokenStream {
    panic_on_ci();
    item
}

/// Alias for `fail_on_ci`
#[proc_macro]
pub fn temp_code(item: TokenStream) -> TokenStream {
    fail_on_ci(item)
}

/// Macro to use inside of if-statements if you need `true` for a temporary test
#[proc_macro]
pub fn temp_true(_item: TokenStream) -> TokenStream {
    panic_on_ci();
    "true".parse().unwrap()
}

/// Macro to use inside of if-statements if you need `false` for a temporary test
#[proc_macro]
pub fn temp_false(_item: TokenStream) -> TokenStream {
    panic_on_ci();
    "false".parse().unwrap()
}

/// Derive macro for temporary test structs
#[proc_macro_derive(FailOnCi)]
pub fn fail_on_ci_derive(_item: TokenStream) -> TokenStream {
    panic_on_ci();
    "".parse().unwrap()
}

/// Alias for `FailOnCi` derive macro
#[proc_macro_derive(TempStruct)]
pub fn fail_on_ci_derive2(item: TokenStream) -> TokenStream {
    fail_on_ci_derive(item)
}

/// Attribute for temporary test function
#[proc_macro_attribute]
pub fn temp_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    panic_on_ci();
    item
}
