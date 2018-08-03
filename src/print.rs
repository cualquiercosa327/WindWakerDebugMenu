use super::get_state;
use libtww::system::gx;
pub fn get_coords(c: char) -> [[f32; 2]; 4] {
    let lx = c as usize % 16;
    let by = c as usize / 16 - 2;

    let ty = (by as f32) / 6.0;
    let by = ((by + 1) as f32) / 6.0;
    let rx = ((lx + 1) as f32 / 16.0) + 0.01;
    let lx = ((lx as f32) / 16.0) + 0.01;

    [[lx, ty], [rx, ty], [lx, by], [rx, by]]
}

pub unsafe fn printf(s: &str, x: f32, y:f32) {
    s.chars().enumerate().for_each(|(i, c)| {
        print_char(c, x + (17.0 * i as f32), y);
    });
}

pub unsafe fn print_char(c: char, x: f32, y: f32) {
        let state = get_state();
        gx::clear_vtx_desc();
        gx::set_vtx_desc(gx::VA_POS as u8, gx::DIRECT);
        gx::set_vtx_desc(gx::VA_TEX0 as u8, gx::DIRECT);

        gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_POS, gx::POS_XY, gx::F32, 0);
        gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_TEX0, gx::TEX_ST, gx::F32, 0);

        gx::set_num_tex_gens(1);
        gx::set_tex_coord_gen(
            gx::TEXCOORD0 as u16,
            gx::TG_MTX2X4,
            gx::TG_TEX0,
            gx::IDENTITY,
        );
        gx::set_tev_op(gx::TEVSTAGE0, gx::REPLACE);
        gx::set_tev_order(gx::TEVSTAGE0, gx::TEXCOORD0, gx::TEXMAP0, gx::COLOR0A0);
        gx::load_tex_obj(&mut state.tex_obj, gx::TEXMAP0 as u8);

        // gx::load_pos_mtx_imm(&mut system::j3d::CAMERA_MATRIX, gx::PNMTX0);
        gx::begin(gx::QUADS, gx::VTXFMT0, 4);
        {
            let coords = get_coords(c);
            gx::submit_f32s(&[x, y]);
            // gx::submit_f32s(&[0.0725, 0.333]);
            gx::submit_f32s(&coords[0]);
            gx::submit_f32s(&[x + 30.0, y]);
            // gx::submit_f32s(&[0.135, 0.333]);
            gx::submit_f32s(&coords[1]);
            gx::submit_f32s(&[x + 30.0, y + 30.0]);
            // gx::submit_f32s(&[0.135, 0.5]);
            gx::submit_f32s(&coords[3]);
            gx::submit_f32s(&[x, y + 30.0]);
            // gx::submit_f32s(&[0.0725, 0.5]);
            gx::submit_f32s(&coords[2]);
        }
        gx::end();
}
