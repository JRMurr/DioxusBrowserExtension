use std::{env, io, process::Child};

fn main() {
    let opt_level: i32 = env::var("OPT_LEVEL").unwrap().parse().unwrap();
    let is_debug = opt_level == 0;

    run_tailwind(is_debug).unwrap();
}

/// Compile TailwindCSS .css file
fn run_tailwind(is_debug: bool) -> io::Result<Child> {
    let mut tailwind_command = std::process::Command::new("tailwind");

    tailwind_command
        .args([
            "-i",
            "src/index.css",
            "-c",
            "tailwind.config.js",
            "-o",
            "public/tailwind.css",
        ])
        .env("NODE_ENV", "production");

    if !is_debug {
        tailwind_command.arg("--minify");
    }

    tailwind_command.spawn()
}
