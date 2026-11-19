//! Native GPU Dual-Kawase backdrop blur for borderless floating surfaces.

use egui::{Color32, Context, CornerRadius, Id, Rect, Shape};

use crate::feedback::ElevationTier;

pub(crate) fn backdrop_blur_state_id() -> Id {
    Id::unique("twoui_backdrop_blur_enabled")
}

/// Enables GPU backdrop blur on floating surfaces when running on a supported `glow` context.
pub fn enable_backdrop_blur(ctx: &Context) {
    set_backdrop_blur_enabled(ctx, true);
}

/// Explicitly enables or disables GPU backdrop blur for floating surfaces on `ctx`.
pub fn set_backdrop_blur_enabled(ctx: &Context, enabled: bool) {
    ctx.data_mut(|d| d.insert_temp(backdrop_blur_state_id(), enabled));
}

/// Returns `true` when GPU backdrop blur is compiled in and enabled on `ctx`.
pub fn is_backdrop_blur_enabled(ctx: &Context) -> bool {
    #[cfg(all(feature = "blur", not(target_arch = "wasm32")))]
    {
        ctx.data(|d| d.get_temp::<bool>(backdrop_blur_state_id()))
            .unwrap_or(false)
    }
    #[cfg(not(all(feature = "blur", not(target_arch = "wasm32"))))]
    {
        let _ = ctx;
        false
    }
}

pub(crate) fn backdrop_blur_shape(
    ctx: &Context,
    rect: Rect,
    corner_radius: CornerRadius,
    fill: Color32,
    tier: ElevationTier,
    dark: bool,
) -> Option<Shape> {
    if !is_backdrop_blur_enabled(ctx)
        || fill.a() == 0
        || rect.width() <= 1.0
        || rect.height() <= 1.0
    {
        return None;
    }

    #[cfg(all(feature = "blur", not(target_arch = "wasm32")))]
    {
        Some(imp::make_blur_shape(
            ctx,
            rect,
            corner_radius,
            fill,
            tier,
            dark,
        ))
    }
    #[cfg(not(all(feature = "blur", not(target_arch = "wasm32"))))]
    {
        let _ = (corner_radius, tier, dark);
        None
    }
}

#[cfg(all(feature = "blur", not(target_arch = "wasm32")))]
mod imp {
    use std::sync::{Arc, Mutex};

    use egui::{
        epaint::Shadow, Color32, Context, CornerRadius, Id, PaintCallback, PaintCallbackInfo, Rect,
        Shape,
    };
    use egui_glow::glow::{self, HasContext as _};
    use egui_glow::{CallbackFn, Painter};

    use crate::feedback::{elevation_shadow, ElevationTier};

    const MAX_CACHE_ENTRIES: usize = 8;

    const VS_QUAD: &str = r#"
out vec2 v_uv;
void main() {
    float x = float((gl_VertexID & 1) << 2) - 1.0;
    float y = float((gl_VertexID & 2) << 1) - 1.0;
    v_uv = vec2(x, y) * 0.5 + 0.5;
    gl_Position = vec4(x, y, 0.0, 1.0);
}
"#;

    const FS_DOWNSAMPLE: &str = r#"
in vec2 v_uv;
out vec4 frag_color;
uniform sampler2D u_tex;
uniform vec2 u_halfpixel;
uniform float u_offset;

void main() {
    vec2 o = u_halfpixel * u_offset;
    vec4 sum = texture(u_tex, v_uv) * 4.0;
    sum += texture(u_tex, v_uv - o);
    sum += texture(u_tex, v_uv + o);
    sum += texture(u_tex, v_uv + vec2(o.x, -o.y));
    sum += texture(u_tex, v_uv - vec2(o.x, -o.y));
    frag_color = sum / 8.0;
}
"#;

    const FS_UPSAMPLE: &str = r#"
in vec2 v_uv;
out vec4 frag_color;
uniform sampler2D u_tex;
uniform vec2 u_halfpixel;
uniform float u_offset;

void main() {
    vec2 hp = u_halfpixel * u_offset;
    vec4 sum = texture(u_tex, v_uv + vec2(-hp.x * 2.0, 0.0));
    sum += texture(u_tex, v_uv + vec2(-hp.x, hp.y)) * 2.0;
    sum += texture(u_tex, v_uv + vec2(0.0, hp.y * 2.0));
    sum += texture(u_tex, v_uv + vec2(hp.x, hp.y)) * 2.0;
    sum += texture(u_tex, v_uv + vec2(hp.x * 2.0, 0.0));
    sum += texture(u_tex, v_uv + vec2(hp.x, -hp.y)) * 2.0;
    sum += texture(u_tex, v_uv + vec2(0.0, -hp.y * 2.0));
    sum += texture(u_tex, v_uv + vec2(-hp.x, -hp.y)) * 2.0;
    frag_color = sum / 12.0;
}
"#;

    const VS_COMPOSITE: &str = r#"
void main() {
    float x = float((gl_VertexID & 1) << 2) - 1.0;
    float y = float((gl_VertexID & 2) << 1) - 1.0;
    gl_Position = vec4(x, y, 0.0, 1.0);
}
"#;

    const FS_COMPOSITE: &str = r#"
out vec4 frag_color;
uniform sampler2D u_tex;
uniform vec2 u_rect_center;
uniform vec2 u_half_size;
uniform vec4 u_radii;
uniform vec2 u_cap_origin;
uniform vec2 u_cap_size;
uniform vec4 u_tint;
uniform float u_presence;
uniform vec4 u_shadow;

float sd_rounded_box(vec2 p, vec2 b, vec4 r) {
    vec2 rr = (p.x > 0.0) ? r.xy : r.zw;
    float rad = (p.y > 0.0) ? rr.x : rr.y;
    vec2 q = abs(p) - b + vec2(rad);
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - rad;
}

void main() {
    vec2 p = gl_FragCoord.xy - u_rect_center;
    float d = sd_rounded_box(p, u_half_size, u_radii);
    float sdf_a = 1.0 - smoothstep(-0.5, 0.5, d);

    vec2 uv_blur = clamp((gl_FragCoord.xy - u_cap_origin) / u_cap_size, vec2(0.0), vec2(1.0));
    vec3 raw_blur = texture(u_tex, uv_blur).rgb;
    float luma = dot(raw_blur, vec3(0.299, 0.587, 0.114));
    vec3 vibrant = clamp(mix(vec3(luma), raw_blur, 1.15), 0.0, 1.0);
    vec3 frosted = mix(vibrant, u_tint.rgb, u_tint.a);

    vec4 shadow_col = vec4(0.0);
    if (u_shadow.w > 0.001) {
        float ds = sd_rounded_box(p - vec2(u_shadow.x, -u_shadow.y), u_half_size, u_radii);
        float spread = max(u_shadow.z, 1.0);
        float s = clamp(1.0 - (ds + spread * 0.25) / (spread * 1.25), 0.0, 1.0);
        float sa = s * s * (3.0 - 2.0 * s) * u_shadow.w * u_presence;
        shadow_col = vec4(0.0, 0.0, 0.0, sa);
    }

    float glass_a = sdf_a * u_presence;
    vec4 glass_col = vec4(frosted * glass_a, glass_a);
    vec4 final_col = glass_col + shadow_col * (1.0 - glass_a);
    if (final_col.a <= 0.001) {
        discard;
    }
    frag_color = final_col;
}
"#;

    #[derive(Clone, Copy)]
    struct BlurDrawParams {
        rect: Rect,
        corner_radius: CornerRadius,
        tint_rgb: [f32; 3],
        tint_alpha: f32,
        presence: f32,
        shadow_offset_y: f32,
        shadow_blur: f32,
        shadow_alpha: f32,
        halo_pt: f32,
    }

    pub(crate) fn make_blur_shape(
        ctx: &Context,
        rect: Rect,
        corner_radius: CornerRadius,
        fill: Color32,
        tier: ElevationTier,
        dark: bool,
    ) -> Shape {
        let [r, g, b, a] = fill.to_srgba_unmultiplied();
        let r_f = f32::from(r) / 255.0;
        let g_f = f32::from(g) / 255.0;
        let b_f = f32::from(b) / 255.0;
        let presence = (f32::from(a) / 204.0).clamp(0.0, 1.0);
        let lum = 0.299 * r_f + 0.587 * g_f + 0.114 * b_f;
        let tint_alpha = if dark || lum < 0.35 { 0.68 } else { 0.72 };

        let shadow = elevation_shadow(tier, dark);
        let (shadow_offset_y, shadow_blur, shadow_alpha) = if shadow == Shadow::NONE {
            (0.0, 0.0, 0.0)
        } else {
            (
                f32::from(shadow.offset[1]),
                f32::from(shadow.blur),
                f32::from(shadow.color.a()) / 255.0,
            )
        };
        let halo_pt = if shadow_alpha > 0.001 {
            (shadow_blur * 1.4 + shadow_offset_y.abs()).ceil() + 2.0
        } else {
            2.0
        };
        let cb_rect = rect.expand(halo_pt);
        let params = BlurDrawParams {
            rect,
            corner_radius,
            tint_rgb: [r_f, g_f, b_f],
            tint_alpha,
            presence,
            shadow_offset_y,
            shadow_blur,
            shadow_alpha,
            halo_pt,
        };

        let engine_id = Id::unique("twoui_glow_blur_engine");
        let engine = ctx.data_mut(|d| {
            d.get_temp_mut_or_insert_with(engine_id, || Arc::new(Mutex::new(BlurEngine::default())))
                .clone()
        });

        Shape::Callback(PaintCallback {
            rect: cb_rect,
            callback: Arc::new(CallbackFn::new(move |info, painter| {
                if let Ok(mut guard) = engine.lock() {
                    guard.render(&info, painter, &params);
                }
            })),
        })
    }

    struct BlurLevel {
        w: i32,
        h: i32,
        tex: glow::Texture,
        fbo: glow::Framebuffer,
    }

    struct BlurPyramid {
        key: (i32, i32),
        last_used: u64,
        levels: Vec<BlurLevel>,
    }

    struct DownUpProgram {
        program: glow::Program,
        u_tex: Option<glow::UniformLocation>,
        u_halfpixel: Option<glow::UniformLocation>,
        u_offset: Option<glow::UniformLocation>,
    }

    struct CompositeProgram {
        program: glow::Program,
        u_tex: Option<glow::UniformLocation>,
        u_rect_center: Option<glow::UniformLocation>,
        u_half_size: Option<glow::UniformLocation>,
        u_radii: Option<glow::UniformLocation>,
        u_cap_origin: Option<glow::UniformLocation>,
        u_cap_size: Option<glow::UniformLocation>,
        u_tint: Option<glow::UniformLocation>,
        u_presence: Option<glow::UniformLocation>,
        u_shadow: Option<glow::UniformLocation>,
    }

    struct BlurPipeline {
        vao: glow::VertexArray,
        down: DownUpProgram,
        up: DownUpProgram,
        comp: CompositeProgram,
    }

    #[derive(Default)]
    struct BlurEngine {
        gl: Option<Arc<glow::Context>>,
        pipeline: Option<BlurPipeline>,
        cache: Vec<BlurPyramid>,
        frame_counter: u64,
        failed: bool,
    }

    impl Drop for BlurEngine {
        fn drop(&mut self) {
            let Some(gl) = self.gl.take() else {
                return;
            };
            unsafe {
                for pyr in self.cache.drain(..) {
                    delete_pyramid(&gl, pyr);
                }
                if let Some(p) = self.pipeline.take() {
                    gl.delete_program(p.down.program);
                    gl.delete_program(p.up.program);
                    gl.delete_program(p.comp.program);
                    gl.delete_vertex_array(p.vao);
                }
            }
        }
    }

    impl BlurEngine {
        fn ensure_pipeline(&mut self, gl: &Arc<glow::Context>) -> bool {
            if self.pipeline.is_some() {
                return true;
            }
            if self.failed {
                return false;
            }
            match create_pipeline(gl) {
                Ok(p) => {
                    self.gl = Some(gl.clone());
                    self.pipeline = Some(p);
                    true
                }
                Err(_) => {
                    self.failed = true;
                    false
                }
            }
        }

        fn acquire_pyramid(
            &mut self,
            gl: &glow::Context,
            key: (i32, i32),
            restore_fbo: Option<glow::Framebuffer>,
        ) -> Option<usize> {
            self.frame_counter = self.frame_counter.wrapping_add(1);
            let stamp = self.frame_counter;

            if let Some(idx) = self.cache.iter().position(|p| p.key == key) {
                self.cache[idx].last_used = stamp;
                return Some(idx);
            }

            if self.cache.len() >= MAX_CACHE_ENTRIES {
                if let Some((evict_idx, _)) = self
                    .cache
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, p)| p.last_used)
                {
                    let evicted = self.cache.swap_remove(evict_idx);
                    unsafe {
                        delete_pyramid(gl, evicted);
                    }
                }
            }

            let pyramid = create_pyramid(gl, key, stamp, restore_fbo).ok()?;
            self.cache.push(pyramid);
            Some(self.cache.len() - 1)
        }

        fn render(&mut self, info: &PaintCallbackInfo, painter: &Painter, params: &BlurDrawParams) {
            let gl = painter.gl();
            if !self.ensure_pipeline(gl) {
                return;
            }

            let ppp = info.pixels_per_point.max(0.5);
            let sw_f = info.screen_size_px[0] as f32;
            let sh_f = info.screen_size_px[1] as f32;
            let sw = info.screen_size_px[0].min(i32::MAX as u32) as i32;
            let sh = info.screen_size_px[1].min(i32::MAX as u32) as i32;
            if sw < 2 || sh < 2 {
                return;
            }

            let rx0 = params.rect.min.x * ppp;
            let rx1 = params.rect.max.x * ppp;
            let ry0 = sh_f - params.rect.max.y * ppp;
            let ry1 = sh_f - params.rect.min.y * ppp;
            let rhw = (rx1 - rx0) * 0.5;
            let rhh = (ry1 - ry0) * 0.5;
            if rhw <= 0.5 || rhh <= 0.5 {
                return;
            }

            let blur_pad = (60.0 * ppp).ceil() as i32;
            let cap_x0 = (rx0.floor() as i32 - blur_pad).clamp(0, sw);
            let cap_x1 = (rx1.ceil() as i32 + blur_pad).clamp(0, sw);
            let cap_y0 = (ry0.floor() as i32 - blur_pad).clamp(0, sh);
            let cap_y1 = (ry1.ceil() as i32 + blur_pad).clamp(0, sh);
            let cap_w = cap_x1 - cap_x0;
            let cap_h = cap_y1 - cap_y0;
            if cap_w < 2 || cap_h < 2 {
                return;
            }

            let halo_px = (params.halo_pt * ppp).ceil() as i32;
            let dx0 = (rx0.floor() as i32 - halo_px).clamp(0, sw);
            let dx1 = (rx1.ceil() as i32 + halo_px).clamp(0, sw);
            let dy0 = (ry0.floor() as i32 - halo_px).clamp(0, sh);
            let dy1 = (ry1.ceil() as i32 + halo_px).clamp(0, sh);
            let dw = dx1 - dx0;
            let dh = dy1 - dy0;
            if dw < 2 || dh < 2 {
                return;
            }

            let cx0 = (info.clip_rect.min.x * ppp).floor().max(0.0) as i32;
            let cx1 = (info.clip_rect.max.x * ppp).ceil().min(sw_f) as i32;
            let cy0 = (sh_f - info.clip_rect.max.y * ppp).floor().max(0.0) as i32;
            let cy1 = (sh_f - info.clip_rect.min.y * ppp).ceil().min(sh_f) as i32;
            let cw = cx1 - cx0;
            let ch = cy1 - cy0;
            if cw <= 0 || ch <= 0 {
                return;
            }

            let Some(pyr_idx) = self.acquire_pyramid(gl, (cap_w, cap_h), painter.intermediate_fbo())
            else {
                return;
            };
            let Some(pipe) = self.pipeline.as_ref() else {
                return;
            };
            let pyr = &self.cache[pyr_idx];
            if pyr.levels.len() < 5 {
                return;
            }

            let max_r = rhw.min(rhh);
            let rad_px = |v: u8| -> f32 { (f32::from(v) * ppp).min(max_r) };
            let r_ne = rad_px(params.corner_radius.ne);
            let r_se = rad_px(params.corner_radius.se);
            let r_nw = rad_px(params.corner_radius.nw);
            let r_sw = rad_px(params.corner_radius.sw);

            unsafe {
                gl.disable(glow::SCISSOR_TEST);
                gl.disable(glow::BLEND);
                gl.disable(glow::DEPTH_TEST);
                gl.disable(glow::CULL_FACE);

                // Copy clean live backdrop into pyramid base level 0.
                gl.bind_framebuffer(glow::READ_FRAMEBUFFER, painter.intermediate_fbo());
                gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(pyr.levels[0].fbo));
                gl.blit_framebuffer(
                    cap_x0,
                    cap_y0,
                    cap_x1,
                    cap_y1,
                    0,
                    0,
                    cap_w,
                    cap_h,
                    glow::COLOR_BUFFER_BIT,
                    glow::NEAREST,
                );

                gl.bind_vertex_array(Some(pipe.vao));
                gl.active_texture(glow::TEXTURE0);

                // Downsample: 0 -> 1 -> 2 -> 3 -> 4
                gl.use_program(Some(pipe.down.program));
                gl.uniform_1_i32(pipe.down.u_tex.as_ref(), 0);
                gl.uniform_1_f32(pipe.down.u_offset.as_ref(), 2.0);
                for pair in pyr.levels.windows(2) {
                    let (src, dst) = (&pair[0], &pair[1]);
                    gl.bind_framebuffer(glow::FRAMEBUFFER, Some(dst.fbo));
                    gl.viewport(0, 0, dst.w, dst.h);
                    gl.bind_texture(glow::TEXTURE_2D, Some(src.tex));
                    gl.uniform_2_f32(
                        pipe.down.u_halfpixel.as_ref(),
                        0.5 / dst.w as f32,
                        0.5 / dst.h as f32,
                    );
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                }

                // Upsample: 4 -> 3 -> 2 -> 1
                gl.use_program(Some(pipe.up.program));
                gl.uniform_1_i32(pipe.up.u_tex.as_ref(), 0);
                gl.uniform_1_f32(pipe.up.u_offset.as_ref(), 2.0);
                for pair in pyr.levels[1..].windows(2).rev() {
                    let (dst, src) = (&pair[0], &pair[1]);
                    gl.bind_framebuffer(glow::FRAMEBUFFER, Some(dst.fbo));
                    gl.viewport(0, 0, dst.w, dst.h);
                    gl.bind_texture(glow::TEXTURE_2D, Some(src.tex));
                    gl.uniform_2_f32(
                        pipe.up.u_halfpixel.as_ref(),
                        0.5 / dst.w as f32,
                        0.5 / dst.h as f32,
                    );
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                }

                // Composite dense frosted glass + analytic shadow into egui's target FBO.
                gl.bind_framebuffer(glow::FRAMEBUFFER, painter.intermediate_fbo());
                gl.enable(glow::SCISSOR_TEST);
                gl.scissor(cx0, cy0, cw, ch);
                gl.enable(glow::BLEND);
                gl.blend_equation_separate(glow::FUNC_ADD, glow::FUNC_ADD);
                gl.blend_func_separate(
                    glow::ONE,
                    glow::ONE_MINUS_SRC_ALPHA,
                    glow::ONE_MINUS_DST_ALPHA,
                    glow::ONE,
                );
                gl.viewport(dx0, dy0, dw, dh);

                gl.use_program(Some(pipe.comp.program));
                gl.bind_texture(glow::TEXTURE_2D, Some(pyr.levels[1].tex));
                gl.uniform_1_i32(pipe.comp.u_tex.as_ref(), 0);
                gl.uniform_2_f32(
                    pipe.comp.u_rect_center.as_ref(),
                    (rx0 + rx1) * 0.5,
                    (ry0 + ry1) * 0.5,
                );
                gl.uniform_2_f32(pipe.comp.u_half_size.as_ref(), rhw, rhh);
                gl.uniform_4_f32(pipe.comp.u_radii.as_ref(), r_ne, r_se, r_nw, r_sw);
                gl.uniform_2_f32(
                    pipe.comp.u_cap_origin.as_ref(),
                    cap_x0 as f32,
                    cap_y0 as f32,
                );
                gl.uniform_2_f32(pipe.comp.u_cap_size.as_ref(), cap_w as f32, cap_h as f32);
                gl.uniform_4_f32(
                    pipe.comp.u_tint.as_ref(),
                    params.tint_rgb[0],
                    params.tint_rgb[1],
                    params.tint_rgb[2],
                    params.tint_alpha,
                );
                gl.uniform_1_f32(pipe.comp.u_presence.as_ref(), params.presence);
                gl.uniform_4_f32(
                    pipe.comp.u_shadow.as_ref(),
                    0.0,
                    params.shadow_offset_y * ppp,
                    params.shadow_blur * ppp,
                    params.shadow_alpha,
                );
                gl.draw_arrays(glow::TRIANGLES, 0, 3);

                gl.bind_texture(glow::TEXTURE_2D, None);
                gl.bind_vertex_array(None);
                gl.use_program(None);
                gl.bind_framebuffer(glow::FRAMEBUFFER, painter.intermediate_fbo());
            }
        }
    }

    fn create_pipeline(gl: &glow::Context) -> Result<BlurPipeline, String> {
        let header = if gl.version().is_embedded {
            "#version 300 es\nprecision highp float;\n"
        } else {
            "#version 330 core\n"
        };

        let down_prog = compile_program(gl, header, VS_QUAD, FS_DOWNSAMPLE)?;
        let up_prog = match compile_program(gl, header, VS_QUAD, FS_UPSAMPLE) {
            Ok(p) => p,
            Err(err) => {
                unsafe { gl.delete_program(down_prog) };
                return Err(err);
            }
        };
        let comp_prog = match compile_program(gl, header, VS_COMPOSITE, FS_COMPOSITE) {
            Ok(p) => p,
            Err(err) => {
                unsafe {
                    gl.delete_program(down_prog);
                    gl.delete_program(up_prog);
                }
                return Err(err);
            }
        };
        let vao = match unsafe { gl.create_vertex_array() } {
            Ok(v) => v,
            Err(err) => {
                unsafe {
                    gl.delete_program(down_prog);
                    gl.delete_program(up_prog);
                    gl.delete_program(comp_prog);
                }
                return Err(err);
            }
        };

        unsafe {
            let down = DownUpProgram {
                u_tex: gl.get_uniform_location(down_prog, "u_tex"),
                u_halfpixel: gl.get_uniform_location(down_prog, "u_halfpixel"),
                u_offset: gl.get_uniform_location(down_prog, "u_offset"),
                program: down_prog,
            };
            let up = DownUpProgram {
                u_tex: gl.get_uniform_location(up_prog, "u_tex"),
                u_halfpixel: gl.get_uniform_location(up_prog, "u_halfpixel"),
                u_offset: gl.get_uniform_location(up_prog, "u_offset"),
                program: up_prog,
            };
            let comp = CompositeProgram {
                u_tex: gl.get_uniform_location(comp_prog, "u_tex"),
                u_rect_center: gl.get_uniform_location(comp_prog, "u_rect_center"),
                u_half_size: gl.get_uniform_location(comp_prog, "u_half_size"),
                u_radii: gl.get_uniform_location(comp_prog, "u_radii"),
                u_cap_origin: gl.get_uniform_location(comp_prog, "u_cap_origin"),
                u_cap_size: gl.get_uniform_location(comp_prog, "u_cap_size"),
                u_tint: gl.get_uniform_location(comp_prog, "u_tint"),
                u_presence: gl.get_uniform_location(comp_prog, "u_presence"),
                u_shadow: gl.get_uniform_location(comp_prog, "u_shadow"),
                program: comp_prog,
            };
            Ok(BlurPipeline {
                vao,
                down,
                up,
                comp,
            })
        }
    }

    fn compile_program(
        gl: &glow::Context,
        header: &str,
        vs_body: &str,
        fs_body: &str,
    ) -> Result<glow::Program, String> {
        let vs_src = format!("{header}{vs_body}");
        let fs_src = format!("{header}{fs_body}");
        let vs = compile_shader(gl, glow::VERTEX_SHADER, &vs_src)?;
        let fs = match compile_shader(gl, glow::FRAGMENT_SHADER, &fs_src) {
            Ok(s) => s,
            Err(err) => {
                unsafe { gl.delete_shader(vs) };
                return Err(err);
            }
        };

        unsafe {
            let prog = match gl.create_program() {
                Ok(p) => p,
                Err(err) => {
                    gl.delete_shader(vs);
                    gl.delete_shader(fs);
                    return Err(err);
                }
            };
            gl.attach_shader(prog, vs);
            gl.attach_shader(prog, fs);
            gl.link_program(prog);
            let ok = gl.get_program_link_status(prog);
            let log = if ok {
                String::new()
            } else {
                gl.get_program_info_log(prog)
            };
            gl.detach_shader(prog, vs);
            gl.detach_shader(prog, fs);
            gl.delete_shader(vs);
            gl.delete_shader(fs);
            if !ok {
                gl.delete_program(prog);
                return Err(log);
            }
            Ok(prog)
        }
    }

    fn compile_shader(
        gl: &glow::Context,
        shader_type: u32,
        source: &str,
    ) -> Result<glow::Shader, String> {
        unsafe {
            let shader = gl.create_shader(shader_type)?;
            gl.shader_source(shader, source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                let log = gl.get_shader_info_log(shader);
                gl.delete_shader(shader);
                return Err(log);
            }
            Ok(shader)
        }
    }

    fn create_pyramid(
        gl: &glow::Context,
        key: (i32, i32),
        stamp: u64,
        restore_fbo: Option<glow::Framebuffer>,
    ) -> Result<BlurPyramid, String> {
        let mut levels = Vec::with_capacity(5);
        let mut w = key.0;
        let mut h = key.1;
        for _ in 0..5 {
            match create_level(gl, w, h, restore_fbo) {
                Ok(lvl) => levels.push(lvl),
                Err(err) => {
                    unsafe {
                        gl.bind_framebuffer(glow::FRAMEBUFFER, restore_fbo);
                        for l in levels {
                            gl.delete_framebuffer(l.fbo);
                            gl.delete_texture(l.tex);
                        }
                    }
                    return Err(err);
                }
            }
            w = (w / 2).max(1);
            h = (h / 2).max(1);
        }
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, restore_fbo);
        }
        Ok(BlurPyramid {
            key,
            last_used: stamp,
            levels,
        })
    }

    fn create_level(
        gl: &glow::Context,
        w: i32,
        h: i32,
        restore_fbo: Option<glow::Framebuffer>,
    ) -> Result<BlurLevel, String> {
        unsafe {
            let tex = gl.create_texture()?;
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                w,
                h,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(None),
            );
            gl.bind_texture(glow::TEXTURE_2D, None);

            let fbo = match gl.create_framebuffer() {
                Ok(f) => f,
                Err(err) => {
                    gl.delete_texture(tex);
                    return Err(err);
                }
            };
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(tex),
                0,
            );
            let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
            if status != glow::FRAMEBUFFER_COMPLETE {
                gl.bind_framebuffer(glow::FRAMEBUFFER, restore_fbo);
                gl.delete_framebuffer(fbo);
                gl.delete_texture(tex);
                return Err(format!("Incomplete FBO status: {status}"));
            }
            Ok(BlurLevel { w, h, tex, fbo })
        }
    }

    unsafe fn delete_pyramid(gl: &glow::Context, pyr: BlurPyramid) {
        for lvl in pyr.levels {
            gl.delete_framebuffer(lvl.fbo);
            gl.delete_texture(lvl.tex);
        }
    }
}
