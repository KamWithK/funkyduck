fn main() {
    // TODO: Fix system QT/KDE styling so the default native option can be used
    let config = slint_build::CompilerConfiguration::new().with_style("cosmic-dark".into());
    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}
