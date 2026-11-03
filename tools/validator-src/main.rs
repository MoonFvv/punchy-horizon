// Use the launcher's actual bundle parser; reading never executes mod code.
fn main() -> std::process::ExitCode {
    let Some(path) = std::env::args_os().nth(1) else {
        eprintln!("usage: punchy-package-validator BUNDLE.hnmod");
        return std::process::ExitCode::FAILURE;
    };
    match horizon_mods::index::native::Bundle::read(std::path::Path::new(&path)) {
        Ok(bundle) => {
            println!(
                "Validated {} {}: {} files",
                bundle.manifest.id,
                bundle.manifest.version,
                bundle.files.len()
            );
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
