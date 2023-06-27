fn main() {
    // Compile TailwindCSS .css file
    std::process::Command::new("tailwind")
        .args([
            "-i",
            "src/index.css",
            "-c",
            "tailwind.config.js",
            "-o",
            "public/tailwind.css",
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .unwrap();
}
