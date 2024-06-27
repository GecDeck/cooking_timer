#[cfg(test)]
use super::*;

#[test]
fn test_timer_display() {
    let mut timer: Timer = Timer::default();
    timer.elapsed = 1.0;

    assert_eq!("00:00:01", &format!("{}", timer));
}

#[test]
fn test_timer_time() {
    let mut timer: Timer = Timer::default();

    let mut running: bool = true;
    while running {
        let delta: f64 = timer.frame_start.elapsed().as_secs_f64();
        timer.frame_start = Instant::now();

        if timer.ticking {
            timer.elapsed += delta * 2.0;
            // Double delta to make the test run faster
            //  This shouldn't affect the results
        }

        if timer.frame_count == FPS * 58 / 2 {
            assert_eq!("00:00:58", &format!("{}", timer));
        }
        if timer.frame_count == FPS * 70 / 2 {
            assert_eq!("00:01:10", &format!("{}", timer));
            running = false;
        }
        if timer.frame_count > FPS * 80 / 2 {
            assert!(false);
        }

        timer.frame_count += 1;

        let mut frame_time: f64 = timer.frame_start.elapsed().as_secs_f64();
        while frame_time < 1.0 / FPS as f64 {
            frame_time = timer.frame_start.elapsed().as_secs_f64();
        }
    }
}
