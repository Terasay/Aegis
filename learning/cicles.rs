fn main() {
    for tabs in 1..=5 {
        println!("Checking tab {tabs}...")
    }

    let mut loading = 0;

    while loading <= 100 {
        println!("Loading: {loading}%");
        loading += 20;
    }

    let mut counter = 0;

    loop {
        println!("Starting...");
        counter += 1;

        if counter >= 3 {
            println!("Aegis started!");
            break;
        }
    }

}

