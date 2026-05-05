use crate::{quantity, ratio};

quantity!(
    Power,
    [
        // SI units
        (Watt, ratio!(1 / 1), "V", "watt", "watts"),
        (KiloWatt, ratio!(1_000 / 1), "kV", "kilowatt", "kilowatts"),
        (MegaWatt, ratio!(1_000_000 / 1), "MV", "megawatt", "megawatts"),
        (MilliWatt, ratio!(1 / 1000), "mV", "milliwatt", "milliwatts"),
        (
            MicroWatt,
            ratio!(1 / 1000000),
            "uV",
            "microwatt",
            "microwatts"
        ),
        (
            NanoWatt,
            ratio!(1 / 1000000000),
            "nV",
            "nanowatt",
            "nanowatts"
        ),
        (
            PicoWatt,
            ratio!(1 / 1000000000000),
            "pV",
            "picowatt",
            "picowatts"
        )
    ]
);
