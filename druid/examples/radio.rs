// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::widget::{Flex, MainAxisAlignment, Padding, Radio, RadioGroup};
use druid::{AppLauncher, Data, LocalizedString, Widget, WindowDesc};

#[derive(Clone, PartialEq, Data)]
enum Choice {
    A,
    B,
    C,
    D,
}

fn build_widget() -> impl Widget<Choice> {
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Padding::new(5.0, Radio::new("First choice", Choice::A)))
        .with_child(Padding::new(5.0, Radio::new("Second choice", Choice::B)))
        .with_child(Padding::new(5.0, Radio::new("Worst choice", Choice::C)))
        .with_child(Padding::new(5.0, Radio::new("Best choice", Choice::D)))
        .with_spacer(32.0)
        .with_child(RadioGroup::new(vec![
            ("Good times", Choice::A),
            ("Ergonomics", Choice::B),
            ("No fourth choice!", Choice::C),
        ]))
}

fn main() {
    let window = WindowDesc::new(build_widget).title(
        LocalizedString::new("radio-demo-window-title").with_placeholder("So many choices!"),
    );
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(Choice::A)
        .expect("launch failed");
}
