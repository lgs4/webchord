use wasm_bindgen::prelude::*;

mod oscillator;
mod envelope;
mod filter;
mod voice;
mod lfo;
mod effects;

use filter::StateVariableFilter;
use voice::Voice;
use lfo::Lfo;
use effects::delay::Delay;
use effects::reverb::Reverb;
use effects::tremolo::Tremolo;
use effects::flanger::Flanger;

const SAMPLE_RATE: f32 = 48000.0;
const MAX_VOICES_PER_ENGINE: usize = 16; // Each engine gets 16 voices

// Dual engine system: separate timeline and live performance engines
struct Engine {
    voices: Vec<Voice>,
    lfo: Lfo,
    filter: StateVariableFilter,
    filter_mode: u8,
    filter_enabled: bool,
    delay: Delay,
    reverb: Reverb,
    tremolo: Tremolo,
    flanger: Flanger,
    delay_enabled: bool,
    reverb_enabled: bool,
    tremolo_enabled: bool,
    flanger_enabled: bool,
    lfo_to_filter: bool,
    base_filter_cutoff: f32,
    detune_cents: f32,
}

impl Engine {
    fn new(sample_rate: f32) -> Self {
        let mut voices = Vec::with_capacity(MAX_VOICES_PER_ENGINE);
        for _ in 0..MAX_VOICES_PER_ENGINE {
            voices.push(Voice::new(sample_rate));
        }

        Engine {
            voices,
            lfo: Lfo::new(sample_rate),
            filter: StateVariableFilter::new(sample_rate),
            filter_mode: 0,
            filter_enabled: false,
            delay: Delay::new(sample_rate, 2000.0),
            reverb: Reverb::new(sample_rate),
            tremolo: Tremolo::new(sample_rate),
            flanger: Flanger::new(sample_rate),
            delay_enabled: false,
            reverb_enabled: false,
            tremolo_enabled: false,
            flanger_enabled: false,
            lfo_to_filter: false,
            base_filter_cutoff: 20000.0,
            detune_cents: 0.0,
        }
    }

    fn process_voices(&mut self, output: &mut [f32]) {
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.process(output);
            }
        }
    }

    fn process_effects(&mut self, buffer: &mut [f32]) {
        for i in 0..buffer.len() {
            let mut sample = buffer[i];

            // Apply LFO modulation to filter cutoff if enabled
            if self.lfo_to_filter {
                let lfo_value = self.lfo.process();
                let modulated_cutoff = self.base_filter_cutoff * (1.0 + lfo_value);
                self.filter.set_cutoff(modulated_cutoff.clamp(20.0, 20000.0));
            }

            // Apply filter
            if self.filter_enabled {
                sample = match self.filter_mode {
                    1 => self.filter.process_highpass(sample),
                    2 => self.filter.process_bandpass(sample),
                    _ => self.filter.process(sample),
                };
            }

            // Apply effects chain
            if self.flanger_enabled {
                sample = self.flanger.process(sample);
            }
            if self.tremolo_enabled {
                sample = self.tremolo.process(sample);
            }
            if self.delay_enabled {
                sample = self.delay.process(sample);
            }
            if self.reverb_enabled {
                sample = self.reverb.process(sample);
            }

            buffer[i] = sample;
        }
    }
}

#[wasm_bindgen]
pub struct AudioEngine {
    timeline_engine: Engine,
    live_engine: Engine,
    timeline_volume: f32,
    live_volume: f32,
    master_volume: f32,
}

#[wasm_bindgen]
impl AudioEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AudioEngine {
        AudioEngine {
            timeline_engine: Engine::new(SAMPLE_RATE),
            live_engine: Engine::new(SAMPLE_RATE),
            timeline_volume: 0.21, // 70% of 0.3 max
            live_volume: 0.21,     // 70% of 0.3 max
            master_volume: 1.0,    // Master is now just a final gain stage
        }
    }

    pub fn process(&mut self, output: &mut [f32]) {
        let len = output.len();
        
        // Process timeline engine
        let mut timeline_buffer = vec![0.0; len];
        self.timeline_engine.process_voices(&mut timeline_buffer);
        self.timeline_engine.process_effects(&mut timeline_buffer);
        
        // Process live engine  
        let mut live_buffer = vec![0.0; len];
        self.live_engine.process_voices(&mut live_buffer);
        self.live_engine.process_effects(&mut live_buffer);
        
        // Mix both engines with independent volumes
        for i in 0..len {
            output[i] = (timeline_buffer[i] * self.timeline_volume + 
                        live_buffer[i] * self.live_volume) * self.master_volume;
        }
    }

    // Live performance note methods (use live_engine)
    pub fn note_on(&mut self, midi_note: u8, velocity: f32) {
        let mut voice_idx = None;
        
        for (i, voice) in self.live_engine.voices.iter().enumerate() {
            if !voice.is_active() {
                voice_idx = Some(i);
                break;
            }
        }

        if voice_idx.is_none() {
            let mut oldest_releasing = None;
            let mut oldest_releasing_age = 0.0;
            
            for (i, voice) in self.live_engine.voices.iter().enumerate() {
                if voice.is_releasing() && voice.get_age() > oldest_releasing_age {
                    oldest_releasing = Some(i);
                    oldest_releasing_age = voice.get_age();
                }
            }
            
            if oldest_releasing.is_some() {
                voice_idx = oldest_releasing;
            } else {
                let mut oldest_age = 0.0;
                for (i, voice) in self.live_engine.voices.iter().enumerate() {
                    if voice.get_age() > oldest_age {
                        oldest_age = voice.get_age();
                        voice_idx = Some(i);
                    }
                }
            }
        }

        if let Some(idx) = voice_idx {
            let freq = midi_to_freq(midi_note);
            self.live_engine.voices[idx].note_on(freq, velocity);
        }
    }

    pub fn note_off(&mut self, midi_note: u8) {
        let freq = midi_to_freq(midi_note);
        for voice in &mut self.live_engine.voices {
            if (voice.get_frequency() - freq).abs() < 0.1 {
                voice.note_off();
            }
        }
    }

    // Timeline note methods (use timeline_engine)
    pub fn timeline_note_on(&mut self, midi_note: u8, velocity: f32) {
        let mut voice_idx = None;
        
        for (i, voice) in self.timeline_engine.voices.iter().enumerate() {
            if !voice.is_active() {
                voice_idx = Some(i);
                break;
            }
        }

        if voice_idx.is_none() {
            let mut oldest_releasing = None;
            let mut oldest_releasing_age = 0.0;
            
            for (i, voice) in self.timeline_engine.voices.iter().enumerate() {
                if voice.is_releasing() && voice.get_age() > oldest_releasing_age {
                    oldest_releasing = Some(i);
                    oldest_releasing_age = voice.get_age();
                }
            }
            
            if oldest_releasing.is_some() {
                voice_idx = oldest_releasing;
            } else {
                let mut oldest_age = 0.0;
                for (i, voice) in self.timeline_engine.voices.iter().enumerate() {
                    if voice.get_age() > oldest_age {
                        oldest_age = voice.get_age();
                        voice_idx = Some(i);
                    }
                }
            }
        }

        if let Some(idx) = voice_idx {
            let freq = midi_to_freq(midi_note);
            self.timeline_engine.voices[idx].note_on(freq, velocity);
        }
    }

    pub fn timeline_note_off(&mut self, midi_note: u8) {
        let freq = midi_to_freq(midi_note);
        for voice in &mut self.timeline_engine.voices {
            if (voice.get_frequency() - freq).abs() < 0.1 {
                voice.note_off();
            }
        }
    }

    // Stop all timeline notes (for loop restart)
    pub fn stop_all_timeline_notes(&mut self) {
        for voice in &mut self.timeline_engine.voices {
            if voice.is_active() {
                voice.note_off();
            }
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_timeline_volume(&mut self, volume: f32) {
        // Scale input 0-1 to output 0-0.3 (30% max to prevent clipping/distortion)
        self.timeline_volume = (volume * 0.3).clamp(0.0, 0.3);
    }

    pub fn set_live_volume(&mut self, volume: f32) {
        // Scale input 0-1 to output 0-0.3 (30% max to prevent clipping/distortion)
        self.live_volume = (volume * 0.3).clamp(0.0, 0.3);
    }

    pub fn set_waveform(&mut self, waveform: u8) {
        for voice in &mut self.live_engine.voices {
            voice.set_waveform(waveform);
        }
        // Timeline engine gets updated when pattern parameters are applied
    }

    pub fn set_adsr(&mut self, attack: f32, decay: f32, sustain: f32, release: f32) {
        for voice in &mut self.live_engine.voices {
            voice.set_adsr(attack, decay, sustain, release);
        }
    }
    
    // Apply synthesis settings to timeline engine (used when pattern parameters are applied)
    pub fn set_timeline_waveform(&mut self, waveform: u8) {
        for voice in &mut self.timeline_engine.voices {
            voice.set_waveform(waveform);
        }
    }

    pub fn set_timeline_adsr(&mut self, attack: f32, decay: f32, sustain: f32, release: f32) {
        for voice in &mut self.timeline_engine.voices {
            voice.set_adsr(attack, decay, sustain, release);
        }
    }

    // Live engine filter controls
    pub fn set_filter_cutoff(&mut self, cutoff: f32) {
        self.live_engine.base_filter_cutoff = cutoff;
        self.live_engine.filter.set_cutoff(cutoff);
    }

    pub fn set_filter_resonance(&mut self, resonance: f32) {
        self.live_engine.filter.set_resonance(resonance);
    }

    pub fn set_filter_mode(&mut self, mode: u8) {
        self.live_engine.filter_mode = mode.min(2);
    }

    pub fn set_filter_enabled(&mut self, enabled: bool) {
        self.live_engine.filter_enabled = enabled;
    }

    // Timeline engine filter controls
    pub fn set_timeline_filter_cutoff(&mut self, cutoff: f32) {
        self.timeline_engine.base_filter_cutoff = cutoff;
        self.timeline_engine.filter.set_cutoff(cutoff);
    }

    pub fn set_timeline_filter_resonance(&mut self, resonance: f32) {
        self.timeline_engine.filter.set_resonance(resonance);
    }

    pub fn set_timeline_filter_mode(&mut self, mode: u8) {
        self.timeline_engine.filter_mode = mode.min(2);
    }

    pub fn set_timeline_filter_enabled(&mut self, enabled: bool) {
        self.timeline_engine.filter_enabled = enabled;
    }

    // Live engine LFO controls
    pub fn set_lfo_rate(&mut self, rate: f32) {
        self.live_engine.lfo.set_rate(rate);
    }

    pub fn set_lfo_depth(&mut self, depth: f32) {
        self.live_engine.lfo.set_depth(depth);
    }

    pub fn set_lfo_waveform(&mut self, waveform: u8) {
        self.live_engine.lfo.set_waveform(waveform);
    }

    pub fn set_lfo_to_filter(&mut self, enabled: bool) {
        self.live_engine.lfo_to_filter = enabled;
    }

    // Timeline engine LFO controls
    pub fn set_timeline_lfo_rate(&mut self, rate: f32) {
        self.timeline_engine.lfo.set_rate(rate);
    }

    pub fn set_timeline_lfo_depth(&mut self, depth: f32) {
        self.timeline_engine.lfo.set_depth(depth);
    }

    pub fn set_timeline_lfo_waveform(&mut self, waveform: u8) {
        self.timeline_engine.lfo.set_waveform(waveform);
    }

    pub fn set_timeline_lfo_to_filter(&mut self, enabled: bool) {
        self.timeline_engine.lfo_to_filter = enabled;
    }

    // Live engine detune
    pub fn set_detune(&mut self, cents: f32) {
        self.live_engine.detune_cents = cents;
        for voice in &mut self.live_engine.voices {
            voice.set_detune(cents);
        }
    }

    pub fn set_glide_time(&mut self, time_ms: f32) {
        for voice in &mut self.live_engine.voices {
            voice.set_glide_time(time_ms);
        }
    }

    // Timeline engine detune
    pub fn set_timeline_detune(&mut self, cents: f32) {
        self.timeline_engine.detune_cents = cents;
        for voice in &mut self.timeline_engine.voices {
            voice.set_detune(cents);
        }
    }

    pub fn set_timeline_glide_time(&mut self, time_ms: f32) {
        for voice in &mut self.timeline_engine.voices {
            voice.set_glide_time(time_ms);
        }
    }

    // ==== LIVE ENGINE EFFECTS CONTROL ====

    pub fn set_delay(&mut self, enabled: bool, time_ms: f32, feedback: f32, mix: f32) {
        self.live_engine.delay_enabled = enabled;
        if enabled {
            self.live_engine.delay.set_delay_time(time_ms);
            self.live_engine.delay.set_feedback(feedback);
            self.live_engine.delay.set_mix(mix);
        }
    }

    pub fn set_reverb(&mut self, enabled: bool, room_size: f32, damping: f32) {
        self.live_engine.reverb_enabled = enabled;
        if enabled {
            self.live_engine.reverb.set_room_size(room_size);
            self.live_engine.reverb.set_damping(damping);
        }
    }

    pub fn set_tremolo(&mut self, enabled: bool, rate: f32, depth: f32) {
        self.live_engine.tremolo_enabled = enabled;
        if enabled {
            self.live_engine.tremolo.set_rate(rate);
            self.live_engine.tremolo.set_depth(depth);
        }
    }

    pub fn set_flanger(&mut self, enabled: bool, rate: f32, depth: f32, feedback: f32, mix: f32) {
        self.live_engine.flanger_enabled = enabled;
        if enabled {
            self.live_engine.flanger.set_lfo_rate(rate);
            self.live_engine.flanger.set_delay_range(depth);
            self.live_engine.flanger.set_feedback(feedback);
            self.live_engine.flanger.set_mix(mix);
        }
    }

    // ==== TIMELINE ENGINE EFFECTS CONTROL ====

    pub fn set_timeline_delay(&mut self, enabled: bool, time_ms: f32, feedback: f32, mix: f32) {
        self.timeline_engine.delay_enabled = enabled;
        if enabled {
            self.timeline_engine.delay.set_delay_time(time_ms);
            self.timeline_engine.delay.set_feedback(feedback);
            self.timeline_engine.delay.set_mix(mix);
        }
    }

    pub fn set_timeline_reverb(&mut self, enabled: bool, room_size: f32, damping: f32) {
        self.timeline_engine.reverb_enabled = enabled;
        if enabled {
            self.timeline_engine.reverb.set_room_size(room_size);
            self.timeline_engine.reverb.set_damping(damping);
        }
    }

    pub fn set_timeline_tremolo(&mut self, enabled: bool, rate: f32, depth: f32) {
        self.timeline_engine.tremolo_enabled = enabled;
        if enabled {
            self.timeline_engine.tremolo.set_rate(rate);
            self.timeline_engine.tremolo.set_depth(depth);
        }
    }

    pub fn set_timeline_flanger(&mut self, enabled: bool, rate: f32, depth: f32, feedback: f32, mix: f32) {
        self.timeline_engine.flanger_enabled = enabled;
        if enabled {
            self.timeline_engine.flanger.set_lfo_rate(rate);
            self.timeline_engine.flanger.set_delay_range(depth);
            self.timeline_engine.flanger.set_feedback(feedback);
            self.timeline_engine.flanger.set_mix(mix);
        }
    }

    pub fn get_sample_rate(&self) -> f32 {
        SAMPLE_RATE
    }
}

fn midi_to_freq(midi: u8) -> f32 {
    440.0 * 2.0_f32.powf((midi as f32 - 69.0) / 12.0)
}

