# bike-shit
Measuring data on a bike

Bikes are great mechanical vehicles, and it would be nice being able to measure accurately the forces at play.

# Goals
- Measure as much data as possible without interacting with the bike

- Simulate the bike in real time on a tiny MCU

- Visualize in real time the calculated data

# Measurable data
- Time (GPS)
- Slope (Accelerometer + Gyro)
- Wheel speed (Hall effect)
- Pedal speed (Hall effect)

# Constants
- Wheel dimensions
- Total weight estimate

# Data to calculate
- Bike speed (Wheel RPM * Wheel circumference)
- Bike torque (Moment of interia * Angular acceleration of wheel)
- Bike traction? (Input power - Output power)

# Peripherals
- UART0 (GPS)
- I2C0 (Accelerometer + Gyro)
- 2x GPIO (Hall effect 0)
- 2x GPIO (Hall effect 1)
- I2C1/SPI0 (Display)
- SPI0/SPI1 (SD Card)?