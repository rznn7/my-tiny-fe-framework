use crate::core::el::El;

pub trait Component {
    fn on_init(&mut self) {}
    fn render(&self) -> El;

    fn mount(mut self) -> El
    where
        Self: Sized,
    {
        self.on_init();
        self.render()
    }
}
