// This code is released under the
// General Public License (GPL), version 3
// http://www.gnu.org/licenses/gpl-3.0.en.html
// (c) Lorenzo Vannucci

#![no_std]
use rebpf::{
    LICENSE,
    VERSION,
    libbpf::{XdpAction, XdpMd},
    rebpf_macro::sec,
};

#[sec("license")]
pub static _license: [u8; 4] = LICENSE;

#[sec("version")]
pub static _version: u32 = VERSION;

#[sec("xdp_pass")]
fn _xdp_pass(_ctx: &XdpMd) -> XdpAction {
    XdpAction::PASS
}
