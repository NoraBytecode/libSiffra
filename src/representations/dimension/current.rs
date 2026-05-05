use crate::{quantity, ratio};

quantity!(
    Current,
    [
        // SI units
        (Ampere, ratio!(1 / 1), "A", "ampere", "amperes"),
        (KiloAmpere, ratio!(1_000 / 1), "kA", "kiloampere", "kiloamperes"),
        (MegaAmpere, ratio!(1_000_000 / 1), "MA", "megaampere", "megaamperes"),
        (MilliAmpere, ratio!(1 / 1000), "mA", "milliampere", "milliamperes"),
        (
            MicrAampere,
            ratio!(1 / 1000000),
            "uA",
            "microampere",
            "microamperes"
        ),
        (
            NanoAmpere,
            ratio!(1 / 1000000000),
            "nA",
            "nanoampere",
            "nanoamperes"
        ),
        (
            PicoAmpere,
            ratio!(1 / 1000000000000),
            "pA",
            "picoampere",
            "picoamperes"
        )
    ]
);
