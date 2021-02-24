pub trait Server {
    fn do_something(&self) -> bool;
}

pub struct MyServer {
    pub test: String,
}

impl Server for MyServer {
    fn do_something(&self) -> bool {
        let mut rtn = false;
        if self.test == "test" {
            rtn = true;
        }
        rtn
    }
}

pub struct Wrapper<T> {
    server: T,
}

impl<T> Wrapper<T>
where
    T: Server,
{
    fn new(t: T) -> Self {
        Wrapper { server: t }
    }

    fn do_something(&self) -> bool {
        self.server.do_something()
    }
}
fn main() {
    let ms = MyServer {
        test: String::from("test"),
    };
    let wr = Wrapper::<MyServer>::new(ms);
    let rtn: bool = wr.do_something();
    //let rtn: bool = ms.do_something();
    println!("Do something! {}", rtn);
}
