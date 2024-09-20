# Alea PRNG in Rust, for testing
This is an implementation of the commonly used JavaScript PRNG algorithm Alea in Rust. It was made for the purpose of running Alea through various random number generator test suits.
I wouldn't recommend using this generator in Rust, as it uses a bunch of floating-point math.

32 bit outputs from this generator (and by association Alea) pass PractRand 0.95-pre to 32TB and pass gjrand --big, though they fail gjrand --huge.

It, as far as I know, should produce the same results as the JavaScript version of Alea, but I have not put much work into testing this. I did test that Mash works the same, so I would expect it to.
