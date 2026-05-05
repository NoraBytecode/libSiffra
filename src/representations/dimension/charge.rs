use crate::{quantity, ratio};

quantity!(
    Charge,
    [
        // SI units
        (Volt, ratio!(1 / 1), "V", "volt", "volts"),
        (KiloVolt, ratio!(1_000 / 1), "kV", "kilovolt", "kilovolts"),
        (MegaVolt, ratio!(1_000_000 / 1), "MV", "megavolt", "megavolts"),
        (MilliVolt, ratio!(1 / 1000), "mV", "millivolt", "millivolts"),
        (
            MicroVolt,
            ratio!(1 / 1000000),
            "uV",
            "microvolt",
            "microvolts"
        ),
        (
            NanoVolt,
            ratio!(1 / 1000000000),
            "nV",
            "nanovolt",
            "nanovolts"
        ),
        (
            PicoVolt,
            ratio!(1 / 1000000000000),
            "pV",
            "picovolt",
            "picovolts"
        )
    ]
);
