use std::{sync::mpsc, time::Duration};

use circular_buffer::CircularBuffer;
use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{
        canvas::{Cache, Frame, Geometry},
        text, Column, Container,
    },
    Alignment, Application, Command, Element, Length, Settings, Size, Theme,
};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend, Renderer};

pub fn init_window(rx: mpsc::Receiver<f32>) {
    let _ = State::run(Settings {
        antialiasing: true,
        ..Settings::with_flags(rx)
    });
}

#[derive(Debug)]
enum Message {
    Tick,
}

struct State {
    position: f32,
    chart: PositionChart,
    rx: mpsc::Receiver<f32>,
}

impl Application for State {
    type Executor = executor::Default;
    type Message = self::Message;
    type Theme = Theme;
    type Flags = mpsc::Receiver<f32>;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                position: -1.0_f32,
                chart: PositionChart {
                    cache: Cache::new(),
                    positions: CircularBuffer::<1000, f32>::new(),
                },
                rx: flags,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Position Plotter".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Tick => {
                if let Ok(pos) = self.rx.try_recv() {
                    self.position = pos;
                    self.chart.add_position(pos);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = Column::new()
            .spacing(20)
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(text(format!("Position : {}", self.position)))
            .push(self.chart.view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        const FPS: u64 = 25;
        // à 25 FPS, on envoie un tick. Sur réception du tick, on fait un try_recv pour
        // voir si la valeur de la position a changée (ainsi on ne bloque rien).
        // Il y aurait sans doute plus éléguant en transformant le mpsc::Receiver en stream et en
        // mappant ce stream en un stream de message contenant directement la position.
        iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| Message::Tick)
    }
}

struct PositionChart {
    cache: Cache,
    positions: circular_buffer::CircularBuffer<1000, f32>,
}

impl PositionChart {
    pub fn add_position(&mut self, position: f32) {
        self.cache.clear();
        self.positions.push_back(position);
    }

    fn view(&self) -> Element<Message> {
        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(5)
                .push(ChartWidget::new(self).height(Length::Fill)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }
}

impl Chart<Message> for PositionChart {
    type State = ();

    #[inline]
    fn draw<R: Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        bounds: Size,
        draw_fn: F,
    ) -> Geometry {
        renderer.draw_cache(&self.cache, bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

        const PLOT_LINE_COLOR: RGBColor = RGBColor(0, 175, 255);

        let mut chart = chart
            .x_label_area_size(0)
            .y_label_area_size(28)
            .margin(20)
            .build_cartesian_2d(0..1000, 0.0_f32..12.)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(plotters::style::colors::BLUE.mix(0.1))
            .light_line_style(plotters::style::colors::BLUE.mix(0.05))
            .axis_style(ShapeStyle::from(plotters::style::colors::BLUE.mix(0.45)).stroke_width(1))
            .y_labels(10)
            .y_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&plotters::style::colors::BLUE.mix(0.65))
                    .transform(FontTransform::Rotate90),
            )
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.positions
                        .iter()
                        .enumerate()
                        .map(|(idx, position)| (idx as i32, *position)),
                    0.0_f32,
                    PLOT_LINE_COLOR.mix(0.175),
                )
                .border_style(ShapeStyle::from(PLOT_LINE_COLOR).stroke_width(2)),
            )
            .expect("failed to draw chart data");
    }
}
