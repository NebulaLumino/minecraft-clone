//! Chunk mesh module
//!
//! Generates vertex and index buffers for chunk rendering.

use voxel_shared::BlockState;
use voxel_shared::world::Chunk;
use voxel_shared::constants::CHUNK_SIZE;

/// Vertex format for chunk mesh
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ChunkVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub color: [f32; 4], // RGBA floats
}

/// Indices for chunk mesh
pub type ChunkIndices = Vec<u32>;

/// Generated mesh for a chunk
#[derive(Debug, Clone)]
pub struct ChunkMesh {
    pub vertices: Vec<ChunkVertex>,
    pub indices: ChunkIndices,
}

impl ChunkMesh {
    /// Create empty mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Generate mesh from chunk data using face culling
    pub fn generate(chunk: &Chunk, get_block: impl Fn(i32, i32, i32) -> BlockState) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_count: u32 = 0;

        // Face definitions: (direction, normal, vertex offsets)
        // 0: +X, 1: -X, 2: +Y, 3: -Y, 4: +Z, 5: -Z
        let faces: [[i32; 3]; 6] = [
            [1, 0, 0],    // +X face
            [-1, 0, 0],   // -X face
            [0, 1, 0],    // +Y face
            [0, -1, 0],   // -Y face
            [0, 0, 1],    // +Z face
            [0, 0, -1],   // -Z face
        ];

        let face_normals: [[f32; 3]; 6] = [
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ];

        // Vertex offsets for each face (4 vertices per face)
        let face_vertices: [[[f32; 3]; 4]; 6] = [
            // +X face
            [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]],
            // -X face
            [[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]],
            // +Y face
            [[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
            // -Y face
            [[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
            // +Z face
            [[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]],
            // -Z face
            [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]],
        ];

        // Texture coordinates for each face (simplified - all blocks use same UV)
        let face_uvs: [[f32; 2]; 4] = [
            [0.0, 1.0], // bottom-left
            [0.0, 0.0], // top-left
            [1.0, 0.0], // top-right
            [1.0, 1.0], // bottom-right
        ];

        // Iterate through all blocks in chunk
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..256i32 {
                for z in 0..CHUNK_SIZE as i32 {
                    let block = chunk.get_block(x, y, z);
                    if block.id == 0 {
                        continue; // Skip air blocks
                    }

                    // Check each face
                    for (face_idx, dir) in faces.iter().enumerate() {
                        let nx = x + dir[0];
                        let ny = y + dir[1];
                        let nz = z + dir[2];

                        // Get neighbor block (use chunk bounds or air if outside)
                        let neighbor = if nx < 0 || nx >= CHUNK_SIZE as i32
                            || nz < 0 || nz >= CHUNK_SIZE as i32
                            || ny < 0 || ny >= 256 {
                            BlockState::AIR // Out of chunk bounds = air
                        } else {
                            get_block(nx, ny, nz)
                        };

                        // Only render face if neighbor is air or transparent
                        if neighbor.id == 0 || is_transparent(neighbor.id) {
                            // Add 4 vertices for this face
                            let normal = face_normals[face_idx];
                            let color = get_block_color(block.id);

                            for (i, vert_offset) in face_vertices[face_idx].iter().enumerate() {
                                vertices.push(ChunkVertex {
                                    position: [
                                        (x as f32) + vert_offset[0],
                                        (y as f32) + vert_offset[1],
                                        (z as f32) + vert_offset[2],
                                    ],
                                    normal,
                                    tex_coord: face_uvs[i],
                                    color,
                                });
                            }

                            // Add indices (two triangles per face)
                            indices.push(vertex_count);
                            indices.push(vertex_count + 1);
                            indices.push(vertex_count + 2);
                            indices.push(vertex_count);
                            indices.push(vertex_count + 2);
                            indices.push(vertex_count + 3);

                            vertex_count += 4;
                        }
                    }
                }
            }
        }

        Self { vertices, indices }
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get index count
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a block is transparent (air, water, etc.)
fn is_transparent(block_id: u16) -> bool {
    matches!(block_id, 0 | 8 | 9) // Air, water, water (flowing)
}

/// Get base color for a block type
fn get_block_color(block_id: u16) -> [f32; 4] {
    match block_id {
        0 => [0.0, 0.0, 0.0, 0.0],                           // Air (invisible)
        1 => [0.5, 0.5, 0.5, 1.0],                           // Stone - gray
        2 => [0.3, 0.55, 0.2, 1.0],                          // Grass - green
        3 => [0.53, 0.38, 0.2, 1.0],                          // Dirt - brown
        4 => [0.64, 0.64, 0.64, 1.0],                        // Cobblestone - light gray
        5 => [0.62, 0.49, 0.32, 1.0],                         // Wood - brown
        6 => [0.81, 0.75, 0.66, 1.0],                        // Sand - beige
        7 => [0.27, 0.27, 0.27, 1.0],                         // Bedrock - dark
        8 | 9 => [0.25, 0.5, 1.0, 0.7],                      // Water - blue transparent
        12 => [0.9, 0.81, 0.6, 1.0],                          // Sand - beige
        14 => [1.0, 0.83, 0.1, 1.0],                         // Gold ore - yellow
        15 => [0.67, 0.52, 0.4, 1.0],                        // Iron ore - brownish
        16 => [0.1, 0.1, 0.1, 1.0],                          // Coal ore - black
        56 => [0.3, 0.85, 0.95, 1.0],                        // Diamond ore - cyan
        _ => [0.5, 0.5, 0.5, 1.0],                           // Unknown - gray
    }
}
