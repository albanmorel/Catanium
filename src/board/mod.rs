use std::collections::HashMap;
use crate::card::{Card, CardType, Cost, RessourceType};
use crate::player::Player;

// Define the structures for HexTile, Vertex, and Edge

struct HexTile {
    axial_q: i32,
    axial_r: i32,
    ressource: TileType,
    number_token: Option<u8>,
    vertex_positions: Vec<(i32, i32)>,
    edge_positions: Vec<((i32, i32), (i32, i32))>,
}

struct Vertex {
    position: (i32, i32),
    settlement: Option<Settlement>, // Settlement or City owned by a player
}

struct Edge {
    start_vertex: (i32, i32),
    end_vertex: (i32, i32),
    road: Option<Road>, // Road owned by a player
}

enum TileType {
    Wood,
    Brick,
    Sheep,
    Wheat,
    Ore,
    Desert,
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Settlement {
    level: i32,
    cost: Cost,
}
pub fn create_settlement(level:i32) -> Settlement {
    let cost: Cost = if level == 1 { Cost::Settlement } else { Cost::City };
    Settlement {level, cost }
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Road {
    cost: Cost::Road
}
pub fn create_road() -> Road {
    Road { cost: Cost::Road }
}

pub struct Board {
    hex_tiles: Vec<HexTile>,
    vertex_map: HashMap<(i32, i32), Vertex>,
    edge_map: HashMap<((i32, i32), (i32, i32)), Edge>,
}

impl Board {
    pub fn create_board() -> Board {
        let mut game_board = Board {
            hex_tiles: Vec::new(),
            vertex_map: HashMap::new(),
            edge_map: HashMap::new(),
        };
        // Loop through axial coordinates to create each hex tile
        for axial_q in -2..=2 {
            for axial_r in -2..=2 {
                // Check if the hex coordinates are within the valid range of a hexagon grid
                if axial_q + axial_r >= -2 && axial_q + axial_r <= 2 {
                    game_board.add_hex_tile(axial_q, axial_r);
                }
            }
        }
        game_board
    }

    fn add_hex_tile(&mut self, axial_q: i32, axial_r: i32) {
        // Example ressource and number token, should be randomized in a real game
        let ressource_type = TileType::Wood;
        let number_token = Some(5);

        // Calculate the positions of the six vertices for this hex tile
        let vertex_positions = vec![
            (axial_q + 1, axial_r),       // Vertex 1
            (axial_q, axial_r + 1),       // Vertex 2
            (axial_q - 1, axial_r + 1),   // Vertex 3
            (axial_q - 1, axial_r),       // Vertex 4
            (axial_q, axial_r - 1),       // Vertex 5
            (axial_q + 1, axial_r - 1),   // Vertex 6
        ];

        // Initialize a vector to store the references to this hex's vertices
        let mut hex_tile_vertices = Vec::new();
        for &vertex_position in &vertex_positions {
            // Ensure each vertex is added to the board's vertex map only once
            self.ensure_vertex_exists(vertex_position);
            hex_tile_vertices.push(vertex_position);
        }

        // Define the positions of the six edges for this hex tile by connecting the vertices
        let edge_positions = vec![
            (vertex_positions[0], vertex_positions[1]), // Edge 1-2
            (vertex_positions[1], vertex_positions[2]), // Edge 2-3
            (vertex_positions[2], vertex_positions[3]), // Edge 3-4
            (vertex_positions[3], vertex_positions[4]), // Edge 4-5
            (vertex_positions[4], vertex_positions[5]), // Edge 5-6
            (vertex_positions[5], vertex_positions[0]), // Edge 6-1
        ];

        // Initialize a vector to store the references to this hex's edges
        let mut hex_tile_edges = Vec::new();
        for &(start_vertex, end_vertex) in &edge_positions {
            // Ensure each edge is added to the board's edge map only once
            let edge_key = self.ensure_edge_exists(start_vertex, end_vertex);
            hex_tile_edges.push(edge_key);
        }

        // Add the hex tile to the board's hex tile collection
        self.hex_tiles.push(HexTile {
            axial_q,
            axial_r,
            ressource: ressource_type,
            number_token,
            vertex_positions: hex_tile_vertices,
            edge_positions: hex_tile_edges,
        });
    }

    // Function to ensure a vertex is added to the vertex map if it doesn't exist already
    fn ensure_vertex_exists(&mut self, vertex_position: (i32, i32)) {
        self.vertex_map.entry(vertex_position).or_insert(Vertex {
            position: vertex_position,
            settlement: None,
        });
    }

    // Function to ensure an edge is added to the edge map if it doesn't exist already
    fn ensure_edge_exists(&mut self, start_vertex: (i32, i32), end_vertex: (i32, i32)) -> ((i32, i32), (i32, i32)) {
        // Ensure the edge key is always stored in the same order (start < end)
        let edge_key = if start_vertex < end_vertex { (start_vertex, end_vertex) } else { (end_vertex, start_vertex) };
        self.edge_map.entry(edge_key).or_insert(Edge {
            start_vertex: edge_key.0,
            end_vertex: edge_key.1,
            road: None,
        });
        edge_key
    }
}
