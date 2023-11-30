#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, Rtc};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let rtc = Rtc::new(peripherals.RTC_CNTL); // define the rtc clock

    let inital_time_stap =rtc.get_time_ms();
    let n = 5500; // Change this to the desired value of 'n'
    let prime = find_n_prime(n);
    let final_time_stap =rtc.get_time_ms();
    println!("The {}th prime number is: {}", n, prime);
    println!("Bench time: {}", final_time_stap-inital_time_stap);
    loop {
        delay.delay_ms(100u32);
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

// sudo chmod a+rw /dev/ttyACM0



