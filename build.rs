
extern crate capnpc;

fn main() {

    ::capnpc::CompilerCommand::new()
        .file("schema/point.capnp")
        .run()
        .expect("compiling schema/point.capnp");
}

