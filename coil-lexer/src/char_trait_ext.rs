pub trait CharTraitExt {
    fn is_combining_character(&self) -> bool;
}

impl CharTraitExt for char {
    /// Returns `true` if this `char` is in one of these ranges:
    /// - `U+300`-`U+36F`
    /// - `U+483`-`U+489`
    /// - `U+7EB`-`U+7F3`
    /// - `U+135F`-`U+135F`
    /// - `U+1A7F`-`U+1A7F`
    /// - `U+1B6B`-`U+1B73`
    /// - `U+1DC0`-`U+1DE6`
    /// - `U+1DFD`-`U+1DFF`
    /// - `U+20D0`-`U+20F0`
    /// - `U+2CEF`-`U+2CF1`
    /// - `U+2DE0`-`U+2DFF`
    /// - `U+3099`-`U+309A`
    /// - `U+A66F`-`U+A672`
    /// - `U+A67C`-`U+A67D`
    /// - `U+A6F0`-`U+A6F1`
    /// - `U+A8E0`-`U+A8F1`
    /// - `U+FE20`-`U+FE26`
    /// - `U+101FD`-`U+101FD`
    /// - `U+1D165`-`U+1D169`
    /// - `U+1D16D`-`U+1D172`
    /// - `U+1D17B`-`U+1D182`
    /// - `U+1D185`-`U+1D18B`
    /// - `U+1D1AA`-`U+1D1AD`
    /// - `U+1D242`-`U+1D244`
    fn is_combining_character(&self) -> bool {
        (0x300..0x36F).contains(&(*self as u32))
            || (0x483..0x489).contains(&(*self as u32))
            || (0x7EB..0x7F3).contains(&(*self as u32))
            || (0x135F..0x135F).contains(&(*self as u32))
            || (0x1A7F..0x1A7F).contains(&(*self as u32))
            || (0x1B6B..0x1B73).contains(&(*self as u32))
            || (0x1DC0..0x1DE6).contains(&(*self as u32))
            || (0x1DFD..0x1DFF).contains(&(*self as u32))
            || (0x20D0..0x20F0).contains(&(*self as u32))
            || (0x2CEF..0x2CF1).contains(&(*self as u32))
            || (0x2DE0..0x2DFF).contains(&(*self as u32))
            || (0x3099..0x309A).contains(&(*self as u32))
            || (0xA66F..0xA672).contains(&(*self as u32))
            || (0xA67C..0xA67D).contains(&(*self as u32))
            || (0xA6F0..0xA6F1).contains(&(*self as u32))
            || (0xA8E0..0xA8F1).contains(&(*self as u32))
            || (0xFE20..0xFE26).contains(&(*self as u32))
            || (0x101FD..0x101FD).contains(&(*self as u32))
            || (0x1D165..0x1D169).contains(&(*self as u32))
            || (0x1D16D..0x1D172).contains(&(*self as u32))
            || (0x1D17B..0x1D182).contains(&(*self as u32))
            || (0x1D185..0x1D18B).contains(&(*self as u32))
            || (0x1D1AA..0x1D1AD).contains(&(*self as u32))
            || (0x1D242..0x1D244).contains(&(*self as u32))
    }
}
