use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::time::Instant;
use esp_idf_hal::delay::FreeRtos;
fn main() {
    esp_idf_sys::link_patches();

    let start_time = Instant::now(); // Record the start time


    let n = 5500; // Change this to the desired value of 'n'
    let prime = find_n_prime(n);

    // Record the end time
    let end_time = Instant::now();
    let execution_time = end_time - start_time;

    // Print the execution time
    println!("The {}th prime number is: {}", n, prime);
    println!("Execution time: {:?}", execution_time);
    loop {FreeRtos::delay_ms(100_u32);
   }
}


fn find_n_prime(n: u32) -> u32 {
   if n <= 0 {
       return 0; // Invalid input
   }
   let mut count = 0;
   let mut num = 2;

   while count < n {
       if is_prime(num) {
           count += 1;
       }
       num += 1;
   }

   num - 1 // Return the (n-1)-th prime number since we incremented 'num' once more after finding the nth prime
}

fn is_prime(x: u32) -> bool {
   /*
   This algorithm could be more efficient, however, the goal of this project is to mesure CPU consuption. 
   So, we dont care if unnecesary caluclations are performed
   */
   if x <= 1 {
       false;
   }
   for i in 2..(x) {
       // println! ("x:{} i:{}", x, i);
       if x % i == 0 {
           return false
       }
   }

   true
}
