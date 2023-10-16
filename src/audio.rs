// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

use bevy::audio::AddAudioSource;
use bevy::prelude::*;
use funutd::Rnd;
use std::time::Duration;

use crate::app_status::AppStatus;
use crate::events;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<EngineNoise>()
            .add_systems(OnExit(AppStatus::Playing), silence_engine_noise)
            // .add_systems(Update, simple_input_system)
            .add_systems(Update, update_engine_noise);
    }
}

#[derive(Clone, Debug, Default)]
pub struct WhiteNoise {
    rnd: Rnd,
}

impl Iterator for WhiteNoise {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let value = self.rnd.f32_in(-0.5, 0.5);
        Some(value)
    }
}

impl bevy::audio::Source for WhiteNoise {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[derive(Asset, Debug, Clone, TypePath, Component)]
pub struct EngineNoise;

use rodio::Source;
impl Decodable for EngineNoise {
    type DecoderItem = f32;
    type Decoder = rodio::source::BltFilter<WhiteNoise>;

    fn decoder(&self) -> Self::Decoder {
        WhiteNoise::default().low_pass(500)
    }
}

fn silence_engine_noise(engine_noise_controller: Query<&AudioSink, With<EngineNoise>>) {
    if let Ok(sink) = engine_noise_controller.get_single() {
        sink.pause()
    }
}

fn update_engine_noise(
    mut pitch_assets: ResMut<Assets<EngineNoise>>,
    engine_noise_controller: Query<&AudioSink, With<EngineNoise>>,
    mut events: EventReader<events::Thruster>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            events::Thruster::Firing => {
                if let Ok(sink) = engine_noise_controller.get_single() {
                    info!("resuming existing noise");
                    sink.play()
                } else {
                    info!(
                        "playing engine noise {}",
                        engine_noise_controller.get_single().is_ok()
                    );
                    commands.spawn((
                        AudioSourceBundle {
                            source: pitch_assets.add(EngineNoise),
                            settings: PlaybackSettings::DESPAWN,
                        },
                        EngineNoise,
                    ));
                }
            }
            events::Thruster::Stopped => {
                if let Ok(sink) = engine_noise_controller.get_single() {
                    info!("stoping engine noise");
                    sink.pause()
                }
            }
        }
    }
}

fn _simple_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<events::Thruster>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        events.send(events::Thruster::Firing)
    } else if keyboard_input.just_released(KeyCode::Up) {
        events.send(events::Thruster::Stopped)
    }
}
