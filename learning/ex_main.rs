fn main() {
    let version = 0.2;
    let is_running = true;
    let another_version = 1.0;

    show_info("Aegis", "Terasay");
    check_status(is_running);
    check_version(version);

    let development = is_development(version);
    if development {
        println!("This is a development version.");
    } else {
        println!("This is a stable version.");
    }

    let another_development = is_development(another_version);
    if another_development {
        println!("This is a development version.");
    } else {
        println!("This is a stable version.");
    }
}

fn show_info(browser_name: &str, developer: &str) {
    println!("{browser_name} browser");
    println!("Developer: {developer}");
}

fn check_status(is_running: bool) {
    if is_running {
        println!("Browser is running");
    } else {
        println!("Browser is offline");
    }
}

fn check_version(version: f64) {
    if version < 1.0 {
        println!("Development version: {version}");
    } else {
        println!("Stable version: {version}");
    }
}

fn is_development(version: f64) -> bool {
    version < 1.0
}