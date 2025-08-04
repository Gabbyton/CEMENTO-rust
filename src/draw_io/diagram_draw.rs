pub use diagram_draw::draw_diagram;

pub mod diagram_draw {
    use std::fs;

    use crate::templates::get_template_dictionary;
    use tera::{Context, Tera};

    pub fn draw_diagram() {
        let templates = get_template_dictionary().expect("Cannot retrieve template dictionary");
        let template_key = "circle";
        let template_path = templates
            .get(template_key)
            .expect(format!("Cannot open template at {template_key}").as_str());

        // TODO: move insert statement properties into serde structs, use serde to insert
        let mut tera = Tera::new("templates/**/*").expect("Cannot read templates folder");
        let mut context = Context::new();
        context.insert("shape_id", "blah");
        context.insert("shape_content", "bleh");
        context.insert("fill_color", "#ff0000");
        context.insert("x_pos", "20");
        context.insert("y_pos", "96");
        context.insert("shape_width", "50");
        context.insert("shape_height", "120");

        let rendered_xml = tera
            .render(template_path, &context)
            .expect("There was an error while templating");

        fs::write("test.xml", &rendered_xml).expect("cannot write to file");
    }
}
