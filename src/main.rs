fn main() {
    if cfg!(target_os = "macos") {
        println!("You are running MacOS!");
    } else if cfg!(target_os = "windows") {
        println!("You are running Windows!");
    } else {
        println!("This is not MacOS or Windows.");
    }

    println!("Hello, world!");
}

#[cfg(target_os = "macos")]
fn macos_only() {
  println!("You are running MacOS!");
}

#[cfg(target_os = "windows")]
fn windows_only() {
  println!("You are running Windows!");
}