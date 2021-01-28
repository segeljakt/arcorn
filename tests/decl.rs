#[test]
fn test_unwrap() {
    use arcorn_decl_macros::unwrap;

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

#[test]
fn test_is() {
    use arcorn_decl_macros::is;

    enum Foo {
        Bar(i32),
        Baz(u32),
    }

    use Foo::Baz;
    let baz = Baz(3);

    assert_eq!(true, is!(Some(3), Some));
    assert_eq!(true, is!(Option::Some(3), Option::Some));
    assert_eq!(true, is!(Foo::Bar(3), Foo::Bar));
    assert_eq!(true, is!(baz, Baz));
}
