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
];

fn is_ci() -> bool {
    CHECK_EXISTENCE.iter().map(std::env::var).flatten().count() > 0
        || CHECK_VALUE
            .iter()
            .filter(|(name, value)| &std::env::var(name).unwrap_or_default() == value)
            .count()
            > 0
}

#[proc_macro]
pub fn fail_on_ci(item: TokenStream) -> TokenStream {
    if is_ci() {
        panic!("Code is not allowed to compile on CI Servers!");
    }
    item
}
