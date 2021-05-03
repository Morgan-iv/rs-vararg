use vararg::vararg;

#[vararg(name = name_1, type = vec)]
fn vararg_func_1(a: &str, arr: Vec<&str>) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[vararg(type = slice, name = name_2)]
fn vararg_func_2(a: &str, arr: &[&str]) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[vararg(name = name_3, type = array,)]
fn vararg_func_3<const L: usize>(a: &str, arr: [&str; L]) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[test]
fn attrs_all_basic_0_last() {
    assert_eq!(vararg_func_1("a", vec![]), name_1!("a"));
}

#[test]
fn attrs_all_basic_1_last() {
    assert_eq!(vararg_func_1("a", vec!["1"]), name_1!("a", "1"));
}

#[test]
fn attrs_all_basic_2_last() {
    assert_eq!(vararg_func_1("a", vec!["1", "2"]), name_1!("a", "1", "2"));
}

#[test]
fn attrs_all_basic_many_last() {
    assert_eq!(
        vararg_func_1("a", vec!["1", "2", "3", "4", "5"]),
        name_1!("a", "1", "2", "3", "4", "5")
    );
}

#[test]
fn attrs_all_reverse_0_last() {
    assert_eq!(vararg_func_2("a", &[]), name_2!("a"));
}

#[test]
fn attrs_all_reverse_1_last() {
    assert_eq!(vararg_func_2("a", &["1"]), name_2!("a", "1"));
}

#[test]
fn attrs_all_reverse_2_last() {
    assert_eq!(vararg_func_2("a", &["1", "2"]), name_2!("a", "1", "2"));
}

#[test]
fn attrs_all_reverse_many_last() {
    assert_eq!(
        vararg_func_2("a", &["1", "2", "3", "4", "5"]),
        name_2!("a", "1", "2", "3", "4", "5")
    );
}

#[test]
fn attrs_all_trailingcomma_0_last() {
    assert_eq!(vararg_func_3("a", []), name_3!("a"));
}

#[test]
fn attrs_all_trailingcomma_1_last() {
    assert_eq!(vararg_func_3("a", ["1"]), name_3!("a", "1"));
}

#[test]
fn attrs_all_trailingcomma_2_last() {
    assert_eq!(vararg_func_3("a", ["1", "2"]), name_3!("a", "1", "2"));
}

#[test]
fn attrs_all_trailingcomma_many_last() {
    assert_eq!(
        vararg_func_3("a", ["1", "2", "3", "4", "5"]),
        name_3!("a", "1", "2", "3", "4", "5")
    );
}

#[vararg(type = array)]
fn vararg_func_4<const L: usize>(a: &str, arr: [&str; L]) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[test]
fn attrs_array_0_last() {
    assert_eq!(vararg_func_4("a", []), vararg_func_4!("a"));
}

#[test]
fn attrs_array_1_last() {
    assert_eq!(vararg_func_4("a", ["1"]), vararg_func_4!("a", "1"));
}

#[test]
fn attrs_array_2_last() {
    assert_eq!(
        vararg_func_4("a", ["1", "2"]),
        vararg_func_4!("a", "1", "2")
    );
}

#[test]
fn attrs_array_many_last() {
    assert_eq!(
        vararg_func_4("a", ["1", "2", "3", "4", "5"]),
        vararg_func_4!("a", "1", "2", "3", "4", "5")
    );
}

#[vararg(type = slice)]
fn vararg_func_5(a: &str, arr: &[&str]) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[test]
fn attrs_slice_0_last() {
    assert_eq!(vararg_func_5("a", &[]), vararg_func_5!("a"));
}

#[test]
fn attrs_slice_1_last() {
    assert_eq!(vararg_func_5("a", &["1"]), vararg_func_5!("a", "1"));
}

#[test]
fn attrs_slice_2_last() {
    assert_eq!(
        vararg_func_5("a", &["1", "2"]),
        vararg_func_5!("a", "1", "2")
    );
}

#[test]
fn attrs_slice_many_last() {
    assert_eq!(
        vararg_func_5("a", &["1", "2", "3", "4", "5"]),
        vararg_func_5!("a", "1", "2", "3", "4", "5")
    );
}

#[vararg(type = vec)]
fn vararg_func_6(a: &str, arr: Vec<&str>) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[test]
fn attrs_vec_0_last() {
    assert_eq!(vararg_func_6("a", vec![]), vararg_func_6!("a"));
}

#[test]
fn attrs_vec_1_last() {
    assert_eq!(vararg_func_6("a", vec!["1"]), vararg_func_6!("a", "1"));
}

#[test]
fn attrs_vec_2_last() {
    assert_eq!(
        vararg_func_6("a", vec!["1", "2"]),
        vararg_func_6!("a", "1", "2")
    );
}

#[test]
fn attrs_vec_many_last() {
    assert_eq!(
        vararg_func_6("a", vec!["1", "2", "3", "4", "5"]),
        vararg_func_6!("a", "1", "2", "3", "4", "5")
    );
}

#[vararg(name = other_name)]
fn vararg_func_7<const L: usize>(a: &str, arr: [&str; L]) -> String {
    format!("first: {}\nlast: {}", a, arr.join(","))
}

#[test]
fn attrs_name_0_last() {
    assert_eq!(vararg_func_7("a", []), other_name!("a"));
}

#[test]
fn attrs_name_1_last() {
    assert_eq!(vararg_func_7("a", ["1"]), other_name!("a", "1"));
}

#[test]
fn attrs_name_2_last() {
    assert_eq!(vararg_func_7("a", ["1", "2"]), other_name!("a", "1", "2"));
}

#[test]
fn attrs_name_many_last() {
    assert_eq!(
        vararg_func_7("a", ["1", "2", "3", "4", "5"]),
        other_name!("a", "1", "2", "3", "4", "5")
    );
}
