# General power consumption

## e-ink screen

3.3V at 20mA for 5s

total: 0.33J at 66mW

## mc

3.3V at 5mA for 1s full power
3.3V at 1mA for lazy state
3.3V at 0.2mA for barely awake

total: 0.016J at 16mW peak; 3.3mW at work; 1mW sleep

# Storage capacity

discharge to 50% voltage
We could use the remaining 1/4 of energy to wipe memory.
The screen should probably be wiped anuway on wake up.

W = 3CV^2/8 = 0.5J

One cell approximately:
U = 1V
C = 8/(3WV^2) = 5.3F

Typical multicell:
U = 2.1V
C = 1.2F

U = 3.3V
C = 0.5F

V = 5.4V
C = 0.2F

## Charging rate

We should probably be able to transfer 50% of energy to Kampela since we can choke TX channel into resonator for power transfer part of cycle.

The phone coil consumes about 40mA at a few V, thus 100-200mW

We could thus collect about 50mW

This would require 10s to fully charge 0.5J system. Sounds doable. We could even burn about 10% into power indicator (1mA, 2V diode for example) without anyone noticing it.


