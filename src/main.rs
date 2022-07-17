#![feature(decl_macro)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(rustc_attrs)]
#![no_core]
#![no_main]

#[lang = "copy"]
pub trait Copy {}

#[lang = "sized"]
pub trait Sized {}

#[rustc_builtin_macro]
pub macro asm() {}

impl<T> Copy for *const T {}
impl<T> Copy for *mut T {}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let stack_top: *const u8;

    asm!(
        // mark outer stack frame
        "xor rbp, rbp",
        "mov {}, rsp",
        // align for sse
        "and rsp, -16",
        out(reg) stack_top,
        options(nostack),
    );

    let _stack_top = stack_top;

    asm!(
        "syscall",
        in("rax") 231_usize,
        in("rdi") 0_usize,
        options(noreturn, nostack),
    );
}
