use crate::configuration::BaseConfig;
use crate::faders::{Fader, fader_from_mapping};
use std::error::Error;
use std::time::Instant;
use std::path::Path;
use std::fs::{DirEntry, File};
use serde_yaml::{from_reader, Mapping};
use log::{info, debug, error};

pub struct ShowUpdate {
    pub song: Option<usize>,
    pub scene: Option<usize>,
    pub tempo: Option<u8>,
    pub off: Option<bool>,
}

pub struct Show {
    name: String,
    songs: Vec<Song>,
    selected_song: usize,
    selected_tempo: u8,
    off: bool,
}

impl Show {
    pub fn update(&mut self, patch: ShowUpdate) {
        if let Some(next_song) = patch.song {
            if self.selected_song != next_song {
                if self.songs.len() > next_song {
                    self.selected_song = next_song;
                    self.print_selected_song();
                    self.songs[self.selected_song].reset();
                }
            }
        }
        if let Some(next_tempo) = patch.tempo {
            if self.selected_tempo != next_tempo {
                self.selected_tempo = next_tempo;
                debug!("Tempo: {}", self.selected_tempo);
            }
        }
        if let Some(_) = patch.off {
            self.off = true;
        }
        if (patch.song.is_some() || patch.scene.is_some()) && patch.off.is_none() {
            self.off = false;
        }
        if self.songs.len() > self.selected_song {
            self.songs[self.selected_song].update(patch);
        }
    }

    pub fn update_state(&mut self) {
        if self.songs.len() > self.selected_song {
            self.songs[self.selected_song].update_state(self.selected_tempo);
        }
    }

    pub fn get_dmx_data(&self) -> [u8; 255] {
        if self.off {
            [0; 255]
        } else if self.songs.len() > self.selected_song {
            self.songs[self.selected_song].get_dmx_data()
        } else {
            [0; 255]
        }
    }

    pub fn print_content(&self) {
        debug!("");
        debug!("{}", self.name);
        for (i, song) in self.songs.iter().enumerate() {
            song.print_content(i);
            debug!("");
        }
        debug!("");
    }

    pub fn print_selected_song(&self) {
        debug!("Song: {}. {}", self.selected_song, self.songs[self.selected_song].name);
    }
}

pub struct Song {
    name: String,
    scenes: Vec<Scene>,
    selected_scene: usize,
}

impl Song {
    pub fn reset(&mut self) {
        self.selected_scene = 0;
        self.print_selected_scene()
    }

    pub fn update(&mut self, patch: ShowUpdate) -> usize {
        if let Some(next_scene) = patch.scene {
            if self.selected_scene != next_scene {
                if self.scenes.len() > next_scene {
                    self.selected_scene = next_scene;
                    self.scenes[self.selected_scene].reset();
                    self.print_selected_scene();
                }
            }
        }
        self.selected_scene
    }

    pub fn update_state(&mut self, selected_tempo: u8) {
        if self.scenes.len() > self.selected_scene {
            self.scenes[self.selected_scene].update_state(selected_tempo);
        }
    }

    pub fn get_dmx_data(&self) -> [u8; 255] {
        if self.scenes.len() > self.selected_scene {
            self.scenes[self.selected_scene].get_dmx_data()
        } else {
            [0; 255]
        }
    }
    
    pub fn print_content(&self, index: usize) {
        debug!("  {} {}", index, self.name);
        for (i, scene) in self.scenes.iter().enumerate() {
            scene.print_content(i);
        }
    }

    pub fn print_selected_scene(&self) {
        debug!("Scene: {}. {}", self.selected_scene, self.scenes[self.selected_scene].name);
    }
}

pub struct Scene {
    name: String,
    start_time: Instant,
    faders: Vec<Fader>
}

impl Scene {
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn update_state(&mut self, selected_tempo: u8) {
        for fader in &mut self.faders {
            fader.update_state(selected_tempo, self.start_time);
        }
    }

    pub fn get_dmx_data(&self) -> [u8; 255] {
        let mut dmx_data = [0; 255];
        for fader in &self.faders {
            dmx_data[fader.get_channel()] = fader.get_value();
        }
        return dmx_data;
    }
    
    pub fn print_content(&self, index: usize) {
        debug!("    {} {}", index, self.name);
    }
}

pub fn load_show (config: &BaseConfig) -> Result<Show, Box<dyn Error>> {
    let mut show_slot: Option<Show> = None;

    if !config.show_path.eq("default_show") && !config.show_path.is_empty() {
        let show_path = Path::new(&config.show_path);
        if show_path.is_dir() {
            show_slot = Some(load_show_from_path(show_path));
        } else {
            error!("!!  Provided show path is not a directory: '{}'  !!", &config.show_path);
            error!("");
        }
    }

    if show_slot.is_none() {
        show_slot = Some(load_default_show());
    }
    let selected_show = show_slot.unwrap();
    info!("Selected show:           {}", &selected_show.name);
    Ok(selected_show)
}

fn load_show_from_path(path: &Path) -> Show {
    let mut show = Show {
        name: String::from(path.file_name().unwrap().to_str().unwrap()),
        songs: Vec::new(),
        selected_song: 0,
        selected_tempo: 120,
        off: false,
    };
    let paths = get_ordered_paths_as_iter(path);
    for subpath in paths {
        if subpath.path().is_dir() {
            if let Some(song) = load_song_from_path(&subpath.path()) {
                show.songs.push(song);
            }
        }
    }
    show
}

fn load_song_from_path(path: &Path) -> Option<Song> {
    let mut song = Song {
        name: String::from(path.file_name().unwrap().to_str().unwrap()),
        scenes: Vec::new(),
        selected_scene: 0,
    };
    let paths = get_ordered_paths_as_iter(path);
    for subpath in paths {
        if subpath.path().is_file() &&
            subpath.path().extension().is_some() &&
            subpath.path().extension().unwrap().eq("yml") &&
            !subpath.file_name().to_str().unwrap().starts_with(".") {
            song.scenes.push(load_scene_from_path(&subpath.path()));
        }
    }
    if song.scenes.len() > 0 {
        Some(song)
    } else {
        None
    }
}

fn load_scene_from_path(path: &Path) -> Scene {
    let scene_file = File::open(&path).unwrap();
    let yaml_data: Mapping = from_reader(scene_file).unwrap();
    let mut scene = Scene {
        name: String::from(path.file_stem().unwrap().to_str().unwrap()),
        start_time: Instant::now(),
        faders: Vec::new(),
    };

    for (key, value) in yaml_data.iter() {
        if key.is_string() && key.eq("01_name") {
                scene.name = value.as_str().unwrap().to_string();
        } else if key.is_string() && key.eq("faders") && value.is_mapping() {
            for (channel, properties) in value.as_mapping().unwrap().iter() {
                if let Some(fader) = fader_from_mapping(channel, properties) {
                    scene.faders.push(fader);
                }
            }
        }
    }

    scene
}

fn get_ordered_paths_as_iter(path: &Path) -> Vec<DirEntry> {
    let mut paths: Vec<DirEntry> = path.read_dir()
                    .expect("read_dir call failed")
                    .filter_map(|r| r.ok())
                    .filter(|dir| !dir.file_name().to_str().unwrap().starts_with("."))
                    .collect();
    paths
        .sort_by_key(|dir| dir.path());
    return paths;
}

fn load_default_show() -> Show {
    let lights_off_scene_1 : Scene = Scene {
        name: String::from("Lights off"),
        start_time: Instant::now(),
        faders: Vec::new(),
    };
    let lights_off_scene_2 : Scene = Scene {
        name: String::from("Lights off"),
        start_time: Instant::now(),
        faders: Vec::new(),
    };
    let lights_off_scene_3 : Scene = Scene {
        name: String::from("Lights off"),
        start_time: Instant::now(),
        faders: Vec::new(),
    };
    let lights_off_scene_4 : Scene = Scene {
        name: String::from("Lights off"),
        start_time: Instant::now(),
        faders: Vec::new(),
    };
    let first_song : Song = Song {
        name: String::from("First song"),
        scenes: vec![lights_off_scene_1, lights_off_scene_2],
        selected_scene: 0,
    };
    let second_song : Song = Song {
        name: String::from("Second song"),
        scenes: vec![lights_off_scene_3, lights_off_scene_4],
        selected_scene: 0,
    };
    let default_show : Show = Show {
        name: String::from("Default show"),
        songs: vec![first_song, second_song],
        selected_song: 0,
        selected_tempo: 120,
        off: false,
    };
    return default_show;
}
