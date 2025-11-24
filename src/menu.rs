//menu.rs

use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    Victory,
}

pub struct Menu {
    pub selected_option: usize,
    pub options: Vec<MapOption>,
}

pub struct MapOption {
    pub name: String,
    pub file_path: String,
    pub display_name: String,
    pub item_configs: Vec<ItemConfig>,
    pub broken_map_texture: String,  // Textura del mapa roto
    pub fixed_map_texture: String, 
}

pub struct ItemConfig {
    pub char_key: char,           // 'b', 'c', 'h'
    pub texture_key: char,        // Caracter para la textura
    pub texture_path: String,     // Ruta del PNG
    pub item_name: String,        // Nombre al recoger
}

impl Menu {
    pub fn new() -> Self {
        let options = vec![
        MapOption {
                name: "colibri".to_string(),
                file_path: "mazeColibri.txt".to_string(),
                display_name: "Mapa Colibri".to_string(),
                broken_map_texture: "assets/Colibri/mColibriRota.png".to_string(),
                fixed_map_texture: "assets/Colibri/mColibri.png".to_string(),
                item_configs: vec![
                    ItemConfig {
                        char_key: 'b',
                        texture_key: '1',
                        texture_path: "assets/Colibri/fColibri1.png".to_string(),
                        item_name: "Cabeza de colibri".to_string(),
                    },
                    ItemConfig {
                        char_key: 'c',
                        texture_key: '2',
                        texture_path: "assets/Colibri/fColibri2.png".to_string(),
                        item_name: "Cola de colibri".to_string(),
                    },
                    ItemConfig {
                        char_key: 'h',
                        texture_key: '3',
                        texture_path: "assets/Colibri/fColibri3.png".to_string(),
                        item_name: "Alas de colibri".to_string(),
                    },
                ],
            },
            // MAPA 2: ORCA
            MapOption {
                name: "orca".to_string(),
                file_path: "mazeOrca.txt".to_string(),
                display_name: "Mapa Orca".to_string(),
                broken_map_texture: "assets/Orca/mOrcaRota.png".to_string(),
                fixed_map_texture: "assets/Orca/mOrca.png".to_string(),
                item_configs: vec![
                    ItemConfig {
                        char_key: 'b',
                        texture_key: '4',
                        texture_path: "assets/Orca/fOrca1.png".to_string(),
                        item_name: "Aleta de Orca".to_string(),
                    },
                    ItemConfig {
                        char_key: 'c',
                        texture_key: '5',
                        texture_path: "assets/Orca/fOrca2.png".to_string(),
                        item_name: "Cabeza de Orca".to_string(),
                    },
                    ItemConfig {
                        char_key: 'h',
                        texture_key: '6',
                        texture_path: "assets/Orca/fOrca3.png".to_string(),
                        item_name: "Cola de Orca".to_string(),
                    },
                ],
            },
            //  MAPA 3: VACA
            MapOption {
                name: "vaca".to_string(),
                file_path: "mazeVaca.txt".to_string(),
                display_name: "Mapa Vaca".to_string(),
                broken_map_texture: "assets/Vaca/mVacaRoto.png".to_string(),
                fixed_map_texture: "assets/Vaca/mVacaRoto.png".to_string(),
                item_configs: vec![
                    ItemConfig {
                        char_key: 'b',
                        texture_key: '7',
                        texture_path: "assets/Vaca/fVaca1.png".to_string(),
                        item_name: "Cabeza vaquita".to_string(),
                    },
                    ItemConfig {
                        char_key: 'c',
                        texture_key: '8',
                        texture_path: "assets/Vaca/fVaca2.png".to_string(),
                        item_name: "vaquita".to_string(),
                    },
                    ItemConfig {
                        char_key: 'h',
                        texture_key: '9',
                        texture_path: "assets/Vaca/fVaca1.png".to_string(),
                        item_name: "Cabeza vaquita".to_string(),
                    },
                ],
            },
        ];

        Menu {
            selected_option: 0,
            options,
        }
    }

    pub fn handle_input(&mut self, window: &RaylibHandle) -> Option<usize> {
        // Navegar hacia arriba
        if window.is_key_pressed(KeyboardKey::KEY_UP) {
            if self.selected_option > 0 {
                self.selected_option -= 1;
            }
        }

        // Navegar hacia abajo
        if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
            if self.selected_option < self.options.len() - 1 {
                self.selected_option += 1;
            }
        }

        // Seleccionar con ENTER
        if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
            return Some(self.selected_option);
        }

        None
    }
}