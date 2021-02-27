use std::usize;

use engine::{core::window::Window, game::{Game, GameContainer, GameData}, renderer::{buffer::{VertexArray, VertexBuffer, VertexBufferLayout}, color::Color, matrix::Mat4x4f, renderer::{init_gl, std_renderer}, shader::Shader, texture::Texture, vector::{Vec3f, Vec4f}}};


#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
enum BlockID {
    None,
    Dirt,
    Stone,
}


struct Chunk {
    blocks: [BlockID; 32*32*32],
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            blocks: [BlockID::None; 32*32*32],
        }
    }

    fn set(&mut self, x: usize, y: usize, z: usize, block_id: BlockID) {
        self.blocks[x+y*32+z*32*32] = block_id;
    }

    fn get(&self, x: usize, y: usize, z: usize) -> BlockID {
        self.blocks[x+y*32+z*32*32]
    }
}

fn draw_plane(pos1: Vec4f, pos2: Vec4f, pos3: Vec4f, col: Color) -> Vec<f32> {
    let x = pos2.x - (pos2.x - pos1.x) - (pos2.x - pos3.x);
    let y = pos2.y - (pos2.y - pos1.y) - (pos2.y - pos3.y);
    let z = pos2.z - (pos2.z - pos1.z) - (pos2.z - pos3.z);
    let pos4 = Vec4f::new(x, y, z, 1.0);

    let pos = [pos1, pos2, pos4, pos2, pos4, pos3];

    let mut verts = Vec::new();

    let color = <(f32, f32, f32, f32)>::from(col);

    let uvs = [
        0.0, 0.0,
        1.0, 0.0,
        0.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
    ];
    
    for i in 0..6 { 
        let p = pos[i];
        verts.push(p.x);
        verts.push(p.y);
        verts.push(p.z);
        verts.push(uvs[i*2]);
        verts.push(uvs[i*2+1]);
        verts.push(color.0);
        verts.push(color.1);
        verts.push(color.2);
    }

    verts
}

fn draw_cube(pos: Vec3f, size: Vec3f, col: Color) -> Vec<f32> {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;
    let width = size.x;
    let height = size.y;
    let depth = size.z;

    let mut verts = Vec::new();

    let mut pos1 = Vec4f::new(x, y, z, 1.0);
    let mut pos2 = Vec4f::new(x, y + height, z, 1.0);
    let mut pos3 = Vec4f::new(x + width, y + height, z, 1.0);
    verts.append(&mut draw_plane(pos1, pos2, pos3, col)); // FRONT

    pos1 = Vec4f::new(x, y, z + depth, 1.0);
    pos2 = Vec4f::new(x, y + height, z + depth, 1.0);
    pos3 = Vec4f::new(x + width, y + height, z + depth, 1.0);
    verts.append(&mut draw_plane(pos1, pos2, pos3, col));  // BAK

    pos1 = Vec4f::new(x, y, z, 1.0);
    pos2 = Vec4f::new(x, y + height, z, 1.0);
    pos3 = Vec4f::new(x, y + height, z + depth, 1.0);
    verts.append(&mut draw_plane(pos1, pos2, pos3, col));  // LEFT

    pos1 = Vec4f::new(x + width, y, z, 1.0);
    pos2 = Vec4f::new(x + width, y + height, z, 1.0);
    pos3 = Vec4f::new(x + width, y + height, z + depth, 1.0);
    verts.append(&mut draw_plane(pos1, pos2, pos3, col));  // RIGHT

    pos1 = Vec4f::new(x, y + height, z, 1.0);
    pos2 = Vec4f::new(x, y + height, z + depth, 1.0);
    pos3 = Vec4f::new(x + width, y + height, z + depth, 1.0);
    verts.append(&mut draw_plane(pos1, pos2, pos3, col));  // TOP

    pos1 = Vec4f::new(x, y, z, 1.0);
    pos2 = Vec4f::new(x, y, z + depth, 1.0);
    pos3 = Vec4f::new(x + width, y, z + depth, 1.0);
    verts.append(&mut draw_plane(pos1, pos2, pos3, col)); // BOTTOM

    verts
}

struct ChunkMesh {
    vb: VertexBuffer,
    va: VertexArray,
    vbl: VertexBufferLayout,
    count: usize,
}

impl ChunkMesh {
    fn new(chunk: &Chunk) -> ChunkMesh {
        let mut verts = Vec::new();

        for z in 0..32 {
            for y in 0..32 {
                for x in 0..32 {
                    let mut check = false;
                    if !(x == 0 || y == 0 || z == 0 || x == 31 || y == 31 || z == 31) {
                        for i in 0..3 {
                            for j in 0..3 {
                                for l in 0..3 {
                                    if chunk.get(x+l-1, y+j-1, z+i-1) == BlockID::None  {
                                        check = true;
                                    }
                                }
                            }
                        }
                    } else {
                        check = true;
                    }

                    if !check {
                        continue;
                    }

                    match chunk.get(x, y, z) {
                        BlockID::None => {}
                        BlockID::Dirt => {
                            verts.append(&mut draw_cube(
                             Vec3f::new(x as f32, y as f32, z as f32),
                            Vec3f::new(1.0, 1.0, 1.0),
                             Color::new(1.0, 1.0, 1.0, 1.0)
                            ));
                        }
                        BlockID::Stone => {
                            verts.append(&mut draw_cube(
                             Vec3f::new(x as f32, y as f32, z as f32),
                            Vec3f::new(1.0, 1.0, 1.0),
                             Color::new(0.0, 1.0, 1.0, 1.0)
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }

        let count = verts.len();

        println!("{}", count);

        let vb = VertexBuffer::new(&verts);
        let mut va = VertexArray::new();
        let mut vbl = VertexBufferLayout::new();
        vbl.push_f32(3); //pos
        vbl.push_f32(2); //uv
        vbl.push_f32(3); //col
        va.add_buffer(&vb, &vbl);

        ChunkMesh {
            vb,
            vbl,
            va,
            count,
        }
    }
}

struct Game3D {
    chunk: Chunk,
    chunk_mesh: ChunkMesh,
    win: Window,
    shader: Shader,
    timer: f32,
    pos: Vec3f,
}

impl Game for Game3D {
    fn on_start(&mut self, _gd: &mut GameData) {
        self.pos.x = -10.0;
        self.pos.z = 10.0;
    }

    fn on_update(&mut self, gd: &mut GameData) {
        self.timer += gd.delta_time();
        if self.timer >= 1.0 {
            self.timer = 0.0;
            println!("{}", gd.frame_rate());
        }
    }

    fn on_render(&mut self, gd: &mut GameData) {
        let fov = 90.0;
        let aspect_ratio = self.win.get_height() as f32 / self.win.get_width() as f32;
        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();

        let projection = Mat4x4f::projection(aspect_ratio, fov_rad, 100.0, 0.1);

        let model = Mat4x4f::mult(&Mat4x4f::rotation_y(0.0), &Mat4x4f::rotation_x(1.0));
        let model = Mat4x4f::mult(&Mat4x4f::translation(self.pos.x, self.pos.y, self.pos.z), &model);
        let mvp = Mat4x4f::mult(&projection, &model);
        //m += 0.001;

        self.pos.z -= 1.0 * gd.delta_time();

        unsafe { std_renderer::clear(std_renderer::ClearTarget::Depth); }
        unsafe { std_renderer::clear(std_renderer::ClearTarget::Color); }

        self.shader.bind();
        self.shader.upload_from_name_4x4f("u_mat", unsafe { &mvp.values });

        self.chunk_mesh.va.bind();
        unsafe { 
            std_renderer::draw_array(
                std_renderer::RenderingPrimitive::Triangles, 
                self.chunk_mesh.count as i32
            ) 
        };

        self.win.poll_events();
        self.win.swap_buffers();
    }
}

fn main() {
    let mut win = Window::new(600, 400, "Game").unwrap();
    win.make_current();
    init_gl(&mut win);

    let mut chunk = Chunk::new();
    
    for i in 0..16 {
        for j in 0..32 {
            for l in 0..32 {
                chunk.set(l, j, i*2, BlockID::Dirt);
                chunk.set(l, j, i*2+1, BlockID::Stone);
            }
        }
    }

    let mesh = ChunkMesh::new(&chunk);

    let shader = Shader::from_file("res/shaders/shader.glsl");
    let texture = Texture::from_file("res/textures/test.png");
    texture.bind(0);

    unsafe { std_renderer::enable(std_renderer::Capability::DepthTest); }

    GameContainer::new().run(Game3D{chunk, chunk_mesh: mesh, win, shader, timer: 0.0, pos: Vec3f::new(0.0, 0.0, 0.0)});
}
