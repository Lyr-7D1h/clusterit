use ssh::*;

fn main() {
    let opt = Opt::from_args();

    SimpleLogger::new().with_level(opt.loglevel).init().unwrap();

    let mut session = Session::new().unwrap();
    session.set_host("192.168.2.3").unwrap();
    session.parse_config(None).unwrap();
    session.connect().unwrap();
    println!("{:?}", session.is_server_known());
    session.userauth_publickey_auto(None).unwrap();
}
