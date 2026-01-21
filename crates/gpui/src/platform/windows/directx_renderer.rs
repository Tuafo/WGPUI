    collections::HashMap,
    transform::GpuTransform,
    globals: DirectXGlobalElements<GlobalParams>,
    context_transforms: AuxBuffer,
struct DirectXGlobalElements<T> {
    _marker: std::marker::PhantomData<T>,
        let context_transforms = {
            let buffer_size = 1024;
            let element_size = std::mem::size_of::<GpuTransform>();
            let buffer = create_buffer(&devices.device, element_size, buffer_size)?;
            let view = create_buffer_view(&devices.device, &buffer)?;
            AuxBuffer {
                buffer,
                buffer_size,
                element_size,
                view,
            }
        };
            context_transforms,
    fn update_context_transforms(
        &mut self,
        device: &ID3D11Device,
        device_context: &ID3D11DeviceContext,
        transforms: &[GpuTransform],
    ) -> Result<()> {
        debug_assert_eq!(
            self.context_transforms.element_size,
            std::mem::size_of::<GpuTransform>()
        );

        if self.context_transforms.buffer_size < transforms.len() {
            let new_buffer_size = transforms.len().next_power_of_two().max(1);
            log::info!(
                "Updating context transforms buffer size from {} to {}",
                self.context_transforms.buffer_size,
                new_buffer_size
            );
            let buffer = create_buffer(device, self.context_transforms.element_size, new_buffer_size)?;
            let view = create_buffer_view(device, &buffer)?;
            self.context_transforms.buffer = buffer;
            self.context_transforms.view = view;
            self.context_transforms.buffer_size = new_buffer_size;
        }

        update_buffer(device_context, &self.context_transforms.buffer, transforms)
    }

        let context_transforms = {
            let buffer_size = 1024;
            let element_size = std::mem::size_of::<GpuTransform>();
            let buffer = create_buffer(&devices.device, element_size, buffer_size)?;
            let view = create_buffer_view(&devices.device, &buffer)?;
            AuxBuffer {
                buffer,
                buffer_size,
                element_size,
                view,
            }
        };
        self.context_transforms = context_transforms;
        segment_pool: &SceneSegmentPool,

        let (device, device_context) = {
            let devices = self.devices.as_ref().context("devices missing")?;
            (devices.device.clone(), devices.device_context.clone())
        };
        let gpu_transforms = segment_pool.transforms.to_gpu_transforms();
        self.update_context_transforms(&device, &device_context, &gpu_transforms)?;

        for batch in scene.batches(segment_pool) {
                PrimitiveBatch::Shadows(shadows, transforms) => self.draw_shadows(shadows, transforms),
                PrimitiveBatch::Quads(quads, transforms) => self.draw_quads(quads, transforms),
                    self.draw_paths_to_intermediate(paths, &gpu_transforms)?;
                    self.draw_paths_from_intermediate(paths, &gpu_transforms)
                }
                PrimitiveBatch::Underlines(underlines, transforms) => {
                    self.draw_underlines(underlines, transforms)
                    transforms,
                } => self.draw_polychrome_sprites(texture_id, sprites, transforms),
                {} paths, {} shadows, {} quads, {} underlines, {} mono, {} subpixel, {} poly, {} custom, {} surfaces",
                scene.paths_len(segment_pool),
                scene.shadows_len(segment_pool),
                scene.quads_len(segment_pool),
                scene.underlines_len(segment_pool),
                scene.monochrome_sprites_len(segment_pool),
                scene.subpixel_sprites_len(segment_pool),
                scene.polychrome_sprites_len(segment_pool),
                scene.shaders.len(),
                scene.surfaces_len(segment_pool),
    fn draw_shadows(
        &mut self,
        shadows: &[Shadow],
        shadow_transforms: &[TransformationMatrix],
    ) -> Result<()> {
        debug_assert_eq!(shadows.len(), shadow_transforms.len());
        self.pipelines.shadow_pipeline.update_aux_buffer(
            &devices.device,
            &devices.device_context,
            shadow_transforms,
        )?;
            &self.context_transforms.view,
    fn draw_quads(
        &mut self,
        quads: &[Quad],
        quad_transforms: &[TransformationMatrix],
    ) -> Result<()> {
        debug_assert_eq!(quads.len(), quad_transforms.len());
        self.pipelines.quad_pipeline.update_aux_buffer(
            &devices.device,
            &devices.device_context,
            quad_transforms,
        )?;
            &self.context_transforms.view,
    fn draw_paths_to_intermediate(
        &mut self,
        paths: &[Path<ScaledPixels>],
        gpu_transforms: &[GpuTransform],
    ) -> Result<()> {
            let t = resolve_world_transform(path.transform_index, gpu_transforms);
            let world_bounds = transform_bounds(path.bounds, t);
            let clipped_bounds = world_bounds.intersect(&path.content_mask.bounds);
            if clipped_bounds.is_empty() {
                continue;
            }
                bounds: clipped_bounds,
                transform_index: path.transform_index,
                _pad: 0,
            &self.context_transforms.view,
    fn draw_paths_from_intermediate(
        &mut self,
        paths: &[Path<ScaledPixels>],
        gpu_transforms: &[GpuTransform],
    ) -> Result<()> {
                .filter_map(|path| {
                    let t = resolve_world_transform(path.transform_index, gpu_transforms);
                    let world_bounds =
                        transform_bounds(path.clipped_bounds(), t).intersect(&path.content_mask.bounds);
                    if world_bounds.is_empty() {
                        None
                    } else {
                        Some(PathSprite { bounds: world_bounds })
                    }
            let mut bounds: Option<Bounds<ScaledPixels>> = None;
            for path in paths {
                let t = resolve_world_transform(path.transform_index, gpu_transforms);
                let world_bounds =
                    transform_bounds(path.clipped_bounds(), t).intersect(&path.content_mask.bounds);
                if world_bounds.is_empty() {
                    continue;
                }
                bounds = Some(match bounds {
                    Some(existing) => existing.union(&world_bounds),
                    None => world_bounds,
                });
            bounds.map(|bounds| vec![PathSprite { bounds }]).unwrap_or_default()
        if sprites.is_empty() {
            return Ok(());
        }
            &self.context_transforms.view,
    fn draw_underlines(
        &mut self,
        underlines: &[Underline],
        underline_transforms: &[TransformationMatrix],
    ) -> Result<()> {
        debug_assert_eq!(underlines.len(), underline_transforms.len());
        self.pipelines.underline_pipeline.update_aux_buffer(
            &devices.device,
            &devices.device_context,
            underline_transforms,
        )?;
            &self.context_transforms.view,
            &self.context_transforms.view,
            &self.context_transforms.view,
        sprite_transforms: &[TransformationMatrix],
        debug_assert_eq!(sprites.len(), sprite_transforms.len());
        self.pipelines.poly_sprites.update_aux_buffer(
            &devices.device,
            &devices.device_context,
            sprite_transforms,
        )?;
            &self.context_transforms.view,
        let shadow_pipeline = PipelineState::new_with_aux(
            std::mem::size_of::<TransformationMatrix>(),
        let quad_pipeline = PipelineState::new_with_aux(
            std::mem::size_of::<TransformationMatrix>(),
        let underline_pipeline = PipelineState::new_with_aux(
            std::mem::size_of::<TransformationMatrix>(),
        let poly_sprites = PipelineState::new_with_aux(
            std::mem::size_of::<TransformationMatrix>(),
impl<T> DirectXGlobalElements<T> {
                ByteWidth: std::mem::size_of::<T>() as u32,
            _marker: std::marker::PhantomData,
struct AuxBuffer {
    buffer: ID3D11Buffer,
    buffer_size: usize,
    element_size: usize,
    view: Option<ID3D11ShaderResourceView>,
}

    aux: Option<AuxBuffer>,
            aux: None,
            blend_state,
            _marker: std::marker::PhantomData,
        })
    }

    fn new_with_aux(
        device: &ID3D11Device,
        label: &'static str,
        shader_module: ShaderModule,
        buffer_size: usize,
        aux_element_size: usize,
        blend_state: ID3D11BlendState,
    ) -> Result<Self> {
        let mut this = Self::new(device, label, shader_module, buffer_size, blend_state)?;
        let aux_buffer = create_buffer(device, aux_element_size, buffer_size)?;
        let aux_view = create_buffer_view(device, &aux_buffer)?;
        this.aux = Some(AuxBuffer {
            buffer: aux_buffer,
            buffer_size,
            element_size: aux_element_size,
            view: aux_view,
        });
        Ok(this)
    }

    fn new_custom(
        device: &ID3D11Device,
        hlsl: &str,
        buffer_size: usize,
        element_size: usize,
        blend_state: ID3D11BlendState,
    ) -> Result<Self> {
        let vertex = {
            let raw_shader = RawShaderBytes::new_custom(&hlsl, ShaderTarget::Vertex)?;
            create_vertex_shader(device, raw_shader.as_bytes())?
        };
        let fragment = {
            let raw_shader = RawShaderBytes::new_custom(&hlsl, ShaderTarget::Fragment)?;
            create_fragment_shader(device, raw_shader.as_bytes())?
        };

        let buffer = create_buffer(device, element_size, buffer_size)?;
        let view = create_buffer_view(device, &buffer)?;

        Ok(PipelineState {
            label: "custom",
            vertex,
            fragment,
            buffer,
            buffer_size,
            view,
            aux: None,
    fn update_aux_buffer<U>(
        &mut self,
        device: &ID3D11Device,
        device_context: &ID3D11DeviceContext,
        data: &[U],
    ) -> Result<()> {
        let aux = self.aux.as_mut().context("aux buffer missing")?;
        debug_assert_eq!(aux.element_size, std::mem::size_of::<U>());

        if aux.buffer_size < data.len() {
            let new_buffer_size = data.len().next_power_of_two();
            log::info!(
                "Updating {} aux buffer size from {} to {}",
                self.label,
                aux.buffer_size,
                new_buffer_size
            );
            let buffer = create_buffer(device, aux.element_size, new_buffer_size)?;
            let view = create_buffer_view(device, &buffer)?;
            aux.buffer = buffer;
            aux.view = view;
            aux.buffer_size = new_buffer_size;
        }
        update_buffer(device_context, &aux.buffer, data)
    }

        context_transforms: &Option<ID3D11ShaderResourceView>,
        let views = [
            self.view.clone(),
            self.aux.as_ref().and_then(|aux| aux.view.clone()),
            context_transforms.clone(),
        ];
            &views,
        context_transforms: &Option<ID3D11ShaderResourceView>,
        let views = [
            self.view.clone(),
            self.aux.as_ref().and_then(|aux| aux.view.clone()),
            context_transforms.clone(),
        ];
            &views,
    transform_index: u32,
    _pad: u32,
fn resolve_world_transform(transform_index: u32, transforms: &[GpuTransform]) -> GpuTransform {
    let mut resolved = GpuTransform::identity();
    let mut current = transform_index;
    for _ in 0..16 {
        if current == 0 {
            break;
        }
        let t = transforms
            .get(current as usize)
            .copied()
            .unwrap_or_else(GpuTransform::identity);
        resolved.offset[0] = resolved.offset[0] * t.scale + t.offset[0];
        resolved.offset[1] = resolved.offset[1] * t.scale + t.offset[1];
        resolved.scale *= t.scale;
        current = t.parent_index;
    }
    resolved
}

fn transform_bounds(bounds: Bounds<ScaledPixels>, t: GpuTransform) -> Bounds<ScaledPixels> {
    Bounds {
        origin: Point::new(
            ScaledPixels(bounds.origin.x.0 * t.scale + t.offset[0]),
            ScaledPixels(bounds.origin.y.0 * t.scale + t.offset[1]),
        ),
        size: Size {
            width: ScaledPixels(bounds.size.width.0 * t.scale),
            height: ScaledPixels(bounds.size.height.0 * t.scale),
        },
    }
}

    use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile;
    use windows::{Win32::Graphics::Direct3D::Fxc::D3DCompileFromFile, core::HSTRING};
            Fxc::{D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION},
        core::PCSTR,
        _blob: Option<ID3DBlob>,
                Ok(Self {
                    inner,
                    _blob: Some(blob),
                })
            }
        }

        pub(crate) fn new_custom(hlsl: &str, target: ShaderTarget) -> Result<Self> {
            let mut compile_blob = None;
            let mut error_blob = None;

            unsafe {
                let ret = D3DCompile(
                    hlsl.as_ptr() as *const _,
                    hlsl.len(),
                    PCSTR::null(),
                    None,
                    None,
                    PCSTR::from_raw(
                        match target {
                            ShaderTarget::Fragment => "fs\0",
                            ShaderTarget::Vertex => "vs\0",
                        }
                        .as_ptr(),
                    ),
                    PCSTR::from_raw(
                        match target {
                            ShaderTarget::Vertex => "vs_4_1\0",
                            ShaderTarget::Fragment => "ps_4_1\0",
                        }
                        .as_ptr(),
                    ),
                    if cfg!(debug_assertions) {
                        D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION
                    } else {
                        0
                    },
                    0,
                    &mut compile_blob,
                    Some(&mut error_blob),
                );

                if ret.is_err() {
                    let Some(error_blob) = error_blob else {
                        return Err(anyhow::anyhow!("{ret:?}"));
                    };

                    let error_string =
                        std::ffi::CStr::from_ptr(error_blob.GetBufferPointer() as *const i8)
                            .to_string_lossy();
                    log::error!("Shader compile error: {}", error_string);
                    return Err(anyhow::anyhow!("Compile error: {}", error_string));
                }
                let inner = std::slice::from_raw_parts(
                    compile_blob.as_ref().unwrap().GetBufferPointer() as *const u8,
                    compile_blob.as_ref().unwrap().GetBufferSize(),
                );
                Ok(Self {
                    inner,
                    _blob: compile_blob,
                })
            Self {
                inner: bytes,
                _blob: None,
            }