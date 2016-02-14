bitflags! {
    flags KeyModifier : u32 {
        const NONEMASK    = (0 << 0),
        const SHIFTMASK   = (1 << 0),
        const LOCKMASK    = (1 << 1),
        const CONTROLMASK = (1 << 2),
        const MOD1MASK    = (1 << 3),
        const MOD2MASK    = (1 << 4),
        const MOD3MASK    = (1 << 5),
        const MOD4MASK    = (1 << 6),
        const MOD5MASK    = (1 << 7),
    }
}

