#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use bonk::{bonk, Bonk};
pub struct Attacker;
impl Bonk for Attacker {
    fn new(_id: usize) -> Self {
        Self {}
    }
    fn check(&mut self, buf: &[u8]) -> bool {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &match (&std::str::from_utf8(&buf).unwrap(),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                },
            ));
        };
        false
    }
}
fn main() {
    let flag = ::std::sync::Arc::new(::std::sync::atomic::AtomicBool::new(false));
    const MAX_SIZE: usize = 10usize;
    static CLASS_1: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();
    let flag_0 = flag.clone();
    let t_0 = ::std::thread::spawn(move || {
        let mut flag = flag_0;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(0usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_1 = flag.clone();
    let t_1 = ::std::thread::spawn(move || {
        let mut flag = flag_1;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(1usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_2 = flag.clone();
    let t_2 = ::std::thread::spawn(move || {
        let mut flag = flag_2;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(2usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_3 = flag.clone();
    let t_3 = ::std::thread::spawn(move || {
        let mut flag = flag_3;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(3usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_4 = flag.clone();
    let t_4 = ::std::thread::spawn(move || {
        let mut flag = flag_4;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(4usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_5 = flag.clone();
    let t_5 = ::std::thread::spawn(move || {
        let mut flag = flag_5;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(5usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_6 = flag.clone();
    let t_6 = ::std::thread::spawn(move || {
        let mut flag = flag_6;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(6usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_7 = flag.clone();
    let t_7 = ::std::thread::spawn(move || {
        let mut flag = flag_7;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(7usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_8 = flag.clone();
    let t_8 = ::std::thread::spawn(move || {
        let mut flag = flag_8;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(8usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_9 = flag.clone();
    let t_9 = ::std::thread::spawn(move || {
        let mut flag = flag_9;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(9usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_10 = flag.clone();
    let t_10 = ::std::thread::spawn(move || {
        let mut flag = flag_10;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(10usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_11 = flag.clone();
    let t_11 = ::std::thread::spawn(move || {
        let mut flag = flag_11;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(11usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_12 = flag.clone();
    let t_12 = ::std::thread::spawn(move || {
        let mut flag = flag_12;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(12usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_13 = flag.clone();
    let t_13 = ::std::thread::spawn(move || {
        let mut flag = flag_13;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(13usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_14 = flag.clone();
    let t_14 = ::std::thread::spawn(move || {
        let mut flag = flag_14;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(14usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let flag_15 = flag.clone();
    let t_15 = ::std::thread::spawn(move || {
        let mut flag = flag_15;
        let mut buf = [0u8; MAX_SIZE];
        let mut bonker = <Attacker as ::bonk::Bonk>::new(15usize);
        for c_0 in CLASS_1[0usize..26usize].iter().copied() {
            buf[0usize] = c_0;
            for c_1 in CLASS_1[0usize..26usize].iter().copied() {
                buf[1usize] = c_1;
                for c_2 in CLASS_1[0usize..26usize].iter().copied() {
                    buf[2usize] = c_2;
                    for c_3 in CLASS_1[0usize..26usize].iter().copied() {
                        buf[3usize] = c_3;
                        for c_4 in CLASS_1[0usize..26usize].iter().copied() {
                            buf[4usize] = c_4;
                            for c_5 in CLASS_1[0usize..26usize].iter().copied() {
                                buf[5usize] = c_5;
                                for c_6 in CLASS_1[0usize..26usize].iter().copied() {
                                    buf[6usize] = c_6;
                                    for c_7 in CLASS_1[0usize..26usize].iter().copied() {
                                        buf[7usize] = c_7;
                                        for c_8 in CLASS_1[0usize..26usize].iter().copied() {
                                            buf[8usize] = c_8;
                                            for c_9 in CLASS_1[0usize..26usize].iter().copied() {
                                                buf[9usize] = c_9;
                                                if flag.load(::std::sync::atomic::Ordering::Relaxed)
                                                    || <Attacker as ::bonk::Bonk>::check(
                                                        &mut bonker,
                                                        &buf[0..10usize],
                                                    )
                                                {
                                                    flag.store(
                                                        true,
                                                        ::std::sync::atomic::Ordering::Relaxed,
                                                    );
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    t_0.join().unwrap();
    t_1.join().unwrap();
    t_2.join().unwrap();
    t_3.join().unwrap();
    t_4.join().unwrap();
    t_5.join().unwrap();
    t_6.join().unwrap();
    t_7.join().unwrap();
    t_8.join().unwrap();
    t_9.join().unwrap();
    t_10.join().unwrap();
    t_11.join().unwrap();
    t_12.join().unwrap();
    t_13.join().unwrap();
    t_14.join().unwrap();
    t_15.join().unwrap();
}
