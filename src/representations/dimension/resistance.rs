use crate::{quantity, ratio};

quantity!(
    Resistance,
    [
        // SI units
        (Ohm, ratio!(1 / 1), "Ω", "ohm", "ohms"),
        (KiloOhm, ratio!(1_000 / 1), "kΩ", "kiloohm", "kiloohms"),
        (MegaOhm, ratio!(1_000_000 / 1), "MΩ", "megaohm", "megaohms"),
        (MilliOhm, ratio!(1 / 1000), "mΩ", "milliohm", "milliohms"),
        (
            MicroOhm,
            ratio!(1 / 1000000),
            "uΩ",
            "microohm",
            "microohms"
        ),
        (
            NanoOhm,
            ratio!(1 / 1000000000),
            "nΩ",
            "nanoohm",
            "nanoohms"
        ),
        (
            PicoOhm,
            ratio!(1 / 1000000000000),
            "pΩ",
            "picoohm",
            "picoohms"
        )
    ]
);
