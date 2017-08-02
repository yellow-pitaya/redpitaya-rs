extern crate redpitaya;

use redpitaya::Channel;
use redpitaya::generator::Waveform;

fn main() {
    let mut t = [0.0; 16_384];
    let mut x = [0.0; 16_384];
    let mut y = [0.0; 16_384];

    redpitaya::init()
        .expect("Red Pitaya API init failed!");

    for i in 1..t.len() {
        t[i] = 2.0 * std::f32::consts::PI / t.len() as f32 * i as f32;
    }

    for i in 0..x.len() {
        x[i] = t[i].sin() + (1.0 / 3.0 * (t[i] * 3.0).sin());
        y[i] = 1.0 / 2.0 * t[i].sin() + (1.0 / 4.0) * (t[i] * 4.0).sin();
    }

    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_ARBITRARY).unwrap();
    redpitaya::generator::waveform(Channel::RP_CH_2, Waveform::RP_WAVEFORM_ARBITRARY).unwrap();

    redpitaya::generator::arb_waveform(Channel::RP_CH_1, &mut x).unwrap();
    redpitaya::generator::arb_waveform(Channel::RP_CH_2, &mut y).unwrap();

    redpitaya::generator::amp(Channel::RP_CH_1, 0.7).unwrap();
    redpitaya::generator::amp(Channel::RP_CH_2, 1.0).unwrap();

    redpitaya::generator::freq(Channel::RP_CH_1, 4_000.0).unwrap();
    redpitaya::generator::freq(Channel::RP_CH_2, 4_000.0).unwrap();

    redpitaya::generator::out_enable(Channel::RP_CH_1).unwrap();
    redpitaya::generator::out_enable(Channel::RP_CH_2).unwrap();

    redpitaya::release()
        .unwrap();
}