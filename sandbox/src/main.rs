fn main() {
    let a: i32 = 10;
    let b: &i32 = &a;

    fn square<'a>(x: &'a i32) -> i32 {
        x * x
    }

    square(b);

    struct Foo<'a> {
        x: &'a i32,
    }

    Foo { x: &a };
}
