use vararg::vararg;

#[vararg]
fn vararg_func_1<const L: usize>(arr: [&str; L]) -> String {
    format!("first:\nlast: {}", arr.join(","))
}

#[test]
fn basic_0_first_0_last() {
    assert_eq!(vararg_func_1([]), vararg_func_1!());
}

#[test]
fn basic_0_first_1_last() {
    assert_eq!(vararg_func_1(["1"]), vararg_func_1!("1"));
}

#[test]
fn basic_0_first_2_last() {
    assert_eq!(vararg_func_1(["1", "2"]), vararg_func_1!("1", "2"));
}

#[test]
fn basic_0_first_many_last() {
    assert_eq!(
        vararg_func_1(["1", "2", "3", "4", "5"]),
        vararg_func_1!("1", "2", "3", "4", "5")
    );
}

#[vararg]
fn vararg_func_2<const L: usize>(a: &str, arr: [&str; L]) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[test]
fn basic_1_first_0_last() {
    assert_eq!(vararg_func_2("a", []), vararg_func_2!("a"));
}

#[test]
fn basic_1_first_1_last() {
    assert_eq!(vararg_func_2("a", ["1"]), vararg_func_2!("a", "1"));
}

#[test]
fn basic_1_first_2_last() {
    assert_eq!(
        vararg_func_2("a", ["1", "2"]),
        vararg_func_2!("a", "1", "2")
    );
}

#[test]
fn basic_1_first_many_last() {
    assert_eq!(
        vararg_func_2("a", ["1", "2", "3", "4", "5"]),
        vararg_func_2!("a", "1", "2", "3", "4", "5")
    );
}

#[vararg]
fn vararg_func_3<const L: usize>(a: &str, b: &str, arr: [&str; L]) -> String {
    format!("first: {},{}\nlast: {}", a, b, arr.join(","))
}

#[test]
fn basic_2_first_0_last() {
    assert_eq!(vararg_func_3("a", "b", []), vararg_func_3!("a", "b"));
}

#[test]
fn basic_2_first_1_last() {
    assert_eq!(
        vararg_func_3("a", "b", ["1"]),
        vararg_func_3!("a", "b", "1")
    );
}

#[test]
fn basic_2_first_2_last() {
    assert_eq!(
        vararg_func_3("a", "b", ["1", "2"]),
        vararg_func_3!("a", "b", "1", "2")
    );
}

#[test]
fn basic_2_first_many_last() {
    assert_eq!(
        vararg_func_3("a", "b", ["1", "2", "3", "4", "5"]),
        vararg_func_3!("a", "b", "1", "2", "3", "4", "5")
    );
}

#[vararg]
fn vararg_func_4<const L: usize>(
    a: &str,
    b: &str,
    c: &str,
    d: &str,
    e: &str,
    arr: [&str; L],
) -> String {
    format!(
        "first: {},{},{},{},{}\nlast: {}",
        a,
        b,
        c,
        d,
        e,
        arr.join(",")
    )
}

#[test]
fn basic_many_first_0_last() {
    assert_eq!(
        vararg_func_4("a", "b", "c", "d", "e", []),
        vararg_func_4!("a", "b", "c", "d", "e")
    );
}

#[test]
fn basic_many_first_1_last() {
    assert_eq!(
        vararg_func_4("a", "b", "c", "d", "e", ["1"]),
        vararg_func_4!("a", "b", "c", "d", "e", "1")
    );
}

#[test]
fn basic_many_first_2_last() {
    assert_eq!(
        vararg_func_4("a", "b", "c", "d", "e", ["1", "2"]),
        vararg_func_4!("a", "b", "c", "d", "e", "1", "2")
    );
}

#[test]
fn basic_many_first_many_last() {
    assert_eq!(
        vararg_func_4("a", "b", "c", "d", "e", ["1", "2", "3", "4", "5"]),
        vararg_func_4!("a", "b", "c", "d", "e", "1", "2", "3", "4", "5")
    );
}
