use zed_extension_api as zed;

struct Liquidsoap;

impl zed::Extension for Liquidsoap {
    fn new() -> Self
    where
        Self: Sized,
    {
        Liquidsoap
    }
}

zed::register_extension!(Liquidsoap);
