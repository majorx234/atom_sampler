use bus::BusReader;
use ringbuf::{
    traits::{Consumer, Observer, Producer},
    HeapCons, HeapProd,
};
use std::{thread, time};

pub fn start_recording(
    mut ringbuffer_left_in: HeapCons<f32>,
    mut ringbuffer_right_in: HeapCons<f32>,
    mut ringbuffer_left_visual_in_opt: Option<HeapProd<(f32, f32)>>,
    mut ringbuffer_right_visual_in_opt: Option<HeapProd<(f32, f32)>>,
    mut rx_stop_rec: BusReader<bool>,
) -> std::thread::JoinHandle<(
    Vec<f32>,
    Vec<f32>,
    HeapCons<f32>,
    HeapCons<f32>,
    Option<HeapProd<(f32, f32)>>,
    Option<HeapProd<(f32, f32)>>,
)> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let wave_size = 192000;
        let mut wave_left = vec![0.0; wave_size];
        let mut wave_right = vec![0.0; wave_size];
        let mut vecpointer_left = 0;
        let mut vecpointer_right = 0;
        println!("start_recording: start rec");

        while run {
            let length_left = 1024.min(ringbuffer_left_in.occupied_len());
            let length_right = 1024.min(ringbuffer_right_in.occupied_len());
            if length_left == 0 && length_right == 0 {
                // TODO duration need to be dependen on sampling rate 48000/1024
                thread::sleep(time::Duration::from_millis(10));
            } else {
                if (vecpointer_left + length_left < wave_size)
                    && (vecpointer_right + length_right < wave_size)
                {
                    wave_left.splice(
                        vecpointer_left..vecpointer_left + length_left,
                        ringbuffer_left_in.pop_iter(),
                    );
                    let max_l = wave_left[vecpointer_left..vecpointer_left + length_left]
                        .iter()
                        .max_by(|a, b| a.total_cmp(b))
                        .unwrap_or(&0.0);
                    let min_l = wave_left[vecpointer_left..vecpointer_left + length_left]
                        .iter()
                        .min_by(|a, b| a.total_cmp(b))
                        .unwrap_or(&0.0);
                    vecpointer_left += length_left;

                    wave_right.splice(
                        vecpointer_right..vecpointer_right + length_right,
                        ringbuffer_right_in.pop_iter(),
                    );
                    let max_r = wave_right[vecpointer_right..vecpointer_right + length_right]
                        .iter()
                        .max_by(|a, b| a.total_cmp(b))
                        .unwrap_or(&0.0);
                    let min_r = wave_right[vecpointer_right..vecpointer_right + length_right]
                        .iter()
                        .min_by(|a, b| a.total_cmp(b))
                        .unwrap_or(&0.0);
                    vecpointer_right += length_right;

                    if let Some(ref mut ringbuffer_left_visual_in) = ringbuffer_left_visual_in_opt {
                        let _ = ringbuffer_left_visual_in.try_push((*max_l, *min_l));
                    }
                    if let Some(ref mut ringbuffer_right_visual_in) = ringbuffer_right_visual_in_opt
                    {
                        let _ = ringbuffer_right_visual_in.try_push((*max_r, *min_r));
                    }
                } else {
                    ringbuffer_left_in.clear();
                    ringbuffer_right_in.clear();
                    run = false;
                    println!("start_recording: wave is full");
                }
            }

            if let Ok(is_stop) = rx_stop_rec.try_recv() {
                if is_stop {
                    run = false;
                    println!("start_recording: stop rec received");
                }
            }
        }
        println!(
            "start_recording: finished: {} {}",
            vecpointer_left, vecpointer_right
        );
        (
            wave_left,
            wave_right,
            ringbuffer_left_in,
            ringbuffer_right_in,
            ringbuffer_left_visual_in_opt,
            ringbuffer_right_visual_in_opt,
        )
    })
}
