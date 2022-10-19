# NFC design notes

## Resosnance parameters

Target frequency `f = 13.56 MHz`

To tune antenna `(2*pi*f)^2 = 1/(L*C)`

## Test bench parameters

For test bench, we attach a PCB coil to input of oscolliscope. Oscilloscope input has input resistance `Rin = 1E6 Ohm`, `Cin = 14E-12 F`

According to specs, the coil has inductance `H = 2.5E-6 H`

Thus, for resonance, `C = 1/(L * (2*pi*f)^2) = 55E-12 F`

Without balancing capacitor (only Cin in loop), about 20mV are measured using Samsung A52 phone as reader.

In practice, we attach 2x series capacitors with `C = 100E-12 F` in parallel with coil, resulting in `C = 64E-12 F`, close enough.

With this balancing, 150mV are detected with the same phone. It is important to note, that fidelity is certainly still below 10, as the envelope shape is still clearly visible.

An NFC tag (Adafruit ST25DV16) brought into field is easily detected by the phone, thus our probe clearly plays small role in the system and does not distort signal much. Upon tag detection, strong continuous sine wave is sent from the phone for a few minutes, after which field disappears and does not re-appear until NFC system is re-set from phone settings. This is something we should be aware of. Probably enough field is reflected to overheat the NFC system in the phone. Fortunately, data transfer and full charging are expected to happen under 1 minute.

Another important observation in initial experiments is that the coil detunes badly when it is brought in close contact with the phone; the best result is observed when there is a gap of 5-10 cm between devices. We could either require a gap or tune antenna to higher frequency to take into account close proximity of the phone metal casing. If the gap is there, on one hand, user could place the phone on the bench and see the side oriented towards the phone, which could be handy, on the other hand, phone camera would see this too.

