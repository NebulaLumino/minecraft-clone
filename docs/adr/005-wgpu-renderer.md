# ADR-005: WGPU-Based Rendering Pipeline

## Status
Accepted

## Context
We need a cross-platform GPU rendering backend that supports modern graphics APIs.

## Decision
Use **WGPU** as the rendering backend, built on Vulkan/Metal/D3D12.

### Renderer Architecture
```
Renderer
├── Device (WGPU Device)
├── Queue (command submission)
├── Surface (window framebuffer)
├── Pipeline (render pipeline state)
├── MeshManager (vertex/index buffers)
└── TextureManager (texture atlas)
```

### Render Pipeline
1. **Vertex Shader**: Transform chunk vertices, pass lighting data
2. **Fragment Shader**: Apply texture atlas, ambient occlusion, fog

### Chunk Meshing Strategy
- **Greedy meshing**: Combine adjacent faces of same block type
- **Face culling**: Don't render faces between solid blocks
- **Frustum culling**: Skip chunks outside camera view
- **Distance culling**: Lower LOD for distant chunks

### Texture Atlas
- Single 256x256 atlas with all block textures
- Procedurally generated at startup
- 16x16 pixels per block face

## Consequences

### Positive
- Cross-platform: Vulkan (Linux/Windows), Metal (macOS), D3D12 (Windows)
- Modern API: aligned with WebGPU standard
- Excellent Rust support with ownership semantics
- Active development and community

### Negative
- WGPU 0.20+ API changes caused initial compatibility issues
- May need wrapper/abstraction for advanced features
- Debugging GPU issues more complex than CPU

## References
- [WGPU Documentation](https://docs.rs/wgpu)
- [wgpu examples](https://github.com/gfx-rs/wgpu/tree/master/examples)
