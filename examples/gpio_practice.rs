// GPIO Practice File - Examples for using gpiocdev
// This file demonstrates various GPIO operations using the gpiocdev crate
//
// To run this file: cargo run --example gpio_practice
// Or compile it: rustc gpio_practice.rs

use gpiocdev::chip::Chip;
use gpiocdev::line::{EdgeDetection, Offset, Value, Values};
use gpiocdev::request::Request;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GPIO Practice Examples ===\n");

    // Example 1: List all GPIO chips
    println!("1. Listing GPIO chips:");
    list_gpio_chips()?;

    // Example 2: Basic output - turn a pin on/off
    println!("\n2. Basic Output Example (commented out - uncomment to use):");
    println!("   // basic_output_example()?;");

    // Example 3: Basic input - read a pin value
    println!("\n3. Basic Input Example (commented out - uncomment to use):");
    println!("   // basic_input_example()?;");

    // Example 4: Edge detection - detect rising/falling edges
    println!("\n4. Edge Detection Example (commented out - uncomment to use):");
    println!("   // edge_detection_example()?;");

    // Example 5: Multiple pins
    println!("\n5. Multiple Pins Example (commented out - uncomment to use):");
    println!("   // multiple_pins_example()?;");

    Ok(())
}

// List all available GPIO chips
fn list_gpio_chips() -> Result<(), Box<dyn std::error::Error>> {
    // Iterate through available chips (usually /dev/gpiochip0, /dev/gpiochip1, etc.)
    for chip_num in 0..10 {
        let chip_path = format!("/dev/gpiochip{}", chip_num);
        match Chip::from_path(&chip_path) {
            Ok(chip) => {
                let info = chip.info()?;
                println!("   Found: {} - {} ({} lines)",
                         chip_path,
                         info.name,
                         info.num_lines);
            }
            Err(_) => break, // No more chips
        }
    }
    Ok(())
}

// Example 1: Basic Output - Set a GPIO pin high/low
#[allow(dead_code)]
fn basic_output_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Setting up GPIO output...");

    // Open the GPIO chip (usually /dev/gpiochip0 on Raspberry Pi)
    let chip = Chip::from_path("/dev/gpiochip0")?;

    // Configure pin 17 as output
    let pin: Offset = 17; // GPIO 17 (physical pin 11 on RPi)

    let req = Request::builder()
        .on_chip(&chip)
        .with_line(pin)
        .as_output(Value::Active) // Start with pin HIGH
        .with_consumer("gpio_practice")
        .request()?;

    println!("   GPIO {} set to HIGH", pin);
    std::thread::sleep(Duration::from_secs(2));

    // Set pin LOW
    req.set_value(pin, Value::Inactive)?;
    println!("   GPIO {} set to LOW", pin);
    std::thread::sleep(Duration::from_secs(2));

    // Set pin HIGH again
    req.set_value(pin, Value::Active)?;
    println!("   GPIO {} set to HIGH", pin);

    Ok(())
}

// Example 2: Basic Input - Read a GPIO pin value
#[allow(dead_code)]
fn basic_input_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Setting up GPIO input...");

    let chip = Chip::from_path("/dev/gpiochip0")?;

    // Configure pin 27 as input
    let pin: Offset = 27; // GPIO 27 (physical pin 13 on RPi)

    let req = Request::builder()
        .on_chip(&chip)
        .with_line(pin)
        .as_input()
        .with_consumer("gpio_practice")
        .request()?;

    // Read the pin value 5 times
    for i in 1..=5 {
        let value = req.value(pin)?;
        println!("   Reading {}: GPIO {} = {:?}", i, pin, value);
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

// Example 3: Edge Detection - Detect rising/falling edges
#[allow(dead_code)]
fn edge_detection_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Setting up edge detection...");

    let chip = Chip::from_path("/dev/gpiochip0")?;

    // Configure pin 22 for edge detection
    let pin: Offset = 22; // GPIO 22 (physical pin 15 on RPi)

    let req = Request::builder()
        .on_chip(&chip)
        .with_line(pin)
        .with_edge_detection(EdgeDetection::BothEdges)
        .with_consumer("gpio_practice")
        .request()?;

    println!("   Waiting for edge events on GPIO {} (press Ctrl+C to stop)...", pin);
    println!("   Connect GPIO {} to 3.3V or GND to trigger events", pin);

    // Read edge events for 10 seconds
    let timeout = Duration::from_secs(10);

    loop {
        match req.read_edge_event_timeout(timeout) {
            Ok(event) => {
                println!("   Edge detected: {:?} at {:?}",
                         event.kind,
                         event.timestamp);
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                println!("   Timeout - no events detected");
                break;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

// Example 4: Multiple Pins - Control multiple GPIO pins at once
#[allow(dead_code)]
fn multiple_pins_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Setting up multiple GPIO pins...");

    let chip = Chip::from_path("/dev/gpiochip0")?;

    // Configure multiple pins as outputs
    let pins: Vec<Offset> = vec![17, 27, 22]; // GPIO 17, 27, 22

    let mut builder = Request::builder();
    builder
        .on_chip(&chip)
        .with_consumer("gpio_practice");

    // Add all pins to the request
    for &pin in &pins {
        builder.with_line(pin);
    }

    let req = builder.as_output(Value::Inactive).request()?;

    // Create a values map to set all pins at once
    let mut values = Values::default();

    // Turn all pins ON
    for &pin in &pins {
        values.set(pin, Value::Active);
    }
    req.set_values(&values)?;
    println!("   All pins set to HIGH");
    std::thread::sleep(Duration::from_secs(1));

    // Turn all pins OFF
    for &pin in &pins {
        values.set(pin, Value::Inactive);
    }
    req.set_values(&values)?;
    println!("   All pins set to LOW");

    // Blink pattern - turn on pins one by one
    println!("   Blinking pins in sequence...");
    for &pin in &pins {
        values.set(pin, Value::Active);
        req.set_values(&values)?;
        println!("   GPIO {} ON", pin);
        std::thread::sleep(Duration::from_millis(300));

        values.set(pin, Value::Inactive);
        req.set_values(&values)?;
        std::thread::sleep(Duration::from_millis(300));
    }

    Ok(())
}

// Additional helper functions for practice

#[allow(dead_code)]
fn get_line_info(chip_path: &str, pin: Offset) -> Result<(), Box<dyn std::error::Error>> {
    let chip = Chip::from_path(chip_path)?;
    let line_info = chip.line_info(pin)?;

    println!("Line {} info:", pin);
    println!("  Name: {:?}", line_info.name);
    println!("  Consumer: {:?}", line_info.consumer);
    println!("  Used: {}", line_info.used);
    println!("  Direction: {:?}", line_info.direction);

    Ok(())
}

#[allow(dead_code)]
fn blink_led(chip_path: &str, pin: Offset, times: u32) -> Result<(), Box<dyn std::error::Error>> {
    let chip = Chip::from_path(chip_path)?;

    let req = Request::builder()
        .on_chip(&chip)
        .with_line(pin)
        .as_output(Value::Inactive)
        .with_consumer("blink")
        .request()?;

    for _ in 0..times {
        req.set_value(pin, Value::Active)?;
        std::thread::sleep(Duration::from_millis(500));
        req.set_value(pin, Value::Inactive)?;
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

/*
PRACTICE EXERCISES:

1. Modify basic_output_example() to blink an LED 5 times
2. Change basic_input_example() to use a pull-up resistor
3. Create a function that reads a button and controls an LED
4. Implement a traffic light sequence with 3 LEDs
5. Create an interrupt-driven button handler using edge detection

COMMON GPIO PINS ON RASPBERRY PI:
- GPIO 17 (Physical Pin 11)
- GPIO 27 (Physical Pin 13)
- GPIO 22 (Physical Pin 15)
- GPIO 23 (Physical Pin 16)
- GPIO 24 (Physical Pin 18)
- GPIO 25 (Physical Pin 22)

SAFETY NOTES:
- Always use appropriate resistors with LEDs (220-330 ohms)
- Don't connect GPIO directly to 5V - it will damage the Pi
- GPIO pins are 3.3V, max current 16mA per pin
- Check pin configuration before connecting hardware
*/
