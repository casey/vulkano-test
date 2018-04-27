pub mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[path = "rsc/vs.glsl"]
    struct _Dummy;

    const _SOURCE: &str = include_str!("../rsc/vs.glsl");
}

pub mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[path = "rsc/fs.glsl"]
    struct _Dummy;

    const _SOURCE: &str = include_str!("../rsc/fs.glsl");
}
