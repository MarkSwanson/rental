#[macro_use]
extern crate rental;

extern crate capnp;

pub mod point_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/point_capnp.rs"));
}

use capnp::{message, serialize_packed};
use point_capnp::point;

rental! {
    pub mod rent_point {
        use point_capnp::point;
        use capnp::message::{self, Reader};
        use capnp::serialize::{OwnedSegments};

        #[rental]
        pub struct RentContextRead {
            reader: Box<Reader<OwnedSegments>>,
            point: point::Reader<'reader>,
        }
    }
}


// Please ignore the fact that this blocks (stdout/stdin issue).
#[test]
fn rent_ref_capnproto() {

    // boiler plate: create a builder, create a reader, then rental stuff.
    let mut message = ::capnp::message::Builder::new_default();
    
    {
        let mut point_builder = message.init_root::<point::Builder>();
        point_builder.set_i(42);
    }
    serialize_packed::write_message(&mut ::std::io::stdout(), &message);
    let stdin = ::std::io::stdin();
    let reader = serialize_packed::read_message(&mut stdin.lock(),
        message::ReaderOptions::new()).unwrap();

    let rent_context = match rent_point::RentContextRead::try_new(
        Box::new(reader),
        |reader| {
            reader.get_root::<point::Reader>()
        },
    ) {
        Ok(rent_ctx) => rent_ctx,
        Err(e) => panic!("todo: {:?}", e.0)
    };

    // Question 1: how do we get a reference to the suffix? This fails:
    /*let impossible1 = rent_context.ref_rent(|point| {
        point
    });*/

    // Question 2: return a Reader/Iterator - why is the returned reference wrong?
    //   Guess: get_strings() transfers ownership and the reference lifetime gets lost.
    //     If true, any workaround?
    //let impossible2 = rent_context.ref_rent(|point| {
        // fails:
        //point.get_strings().unwrap()
        // also fails:
        //&(point.get_strings().unwrap())
    //});

    // This compiles fine, and would work fine (test harness stdout/stdin issue).
    let s = rent_context.ref_rent(|point| {
        let strings = point.get_strings().unwrap();
        for s in strings.iter() {
            return s.unwrap()
        }
        panic!("no string")
    });

    println!("s: {}", s);

}


