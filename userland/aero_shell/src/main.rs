/*
* Copyright (C) 2021-2022 The Aero Project Developers.
 *
 * This file is part of The Aero Project.
 *
 * Aero is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Aero is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Aero. If not, see <https://www.gnu.org/licenses/>.
 */


use aero_syscall::*;

const MAGENTA_FG: &str = "\x1b[1;35m";
const RESET: &str = "\x1b[0m";
const UWUFETCH_LOGO: &str = r#"
*insert uwu-chan here*
"#;

macro_rules! error {
    ($($arg:tt)*) => {
        std::print!("\x1b[1;31merror\x1b[0m: {}\n", format_args!($($arg)*))
    }
}

fn repl(history: &mut Vec<String>) -> Result<(), AeroSyscallError> {
    let mut hostname_buf = [0; 64];
    let mut pwd_buffer = [0; 1024];
    let mut cmd_buffer = [0; 1024];

    let hostname_len = sys_gethostname(&mut hostname_buf)?;
    let hostname = unsafe { core::str::from_utf8_unchecked(&hostname_buf[0..hostname_len]) };
    let username = "root"; // TODO: Unhardcode this at some point :^)

    let pwd_length = sys_getcwd(&mut pwd_buffer)?;
    let pwd = unsafe { core::str::from_utf8_unchecked(&pwd_buffer[0..pwd_length]) };

    print!(
        "\x1b[1;32m{}@{}\x1b[0m:\x1b[1;34m{}\x1b[0m ",
        username, hostname, pwd
    );

    let cmd_length = sys_read(0, &mut cmd_buffer)?;
    let cmd_string = unsafe { core::str::from_utf8_unchecked(&cmd_buffer[0..cmd_length]).trim() };

    let mut args = cmd_string.split_whitespace();

    if let Some(cmd) = args.next() {
        history.push(cmd_string.to_string());

        match cmd {
            "ls" => list_directory(args.next().unwrap_or("."))?,
            "pwd" => println!("{}", pwd),
            "cd" => {
                sys_chdir(args.next().unwrap_or(".."))?;
            }
            "exit" => match args.next() {
                Some(status) => match status.parse::<usize>() {
                    Ok(exit_code) => sys_exit(exit_code),
                    Err(_) => error!("exit: invalid operand"),
                },
                None => sys_exit(0),
            },
            "clear" => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
            "dmsg" => print_kernel_log()?,
            "uwufetch" => uwufetch()?,
            "uname" => uname()?,
            "history" => {
                for entry in history.iter() {
                    println!("{}", entry);
                }
            }

            "uwutest" => {
                // TODO: Make a uwutest program that is executed by the kernel
                // if the test kernel is built instead of randomly bloating the shell
                // with tests :).

                // let mut pipe = [0usize; 2];
                // sys_pipe(&mut pipe, OpenFlags::empty())?;

                // let child = sys_fork()?;

                // if child == 0 {
                //     sys_close(pipe[0])?; // close the read end

                //     let mut buffer = [0; 1024];
                //     let length = sys_read(0, &mut buffer)?;

                //     sys_write(pipe[1], &buffer[0..length])?;

                //     sys_close(pipe[1])?; // close the write end
                //     sys_exit(0)
                // } else {
                //     let mut status = 0;
                //     sys_waitpid(child, &mut status, 0)?;

                //     sys_close(pipe[1])?; // close the write end

                //     let mut buffer = [0; 1024];
                //     let length = sys_read(pipe[0], &mut buffer)?;

                //     println!("{}", unsafe {
                //         core::str::from_utf8_unchecked(&buffer[0..length])
                //     });

                //     sys_close(pipe[0])?; // close the read end
                // }

                // let fb = sys_open("/dev/fb", OpenFlags::O_RDWR)?;

                // let buffer = &[u32::MAX; (1024 * 768)];
                // let casted = buffer.as_ptr() as *mut u8;
                // let casted = unsafe { core::slice::from_raw_parts(casted, (1024 * 768) as usize) };

                // println!("writing to fb");
                // sys_write(fb, casted)?;
                // sys_close(fb)?;

                uwutest()?;
            }

            "pid" => {
                println!("{}", sys_getpid()?);
            }

            "uptime" => {
                print!("{}", get_uptime()?);
            }
            "rmdir" => match args.next() {
                Some(path) => {
                    sys_rmdir(path)?;
                }
                None => error!("rmdir: missing operand"),
            },

            _ => {
                let child = sys_fork()?;

                if child == 0 {
                    let args = args.collect::<Vec<_>>();
                    let mut argv = Vec::new();

                    argv.push(cmd);
                    argv.extend(args);

                    let argv = argv.as_slice();

                    match sys_exec(&["/bin/", cmd].join(""), argv, &[]) {
                        Ok(_) => core::unreachable!(),
                        Err(AeroSyscallError::EISDIR) => error!("{}: is a directory", cmd),
                        Err(AeroSyscallError::ENOENT) => error!("{}: command not found", cmd),
                        Err(err) => error!("{}: {:?}", cmd, err),
                    }

                    sys_exit(0);
                } else {
                    // Wait for the child
                    let mut status = 0;
                    sys_waitpid(child, &mut status, 0)?;

                    let exit_code = status & 0xff;

                    if exit_code != 0 {
                        error!("{} exited with a non-zero status code: {} ", cmd, exit_code);
                    }
                }
            }
        }
    }

    Ok(())
}

fn list_directory(path: &str) -> Result<(), AeroSyscallError> {
    let dir_fd = sys_open(path, OpenFlags::O_DIRECTORY)?;

    loop {
        let mut dir_ents_buffer = [0; 1024];

        let size = sys_getdents(dir_fd, &mut dir_ents_buffer)?;

        if size == 0 {
            break;
        }

        let dir_entry = unsafe { &*(dir_ents_buffer.as_ptr() as *const SysDirEntry) };

        let name_start = core::mem::size_of::<SysDirEntry>();
        let name_end = dir_entry.reclen;

        let name =
            unsafe { core::str::from_utf8_unchecked(&dir_ents_buffer[name_start..name_end]) };

        print!("{} ", name);
    }

    println!();

    Ok(())
}

fn cat_file(path: Option<&str>) -> Result<(), AeroSyscallError> {
    // On the `None` arm we default to 0 to take input from stdin.
    // This is the behaviour of `cat` that comes with any modern Linux distro.
    let fd = match path {
        Some(path) => sys_open(path, OpenFlags::O_RDONLY)?,
        None => 0,
    };

    sys_seek(fd, 0, SeekWhence::SeekSet)?;

    let mut buffer = [0; 1024];

    loop {
        let length = sys_read(fd, &mut buffer)?;

        if length == 0 {
            break;
        }

        let contents = unsafe { core::str::from_utf8_unchecked(&buffer[0..length]) };

        print!("{}", contents);
    }

    if fd != 0 {
        sys_close(fd)?;
    }

    print!("\n");
    Ok(())
}

fn print_kernel_log() -> Result<(), AeroSyscallError> {
    // dmsg is just a wrapper around `cat /dev/kmsg`
    // TODO: Add colored output back :^)

    cat_file(Some("/dev/kmsg"))
}

fn uwutest() -> Result<(), AeroSyscallError> {
    const SOCKET_PATH: &str = "/socket.unix";

    let socket = sys_socket(AF_UNIX, SOCK_STREAM, 0)?;

    println!("uwutest: socket file descriptor is: {}", socket);

    let mut sock_addr = SocketAddrUnix {
        family: AF_UNIX as i16,
        path: [0; 108],
    };

    sock_addr.path[0..SOCKET_PATH.len()].copy_from_slice(SOCKET_PATH.as_bytes());

    sys_bind(
        socket,
        &SocketAddr::Unix(sock_addr),
        core::mem::size_of::<SocketAddrUnix>() as u32,
    )?;

    println!("uwutest: successfully bound to {:?}", SOCKET_PATH);
    list_directory(".")?;

    Ok(())
}

fn get_uptime() -> Result<String, AeroSyscallError> {
    let mut info = unsafe { core::mem::zeroed() };

    sys_info(&mut info)?;

    let mut uptime = String::new();

    let days = info.uptime / (3600 * 24);
    let hours = info.uptime % (3600 * 24) / 3600;
    let minutes = info.uptime % 3600 / 60;
    let seconds = info.uptime % 60;

    if days > 0 {
        uptime.push_str(&format!(
            "{} day{}, ",
            days,
            if days == 1 { "" } else { "s" }
        ));
    }

    if hours > 0 {
        uptime.push_str(&format!(
            "{} hour{}, ",
            hours,
            if hours == 1 { "" } else { "s" },
        ));
    }

    if minutes > 0 {
        uptime.push_str(&format!(
            "{} minute{}, ",
            minutes,
            if minutes == 1 { "" } else { "s" }
        ));
    }

    uptime.push_str(&format!(
        "{} second{}",
        seconds,
        if seconds == 1 { "" } else { "s" }
    ));

    Ok(uptime)
}

fn uwufetch() -> Result<(), AeroSyscallError> {
    let print_prefix = |prefix| {
        print!("{}{}{}: ", MAGENTA_FG, prefix, RESET);
    };

    let mut hostname_buf = [0; 64];

    let hostname_len = sys_gethostname(&mut hostname_buf)?;
    let hostname = unsafe { core::str::from_utf8_unchecked(&hostname_buf[0..hostname_len]) };
    let username = "root"; // TODO: Unhardcode this at some point :^)

    for (i, line) in UWUFETCH_LOGO.lines().skip(1).enumerate() {
        print!(" {}{:<19}{}", MAGENTA_FG, line, RESET);

        if i == 1 {
            println!("{}@{}", username, hostname);
        } else if i == 2 {
            println!("{}", "-".repeat(username.len() + hostname.len() + 1));
        } else if i == 3 {
            print_prefix("OS");
            println!("Uwux");
        } else if i == 4 {
            let tty_fd = sys_open("/dev/tty", OpenFlags::O_RDONLY)?;

            let mut resolution = WinSize::default();
            sys_ioctl(tty_fd, TIOCGWINSZ, &mut resolution as *mut _ as usize)?;

            sys_close(tty_fd)?;

            print_prefix("Resolution");
            println!("{}x{}", resolution.ws_xpixel, resolution.ws_ypixel);
        } else if i == 5 {
            let mut uname_info = Utsname::default();

            sys_uname(&mut uname_info)?;

            print_prefix("Kernel");
            println!(
                "{} {} ({})",
                uname_info.name(),
                uname_info.version(),
                uname_info.machine()
            );
        } else if i == 6 {
            print_prefix("Uptime");
            println!("{}", get_uptime()?);
        } else {
            println!();
        }
    }

    Ok(())
}

fn uname() -> Result<(), AeroSyscallError> {
    let mut uname_info = Utsname::default();

    sys_uname(&mut uname_info)?;

    println!(
        "{} {} {} {} {}",
        uname_info.name(),
        uname_info.nodename(),
        uname_info.release(),
        uname_info.version(),
        uname_info.machine()
    );

    Ok(())
}

fn main() {
    let mut history = vec![];

    loop {
        if let Err(error) = repl(&mut history) {
            error!("{:?}", error);
        }
    }
}
