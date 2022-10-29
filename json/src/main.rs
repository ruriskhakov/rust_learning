fn main() {
    let o_json = r#"["foo",{"bar": ["baz", null, 1.0, 2]}]"#;
    let o: serde_json::Value = serde_json::from_str(o_json).unwrap();
    dbg!(&o[1]["bar"]);
    println!("{}", &o[1]["bar"]);
}
