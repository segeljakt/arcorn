use arcorn_decl_macros::unwrap;

#[test]
fn test() {

    enum Foo {
        Bar(i32),
        Baz(u32),
    }

    use Foo::Baz;
    let baz = Baz(3);

    assert_eq!(3, unwrap!(Some(3), Some));
    assert_eq!(3, unwrap!(Option::Some(3), Option::Some));
    assert_eq!(3, unwrap!(Foo::Bar(3), Foo::Bar));
    assert_eq!(3, unwrap!(baz, Baz));
}
