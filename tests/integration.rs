use minigrep;

#[test]
fn it_works() {
    let args = [
        String::from("minigrep"),
        String::from("body"),
        String::from("poem.txt"),
    ];

    let config = minigrep::Config::new(&args, false).unwrap();

    assert!(minigrep::run(config).is_ok());
}

#[test]
fn it_does_not_work() {
    let args = [
        String::from("minigrep"),
        String::from("body"),
        String::from("doesnt_exist.txt"),
    ];

    let config = minigrep::Config::new(&args, false).unwrap();

    assert!(minigrep::run(config).is_err());
}
