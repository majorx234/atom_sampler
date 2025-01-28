use bus::BusReader;
use ringbuf::{
    traits::{Consumer, Observer, Producer},
    HeapCons, HeapProd,
};

pub fn start_recording(
    mut ringbuffer_left_in: HeapCons<f32>,
    mut ringbuffer_right_in: HeapCons<f32>,
    ringbuffer_left_visual_in_opt: &mut Option<HeapProd<(f32, f32)>>,
    ringbuffer_right_visual_in_opt: &mut Option<HeapProd<(f32, f32)>>,

    mut rx_stop_rec: BusReader<bool>,
) -> std::thread::JoinHandle<(Vec<f32>, Vec<f32>)> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let wave_size = 192000;
        let mut wave_left = vec![0.0; wave_size];
        let mut wave_right = vec![0.0; wave_size];
        let mut vecpointer_left = 0;
        let mut vecpointer_right = 0;
        while run {
            let length_left = 1024.min(ringbuffer_left_in.occupied_len());
            let length_right = 1024.min(ringbuffer_right_in.occupied_len());

            if (vecpointer_left + length_left < wave_size)
                && (vecpointer_right + length_right < wave_size)
            {
                wave_left.splice(vecpointer_left..length_left, ringbuffer_left_in.pop_iter());
                vecpointer_left += length_left;
                wave_right.splice(
                    vecpointer_right..length_right,
                    ringbuffer_right_in.pop_iter(),
                );
                vecpointer_right += length_right;
            } else {
                ringbuffer_left_in.clear();
                ringbuffer_right_in.clear();
                run = false;
            }
            if let Ok(is_stop) = rx_stop_rec.try_recv() {
                if is_stop {
                    run = false;
                }
            }
        }
        (wave_left, wave_right)
    })
}
