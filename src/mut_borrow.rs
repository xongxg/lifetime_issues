// struct Interfaces<'b, 'a: 'b> {
//     manager: &'b mut Manager<'a>,
// }

/// 对抗Rust编译检查 | 生命周期声明过大【上篇】
/// https://zhuanlan.zhihu.com/p/444395977

struct Interfaces<'text, 'manager> {
    manager: &'manager mut Manager<'text>,
}

// impl<'b, 'a: 'b> Interfaces<'b, 'a> {
//     pub fn noop(self) {
//         println!("interface consumed");
//     }
// }

impl<'text, 'manager> Interfaces<'text, 'manager> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

// struct Manager<'a> {
//     text: &'a str,
// }

struct Manager<'text> {
    text: &'text str,
}

// struct List<'a> {
//     manager: Manager<'a>,
// }

struct List<'text> {
    manager: Manager<'text>,
}

impl<'text> List<'text> {
    pub fn get_interfaces<'manager>(&'manager mut self) -> Interfaces<'text, 'manager> {
        Interfaces {
            manager: &mut self.manager,
        }
    }
}

#[test]
fn test_borrow() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interfaces().noop();
    println!("Interface should be dropped here and the borrow released");

    // this fails because inmutable/mutable borrow
    // but Interface should be already dropped here and the borrow released
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

///
/// 对抗Rust编译检查 | 生命周期声明过大【下篇】
/// https://zhuanlan.zhihu.com/p/445777626
fn bar(writer: &mut Writer) {
    baz(&mut writer.indent());
    writer.write("world");
}

fn baz(writer: &mut Writer) {
    writer.write("hello");
}

pub struct Writer<'a> {
    target: &'a mut String,
    indent: usize,
}

impl<'a> Writer<'a> {
    fn indent(&mut self) -> Writer {
        Writer {
            target: self.target,
            indent: self.indent + 1,
        }
    }

    fn write(&mut self, s: &str) {
        for _ in 0..self.indent {
            self.target.push(' ');
        }
        self.target.push_str(s);
        self.target.push('\n');
    }
}

#[test]
fn test_writer() {}
