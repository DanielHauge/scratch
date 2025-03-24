include!(concat!(env!("OUT_DIR"), "/small_generated.rs"));

fn main() {
    let mut b = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let name = b.create_string("Yo");
    let id = 12345;
    let d = data::Data::create(
        &mut b,
        &data::DataArgs {
            name: Some(name),
            id,
        },
    );
    b.finish(d, None);

    let bytes = b.finished_data();
    let deserialized = data::root_as_data(bytes).unwrap();
    println!("{:?}", deserialized);
}
