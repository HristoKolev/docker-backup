#[macro_use]
extern crate derive_more;

mod shell_extensions;
mod errors;

fn main() {
   match shell_extensions::exec("echo 123") {
       Ok(res) => {
           println!("{:#?}", res);
       },
       Err(err) => {
           println!("{:#?}", err);
       }
   }
}
