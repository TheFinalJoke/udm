# GPIO Practice Guide

This guide helps you practice using the `gpiocdev` crate with Raspberry Pi GPIO pins.

## Quick Start

### Run the practice file

```bash
# Just list available GPIO chips (safe to run without hardware)
rustc gpio_practice.rs && ./gpio_practice

# Or using cargo script
cargo script gpio_practice.rs
```

### Uncomment examples to test

Edit [gpio_practice.rs](gpio_practice.rs) and uncomment the example functions you want to try:

```rust
// Change this:
println!("   // basic_output_example()?;");

// To this:
basic_output_example()?;
```

## Examples Included

1. **List GPIO Chips** - Safe, no hardware needed
   - Shows available GPIO chips on your system

2. **Basic Output** - Requires LED + resistor
   - Turn GPIO pins ON/OFF
   - Control an LED

3. **Basic Input** - Requires button or jumper wire
   - Read GPIO pin values
   - Detect button presses

4. **Edge Detection** - Requires button or jumper wire
   - Detect rising/falling edges
   - Interrupt-driven input

5. **Multiple Pins** - Requires multiple LEDs
   - Control multiple GPIOs simultaneously
   - Create blinking patterns

## Hardware Setup Examples

### LED Circuit (for output examples)
```
GPIO Pin (17) ──→ 220Ω Resistor ──→ LED+ ──→ LED- ──→ GND
```

### Button Circuit (for input examples)
```
3.3V ──→ Button ──→ GPIO Pin (27)
                ↓
             10kΩ Resistor
                ↓
               GND
```

## Common GPIO Pins

| GPIO | Physical Pin | Common Use |
|------|--------------|------------|
| 17   | 11           | Output     |
| 27   | 13           | Input      |
| 22   | 15           | Output     |
| 23   | 16           | Output     |
| 24   | 18           | Output     |

## Practice Exercises

Try these modifications to learn:

1. Make an LED blink 10 times instead of once
2. Read a button and control an LED based on its state
3. Create a traffic light sequence (Red → Yellow → Green)
4. Detect button press and release events
5. Control 5 LEDs in a Knight Rider pattern

## Safety Reminders

- GPIO pins are **3.3V max**, not 5V
- Maximum current per pin: **16mA**
- Always use resistors with LEDs (220-330Ω)
- Never short GPIO to GND or 3.3V directly
- Double-check connections before powering on

## Troubleshooting

**Permission denied errors:**
```bash
sudo chmod a+rw /dev/gpiochip0
# Or add user to gpio group
sudo usermod -a -G gpio $USER
```

**Can't find gpiochip:**
```bash
ls -l /dev/gpiochip*
# Should show gpiochip0, gpiochip1, etc.
```

**Pin already in use:**
```bash
# Check what's using the pin
cat /sys/kernel/debug/gpio
```

## Next Steps

After mastering these basics, try:
- PWM (Pulse Width Modulation) for LED brightness
- Servo motor control
- Reading sensors (DHT22, ultrasonic, etc.)
- I2C/SPI communication
- Interrupt handling with async/await

## Resources

- [gpiocdev crate docs](https://docs.rs/gpiocdev/)
- [Raspberry Pi GPIO pinout](https://pinout.xyz/)
- [Linux GPIO user space API](https://www.kernel.org/doc/html/latest/driver-api/gpio/consumer.html)
