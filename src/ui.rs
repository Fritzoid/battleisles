use bevy::prelude::*; 

pub struct Ui {
}

impl Ui {
    fn new() -> Ui {
        Ui {}
    }

    pub fn setup_ui(commands: &mut Commands) {
        commands.spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                border: UiRect::all(Val::Px(5.0)),
                ..Default::default()
            },
            border_color: BorderColor(Color::rgb(0.5, 0.5, 0.5)),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    width: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(1.0, 1.0, 1.0)),
                z_index: ZIndex::Global(1),
                ..Default::default()
            });
        });
    }
}