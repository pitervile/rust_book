use std::io;


fn main() {
    if cfg!(target_os = "macos") {
        println!("You are running MacOS!");
    } else if cfg!(target_os = "windows") {
        println!("You are running Windows!");
    } else {
        println!("This is not MacOS or Windows.");
    }

    println!("Budzisz się w Lunolesie na rozdrożu wydeptanych ścieżek. Możesz iść na północ, wschód lub południe. Wybierz kierunek (n/e/s):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let direction = input.trim().to_lowercase();
    match direction.as_str() {
        "n" => println!("Spotykasz Piotra wilkołaka-przytulaka. On jest bardzo przyjazny! Jeśli go pogłaszczesz, może cię poprowadzić dalej. Liże cię po twarzy i daje ci siłę!"),
        "e" => println!("Spotykasz Piotra Wschodzące Słońce. Daje Ci siły i motywację do dalszej wędrówki. Całuje Cię swoimi promieniami i obiecuje, że zawsze będzie świecić na twojej drodze."),
        "s" => println!("Spotykasz Piotra Południowego Wiatr. On jest trochę tajemniczy, ale może podzielić się z tobą swoją mądrością. Otula cię swoimi skrzydłami i szepcze, że zawsze będziesz miał wsparcie w trudnych chwilach."),
        _ => println!("Nie rozumiem tego kierunku!"),
    }
}

#[cfg(target_os = "macos")]
fn macos_only() {
  println!("You are running MacOS!");
}

#[cfg(target_os = "windows")]
fn windows_only() {
  println!("You are running Windows!");
}