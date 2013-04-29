fn main() {
    /* A simple loop */
    loop {
        // A tricky calculation
        if universe::recalibrate() {
            return;
        }
    }
}

// WARNING: this file does not compile as universe::recalibrate does not exist
