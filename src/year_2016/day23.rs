// This is going to sound crazy, but I decided to also implement a JIT compiler for day 23.
//
// In a way it makes sense. We have some source code which can modify itself. A JIT is able to
// re-generate the code. All we need to do it jump back into the compiler from the generated code.
//
// Ideally, you would want to setup a trap/interrupt handler to do that. To keep things simple,
// I'm directly calling the handler from the JITed code using a call.
// The code now needs to handle a jnz where the offset is a register. I also solve this using a
// handler.
//
// Things get a little tricky because the re-JITed code can have different size.
//
// To summarize. This code
// - parses the input and creates a Vec<Cmd>.
// - writes the x64 machine code into a buffer. The tgl instruction calls a handler written in
//   rust.
// - jump to the start of the buffer and execute the input.
// - the tgl handler re-JITs the code and fixes the return address on the stack.

// I found a tutorial by Jonathan Turner, which was very helpful:
// http://www.jonathanturner.org/2015/12/building-a-simple-jit-in-rust.html
// In addition, http://os.phil-opp.com/catching-exceptions.html provided some useful information
// and Julia Evans' posts about writing an OS in rust was useful too
// http://jvns.ca/blog/2014/03/12/the-rust-os-story/.

extern crate regex;
extern crate libc;

use self::regex::Regex;
use std::mem;
use std::ops::{Index, IndexMut};

pub fn solve(input: &str) {
    let test_input = "cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a";
    assert_eq!(_solve(test_input, (0, 0, 0, 0)), 3);

    println!("part 1: {}", _solve(input, (7, 0, 0, 0)));
    println!("part 2: {}", _solve(input, (12, 0, 0, 0)));
}

#[derive(Debug, Clone)]
struct CmdOffset {
    cmd: Cmd,
    offset: usize,
}

// Normally, compilers use multiple intermediate representations. Our input language is close
// enough to x64 that we only need one representation.
#[derive(Debug, Clone)]
enum Cmd {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Tgl(Arg),
    Nop,
}

#[derive(Debug, Clone)]
enum Arg {
    Immediate(i64),
    Register(u8),
}

/**
 * For debugging purpose...
 */
// fn print_cmds(cmds: &Vec<CmdOffset>) {
//     for i in 0..cmds.len() {
//         println!("{} {:x} {:?}", i, cmds[i].offset, cmds[i].cmd);
//     }
// }

fn i64_to_bytes(v: i64) -> Vec<u8> {
    vec![(v & 0xff) as u8,
         ((v >> 8) & 0xff) as u8,
         ((v >> 16) & 0xff) as u8,
         ((v >> 24) & 0xff) as u8]
}

// We are lucky that we need only 4 registers and we can use r8 to r11. These registers
// are encoded consecutively using 3 bits.
fn reg_to_bits(r: u8) -> u8 {
    assert!(r == b'a' || r == b'b' || r == b'c' || r == b'd');
    r - b'a'
}

// Converting instructions to machine code is a pain. Thankfully, we just care about a subset.
// If you want to see the amount of work it takes to support all the instructions on x64, take a
// look at Intel's encoder/decoder library: https://intelxed.github.io
//
// We assume that a short jump is enough. Ideally, we should plan for a long jump and then convert
// to a short jump after we know the address of everything. Doing that however is slightly more
// complicated.
fn get_bytes(cmd: CmdOffset, offset: usize, offsets: &Vec<CmdOffset>) -> Vec<u8> {
    let mut bytes = vec![];
    match cmd.cmd {
        // Cpy
        Cmd::Cpy(Arg::Immediate(i), Arg::Register(r)) => {
            bytes.extend_from_slice(&[0x49, 0xc7, 0xc0 | reg_to_bits(r)]);
            bytes.append(&mut i64_to_bytes(i));
        }
        Cmd::Cpy(Arg::Register(src), Arg::Register(dst)) =>
            bytes.extend_from_slice(&[0x4d, 0x89, 0xc0 | (reg_to_bits(src) << 3) | reg_to_bits(dst)]),
        Cmd::Cpy(_, Arg::Immediate(_)) => {}

        // Inc
        Cmd::Inc(Arg::Immediate(_)) => {}
        Cmd::Inc(Arg::Register(r)) => bytes.extend_from_slice(&[0x49, 0xff, 0xc0 | reg_to_bits(r)]),

        // Dec
        Cmd::Dec(Arg::Immediate(_)) => {}
        Cmd::Dec(Arg::Register(r)) => bytes.extend_from_slice(&[0x49, 0xff, 0xc8 | reg_to_bits(r)]),

        // Jnz
        // TODO: make jnz _, immediate should also dispatch to jmp_handler?
        Cmd::Jnz(Arg::Immediate(i), Arg::Immediate(o)) => {
            let mut delta: isize = offsets[(offset as i64 + o as i64 - 1) as usize].offset as isize;
            delta -= offsets[offset].offset as isize;
            assert_eq!(delta, (delta as i8) as isize);
            bytes.extend_from_slice(&[0x48, 0xc7, 0xc0]);
            bytes.append(&mut i64_to_bytes(i)); // mov rax, <i>
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x75, delta as u8]); // jne <offset>
        }
        Cmd::Jnz(Arg::Register(r), Arg::Immediate(o)) => {
            let mut delta: i64 = offsets[(offset as i64 + o as i64) as usize].offset as i64;
            delta -= offsets[offset + 1].offset as i64;
            bytes.extend_from_slice(&[0x4c, 0x89, 0xc0 | (reg_to_bits(r) << 3)]); // mov rax, <r>
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x0f, 0x85]);
            bytes.append(&mut i64_to_bytes(delta)); // jne <relative offset (32 bits)>
        }
        Cmd::Jnz(Arg::Immediate(i), Arg::Register(j)) => {
            // We need to convert j to the address of an instruction. It's easier to drop back
            // into rust code.
            bytes.extend_from_slice(&[0x48, 0xc7, 0xc0]);
            bytes.append(&mut i64_to_bytes(i)); // mov rax, <i>
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x74, 17]); // jeq skip

            bytes.extend_from_slice(&[0x4c, 0x89, (0xc7 | reg_to_bits(j) << 3)]);  // mov rdi, <j>
            bytes.extend_from_slice(&[0x48, 0xb8]); // movabs rax, <immediate>
            bytes.append(&mut i64_to_bytes(jmp_handler as i64));
            bytes.append(&mut i64_to_bytes((jmp_handler as i64) >> 32));
            bytes.extend_from_slice(&[0xff, 0xd0]); // call rax
            // return value contains address of j, simply jump there
            bytes.extend_from_slice(&[0xff, 0xe0]); // jmp rax
        }
        Cmd::Jnz(Arg::Register(c), Arg::Register(j)) => {
            bytes.extend_from_slice(&[0x4c, 0x89, 0xc0 | (reg_to_bits(c) << 3)]); // mov rax, <r>
            bytes.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax, rax
            bytes.extend_from_slice(&[0x74, 17]); // jeq skip

            bytes.extend_from_slice(&[0x4c, 0x89, (0xc7 | reg_to_bits(j) << 3)]);  // mov rdi, <j>
            bytes.extend_from_slice(&[0x48, 0xb8]); // movabs rax, <immediate>
            bytes.append(&mut i64_to_bytes(jmp_handler as i64));
            bytes.append(&mut i64_to_bytes((jmp_handler as i64) >> 32));
            bytes.extend_from_slice(&[0xff, 0xd0]); // call rax
            // return value contains address of j, simply jump there
            bytes.extend_from_slice(&[0xff, 0xe0]); // jmp rax
        }

        // Tgl
        Cmd::Tgl(Arg::Register(r)) => {
            bytes.extend_from_slice(&[0x4c, 0x89, (0xc7 | reg_to_bits(r) << 3)]);  // mov rdi, <r>
            bytes.extend_from_slice(&[0x48, 0xb8]); // movabs rax, <i>
            bytes.append(&mut i64_to_bytes(toggle_handler as i64));
            bytes.append(&mut i64_to_bytes((toggle_handler as i64) >> 32));
            bytes.extend_from_slice(&[0xff, 0xd0]); // call rax
        }
        Cmd::Tgl(Arg::Immediate(i)) => {
            bytes.extend_from_slice(&[0x48, 0xbf]); // movabs rdi, <i>
            bytes.append(&mut i64_to_bytes(i as i64));
            bytes.append(&mut i64_to_bytes((i as i64) >> 32));

            bytes.extend_from_slice(&[0x48, 0xb8]); // movabs rax, <i>
            bytes.append(&mut i64_to_bytes(toggle_handler as i64));
            bytes.append(&mut i64_to_bytes((toggle_handler as i64) >> 32));
            bytes.extend_from_slice(&[0xff, 0xd0]); // call rax
        }

        // Nop
        Cmd::Nop => bytes.push(0x90),
    }
    bytes
}

/**
 * We call into the jmp_handler when a jnz has a register as the offset argument. We need
 * to look at the CMDS data structure and map the command offset to an address.
 */
extern "C" fn jmp_handler(arg: isize) -> u64 {
    unsafe {
        asm!("push %r8");
        asm!("push %r9");
        asm!("push %r10");
        asm!("push %r11");
    }

    // println!("Handling jmp arg={}", arg);

    // Find current address by looking at the return address on the stack.
    let addr: u64;
    unsafe {
        asm!("mov 8(%rbp), $0" : "=r"(addr));
    }
    // Find which instruction this maps to. We need to add 2, because there's a "jmp rax" in the
    // JITed code. If we can't find the instruction, we jump to the last instruction (nop).
    let base = JIT_MEMORY.with(|jit| jit.borrow().contents as u64);
    let current_cmd_offset = CMDS.with(|cmds| {
        let cmds = cmds.borrow();
        let item = cmds.iter().position(|x| x.offset as u64 == addr + 2 - base).unwrap();
        (item - 1) as isize
    });
    // println!("current pc: {}", current_cmd_offset);
    let num_commands = CMDS.with(|cmds| cmds.borrow().len() as isize);
    let dest_cmd_offset = if (current_cmd_offset + arg < 0) ||
                             (current_cmd_offset + arg >= num_commands) {
        num_commands - 1
    } else {
        current_cmd_offset + arg
    };
    // println!("here: {}", dest_cmd_offset);
    let r = CMDS.with(|cmds| cmds.borrow()[dest_cmd_offset as usize].offset) + base as usize;
    // println!("Handling jmp arg={}, addr={}, base={:?}, r={}", arg, addr, base, r);

    unsafe {
        asm!("pop %r11");
        asm!("pop %r10");
        asm!("pop %r9");
        asm!("pop %r8");
    }
    r as u64
}

extern "C" fn toggle_handler(arg: isize) {
    unsafe {
        asm!("push %r8");
        asm!("push %r9");
        asm!("push %r10");
        asm!("push %r11");
    }

    // println!("Handling tgl {}", arg);

    // Find current address by looking at the return address on the stack.
    let addr: u64;
    unsafe {
        asm!("mov 8(%rbp), $0" : "=r"(addr));
    }
    // Find which instruction this maps to.
    let base = JIT_MEMORY.with(|jit| jit.borrow().contents as u64);
    let current_cmd_offset = CMDS.with(|cmds| {
        let cmds = cmds.borrow();
        let item = cmds.iter().position(|x| x.offset as u64 == addr - base).unwrap();
        (item - 1) as isize
    });
    // println!("here: {}", current_cmd_offset);

    // modify current_cmd_offset + arg.

    let cmd_offset = current_cmd_offset + arg;
    // println!("cmd_offset {}", cmd_offset);
    CMDS.with(|cmds| {
        let mut cmds = cmds.borrow_mut();
        if (cmd_offset >= 0) && (cmd_offset < cmds.len() as isize) {
            let cmd = cmds[cmd_offset as usize].cmd.clone();
            let new_cmd = match cmd.clone() {
                Cmd::Inc(a) => Cmd::Dec(a),
                Cmd::Tgl(a) | Cmd::Dec(a) => Cmd::Inc(a),
                Cmd::Jnz(a, b) => Cmd::Cpy(a, b),
                Cmd::Cpy(a, b) => Cmd::Jnz(a, b),
                Cmd::Nop => Cmd::Nop,
            };
            cmds[cmd_offset as usize].cmd = new_cmd.clone();
            // println!("before: {:?}, now: {:?}", cmd, new_cmd);
        }
    });

    // Re-jit the code.
    emit_machine_code();
    // CMDS.with(|cmds| print_cmds(&cmds.borrow()));

    // look up the new address for current_cmd_offset+1
    // println!("old address: {}", addr);
    let new_addr = CMDS.with(|cmds| cmds.borrow()[(current_cmd_offset + 1) as usize].offset) +
                   base as usize;
    // println!("new address: {}", new_addr);

    unsafe {
        asm!("mov $0, 8(%rbp)" :: "r"(new_addr));
        asm!("pop %r11");
        asm!("pop %r10");
        asm!("pop %r9");
        asm!("pop %r8");
    }
}

const N_PAGES: usize = 1;
use std::cell::RefCell;
thread_local! {
    static JIT_MEMORY: RefCell<JitMemory> = RefCell::new(JitMemory::new(N_PAGES));
    static CMDS: RefCell<Vec<CmdOffset>> = RefCell::new(vec![]);
}

fn cap_to_arg(cap: Option<&str>) -> Arg {
    let cap = cap.unwrap();
    if let Some(cap) = Regex::new(r"(-?\d+)").unwrap().captures(cap) {
        Arg::Immediate(cap.at(1).unwrap().parse().unwrap())
    } else {
        Arg::Register(cap.bytes().nth(0).unwrap())
    }
}

fn _solve(input: &str, (reg_a, reg_b, reg_c, reg_d): (i64, i64, i64, i64)) -> i64 {
    // set the initial values for our registers.
    CMDS.with(|cmds| {
        let mut cmds = cmds.borrow_mut();
        cmds.clear();

        cmds.push(CmdOffset {
            cmd: Cmd::Cpy(Arg::Immediate(reg_a), Arg::Register(b'a')),
            offset: 0,
        });
        cmds.push(CmdOffset {
            cmd: Cmd::Cpy(Arg::Immediate(reg_b), Arg::Register(b'b')),
            offset: 0,
        });
        cmds.push(CmdOffset {
            cmd: Cmd::Cpy(Arg::Immediate(reg_c), Arg::Register(b'c')),
            offset: 0,
        });
        cmds.push(CmdOffset {
            cmd: Cmd::Cpy(Arg::Immediate(reg_d), Arg::Register(b'd')),
            offset: 0,
        });

        // parse input
        for line in input.trim().split('\n') {
            if let Some(cap) = Regex::new(r"^cpy (\S+) (\S+)").unwrap().captures(line) {
                cmds.push(CmdOffset {
                    cmd: Cmd::Cpy(cap_to_arg(cap.at(1)), cap_to_arg(cap.at(2))),
                    offset: 0,
                });
                continue;
            }
            if let Some(cap) = Regex::new(r"^inc (\S+)").unwrap().captures(line) {
                cmds.push(CmdOffset {
                    cmd: Cmd::Inc(cap_to_arg(cap.at(1))),
                    offset: 0,
                });
                continue;
            }
            if let Some(cap) = Regex::new(r"^dec (\S+)").unwrap().captures(line) {
                cmds.push(CmdOffset {
                    cmd: Cmd::Dec(cap_to_arg(cap.at(1))),
                    offset: 0,
                });
                continue;
            }
            if let Some(cap) = Regex::new(r"^jnz (\S+) (\S+)").unwrap().captures(line) {
                cmds.push(CmdOffset {
                    cmd: Cmd::Jnz(cap_to_arg(cap.at(1)), cap_to_arg(cap.at(2))),
                    offset: 0,
                });
                continue;
            }
            if let Some(cap) = Regex::new(r"^tgl (\S+)").unwrap().captures(line) {
                cmds.push(CmdOffset {
                    cmd: Cmd::Tgl(cap_to_arg(cap.at(1))),
                    offset: 0,
                });
                continue;
            }
            panic!("unknown command: {}", line);
        }
        // Put a nop at the end to handle the case where the last instruction is a jump.
        cmds.push(CmdOffset {
            cmd: Cmd::Nop,
            offset: 0,
        });
    });

    // We don't need to save any registers because we only use eax and r8-r11
    emit_machine_code();
    // CMDS.with(|cmds| print_cmds(&cmds.borrow()));

    // Jump into code.
    let addr = JIT_MEMORY.with(|jit| {
        let j = jit.borrow();
        j.contents
    });
    // println!("base addr: {:?}", addr);
    let fun: fn() -> i64 = unsafe { mem::transmute(addr) };
    debug_me(fun)
}

fn emit_machine_code() {
    let mut bytes = vec![];
    CMDS.with(|cmds| {
        let mut cmds = cmds.borrow_mut();

        // Emit the machine code. We emit it twice so that we have all the right addresses for jumps.
        let copy = cmds.clone();
        for i in 0..cmds.len() {
            cmds[i].offset = bytes.len();
            bytes.append(&mut get_bytes(cmds[i].clone(), i, &copy));
        }

        // Second pass to resolve jumps.
        bytes.truncate(0);
        let copy = cmds.clone();
        for i in 0..cmds.len() {
            bytes.append(&mut get_bytes(cmds[i].clone(), i, &copy));
        }

        // Since we don't save any registers, we don't need to restore any.
        // Put the result in rax and return.
        bytes.extend_from_slice(&[0x4c, 0x89, 0xc0]); // mov rax, r8
        bytes.extend_from_slice(&[0xc3]); // ret

        // Allocate a buffer using posix_memalign, copy our bytes and run the code.
        // note: we never bother to free our buffer.
        let n_pages = (bytes.len() as f64 / PAGE_SIZE as f64).ceil() as usize;
        if n_pages > N_PAGES {
            panic!("increase N_PAGES!");
        }
    });

    JIT_MEMORY.with(|jit| {
        let mut j = jit.borrow_mut();
        for i in 0..bytes.len() {
            j[i] = bytes[i];
        }
    });
}

// Instead of directly calling fun() above, we go through this function. It makes setting
// breakpoints in lldb easier.
fn debug_me(fun: fn() -> i64) -> i64 {
    fun()
}

// The code below is mostly copied verbatim from https://github.com/jonathandturner/rustyjit

const PAGE_SIZE: usize = 4096;

struct JitMemory {
    contents: *mut u8,
}

impl JitMemory {
    fn new(num_pages: usize) -> JitMemory {
        let contents: *mut u8;
        unsafe {
            let size = num_pages * PAGE_SIZE;
            let mut _contents: *mut libc::c_void = mem::uninitialized();
            libc::posix_memalign(&mut _contents, PAGE_SIZE, size);
            // We should probably check that posix_memalign worked by making sure _contents isn't
            // NULL?
            libc::mprotect(_contents,
                           size,
                           libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);

            contents = mem::transmute(_contents);
        }

        JitMemory { contents: contents }
    }
}

impl Index<usize> for JitMemory {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.contents.offset(_index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe { &mut *self.contents.offset(_index as isize) }
    }
}
