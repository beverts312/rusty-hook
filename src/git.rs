pub fn get_root_directory_path<F>(run_command: F) -> Result<String, String>
where
    F: Fn(&str, &str) -> Result<String, String>,
{
    run_command("git rev-parse --show-toplevel", "")
}

fn get_hooks_directory<F>(run_command: F) -> Result<String, String>
where
    F: Fn(&str, &str) -> Result<String, String>,
{
    run_command("git rev-parse --git-path hooks", "")
}

const HOOK_FILE_TEMPLATE: &str = "#!/bin/sh
# rusty-hook
# version {{VERSION}}

hookName=`basename \"$0\"`
gitParams=\"$*\"

if command -v rusty-hook >/dev/null 2>&1; then
  echo \"rusty-hook version: $(rusty-hook --version)\"
  echo \"hook file version: {{VERSION}}\"
  rusty-hook run --hook $hookName \"$gitParams\"
else
  echo \"Can't find rusty-hook, skipping $hookName hook\"
  echo \"You can reinstall it using 'cargo install rusty-hook' or delete this hook\"
fi";

const HOOK_NAMES: [&str; 19] = [
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-push",
    "pre-receive",
    "update",
    "post-receive",
    "post-update",
    "push-to-checkout",
    "pre-auto-gc",
    "post-rewrite",
    "sendemail-validate",
];

pub fn create_hook_files<F, G>(
    run_command: F,
    write_file: G,
    root_directory_path: &str,
) -> Result<(), String>
where
    F: Fn(&str, &str) -> Result<String, String>,
    G: Fn(&str, &str, bool) -> Result<(), String>,
{
    let hooks_directory = match get_hooks_directory(&run_command) {
        Ok(path) => path,
        Err(_) => return Err(String::from("Failure determining git hooks directory")),
    };
    let version = env!("CARGO_PKG_VERSION");
    let hook_file_contents = String::from(HOOK_FILE_TEMPLATE).replace("{{VERSION}}", version);
    for hook in HOOK_NAMES.iter() {
        if write_file(
            &format!("{}/{}/{}", root_directory_path, hooks_directory, hook),
            &hook_file_contents,
            true,
        )
        .is_err()
        {
            return Err(String::from(
                "Fatal error encountered while trying to create git hook files",
            ));
        };
    }
    Ok(())
}

#[cfg(test)]
#[path = "git_test.rs"]
mod git_tests;
