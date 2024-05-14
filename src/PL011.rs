// Register offsets
static DR_OFFSET: u32    = 0x000;
static FR_OFFSET: u32    = 0x018;
static IBRD_OFFSET: u32  = 0x024;
static FBRD_OFFSET: u32  = 0x028;
static LCR_OFFSET: u32   = 0x02c;
static CR_OFFSET: u32    = 0x030;
static IMSC_OFFSET: u32  = 0x038;
static DMACR_OFFSET: u32 = 0x048;

pub struct pl011 {
    base_addr: u64,
}

impl pl011 {
    pub fn new() -> Self {
        pl011 {
            base_addr: 0,
        }
    }

    pub fn pl011_send(data: [u8], ) {

    }

    pub fn wait_tx_complete () {
        while true {
            
        };
    }

    fn reg () -> Result<*u8, _>{

    }
}
