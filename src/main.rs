fn main() {
    let tabs = ["Google", "Youtube", "Github"];
    for tab in tabs {
        println!("Opened tab: {}", tab);
    }

    let mut taabs = vec!["Google", "YouTube"];
    taabs.push("Github");
    taabs.push("Reddit");
    taabs.remove(1);

    println!("Current tabs:");
    println!("{:?}", taabs);
    println!("Total tabs:", taabs.len());
}

