pub trait RenderingComponent {
    type RenderType;

    fn tick(&mut self);

    fn get_drawing_coordinates(&mut self) -> Self::RenderType;

    fn handle_request(&mut self) -> Self::RenderType {
        self.tick();
        self.get_drawing_coordinates()
    }
}
