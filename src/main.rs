use chrono::Local;
   use colored::*;
   use rand::Rng;

   fn main() {
       let now = Local::now();
       let mut rng = rand::rng();
       let magical_number = rng.random_range(1..100);
       
       println!("{}", "=== TRIXLAB WORKSHOP JOURNAL ===".green().bold());
       println!("Date: {}", now.format("%Y-%m-%d %H:%M:%S"));
       println!("Arcane energy level: {}", magical_number);
       
       if magical_number > 75 {
           println!("{}", "WARNING: High energy levels!".red().bold());
       } else if magical_number > 50 {
           println!("{}", "Stable energy, optimal conditions.".yellow());
       } else {
           println!("{}", "Low energy, rest recommended.".blue());
       }
       
       println!("{}", "=========================================".green().bold());
   }