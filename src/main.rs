extern {
    fn draw(data: *const u8, counter: i32);
}

pub struct Gamestate {
    data: Vec<u8>,
    counter: i32,
}

impl Gamestate {
    pub fn new() -> Self {
        Gamestate {
            // NOTE: changing this to `66_000` will make the web
            // client show zeros instead of the initial values set
            // here as well as in `update`.
            data: vec![5; 65_000],
            counter: 0,
        }
    }
}


#[no_mangle]
pub fn start() -> *mut Gamestate {
    let state = Box::new(Gamestate::new());
    Box::into_raw(state)
}

#[no_mangle]
pub fn update(state_ptr: *mut Gamestate) {
    let mut state = unsafe { Box::from_raw(state_ptr) };

    // update the counter
    state.counter += 1;
    let counter = state.counter;

    // update data
    for num in state.data.iter_mut().take(2) {
        *num = 42 + counter as u8;
    }

    let data_ptr = state.data.as_ptr();
    std::mem::forget(state);

    unsafe {
        draw(data_ptr, counter)
    }
}

fn main() {}
