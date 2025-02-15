use fornjot_d::model;

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = model([3., 2., 1.], &mut fj.core);
    fj.process_model(&model)?;
    Ok(())
}
