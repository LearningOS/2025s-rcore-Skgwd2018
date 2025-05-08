//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

// 强制要求文档注释：启用后，如果代码中存在没有文档注释（/// 或 //!）的公共项（pub 类型、函数、模块等），编译器会报错（而非警告）。
// 通常用于库项目，确保代码的公共接口有完整文档。
// 用于确保所有公共 API 有文档
#![deny(missing_docs)]
// 将所有警告视为错误：任何编译器警告（如未使用的变量、废弃的语法等）都会导致编译失败。
// 用于严格的项目中，确保代码零警告。
// 用于禁止任何警告
#![deny(warnings)]
#![no_std]
#![no_main]
// 启用实验性功能：panic_info_message 是一个Nightly 特性，允许在 panic 时获取更详细的错误信息（如 panic 消息）。
// 仅在使用 Nightly 版本的 Rust 时有效（通过 rustup default nightly 切换）。
// 启用自定义 panic 处理
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world! & Hello, RISC-V!");
    trace!("[kernel] .text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!("[kernel] .rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!("[kernel] .data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!("[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}", boot_stack_top as usize, boot_stack_lower_bound as usize);
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success();
    // CI autotest success
    //crate::board::QEMU_EXIT_HANDLE.exit_failure();
    // CI autotest failed
}
