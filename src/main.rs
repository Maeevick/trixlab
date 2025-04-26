fn main() {
    let workshop_name = "TrixLab";
    let version = 0.1;
    let year = 2025;
    let is_experimental = true;
    
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("Workshop name: {}", workshop_name);
    println!("Version: {:.1}", version);  // Formatting to 1 decimal place
    println!("Year: {year}");  // Direct use of variable name
    println!("Experimental mode: {}", if is_experimental { "ACTIVATED" } else { "DEACTIVATED" });
    println!("{:-^40}", " WORKSHOP JOURNAL ");  // Centered with dash filling
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
}
