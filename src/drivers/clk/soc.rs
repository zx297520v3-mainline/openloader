use crate::drivers::{
    Driver,
    clk::{Gate, Mux, gate, mux, parents, pll::TOPCRM_BASE},
    writel,
};

const TOPCRM_M0_SEL: usize = TOPCRM_BASE + 0x038;
const TOPCRM_HS_CLK: usize = TOPCRM_BASE + 0x03c;

const MATRIX_BASE: usize = 0x1306000;
const MATRIX_AXI_SEL: usize = MATRIX_BASE;
const MATRIX_PS_SEL: usize = MATRIX_BASE + 0x20;
const MATRIX_PHY_SEL: usize = MATRIX_BASE + 0x30;
const MATRIX_AP_SEL: usize = MATRIX_BASE + 0x40;

pub struct SoCClocks;
impl Driver for SoCClocks {
    unsafe fn init() {
        unsafe {
            M0Mux::set_parent(M0Parents::Clk26m);
            M0Gate::ungate();

            AHBUnkGate::gate();
            AHBUnk2Gate::ungate();
            AHBMux::set_parent(AHBParents::Clk26m);

            writel(MATRIX_AXI_SEL, 0x10001);
            writel(MATRIX_PS_SEL, 1);
            writel(MATRIX_PHY_SEL, 1);
            writel(MATRIX_AP_SEL, 1);
        }
    }
}

parents!(M0Parents: Clk104m, Clk26m, Clk78m, Clk32k);
mux!(M0Mux, TOPCRM_M0_SEL, 0, 2, M0Parents);
gate!(M0Gate, TOPCRM_M0_SEL, 2);

parents!(AHBParents: Clk104m, Clk26m, Clk78m, Clk32k);
gate!(AHBUnkGate, TOPCRM_HS_CLK, 0);
gate!(AHBUnk2Gate, TOPCRM_HS_CLK, 1);
mux!(AHBMux, TOPCRM_HS_CLK, 4, 2, AHBParents);
