# Rust_Polymorphism

This design allows for dependency injection in Rust.

#### Here we are injecting a normal Server object into Wrapper and calling do_something()

```
let ms = MyServer {
        test: String::from("test"),
    };
    let wr = Wrapper::<MyServer>::new(ms);
    let rtn: bool = wr.do_something();

```


#### Here we inject a MockServer object into Wrapper and call do_something()

```
let mwr = Wrapper::<MockMyServer>::new(MockMyServer {
        test: String::from("mock test"),
    });

    let rtn: bool = mwr.do_something();

```