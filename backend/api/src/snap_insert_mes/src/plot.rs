use lazy_static::lazy_static;
use plotly::{Layout, Plot, Scatter, common::Mode, layout::Axis};

lazy_static! {
    pub static ref PLOT_FILENAME: String = "plot.html".to_string();
}

pub fn plot_graphs(plot_name: &str, graph_names: &[String], x: &[usize], ys: &[Vec<f64>]) -> Plot {
    let mut plot = Plot::new();

    let layout = Layout::new()
        .title(plot_name)
        .x_axis(Axis::new().title("Кол-во вставляемых строк"))
        .y_axis(Axis::new().title("Время выполнения(нс)"));

    plot.set_layout(layout);

    for (i, y) in ys.iter().enumerate() {
        let scatter = Scatter::new(x.to_vec(), y.to_vec())
            .mode(Mode::LinesMarkers)
            .name(&graph_names[i]);

        plot.add_trace(scatter);
    }

    plot
}

pub fn show_plot(plt: &Plot) {
    plt.show()
}

pub fn plot_to_htmlfile(filename: &str, plt: &Plot) {
    let file_path = filename;
    let html_plt = plt.to_html();

    std::fs::write(&file_path, html_plt)
        .expect(&format!("Не удалось сохранить график в файл {}", filename));
}
