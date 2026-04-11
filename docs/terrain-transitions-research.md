# Hex Terrain Transition Techniques — Research

## How Actual Hex Games Handle Terrain Transitions

### Original Battle Isle
Simple discrete hex sprites — no blending at all. Each hex had one fixed terrain sprite. Strategic readability over visual polish. The open-source clone **Advanced Strategic Command** ([GitHub](https://github.com/ValHaris/asc-hq)) works the same way.

### Civilization V/VI
**Not hex tiles at all visually.** The hex grid exists only as data. The terrain is a continuous 3D mesh with heightmaps and texture splatting. Transitions are blurred at the splat-map level in shaders. Overkill for most indie projects.
- [Unreal Engine Forum analysis](https://forums.unrealengine.com/t/how-to-create-terrain-like-in-civiliazation-5/14311)
- [GameDev.net discussion](https://gamedev.net/forums/topic/705413-some-doubts-regarding-hex-grid-terrains-like-civ56-or-aow/)

### Battle for Wesnoth
**Layered edge overlays with priority flags.** Each terrain has a layer priority. Higher-priority terrains draw transition images over lower ones. Pre-drawn images for each edge direction (6 basic + compound 2/3/4-side variants). Full artistic control.
- [Wesnoth Wiki: Tiles Tutorial](https://wiki.wesnoth.org/Tiles_Tutorial)
- [Wesnoth Wiki: TerrainMacrosWML](https://wiki.wesnoth.org/TerrainMacrosWML)
- [Source: terrain-graphics.cfg](https://github.com/wesnoth/wesnoth/blob/master/data/core/terrain-graphics.cfg)

### Humankind (Amplitude Studios)
Fully 3D terrain where each hex tile has elevation. Adjacent tiles create gentle slopes or impassable cliffs. Continuous and three-dimensional rather than flat tile-based. No detailed technical breakdown published.
- [Humankind Feature Focus: Reimagining Terrain](https://community.amplitude-studios.com/amplitude-studios/humankind/blogs/722-humankind-feature-focus-02-reimagining-terrain)

---

## Technical Approaches

### A. Barycentric Coordinate Blending (Sub-Hex Triangles)

Any point on the map sits inside a triangle formed by the 3 nearest hex centers. Barycentric coordinates give blend weights: (1,0,0) at a hex center, (1/3,1/3,1/3) where 3 hexes meet, (1/2,1/2,0) at the midpoint of an edge between two hex centers.

**Noise modulation** applied to these weights produces organic, irregular biome boundaries. Power-based blending (raising weights to exponents 1-50) shifts between smooth linear and sharp "max" behavior.

**Limitation:** Requires vertex duplication (one copy per triangle instead of shared vertices) to pass per-triangle barycentric values, unless the GPU supports `GL_EXT_fragment_shader_barycentric`.

- [Red Blob Games: Terrain Shader Experiments](https://www.redblobgames.com/x/1730-terrain-shader-experiments/)
- [Gobs & Gods: Hexagonal Grid Shader](https://www.gobsandgods.com/blog/hexagonal-shader.html)
- [Godot Shaders: Hexagonal Tilemap with Blending](https://godotshaders.com/shader/hexagonal-tilemap-with-blending/)

### B. Vertex Color Splat Maps (Catlike Coding)

Each hex cell stores a terrain type index. The mesh is subdivided into triangles for edges and corners. Vertex colors encode a splat map: R/G/B channels represent blend weights for up to 3 terrain types per triangle.

- **Cell triangles:** Single terrain, all vertices same color (1,0,0)
- **Edge triangles:** Blend between 2 terrains, gradient from red to green
- **Corner triangles:** Blend between 3 terrains, RGB barycentric interpolation

The shader samples a texture array 3 times (once per terrain index), multiplies by the corresponding splat weight, and combines.

- [Catlike Coding: Hex Map 14 - Terrain Textures](https://catlikecoding.com/unity/tutorials/hex-map/part-14/)
- [Catlike Coding: Hex Map Tutorial Series](https://catlikecoding.com/unity/tutorials/hex-map/)

### C. Bitmask/Autotiling (Hex Marching Squares)

Each cell examines its neighbors and computes a bitmask. For hex grids: 6 neighbors = 6-bit mask = 64 possibilities. A precomputed lookup table maps each mask to a pre-drawn tile variant. Frame resolution is a single O(1) array lookup.

**Multi-material extension (Autotile Routing Pipeline):** Treats materials as a graph, uses BFS to find shortest paths between terrain types that lack direct transition tilesets, and chains through intermediate materials automatically.

- [Envato Tuts+: How to Use Tile Bitmasking](https://code.tutsplus.com/how-to-use-tile-bitmasking-to-auto-tile-your-level-layouts--cms-25673t)
- [DEV.to: Autotile Routing Pipeline](https://dev.to/tundraray/autotile-routing-pipeline-automatic-tile-transition-selection-for-2d-maps-26bk)

### D. Wang Corner Tiles

Assign terrain types to each tile corner. Each tile's visual content transitions between the corner terrain types. For hex tiles with 2 terrain types: 2^6 = 64 tiles.

**Wangscape** is an open-source C++ tool that converts standard terrain tiles into corner Wang tilesets. It generates alpha masks for each corner combination and composites them into seamless tiles. Terrains are organized into cliques (up to 4 per clique) to manage tileset sizes.

- [Wangscape GitHub](https://github.com/Wangscape/Wangscape)
- [Procedural World: Introduction to Wang Tiles](http://procworld.blogspot.com/2013/01/introduction-to-wang-tiles.html)
- [DEV.to: Wang 2-Corner Tiles](https://dev.to/joestrout/wang-2-corner-tiles-544k)

### E. Nick Chavez's 3-Cell / 6-Cell Patterns

Most detailed public writeup of a Civ-VI-style hex renderer.

**3-Cell Pattern (texture blending):** Each hex intersection involves 3 hexagons. Each cell is either one biome or another, yielding 2^3 = 8 permutations.

**6-Cell Pattern (heightmap blending):** Expands to 6 cells (the 3-cell intersection plus 3 outer neighbors), creating 2^6 = 64 permutations. Produces more interesting patterns with seamless joins.

**River handling:** 9 cell edges at 6-cell intersections = 2^9 = 512 permutations.

Transitions pre-baked offline using Python and ImageMagick.

- [Nick Chavez: Hex Strategy Map Render](https://nicolaschavez.com/projects/hex-map-render/)
- [Nick Chavez: Hex Strategy Map Design](https://nicolaschavez.com/projects/hex-map-design/)

### F. Layered Edge Overlays (Wesnoth-style)

Pre-drawn transition images for each edge direction, drawn on top of base terrain in priority order. Requires many art assets (6+ directional variants per terrain pair, plus compound 2-side, 3-side, 4-side variants) but is simple to implement and gives full artistic control. Battle-tested in shipped games.

### G. Dual Grid / Offset Grid

Separates game logic from visual presentation using two overlapping tilemaps at half-tile offset. The graphics tilemap only needs 5 sprite types (edge, inner corner, outer corner, filled, opposite corners) plus rotations to create all transitions. Primarily documented for square grids but adaptable to hex.

- [Excalibur.js: Dual Tilemap Autotiling Technique](https://excaliburjs.com/blog/Dual%20Tilemap%20Autotiling%20Technique/)

---

## Comparison

| Approach | Art effort | Code complexity | Visual quality |
|----------|-----------|----------------|---------------|
| Wesnoth-style overlays | High (many edge sprites) | Low | Great artistic control |
| Bitmask autotile | Medium (64 variants) | Low | Good, tile-based look |
| Barycentric shader | Low (just base textures) | Medium (WGSL shader) | Smooth, organic |
| Vertex splat maps | Low (base textures) | Medium | Smooth, controlled |
| Wang corner tiles | Medium (auto-generated) | Low-Medium | Good variety |
| Civ-style continuous mesh | Very high | Very high | AAA quality |

---

## Open Source Implementations

| Project | Language | Technique | URL |
|---------|----------|-----------|-----|
| Wesnoth | C++ | Layered edge overlays with priority flags | [GitHub](https://github.com/wesnoth/wesnoth) |
| Wangscape | C++ | Corner Wang tile generation from source textures | [GitHub](https://github.com/Wangscape/Wangscape) |
| Advanced Strategic Command | C++ | Battle Isle clone, discrete hex tiles | [GitHub](https://github.com/ValHaris/asc-hq) |
| threejs-hex-map | TypeScript | 3D hex map with blending mask textures | [GitHub](https://github.com/Bunkerbewohner/threejs-hex-map) |
| Catlike Coding HexMap | C#/Unity | Vertex color splat maps with texture arrays | [Tutorial](https://catlikecoding.com/unity/tutorials/hex-map/part-14/) |
| Godot Hex Tilemap Shader | GLSL | Barycentric coordinate blending with noise | [Godot Shaders](https://godotshaders.com/shader/hexagonal-tilemap-with-blending/) |
| Gobs & Gods Hex Shader | GLSL | Barycentric 3-nearest-neighbor blending | [Blog](https://www.gobsandgods.com/blog/hexagonal-shader.html) |
