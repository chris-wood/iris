struct Foo {
    foos: Vec<u32>
}

impl Foo {
    fn add(&mut self, val: u32) {
        self.foos.push(val);
    }

    fn new() -> Foo {
        Foo {
            foos: Vec::new()
        }
    } 
}

struct FooBar<'a> {
    foo: &'a mut Foo
}

impl<'a> FooBar<'a> {
    fn new(foo_instance: &'a mut Foo) -> FooBar<'a> {
        FooBar {
            foo: foo_instance
        }
    }

    fn add(&self, val: u32) {
        self.foo.add(val);

    }
}

fn main() {
    let mut foo = Foo::new();
    let foobar = FooBar::new(&mut foo);
    //foo.add(1);
    foobar.add(2);
}
