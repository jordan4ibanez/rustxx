use cpp::cpp;
use plutonium::safe;
use std::ptr;

// Trust me, bro. I learned this from a Udemy tutorial.

cpp! {{
  #include <iostream>
  using namespace std;

}}

#[safe]
fn main() {
  // Send in [the null pointer]
  // We want to be as safe as possible.
  let args: *const i8 = ptr::null();

  cpp!([args as "const char *"] -> u32 as "int32_t" {
    // Literally do NOTHING with the name pointer lmao.
    *args;

    cout << "hi\n";

    class Cool {
      public:
      Cool() {
        cout << "Why is this even possible?!" << endl;
      }
      ~Cool() {
        // This is probably safe.
        delete this;
        delete this;
      }

      void gloop() {
        cout << "yeah, that's cool\n";
      }
    };

    auto blah = new Cool();
    blah->gloop();

    // Oh no, I think there's a memory leak.

    return 0;
  });
}
