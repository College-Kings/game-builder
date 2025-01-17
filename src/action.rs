use bitflags::bitflags;

bitflags! {
    pub struct Action: u8 {
        const STEAM = 1 << 0;
        const PATREON = 1 << 1;
        const OPPAIMAN = 1 << 2;
    }
}
