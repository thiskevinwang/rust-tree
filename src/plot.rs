use inline_python::{python, Context};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{
    cell::RefCell,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};

use std::fs::write;

pub fn plot() {
    // https://static.rust-lang.org/doc/master/std/sync/struct.Mutex.html
    let mutex = Arc::new(Mutex::new(Vec::<f64>::new()));
    let (tx, rx) = channel();
    for _ in 0..10000 {
        let (data, tx) = (Arc::clone(&mutex), tx.clone());
        thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let mut data = data.lock().unwrap();

            let start = Instant::now();
            sleep(Duration::from_millis(1));
            let duration = start.elapsed();
            data.push(duration.as_secs_f64() * 1e6);
            tx.send(()).unwrap();
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    // for _ in 0..1000 {
    //     let start = Instant::now();
    //     sleep(Duration::from_millis(1));
    //     let duration = start.elapsed();
    //     data.push(duration.as_secs_f64() * 1e6);
    // }

    let data = mutex.lock().unwrap().clone();

    /// for installing python packages, see https://github.com/PyO3/pyo3
    /// ```bash
    /// python -m venv .env
    /// source .env/bin/activate
    /// pip install matplotlib
    /// RUST_BACKTRACE=1 cargo run
    /// ```
    /// Improve spacing in matplotlib
    /// https://stackoverflow.com/questions/6541123/improve-subplot-size-spacing-with-many-subplots-in-matplotlib
    let c: Context = python! {
        import base64
        from io import BytesIO
        from matplotlib.figure import Figure

        fig = Figure()
        fig.subplots_adjust(left  = 0.125, top = 0.9, bottom = 0.1, hspace = 0.4)

        ax1, ax2 = fig.subplots(2, 1)

        ax1.plot('data, ".")
        ax1.set_title("measurements")
        ax1.set_ylabel("μs")

        ax2.hist('data, bins=250)
        ax2.set_title("histogram")
        ax2.set_xlabel("μs")

        buf = BytesIO()
        fig.savefig(buf, format="png")

        b64_output = base64.b64encode(buf.getbuffer()).decode("ascii")
        html_output = "<img src=\"data:image/png;base64,{b64_output}\">".format(b64_output=b64_output)
    };

    write("./src/out.md", c.get::<String>("html_output")).expect("Unable to write file");

    println!("{}", c.get::<String>("html_output"))
}
