extern crate hdf5_types;
#[macro_use]
extern crate hdf5_types_derive;

#[derive(H5Type)]
//~^ ERROR proc-macro derive
//~^^ HELP Cannot derive H5Type for unit structs
struct Foo;
